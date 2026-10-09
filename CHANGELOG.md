# Changelog

## 0.3.1

- Maintenance release. Nothing changes in the plugin itself.

## 0.3.0

Git Client is a program of its own now, instead of a sandboxed WebAssembly component.
Sicompass starts it and talks to it, one per tab, and it runs with your rights, so
its entry in the Store says what it does before you install it, and installing it is
your approval.

- It runs your `git`, found on your `PATH` or in `~/.local/bin`. Fetch, pull and push still run in the background.
- One build for each of Linux (x86_64 and arm64, static), macOS (Apple Silicon and
  Intel) and Windows.
- Needs a Sicompass that runs plugin programs. An older Sicompass keeps the 0.2
  version it has.
