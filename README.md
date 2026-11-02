# Flamework Plugin Template

A Flamework v2 (beta) plugin, published as an npm package, with a game that installs it and runs
its tests in Roblox Studio, both built with `rbxtsc-fw`: the roblox-ts compiler rewritten in Rust
([rbxts-rs](https://github.com/rbxts-rs/rbxts-rs), TypeScript 7) with Flamework's transformer
compiled in ([flamework-rs](https://github.com/rbxts-rs/flamework-rs)). The example plugin,
`@your-scope/player-events`, tells providers about players joining and leaving and gives them a
roster of the players in the game; replace it with yours.

**Windows x64 only for now:** flamework-rs releases `rbxtsc-fw` for Windows x64 alone. On another
platform, build it from source (below), which is untested there.

## Install

You need [bun](https://bun.sh) and [Rokit](https://github.com/rojo-rbx/rokit), plus the Rojo plugin
for Studio.

```sh
git clone <this repository> my-plugin
cd my-plugin
rokit install
bun install
bun run build
rojo serve
```

`rokit install` installs the tools in `rokit.toml`: Rojo 7.7, Lune and `rbxtsc-fw` (the first
time, it asks you to trust `rbxts-rs/flamework-rs`, the repository `rbxtsc-fw` is released from).
Open a place in Studio (a new Baseplate will do), connect from the Rojo plugin and press Play.
The output prints `Welcome, <your name>: 1 in the game` from the server, and `<your name> is in
the game` from the client.

### Without Rokit: build `rbxtsc-fw` from source

With [Rust](https://rustup.rs) installed, Cargo builds `rbxtsc-fw` from the flamework-rs commit
that the release was made from:

```sh
rustup toolchain install 1.99.1 --profile minimal
cargo +1.99.1 install --locked --git https://github.com/rbxts-rs/flamework-rs --rev <hash> rbxtsc_fw
```

`<hash>` is the commit of flamework-rs's `v0.1.0` tag. `1.99.1` is the Rust version flamework-rs
pins (its `rust-toolchain.toml`): `cargo install --git` builds with the toolchain of the folder it
runs in, so an older default fails. The build takes a while (release profile, fat LTO). It puts
`rbxtsc-fw` in Cargo's `bin` folder, which rustup adds to `PATH`; add `--root <folder>` to keep one
folder per version. Install Rojo 7.7 your own way, and remove the `rbxtsc-fw` line from
`rokit.toml` if you use Rokit for Rojo only.

### Versions

`rbxtsc-fw` has Flamework's transformer built in, so its version goes with the
`@flamework-experimental/*` packages' version:

| `rbxtsc-fw` (flamework-rs) | `@flamework-experimental/*` |
| -------------------------- | --------------------------- |
| 0.1.0                      | 2.0.0-beta.0                |

Upgrade them together: the `rbxtsc-fw` line in `rokit.toml`, and every `@flamework-experimental/*`
package in `package.json` (all share one version). `rbxtsc-fw --build-info` prints the Flamework
version it has (its `plugin` line).

## Editor

`rbxtsc-fw` is TypeScript 7, and so is the editor here: VS Code's built-in TypeScript 5 service
misreads these tsconfigs (`module` `preserve`, no `baseUrl`) and the TypeScript 7 typings of
`@rbxts/compiler-types` (iterables with three type parameters), and reports errors the build does
not have.

- Install the recommended extensions (`.vscode/extensions.json`): **TypeScript 7**
  (`TypeScriptTeam.native-preview`), Rojo and Prettier.
- `.vscode/settings.json` turns TypeScript 7 on (`js/ts.experimental.useTsgo`) and points it at the
  project's TypeScript 7, the `typescript-7` package (`js/ts.tsdk.path`, installed by
  `bun install`). VS Code asks once whether to trust the workspace and use its version; say yes.
  Without the extension, VS Code falls back to its TypeScript 5 service and its false errors.
- **TypeScript 5 is here for `flamework-test` alone.** The `typescript` package is 5.9.3 because
  `flamework-test` (2.0.0-beta.0) reads `flamework.config.json` through
  `@flamework-experimental/transformer`, which needs TypeScript 5's API. Nothing else uses it: the
  build is `rbxtsc-fw` and the editor TypeScript 7. Once a `flamework-test` release no longer needs
  it, remove it and make `typescript-7` the `typescript` package again.
- The roblox-ts extension (`roblox-ts.vscode-roblox-ts`) is a plugin of the TypeScript 5 service and
  starts the npm `rbxtsc`, so it does nothing here. Use `bun run watch` in a terminal.
- From the command line, `bun run typecheck` checks the package and the game with the same
  TypeScript 7 (`tsc --noEmit --skipLibCheck`; `@rbxts/types` has errors of its own that the build
  never reports, hence `--skipLibCheck`). The game's check reads the package's typings, so build
  the package first (`bun run build:package`).

## Layout

```
package/                    the plugin: the npm package you publish
  src/index.ts              createPlayerEventsPlugin(options), its interfaces and PlayerRoster
  package.json              its name, version and files; core as a peer dependency
  tsconfig.json             a package build: typings, with Flamework on
src/                        a game that uses the plugin and tests it; never published
  server/runtime.server.ts  includes the plugin, as a game that installed it would
  client/runtime.client.ts  the same on the client
  server/services/          a provider that uses the plugin
  client/controllers/       a controller that uses the plugin
  server/tests/             tests of the game's use of the plugin
  shared/tests/             tests of the plugin itself, run on the server and on the client
  shared/fixtures/          helpers and providers the tests use, outside every registered folder
tests/
  place.rbxlx               the place the tests run in
scripts/
  test.mjs                  what `bun run test` runs
```

The root is a bun workspace with `package/` in it. The game depends on
`"@your-scope/player-events": "workspace:*"`, so its `node_modules` links to `package/`, and the game
always runs the plugin's latest build.

- `bun run build` builds the package, then the game. `bun run build:package` builds the package
  alone.
- `bun run watch:package` and `bun run watch` rebuild the package and the game as you edit; run
  each in a terminal of its own.
- `bun run place` builds `place.rbxl`, and `bun run format` formats `src/` and `package/src/`.

## Tests

```sh
bun run test
```

This builds the package, then the game with the `testing` scope, and lays the build over
`tests/place.rbxlx`. It then runs the tests in Studio, on the server and on the client, and prints
each realm's passed, failed and skipped tests. Last, whatever the result, it rebuilds the game's
`out/` without the `testing` scope.

- **Needs:** Studio with "MCP server" turned on in its Assistant settings, and
  [Lune](https://lune-org.github.io/docs) (in `rokit.toml`).
- **Studio's window:** on Windows the run opens Studio on a hidden desktop of its own, so nothing
  pops up and your focus stays where it is; `bun run test --show-windows` opens it where you can
  watch. One test run uses Studio at a time on the machine: while another project's run or window
  holds it, a run waits, up to 300 seconds.
- **Testing the plugin itself:** `src/shared/tests/player-events.ts` ignites a module of its own per
  test, with the plugin and the fixtures it needs, and extinguishes it afterwards. So each test
  gets a fresh plugin, with the options it wants.
- **One section:** `bun run test --sections player-events` runs one section, in each realm that has
  it.
- **Skips:** a test calls `skip(reason)` when something known only at run time rules it out, and
  `test.skip(name, body)` parks one. Each skip is listed with its reason, and fails the run only
  under `bun run test --fail-on-skip`.
- **Unattended runs:** `bun run test --keep-awake` keeps the display on while it runs. A sleeping
  display stops RenderStepped, which fails client tests that wait for a frame.
- **Other builds:** keep the `testing` scope out of `.env` and `.env.local`, which every build
  reads. A run stopped with Ctrl+C leaves the test build in `out/`: run `bun run build` before
  `rojo serve`.

## Making it yours

1. **Name it.** Pick a scoped name, `@scope/name`: roblox-ts and Flamework treat only a scoped name
   as a package. Set it in `package/package.json`, then replace `@your-scope/player-events` in the
   root `package.json` (the dependency), `tsconfig.json` (`typeRoots` lists the scope, and `paths`),
   `default.project.json` (the Rojo mapping) and the imports in `src/`. Run `bun install` afterwards.
2. **Write the plugin** in `package/src/`. Read the rules it has to follow first:
   `.claude/rules/plugin.md`, or guide 08 (below). A published plugin cannot use path macros such as
   `registerProviders("src/...")`; it registers its classes with `target.registerClassProvider`.
3. **Replace the example's use** in `src/server/services/` and `src/client/controllers/`, and its
   tests in `src/server/tests/` and `src/shared/tests/`. Every folder an entry point registers must
   keep at least one module, or lose its line in the entry point: the build warns about each one
   that does not.
4. **Rename the project:** `name` in the root `package.json` and in `default.project.json`, and the
   titles of this README and of `CLAUDE.md`.

## Publishing

```sh
cd package
bun publish
```

Set `version` in `package/package.json` first. `prepublishOnly` builds the package with
`rbxtsc-fw`, and the package publishes `out/` and `flamework.build`. A game's build reads the
package's ids from `flamework.build`, so it has to ship.

A game then installs it with `bun add @your-scope/player-events`, adds `node_modules/@your-scope` to
`typeRoots` in its `tsconfig.json`, maps `node_modules/@your-scope` in its Rojo project next to
`@flamework-experimental`, and includes the plugin in each realm's module. Its core has to be the
version the peer dependency names.

## Docs

The Flamework guides for the installed version are in
`node_modules/@flamework-experimental/core/docs/guide/`. For a plugin: `08-plugins.md`, and
`09-project-structure.md` and `07-macros.md` ("Paths") for what a published package may not do.
They are written for roblox-ts 3.0.0 with Flamework's npm transformer; here `rbxtsc-fw` replaces
both (no `roblox-ts` to install; the transformer and TypeScript 5 that `bun install` brings are
for `flamework-test` alone, see "Editor"), and the tsconfigs are in TypeScript 7's form. The
Flamework website documents v1, most of which no longer applies.

`CLAUDE.md` and `.claude/rules/` are the instructions for Claude Code. `CLAUDE.md` loads
Flamework's own, which ship in core next to the guides (`docs/ai/`), and the rules add only what
is specific to this template.
