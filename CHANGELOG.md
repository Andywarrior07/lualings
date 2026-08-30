# Changelog

All notable changes to this project will be documented in this file.


The format is based on [Keep a Changelog](https://keepchangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Basic `.lua` file change detection via `notify` (`watcher::watch`), filtering to `.lua` files
and running continuously without needing to be restarted after each event. Debounce and active-exercise
filtering are separate, not-yet-scheduled tasks.
- Debounce for `watcher::watch`: raw filesystem events for the same `.lua` path arriving within
`DEFAULT_DEBOUNCE_WINDOW` (300ms, a conservative starting point subject to adjustment) now collapse
into a single logical `ChangeEvent`, so editors that emit several raw events per save (e.g. Helix's
default atomic save) no longer trigger multiple re-evaluations for one save.
- Active-exercise filtering for `watcher::watch` via `WatchHandle::set_active`: once called, the
watcher emits `ChangeEvent`s only for the currently active exercise's path (canonicalized comparison,
`Non` safely blocks everything), and the active exercise can be changed at runtime. Callers that
never call `set_active` keep seeing every `.lua` change, unchanged from before this task.
- CLI subcommand structure (`cli::CLI`/ `cli::Commands`, `clap` derive): `list`, `run <name>`, `watch`,
`init`, `hint <name>`, `hint --solution <name>` all parse correctly and reject a missing `<name>` with
a clear error; running with no subcommand shows `--help`. Parsing only - `main.rs` still `todo!()`s
every subcommand, since implementing their actual behavior is separate, not-yet-scheduled tasks.
- `list` subcommand implemented: loads exercises from `info.json` (new `exercise::load`m since no
`info.json` exists in the repo yet - that's real curriculum content, scheduled for Epics 10-22) and
progress from `progress.json` via `ProgressStore` (already tolerant of a missing file), then prints
them grouped by level/module in `info.json`'s original order (never reordered) with a `[x]`/`[ ]`
checkbox per exercise (`cli::render_exercise_list`). A missing or corrupt `info.json` is a clear error
expected first-run state. A small fixture at `test/fixtures/info.json` (separate from real curriculum
content, same principle planned for epic 9's integration tests) exists for manual end-to-end checks
until real content lands. `run`/`watch`/`init`/`hint` remain `todo!()`.
- `run <name>` subcommand implemented: finds the exercise by name (`Exercise::find_by_name`),
reads itts `.lua` from disk (`Exercise::read_source`), dispatches to `lua_runner::run_compile`/`run_test`
based on `Exercise.mode`, and prints the result (`cli::render_run_result`) mapping `Outcome`'s three
variants directly - `[PASS]`, `[FAIL]` with the failure message, `[TIMEOUT]` mentioning the real
budget (`lua_runner::DEFAULT_TIMEOUT_BUDGET`, now public). A pass marks the exercise done via
`ProgressStore::mark_done`; fail/timeout never do. Exit codes are now a fixed 3-level contract,
decided now instead of deferred to Epic 9's integration test: `0` pass, `1` (`cli::EXIT_CONTENT_FAILURE`)
fail/timeout, `2` (`cli::EXIT_OPERATIONAL_ERROR`) for operational in `info.json` (distinct message
from "unknown name" - one is user input, the other is a data/filesystem inconsistency) opr a failure
loading `info.json`/`progress.json`. `list`'s existing exit code for the latter was retroactively
bumped from `1` to `2` to fit this same contract, since it now collides with the new meaning of `1`.
The fixture at `tests/fixtures/` gained real `.lua` files backing its declared exercises (`hello.lua`
pass, `goodby.lua` fail, `extra.lua` pass via `Mode::Test`) plus a `loop_infinity` entry, so `run`'s
three outcomes are exercisable end-to-end. `watch`/`init`/`hint` remain `todo!()`.
