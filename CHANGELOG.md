# Changelog

All notable changes to this project will be documented in this file.


The format is based on [Keep a Changelog](https://keepchangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Basic `.lua` file change detection via `notify` (`watcher::watch`), filtering to `.lua` files
and running continuously without needing to be restarted after each event. Debounce and active-exercise
filtering are separate, not-yet-scheduled tasks.
