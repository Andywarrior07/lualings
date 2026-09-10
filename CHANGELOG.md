# Changelog

All notable changes to this project will be documented in this file.


The format is based on [Keep a Changelog](https://keepchangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

* `07_error_handling_advanced` (mid) module complete - exercises, mirrored solutions and exercise hints for `error_adv1`/`error_adv2`
* `06_modules` (mid) module complete - exercises, mirrored solutions and exercise hints for `modules1`/`modules2`.
* `05_string_patterns` (mid) module complete -exercises, mirrored solutions and exercise hints for `patterns1`/`patterns2`/`patterns3`
* `04_coroutines` (mid) module complete - exercises, mirrored solutions and exercise hints for `coroutines1`/`coroutines2`/`coroutines3`
* `03_loop` (mid) module complete - exercises, mirrored solutions and exercise hints for `oop1`/`oop2`/`oop3`
* `02_metatables` (mid) module complete - exercises, mirrored solutions and exercise hints for `metatables1`/`metatables2`/`metatables3`/`metatables4`.
* `01_closures` (mid) module complete - exercises, mirrored solutions and exercise hints for `closures1`/`closures2`/`closures3`.
* `08_error_handling_basics` module complete - exercises, mirrored solutions and exercise hints for `error1`/`error2`
* `07_strings` module complete - exercises, mirrored solutions and exercise hints for `string1`/`string2`.
* `06_scope` module complete - exercises, mirrored solutions and exercise hints for `scope1`/`scope2`.
* `05_tables_basics` module complete - exercises, mirrored solutions and exercise hints for `tables1`-`tables3`.
* `04_functions` module complete - exercises, mirrored solutions and exercise hints for `function1`-`function4`
* `03_control_flow` module complete - exercises, mirrored solutions and exercise hints for `control_flow1`-`control-flow4.`
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
