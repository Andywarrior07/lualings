# Changelog

All notable changes to this project will be documented in this file.


The format is based on [Keep a Changelog](https://keepchangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

* `02_types` module complete - exercises, mirrored solutions and exercise hints for `type1`/`type2`/`type3`.
* `01_variables` module complete - exercises, mirrored solutions and exercises hints for `variables1`/`variables2`/`variables3`, all  on `Mode::Compile`.
* `init` implemented, plus the minimal Epic 8 embedding slice it needed (new `embed` module,
`include_dir!`/`include_str!`, `extract_to` that never overwrites existing files). All 6 Epic 6
subcommands are now implemented.
* Real `info.json` generated from the `exercises/` tree (53 exercises) - `list`/`run`/`hint` now work
agains real content, not only the fixture.
* `hint`/`hint --solution` implemented: shows `Exercise.hint`, or the mirrored file under `solutions/`.
* `watch` implemented: watches `exercises/`, auto-advances through pending exercises, exits onece
everything is complete.
* `run <name>` implemented: executes an exercise, marks it done on pass, fixed exit code contract
(`0`/`1`/`2`).
* `list` implemented: prints exercises grouped by level/module with a `[x]/[ ]` per exercise.
* CLI subcommand structure (`list`, `run`, `watch`, `init`, `hint`) defined with `clap`.
* Active-exercise filtering for `watcher::watch` via `WatchHandle::set_active`.
* Debounce for `watcher::watch`, collapsing multiple raw save events into one.
* Basic `.lua` file change detection via `notify` (`watcher::watch`).
