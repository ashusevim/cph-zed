# Publishing CPH for Zed

**Short version: this extension cannot be published to the public registry. Install it
as a dev extension instead.**

## Why publishing is not possible

Zed's [publishing prerequisites][prereq] list which kinds of extensions are accepted:

> Extensions can provide: Languages, Debuggers, Themes, Icon Themes, Snippets, MCP Servers

and explicitly rule out the category this extension falls into:

> ### Agent Server and Slash Command Extensions
> Agent server and slash command extensions have been deprecated and submissions will
> no longer be accepted.

`cph-zed` provides nothing but slash commands (`cph-fetch`, `cph-run`, `cph-test`,
`cph-server`). There is no grammar, language server, theme, icon theme, snippet, or MCP
server. A pull request against `zed-industries/extensions` will be closed unreviewed, per
the registry's own policy:

> Why was my PR closed? — It severely violated the publishing prerequisites.

## Install as a dev extension

This is a first-class supported workflow and needs no review.

1. Open the extensions pane (`zed: extensions`).
2. Click **Install Dev Extension**.
3. Select this directory.

Zed compiles it for you with `cargo build --target wasm32-wasip2`. Requirements:

- Rust installed via [rustup](https://www.rust-lang.org/tools/install). If Zed was
  built against a toolchain that is not rustup-managed, install the target yourself:
  ```bash
  rustup target add wasm32-wasip2
  ```
- No `wasi-sdk` needed — that is only for extensions that ship Tree-sitter grammars.

To rebuild after a code change, reopen the extensions pane and reinstall, or run:

```bash
cargo build --target wasm32-wasip2
```

Debug output from the extension goes to Zed's log (`zed: open log`). For live
`println!` output, launch Zed in the foreground:

```bash
zeditor --foreground
```

## Optional: how the original works

The upstream [agrawal-d/cph][orig] is a VS Code extension. It listens on port 27121
inside the VS Code extension host and writes files directly into your workspace, which
is why it can fetch problems with one click.

A Zed extension cannot replicate that: the WASM sandbox has no socket API, and only the
extension's own work directory is writable. Use `/cph-fetch` with pasted JSON instead.

[orig]: https://github.com/agrawal-d/cph
[prereq]: https://zed.dev/docs/extensions/publishing/prerequisites
