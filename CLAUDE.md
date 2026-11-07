# Flamework Template

A Roblox game in roblox-ts on Flamework v2, built with rbxts-rs (`rbxtsc-fw`). It started from the
Flamework template, whose coin example (press F to spawn a coin, touch it to collect it) is there
to be replaced.

## Flamework

Flamework's own instructions ship in core, so they match the installed version. This line loads
them (after `bun install`; without `node_modules` it loads nothing):

@node_modules/@flamework-experimental/core/docs/ai/flamework.md

The guides they point to are for the installed version. Most of their install commands use npm;
use bun here. They are written for roblox-ts 3.0.0 with Flamework's npm transformer: here
`rbxtsc-fw` replaces `rbxtsc` and the transformer (see [Stack](#stack)). Never install
`roblox-ts`, TypeScript 5 or `@flamework-experimental/transformer`.

## Stack

- `rbxtsc-fw` compiles `src/` to Luau in `out/`. It is rbxts-rs, roblox-ts rewritten in Rust on
  TypeScript 7.1, with Flamework's transformer compiled in, so it takes `rbxtsc`'s arguments and
  writes what roblox-ts 3.0.0 writes, apart from the differences under Gotchas. It is the
  project's own, in `compiler/`: a Cargo package pinning rbxts-rs and flamework-rs by tag, and the
  launcher `compiler/rbxtsc.mjs`, which runs `compiler/bin/<target triple>/rbxtsc-fw`. The first
  build downloads the release binary (Windows x64) or builds it with Rust (README, "The compiler";
  `compiler/README.md`). Not from npm, not from Rokit; run it through the launcher, never a global
  `rbxtsc-fw`.
- `tsconfig.json` is in TypeScript 7's form: `module` `preserve`, `moduleResolution` `bundler`,
  `types` listing the global typings (`"types", "compiler-types"`), `paths` instead of `baseUrl`,
  no `downlevelIteration`. `@rbxts/compiler-types` is the TypeScript 7 build (iterables with three
  type parameters), installed from its GitHub release `.tgz`.
- The `typescript` dev dependency is TypeScript 7, for the editor and `bun run typecheck` (README,
  "Editor").
- Rojo 7.7 (`rokit.toml`) builds the place from `default.project.json`.
- **Flamework v2 beta:** `@flamework-experimental/core`, `components`, `networking` and `testing`,
  all pinned exactly at 2.0.0-beta.1. Every package shares one version; upgrade them together, and
  `compiler/` with them: its `rbxtsc-fw` 0.1.2 has Flamework 2.0.0-beta.1's transformer built in
  (README, "Versions"). The tsconfig's `plugins` entry `@flamework-experimental/transformer` only
  turns Flamework on in `rbxtsc-fw`; there is no npm package behind it.
  - `testing` is in every build, not only test builds: both entry points include its
    `TestingPlugin`, which stays inert without the `testing` scope. A mismatched version breaks
    the game too.
- Package manager: bun, one lockfile (`bun.lock`).

## Commands

- `bun install`.
- `bun run build` runs `rbxtsc-fw` through the launcher (`node compiler/rbxtsc.mjs build`), the
  check described under "Building" in the imported flamework.md.
- `bun run watch` runs `rbxtsc-fw -w` through the launcher, which rebuilds on change.
- `bun run serve` runs `rojo serve` to sync into Studio, and `bun run place` builds `place.rbxl`.
  Build first: the project maps `out/` and `include/`.
- `bun run test` runs the tests in Studio; see [Tests](#tests).
- `bun run format` runs Prettier on `src/`: tabs, a width of 100, trailing commas.
- Add packages with `bun add <name>`, or `bun add -d <name>` for build tools. Add a
  `@flamework-experimental/*` package with `bun add --exact <name>@2.0.0-beta.1`, the version the
  others have. It needs no mapping: `default.project.json` maps the whole scope.

## Long-running commands: don't poll

- Run a long command (a test run, Studio, a long build) in the background, and let its completion
  notification wake you. Don't keep reading its output, or sleeping and looking again.
- No polling: it costs a lot of usage. Two health checks of your own are allowed if needed
  (reading its output, a status command): one near the start (did it start properly) and one
  midway. More only when the user asks for them. The user asking how it is going is not one of
  yours.
- Next to a long command, start a watcher in the background that ends, and so wakes you, when
  something goes wrong: the first failure line, output silent for minutes (a hang), or the command
  dying without finishing. One background watcher is not polling; checking again and again
  yourself is.
- Commands that finish in a minute or two need none of this: run them in the foreground, with a
  timeout.

## Where things live

| Path                           | Realm  | Holds                                                               |
| ------------------------------ | ------ | ------------------------------------------------------------------- |
| `src/server/runtime.server.ts` | server | the entry point: builds and ignites the server's module             |
| `src/server/services/`         | server | `@Provider` classes, registered by `registerProviders`              |
| `src/server/components/`       | server | `@Component` classes, registered by `ComponentPlugin.fromPath`      |
| `src/server/network.ts`        | server | the server's `Events` and `Functions`, with their middleware        |
| `src/server/middleware/`       | server | networking middleware                                               |
| `src/client/runtime.client.ts` | client | the entry point: builds and ignites the client's module             |
| `src/client/controllers/`      | client | `@Provider` classes, registered by `registerProviders`              |
| `src/client/components/`       | client | `@Component` classes, registered by `ComponentPlugin.fromPath`      |
| `src/client/network.ts`        | client | the client's `Events` and `Functions`                               |
| `src/shared/network.ts`        | both   | the network declarations: `GlobalEvents`, `GlobalFunctions`         |
| `src/shared/`                  | both   | plain modules both realms import, such as `tags.ts` and `levels.ts` |

- In the place: `src/server` is `ServerScriptService.TS`, `src/client` is
  `StarterPlayer.StarterPlayerScripts.TS`, and `src/shared` is `ReplicatedStorage.TS`.
- Components for both realms go in `src/shared/components/`, registered from both entry points.
- `out/`, `include/` and `flamework.build` are git-ignored; `flamework.config.json` is committed.

## Tests

Read `node_modules/@flamework-experimental/core/docs/ai/testing.md` before you write or run tests;
`.claude/rules/testing.md` adds what this template does.

- `bun run test` (`scripts/test.mjs`) builds with `FLAMEWORK_SCOPES=testing` and makes `test.rbxl`.
  `flamework-test` then lays that over `tests/place.rbxlx` and runs the sections in Studio, on the
  server and then on the client. Extra arguments go to `flamework-test`. It needs Studio's "MCP
  server" setting on, and `lune`.
- Tests live in `src/server/tests`, `src/client/tests` and `src/shared/tests` (both realms);
  `src/server/tests/players.ts` is a plain module of helpers beside them. The entry points register
  those folders only under the `testing` scope.
- Whether the tests pass or fail, `bun run test` ends by rebuilding `out/` with `FLAMEWORK_SCOPES`
  set to nothing, so `rojo serve` and `bun run place` never ship the test host. Never put the scope
  in `.env` or `.env.local`: every other build reads them. A run stopped with Ctrl+C skips the
  rebuild, so `out/` keeps the test build: run `bun run build`. Every run leaves `test.rbxl` and
  `test.patched.rbxl` behind; they are git-ignored.
- **How much to run:** as "How much to test" in the imported flamework.md says. A Studio run of
  one feature is `bun run test --sections <section>` (`coin`, `coin-service`, `throttle`,
  `coin-spin`, `levels`), plus `--realm` when one realm is enough. The whole `bun run test` only
  when the user asks or agrees.
- **Unattended:** give a run nobody watches `--keep-awake`: while the display sleeps,
  RenderStepped stops and `onRender` tests fail.
- **Hidden window:** on Windows the run opens Studio on a hidden desktop, so nothing shows and the
  user's focus is left alone. Add `--show-windows` only when the user wants to watch, or when a
  hidden window `never showed up on the MCP proxy`.
- **The Studio lock:** one `flamework-test` command uses Studio at a time on the machine. A run
  that finds another project's window or run holding it says whose and waits, up to 300 seconds,
  then fails naming it. Wait or ask the user; never close a window you did not open.

## Gotchas

- The editor must run TypeScript 7 (the "TypeScript 7" extension, `.vscode/settings.json`). VS
  Code's built-in TypeScript 5 service misreads `tsconfig.json` and the iterables of
  `@rbxts/compiler-types`, and reports errors the build does not have. To check from the command
  line: `bun run typecheck` (TypeScript 7, `--skipLibCheck`: `@rbxts/types` has errors of its
  own that the build never reports).
- `rbxtsc-fw` follows roblox-ts's master (68648b0, newer than the npm 3.0.0) and TypeScript 7.1.
  Next to a roblox-ts 3.0.0 build, `out/` differs in layout, not in behaviour: doc comments lose
  their `*` margins, some temporaries are gone (`local _player = player`), and the names of
  Flamework's generated locals, the order of union members and exports, and some diagnostics
  differ.
- `default.project.json` maps all of `node_modules/@flamework-experimental` in one line, so the
  place gets more than code: core's docs as Folders, `core.docs` holding the empty `core.docs.ai`
  and `core.docs.guide`. All expected; nothing reads them.
- The build is not incremental (`tsconfig.json` sets no `incremental`), so an upgraded `rbxtsc-fw`
  just rebuilds.
- `compiler/` is part of the project: a change to its `Cargo.toml`, `Cargo.lock`,
  `rust-toolchain.toml`, `.cargo/config.toml` or `src/` changes its config hash, and the launcher
  then builds `rbxtsc-fw` with Rust instead of downloading it (`compiler/README.md`). Don't edit
  them without being asked; never commit `compiler/bin/`.
- Line endings are LF everywhere: `.gitattributes` sets `eol=lf`, which overrides `core.autocrlf`.
