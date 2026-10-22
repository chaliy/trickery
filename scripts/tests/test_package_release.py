import importlib.util
import json
import tarfile
import tempfile
import unittest
import zipfile
from pathlib import Path
from unittest.mock import patch


spec = importlib.util.spec_from_file_location("package_release", Path(__file__).parents[1] / "package_release.py")
release = importlib.util.module_from_spec(spec)
spec.loader.exec_module(release)


class PackageReleaseTests(unittest.TestCase):
    def test_archives_include_binary_lock_provenance_and_matching_checksum(self):
        for target, binary_name in [("x86_64-pc-windows-msvc", "trickery.exe"), ("x86_64-unknown-linux-gnu", "trickery"), ("aarch64-apple-darwin", "trickery")]:
            with self.subTest(target=target), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                (root / "Cargo.toml").write_text('[package]\nversion = "0.1.4"\n')
                (root / "Cargo.lock").write_text('[[package]]\nname = "trickery"\nversion = "0.1.4"\n')
                (root / "LICENSE").write_text("MIT")
                (root / "README.md").write_text("readme")
                binary = root / binary_name
                binary.write_bytes(b"native fixture")

                def run(*args):
                    if args[-1] == "--version":
                        return "trickery 0.1.4"
                    if args[0] == "readelf":
                        return "Name: GLIBC_2.34 Name: GLIBC_2.9"
                    if args[0] == "otool":
                        return "cmd LC_LOAD_DYLIB\n current version 1208.0\nLoad command 1\ncmd LC_BUILD_VERSION\n minos 11.0\n sdk 14.0"
                    return "fixture"

                with patch.object(release, "run", side_effect=run):
                    archive, checksum, manifest = release.package(root, binary, target, root / "dist")
                self.assertEqual(checksum.read_text(), f"{release.sha256(archive)}  {archive.name}\n")
                evidence = json.loads(manifest.read_text())
                self.assertEqual(evidence["cargo_lock_sha256"], release.sha256(root / "Cargo.lock"))
                self.assertEqual(evidence["binary_sha256"], release.sha256(binary))
                self.assertEqual(evidence["smoke"]["version"], "trickery 0.1.4")
                if "linux" in target:
                    self.assertEqual(evidence["required_glibc"], "2.34")
                    with tarfile.open(archive) as handle:
                        names = handle.getnames()
                elif "windows" in target:
                    with zipfile.ZipFile(archive) as handle:
                        names = handle.namelist()
                else:
                    self.assertEqual(evidence["minimum_macos"], "11.0")
                    with tarfile.open(archive) as handle:
                        names = handle.getnames()
                self.assertEqual({Path(name).name for name in names}, {binary_name, "LICENSE", "README.md", "Cargo.lock", "build.json"})

    def test_mismatched_binary_version_is_not_packaged(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "Cargo.toml").write_text('[package]\nversion = "0.1.4"\n')
            binary = root / "trickery"
            binary.touch()
            with patch.object(release, "run", return_value="trickery 0.1.3"):
                with self.assertRaisesRegex(ValueError, "does not match"):
                    release.package(root, binary, "aarch64-apple-darwin", root / "dist")
            self.assertFalse((root / "dist").exists())

    def test_lock_differing_from_committed_bytes_is_not_packaged(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "Cargo.toml").write_text('[package]\nversion = "0.1.4"\n')
            (root / "Cargo.lock").write_text('[[package]]\nname = "trickery"\nversion = "0.1.4"\n')
            (root / "LICENSE").write_text("MIT")
            (root / "README.md").write_text("readme")
            binary = root / "trickery.exe"
            binary.write_bytes(b"native fixture")

            def run(*args):
                if args[-1] == "--version":
                    return "trickery 0.1.4"
                if args[-1] == "HEAD:Cargo.lock":
                    return "committed-lock-blob"
                if "hash-object" in args:
                    return "different-working-lock-blob"
                return "fixture"

            with patch.object(release, "run", side_effect=run):
                with self.assertRaisesRegex(ValueError, "differs from committed"):
                    release.package(root, binary, "x86_64-pc-windows-msvc", root / "dist")
            self.assertFalse((root / "dist").exists())


if __name__ == "__main__":
    unittest.main()
