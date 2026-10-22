# Releasing Trickery

The source build requires Rust 1.85 or newer. Version 0.1.4 corrects the old
1.75 metadata: the published 0.1.3 lockfile already required Rust 1.83 through
ICU, and the patched dependency graph requires Rust 1.85. Prebuilt binaries
do not require a Rust installation. CLI behavior is unchanged in 0.1.4.

## Prepare and validate

1. Bump `Cargo.toml`, then run `cargo update --workspace`. Check that
   `Cargo.lock` changes only the Trickery package version when dependencies
   were already reviewed and patched.
2. Run formatting, Clippy, `cargo test --locked`, and `cargo build --locked`.
   Run `python -m unittest discover -s scripts/tests` for archive validation.
3. Open a PR. CI checks the locked graph, minimum compiler, and native builds.
   Merge only after all checks pass.
4. Run **Native binaries** manually with the exact reviewed release commit
   as `source_sha`. PR artifacts are validation evidence; release assets
   must identify the final release commit.

The native workflow uses Rust 1.99.0 and `cargo build --release --locked`.
It does not publish a crate, create a tag, or upload release assets.

| Archive target | Native build/test host | Runtime evidence |
| --- | --- | --- |
| x86_64-unknown-linux-gnu | Ubuntu 22.04 x86_64 | Actual required GLIBC symbols in build JSON |
| aarch64-unknown-linux-gnu | Ubuntu 24.04 arm64 | Actual required GLIBC symbols in build JSON |
| aarch64-apple-darwin | macOS 14 arm64 | macOS deployment target 11.0; actual Mach-O minimum recorded |
| x86_64-apple-darwin | macOS 15 Intel | macOS deployment target 11.0; actual Mach-O minimum recorded |
| x86_64-pc-windows-msvc | Windows Server 2022 x86_64 | Native host and target recorded |

Linux archives use GNU libc and are not universal Linux or musl binaries.
Check the per-archive JSON for the required GLIBC version. Other OS versions
and devices are not implied to have been tested by the build-host matrix.

## Inspect and publish

1. Download all five native workflow artifacts from the final source commit.
   Verify each archive checksum, `source_sha`, `cargo_lock_sha256`, target,
   and binary version. The packager extracts the actual archive and runs
   its binary with `--version` and `--help` on the native host.
2. Combine archive checksum lines into `SHA256SUMS`. Keep the per-target JSON
   files with the release for compiler, source, lockfile, ABI, and smoke-test
   evidence. Each archive also contains its build JSON and `Cargo.lock`.
3. Inspect `cargo package --locked` output before publication. Confirm the
   packaged lockfile contains the reviewed versions, not a freshly resolved
   dependency graph.
4. Create the matching numeric tag and a draft GitHub release on the exact
   reviewed commit. Attach the verified archives, JSON files and SHA256SUMS.
   Publish only when every artifact is available and verified.
5. Publishing the GitHub release starts the existing crates.io publisher.
   It checks tag/version agreement, runs tests with `--locked`, and publishes
   with `--locked` using the existing `CARGO_REGISTRY_TOKEN` secret.
6. Verify the public crates.io version/checksum and GitHub assets before
   reporting the release complete. For reproducible source installs, use
   `cargo install --locked trickery --version <version>`.

If publication has an uncertain outcome, inspect the registry and workflow
before retrying. Published crate versions are immutable.
