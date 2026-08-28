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
