# Project Instructions

gitclient-plugin-sicompass was split out of the
[sicompass](https://github.com/friendlyflow/sicompass) workspace, and its git
history before that point is the history of `lib/lib_gitclient` there. Work on it is
usually driven from a sicompass checkout next to this one (`../sicompass`),
whose `/commit-and-push`, `/release`, `/sync` and `/update-cargo` take this
repo's name as their first argument and then follow the skills in this repo's
`.claude/skills/`.

It is a sicompass **plugin process**: a program (`src/main.rs`) built with the
SDK's `plugin` feature, which sicompass starts and talks to over its stdin and
stdout. It runs with the user's rights. The Store installs it from this repo's
GitHub releases, one build per platform. The plugin platform is described in
`../sicompass/docs/plugin-platform.md`.

- `plugin.json` is the manifest. Its `name` is `gitclient` and its
  `displayName` `git client` is the settings section (`gitAutofetchMinutes`,
  the same key the built-in had, so a saved value carries over). It asks for
  `"filesystem": ["/"]` (the folder listing picks the repository) and
  `"process": ["git"]`. They are what the plugin declares it does, shown to
  the user before install.
- `locales/<lang>.ftl`, every id prefixed `gitclient-`, in all four
  languages. A test checks every id is used (in the code or the manifest) and
  every one the code asks for exists.

## How it runs

- **git** is an ordinary child process (`src/git.rs`), started with
  `sicompass_sdk::plugin::command` (no console window on Windows), with
  `GIT_DIR` and its kind removed from the environment. It is `git` from `PATH`,
  then from `~/.local/bin`. There is no `gitBinary` setting.
- **fetch, pull and push** run on a thread (`worker::Network`) so the UI never
  waits on the network. Every call from the app has a 10-second deadline, so
  nothing slow may run inside one.
- **The watcher** that notices commits made elsewhere stats `.git` from
  `poll`, at most once a second, instead of from a thread.
- **Strings** come from the app (`host::translate`), which holds this plugin's
  `locales/`. The unit tests run outside sicompass and read the English bundle
  instead (`src/localize.rs`).
- Opening a repository moves the plugin without a navigation call. The SDK's
  runtime tells the app after every call that moved it, so nothing here has to.
- stdout is the channel to the app. `println!` lands in stderr, the app's log.

## Environment (Nix)

The toolchain comes from the flake dev shell in [flake.nix](flake.nix): Rust
from rust-overlay with this computer's plugin target (static musl on Linux,
which nixpkgs' rustc has no `std` for), `jq` and `git`. Nothing is installed
system-wide.

- **Check once per session**, then stick with the answer: `command -v cargo`.
  - Non-empty: the shell is inside `nix develop`, so run `cargo ...` directly.
  - Empty: prefix every toolchain command with `nix develop -c`.
- `nix develop -c <cmd>` prints a `warning: Git tree ... is dirty` line on
  stderr first. That warning is noise, not a failure.
- Evaluate the flake through `git+file://$PWD`, never a plain path (a plain path
  copies `target/` into the store and hangs), and always under `timeout`.
- The version lives in `plugin.json` and in `[package] version` in `Cargo.toml`.
  Bump both together.

## Generated files that are committed

- `THIRD-PARTY-LICENSES.html`: `cargo about generate about.hbs -o
  THIRD-PARTY-LICENSES.html` (cargo-about 0.9.2, the version the `licenses.yml`
  workflow pins). Regenerate and commit it with any dependency change. The
  workflow fails if it drifts.

## Code Style

Follow standard Rust idioms. Use `#[allow(...)]` sparingly and only when
justified. In `README.md`, do not use em dashes or semicolons. Use commas
instead, or split into separate sentences.

## Testing

- After implementing changes, always run the tests before finishing:
  `cargo test`, and `./scripts/release-plugin.sh --dry-run`, which also builds
  this computer's release and verifies it the way the Store will.
- When adding new code, write or update tests.
- If tests fail, fix the code. Never leave a task with failing tests.

## Test Integrity

- Never remove or weaken test assertions to make a failing test pass. Fix the
  code instead.
- If a test itself is genuinely wrong and needs changing, **ask the user
  first** before modifying it.

## Releasing

A release is a `vX.Y.Z` tag on `main`, equal to `plugin.json`'s version. See
`.claude/skills/release/SKILL.md`. Before tagging, run
`nix develop -c ./scripts/release-plugin.sh --dry-run` (needs the
`sicompass-plugin` tool: `cargo install --git
https://github.com/friendlyflow/sicompass-plugin-sdk sicompass-plugin`). The
release workflow signs with the `PLUGIN_SIGNING_KEY` secret and checks it
against the `PLUGIN_PUBLIC_KEY` variable, the key the sicompass store list
names. The secret key file is `~/.config/sicompass/plugin-keys/gitclient.key`
on the maintainer's machine. Never print, copy or commit it.

The SDK comes from crates.io (the source is `../sicompass-plugin-sdk`). The
commented-out `[patch]` in `Cargo.toml` is for working on them together, and
stays commented on main.

A release has one archive per platform. The release workflow builds them on
five runners (Linux x86_64 and arm64 as static musl, macOS arm64 and x86_64,
Windows x86_64), then packs, signs and verifies them in one job.
