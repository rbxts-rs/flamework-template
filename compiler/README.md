# compiler/: this project's rbxtsc-fw

This folder is the project's own roblox-ts compiler, `rbxtsc-fw`: [rbxts-rs](https://github.com/rbxts-rs/rbxts-rs)'s
`rbxtsc` with Flamework compiled in ([flamework-rs](https://github.com/rbxts-rs/flamework-rs)), and any other plugins
the project adds. It is a small Cargo package and a launcher:

| File | What it is |
| --- | --- |
| `Cargo.toml`, `Cargo.lock` | the package: `rbxts_cli`, `flamework_rs` and any other plugins, pinned to exact versions; the release profile |
| `rust-toolchain.toml` | the Rust version it builds with |
| `.cargo/config.toml` | Cargo's settings for its builds: reproducible Windows binaries (`/Brepro`) |
| `src/main.rs` | `rbxts_cli::Cli::new().plugin(flamework_rs::Flamework::new()).run()`, with a `.plugin(...)` per further plugin |
| `preset.json` | the prebuilt binaries that match this exact config |
| `rbxtsc.mjs` | the launcher (Node 18 or newer, no npm dependencies) |
| `bin/<target triple>/` | the binary and its `.stamp`: not committed (`.gitignore`) |

## Running

```sh
node compiler/rbxtsc.mjs build           # as `rbxtsc-fw build`
node compiler/rbxtsc.mjs -w              # watch mode; Ctrl+C stops it
```

In `package.json`: `"build": "node compiler/rbxtsc.mjs build"`, `"watch": "node compiler/rbxtsc.mjs -w"`. Every
argument goes to `rbxtsc-fw` unchanged, except the launcher's own flags:

- `--where`: prints the binary's path, the state of its stamp, the config hash and the preset, and exits;
- `--build`: builds the binary with Rust when there is no matching one, without asking;
- `--rebuild`: builds it with Rust even when it is up to date;
- `--`: everything after it goes to `rbxtsc-fw` (the `--` itself does not), launcher flags included.

The launcher runs `bin/<target triple>/rbxtsc-fw[.exe]` when the stamp beside it holds the current config hash. It
passes the exit code through and forwards Ctrl+C and SIGTERM to it, waiting for it to end. Otherwise:

1. **Download.** If the config still hashes to `preset.json`'s `config` and the preset lists a binary for this
   platform, the launcher downloads it (a few seconds), checks its sha256 (a mismatch aborts and installs nothing),
   unzips it into `bin/<target triple>/` and stamps it. It prints one line:

   ```text
   rbxtsc.mjs: downloaded rbxtsc-fw 0.1.1 (x86_64-pc-windows-msvc) from https://github.com/rbxts-rs/flamework-rs/releases/download/v0.1.1/flamework-rs-0.1.1-windows-x64.zip, sha256 <sha256>
   ```

2. **Build.** Otherwise the binary has to be built with Rust: the config changed (a plugin added, a version moved),
   or there is no prebuilt binary for this platform. The launcher says which, then on a terminal asks
   `Build it with Rust now? This takes about 3 min. [y/N]`. Without a terminal (CI) it prints the command,
   `node compiler/rbxtsc.mjs --build`, and exits 1; `--build` or `RBXTSC_BUILD=1` builds without asking
   (`RBXTSC_INTERACTIVE=0` never asks, `=1` asks and reads the answer from stdin).

## The config hash

The hash is a SHA-256 over `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/config.toml` and every file
under `src/`, sorted by path, each as its path, a NUL, its length, a NUL and its bytes, with CRLF read as LF (so git's `autocrlf` does not
change it). The same checkout hashes the same on Windows, Linux and macOS. Nothing else in this folder counts:
editing this README or `preset.json` keeps the binary, and the `.DS_Store` and `Thumbs.db` files Finder and
Explorer leave under `src/` are not part of it.

## Building with Rust

Building needs [rustup](https://rustup.rs) (it brings `cargo`); without it the launcher says so and exits. rustup
installs the Rust version `rust-toolchain.toml` names by itself. The launcher then runs

```sh
cargo build --release --locked --target-dir <OS temp>/rbxtsc-build-<hash prefix>-<launcher process id>
```

in this folder, copies the binary into `bin/<target triple>/`, stamps it, and deletes the temporary target folder
(also when the build fails, and after Ctrl+C: the launcher waits for cargo to stop first). A build killed outright
(its terminal closed, its process ended) cannot clean up: the next build deletes the target folders of launchers that
are no longer running. A clean build takes about 3 min on a desktop PC.

- **`Cargo.lock` out of date** (a plugin added to `Cargo.toml`): the launcher says so, builds without `--locked`,
  which updates `Cargo.lock`, and asks you to commit it.
- **Plugins built for different rbxts-rs versions:** before building, the launcher checks that `rbxts_cli` and
  `rbxts_plugin` resolve to one rbxts-rs. If not it stops with `all plugins must target rbxts-rs <tag>` and the
  copies it found, instead of a long build ending in a type error.

## Adding a plugin

1. Add the plugin crate to `Cargo.toml`, pinned to an exact version, a git tag or rev (never a branch), built for
   the same rbxts-rs tag as `rbxts_cli` (and `flamework_rs`):

   ```toml
   my_plugin = { git = "https://github.com/me/my-plugin", tag = "v1.0.0" }
   ```

2. Register it in `src/main.rs`, after Flamework: `.plugin(flamework_rs::Flamework::new()).plugin(my_plugin::MyPlugin::new())`.
3. Turn it on in the tsconfig's `compilerOptions.plugins`, as the plugin's documentation says.
4. Run `node compiler/rbxtsc.mjs --build`, then commit `Cargo.toml`, `Cargo.lock` and `src/`.

A config with another plugin no longer matches the preset, so every machine (and CI) builds it once. CI can keep
`compiler/bin/` in its cache, keyed on a hash of `compiler/Cargo.toml`, `compiler/Cargo.lock`,
`compiler/rust-toolchain.toml`, `compiler/.cargo/config.toml` and `compiler/src/**`.

## Changing versions

Versions are exact everywhere: `rbxts_cli` and each plugin by git tag (or rev), crates.io packages with `=`, and
`Cargo.lock` committed. A flamework-rs release targets one rbxts-rs release and one Flamework version. To move to
another flamework-rs release:

1. Replace this folder's `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/config.toml`, `src/`, `preset.json`
   and `rbxtsc.mjs` with those of the [Flamework template](https://github.com/rbxts-rs/flamework-template)'s
   `compiler/` at the commit that moved it to that flamework-rs release, and move every `@flamework-experimental`
   package to the Flamework version that release goes with (the template's README, "Versions").
   Without other plugins that is all: the preset matches the config, so the binary is downloaded again.
2. **With other plugins:** add them again as in "Adding a plugin", each at a version built for the rbxts-rs that
   flamework-rs release targets, and run
   `node compiler/rbxtsc.mjs --build`. The lock keeps the release's versions for everything it already had and adds
   the plugins. Commit.

## preset.json

```json
{
	"bin": "rbxtsc-fw",
	"version": "0.1.1",
	"config": "<config hash>",
	"binaries": {
		"x86_64-pc-windows-msvc": { "url": "https://github.com/…/flamework-rs-0.1.1-windows-x64.zip", "sha256": "<sha256 of the zip>" }
	}
}
```

`bin` is the binary's name, `version` the release it comes from, `config` the hash of the config the binaries were
built from, and `binaries` a zip per Rust target triple, holding exactly one file named `bin` (`bin.exe` on
Windows), at an `https:` URL (the launcher refuses any other; it follows redirects, and the sha256 pins the bytes). This preset is written by the flamework-rs release that builds the binaries from this folder
(with rbxts-rs's `tools/release-binary.mjs`, whose header gives the release order). An empty `binaries` means every platform builds with Rust.
