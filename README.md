# Flamework Template (rbxts-rs)

A minimal Roblox game on Flamework v2 (beta), built with `rbxtsc-fw`: the roblox-ts compiler
rewritten in Rust ([rbxts-rs](https://github.com/rbxts-rs/rbxts-rs), TypeScript 7) with Flamework's
transformer compiled in ([flamework-rs](https://github.com/rbxts-rs/flamework-rs)). It has
Flamework's core, components and networking; a small coin example shows each part, to be replaced
with your game.

**Windows x64 prebuilt only for now:** flamework-rs releases `rbxtsc-fw` for Windows x64 alone. On
another platform the first build compiles it with Rust (below), which is untested there.

## Install

You need [Node](https://nodejs.org) 18 or newer, [bun](https://bun.sh) and
[Rokit](https://github.com/rojo-rbx/rokit), plus the Rojo plugin for Studio.

```sh
git clone <this repository> my-game
cd my-game
rokit install
bun install
bun run build
rojo serve
```

`rokit install` installs Rojo 7.7 (`rokit.toml`), and the first `bun run build` gets the compiler
(below). Open a place in Studio (a new Baseplate will do), connect from the Rojo plugin and press
Play. Press F to spawn a coin in front of you, then walk into it: the output prints your coins and
level.

`bun run watch` rebuilds as you edit, `bun run place` builds `place.rbxl`, and `bun run format`
formats `src/`.

### The compiler

The project carries its own `rbxtsc-fw` in `compiler/`: a small Cargo package (rbxts-rs's command
line with Flamework compiled in, every version pinned exactly) and a launcher,
`compiler/rbxtsc.mjs`, which `bun run build` and `bun run watch` run. Nothing to install for it:
the first build puts the binary in `compiler/bin/` (git-ignored).

- **Windows x64:** the launcher downloads the prebuilt `rbxtsc-fw` of the flamework-rs release that
  `compiler/` matches (a few seconds, its sha256 checked).
- **Another platform**, or a changed `compiler/` (a plugin added, a version moved): there is no
  prebuilt binary for it, so it is built with Rust. On a terminal the launcher asks first
  (`Build it with Rust now?`); `node compiler/rbxtsc.mjs --build` builds without asking. It needs
  [rustup](https://rustup.rs), which installs the Rust version `compiler/rust-toolchain.toml` names
  by itself; a clean build takes a few minutes.

`node compiler/rbxtsc.mjs --where` shows the binary, whether it is up to date, and the config hash.
`compiler/README.md` covers the launcher, adding a plugin and changing versions.

### Versions

`rbxtsc-fw` has Flamework's transformer built in, so its version goes with the
`@flamework-experimental/*` packages' version:

| `rbxtsc-fw` (flamework-rs) | `@flamework-experimental/*` |
| -------------------------- | --------------------------- |
| 0.1.0                      | 2.0.0-beta.1                |

Upgrade them together: `compiler/` (`compiler/README.md`, "Changing versions"), and every
`@flamework-experimental/*` package in `package.json` (all share one version).
`node compiler/rbxtsc.mjs --build-info` prints the Flamework version it has (its `plugin` line).

## Editor

`rbxtsc-fw` is TypeScript 7, and so is the editor here: VS Code's built-in TypeScript 5 service
misreads this `tsconfig.json` (`module` `preserve`, no `baseUrl`) and the TypeScript 7 typings of
`@rbxts/compiler-types` (iterables with three type parameters), and reports errors the build does
not have.

- Install the recommended extensions (`.vscode/extensions.json`): **TypeScript 7**
  (`TypeScriptTeam.native-preview`), Rojo and Prettier.
- `.vscode/settings.json` turns TypeScript 7 on (`js/ts.experimental.useTsgo`) and points it at the
  project's `typescript` package (`js/ts.tsdk.path`, TypeScript 7, installed by `bun install`). VS
  Code asks once whether to trust the workspace and use its version; say yes. Without the
  extension, VS Code falls back to its TypeScript 5 service and its false errors.
- The roblox-ts extension (`roblox-ts.vscode-roblox-ts`) is a plugin of the TypeScript 5 service and
  starts the npm `rbxtsc`, so it does nothing here. Use `bun run watch` in a terminal.
- From the command line, `bun run typecheck` checks the project with the same TypeScript 7
  (`tsc --noEmit --skipLibCheck`; `@rbxts/types` has errors of its own that the build never
  reports, hence `--skipLibCheck`).

## Layout

```
src/
  server/
    runtime.server.ts   entry point: registers the folders below and ignites
    services/           providers (server)
    components/         components (server)
    network.ts          the server's events and functions, with middleware
    middleware/         networking middleware
  client/
    runtime.client.ts   entry point: registers the folders below and ignites
    controllers/        providers (client)
    components/         components (client)
    network.ts          the client's events and functions
  shared/
    network.ts          the network declarations
    levels.ts, tags.ts  plain modules both realms use
```

## Docs

The Flamework guides for the installed version are in
`node_modules/@flamework-experimental/core/docs/guide/`. Start with `01-getting-started.md`. They
are written for roblox-ts 3.0.0 with Flamework's npm transformer; here `rbxtsc-fw` replaces both
(no `roblox-ts`, `typescript` 5 or `@flamework-experimental/transformer` to install), and the
tsconfig is in TypeScript 7's form. The Flamework website documents v1, most of which no longer
applies.

`CLAUDE.md` and `.claude/rules/` are the instructions for Claude Code. `CLAUDE.md` loads
Flamework's own, which ship in core next to the guides (`docs/ai/`), and the rules add only what
is specific to this template.

## Renaming the project

1. Set `name` in `package.json` and in `default.project.json`.
2. Change the title of this README and of `CLAUDE.md`, and the first paragraph of `CLAUDE.md`.
3. Delete the coin example: `src/*/components/coin*.ts`, `src/server/services/coin-service.ts`,
   `src/client/controllers/coin-controller.ts`, `src/shared/levels.ts`, and its members in
   `src/*/network.ts` and `src/shared/tags.ts`. Every folder an entry point registers must keep at
   least one module, or lose its line in the entry point: the build warns about each one that does
   not.
