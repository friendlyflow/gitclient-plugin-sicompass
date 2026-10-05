# gitclient-plugin-sicompass

*Your repositories, in Sicompass.*

This plugin is part of [Sicompass](https://github.com/friendlyflow/sicompass), a
keyboard-first, accessibility-first way to use your entire computer.

The git client shows a repository as changes, graph, branches, stashes and
remotes. Browse to a folder inside a repository and press : to open it. The
changes section holds the commit message and four commit buttons, then every
changed file, conflicts first, and Right on a file reads its diff line by line.
Fetch, pull, push, stage, branch, merge, rebase and stash are colon commands,
and staging and committing undo with ctrl+z.

It runs the `git` you already have, with your own config, ssh agent and
credential helpers, so a repository you can push from a terminal is one you can
push from here. Nothing is fetched in the background unless you ask for it in
Settings, under git client.

The git client asks for your whole disk, to find your repositories, and to run
`git`. The Store shows that before you install it, and installing it is your
approval.

## Install

In Sicompass, open store, then programs, and press Enter on install next to
gitclient. The Store checks the release's signature before installing it, and
keeps it up to date.

## Building from source

```bash
nix develop          # the toolchain, and git
cargo test           # against throwaway repositories
cargo build --release
cp target/release/gitclient-plugin plugin
```

`./scripts/release-plugin.sh --dry-run` builds this computer's release, packs
it, and signs and verifies it with a throwaway key, the way a release is made.

## Related repositories

- [sicompass](https://github.com/friendlyflow/sicompass), the application
- [sicompass-plugin-sdk](https://github.com/friendlyflow/sicompass-plugin-sdk),
  the SDK, the plugin kit and the cloud backup library

## Community

Join the conversation on
[Discord](https://discord.com/channels/1464152138753249313/1464152139231137894).

## License

#### Open source license

If you are creating an open source application under a license compatible with
the GNU GPL license v3, you may use this project under the terms of the GPLv3.
See [LICENSE](LICENSE).

## Contributing

Contributions are welcome. Whether it is code, documentation, or feedback, your
input helps make computing more accessible for everyone.
