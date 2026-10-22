"""Package a native CLI with its exact source and locked dependency evidence."""

import argparse
import hashlib
import json
import platform
import re
import subprocess
import tarfile
import tempfile
import tomllib
import zipfile
from pathlib import Path


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(*args):
    return subprocess.check_output(args, text=True).strip()


def package(root, binary, target, output):
    version = tomllib.loads((root / "Cargo.toml").read_text())["package"]["version"]
    if not re.fullmatch(r"\d+\.\d+\.\d+", version):
        raise ValueError("release version must be a stable semantic version")
    if not re.fullmatch(r"[a-z0-9_]+(?:-[a-z0-9_]+)+", target):
        raise ValueError("invalid Rust target triple")
    expected_name = "trickery.exe" if "windows" in target else "trickery"
    if binary.name != expected_name or not binary.is_file():
        raise ValueError("native trickery binary is missing")
    version_output = run(str(binary), "--version")
    if version_output != f"trickery {version}":
        raise ValueError(f"binary version does not match manifest: {version_output}")
    run(str(binary), "--help")

    lock_path = root / "Cargo.lock"
    committed_blob = run("git", "-C", str(root), "rev-parse", "HEAD:Cargo.lock")
    working_blob = run("git", "-C", str(root), "hash-object", "--no-filters", str(lock_path))
    if committed_blob != working_blob:
        raise ValueError("build Cargo.lock differs from committed source bytes")
    lock = tomllib.loads(lock_path.read_text())
    manifest = {
        "version": version,
        "target": target,
        "source_sha": run("git", "-C", str(root), "rev-parse", "HEAD"),
        "cargo_lock_sha256": sha256(lock_path),
        "binary_sha256": sha256(binary),
        "rustc": run("rustc", "-Vv"),
        "build_host": platform.platform(),
        "build_architecture": platform.machine(),
        "smoke": {"version": version_output, "help": "passed"},
        "locked_dependencies": [
            {"name": p["name"], "version": p["version"]}
            for p in lock["package"]
        ],
    }
    if "linux" in target:
        versions = re.findall(r"GLIBC_(\d+(?:\.\d+)+)", run("readelf", "--version-info", str(binary)))
        manifest["required_glibc"] = max(versions, key=lambda v: tuple(map(int, v.split("."))))
        manifest["build_libc"] = platform.libc_ver()
    if "apple-darwin" in target:
        commands = run("otool", "-l", str(binary))
        minimum = re.search(r"cmd LC_BUILD_VERSION(?:(?!Load command).)*?minos\s+(\d+\.\d+(?:\.\d+)?)", commands, re.DOTALL)
        if not minimum:
            minimum = re.search(r"cmd LC_VERSION_MIN_MACOSX(?:(?!Load command).)*?version\s+(\d+\.\d+(?:\.\d+)?)", commands, re.DOTALL)
        if not minimum:
            raise ValueError("could not determine minimum macOS version")
        manifest["minimum_macos"] = minimum.group(1)

    output.mkdir(parents=True, exist_ok=True)
    name = f"trickery-{version}-{target}"
    manifest_path = output / f"{name}.json"
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n")
    files = [(binary, expected_name), (root / "LICENSE", "LICENSE"), (root / "README.md", "README.md"), (lock_path, "Cargo.lock"), (manifest_path, "build.json")]
    if "windows" in target:
        archive = output / f"{name}.zip"
        with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED) as handle:
            for source, destination in files:
                handle.write(source, arcname=f"{name}/{destination}")
    else:
        archive = output / f"{name}.tar.gz"
        with tarfile.open(archive, "w:gz") as handle:
            for source, destination in files:
                handle.add(source, arcname=f"{name}/{destination}")
    with tempfile.TemporaryDirectory() as directory:
        if "windows" in target:
            with zipfile.ZipFile(archive) as handle:
                handle.extractall(directory)
        else:
            with tarfile.open(archive) as handle:
                handle.extractall(directory, filter="data")
        packaged = Path(directory) / name / expected_name
        if sha256(packaged) != manifest["binary_sha256"]:
            raise ValueError("packaged binary checksum changed")
        if run(str(packaged), "--version") != version_output:
            raise ValueError("packaged binary version changed")
        run(str(packaged), "--help")
    print("Packaged native binary --version and --help passed")
    checksum = output / f"{archive.name}.sha256"
    checksum.write_text(f"{sha256(archive)}  {archive.name}\n")
    return archive, checksum, manifest_path


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--target", required=True)
    parser.add_argument("--output", type=Path, default=Path("dist"))
    args = parser.parse_args()
    root = Path(__file__).resolve().parent.parent
    for path in package(root, args.binary.resolve(), args.target, args.output.resolve()):
        print(path.name)
