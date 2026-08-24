# LuaLings

> Interactive exercises to learn, reinforce, and — hopefully — end up loving Lua, directly inspired by [rustlings](https://github.com/rust-lang/rustlings).

![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)

> 🚧 **Actively in development.** The CLI and TUI described in this README are the project's target design; not everything is implemented yet. This README gets updated as each piece becomes usable end to end.

## Why LuaLings?

Lua shows up in places you wouldn't expect: WoW add-on scripting, Redis `EVAL`, Nginx/OpenResty configuration, game engines... and yet almost nobody learns it "seriously" — it's usually picked up in passing, in whatever config file needs it.

LuaLings isn't just about beginners knowing Lua: it's about them actually **enjoying it**. That's why every level comes with real-world context for where it's used (not just a standalone exercise) and ends in a full project, not a snippet.

Three levels — **junior**, **mid**, **senior** — each with small, self-checked exercises and, at the end, a real project.

## Installation

Not published on [crates.io](https://crates.io) yet. For now, clone and build locally:

```bash
git clone https://github.com/your-username/lualings
cd lualings
cargo install --path .
```

Once the project reaches the distribution stage (see Project status), this will simply become:

```bash
cargo install lualings
lualings init
```

## Usage

```bash
lualings init                          # extracts the exercises into the current directory
lualings list                          # lists all exercises and their status
lualings watch                         # watch mode: detects changes and re-evaluates
lualings run exercise_name             # runs a single exercise
lualings hint exercise_name            # shows a hint
lualings hint --solution exercise_name # shows the full solution
```

The intended flow is: run `lualings watch`, edit the `.lua` file in your usual editor (there's no embedded editor in the TUI — on purpose), save, and the TUI shows you whether it passed.

## Structure

- **Junior** (8 modules) — variables, types, control flow, functions, basic tables, scope, strings, basic error handling.
- **Mid** (8 modules) — closures, metatables, OOP with metatables, coroutines, string patterns, modules, advanced error handling, basic performance.
- **Senior** (5 modules) — GC and weak tables, metaprogramming with `_ENV`, differences across Lua versions, C API (conceptual), LuaJIT FFI (conceptual).

Each level ends with a **real project** to tie everything together: a text adventure, a table-based calculator, a coroutine-driven event engine, a JSON parser, a Redis-style script, a LÖVE2D game, a C binding, and a Wireshark protocol dissector.

## Project status

Under development following an Epic-based backlog: Lua engine, exercise system, persistent progress, file watcher, CLI, TUI, content for all three levels, and finally distribution via crates.io. No published version yet — see `CHANGELOG.md` for details on what's ready so far.

## Contributing

Not open to external content contributions yet — that's on hold until the project has a stable foundation. Once that changes, this section will point to a `CONTRIBUTING.md` guide, rustlings-style.

## License

Distributed under the MIT license — see [LICENSE](./LICENSE).
