# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Rusty Time Interpreter (rti)** is a Rust CLI tool for converting between Unix epoch timestamps and human-readable datetime strings. It supports bidirectional conversion, special keywords (`now`, `yesterday`, `tomorrow`), multiple format patterns, timezone configuration, and user-defined custom parsing tokens.

## Commands

```bash
cargo build          # Build the project
cargo test           # Run all tests
cargo test <filter>  # Run a specific test, e.g.: cargo test test_dst_changeover
```

There is no separate lint step configured; `cargo build` catches type/compile errors.

## Architecture

Four modules, all in `src/`:

- **`main.rs`** — Entry point. Calls `parse_input()` then `execute_action()`, routing to either datetime conversion or config commands.
- **`cli.rs`** — CLI parsing. Defines the `Action` enum and `parse_input()`. No external arg-parsing crate; manual string matching.
- **`config.rs`** — Persistent config via the `confy` crate. Stores default timezone and custom parsing tokens. Timezone resolution precedence: `TIMEZONE` env var → config file → UTC.
- **`datetime_parsing.rs`** — Core logic. `parse_arg()` tries custom tokens first, then iterates 6 date patterns × 10 time patterns (via `itertools::iproduct!`) to find a match. `epoch_to_datetime()` handles millisecond detection for large values.

### Key design notes

- **Pattern order matters** in `date_time_patterns` — some patterns will greedily match others if ordering is wrong.
- **Error type:** `OkOrStringError` is a `Result<T, String>` alias used throughout.
- **Tests** are in submodules within `datetime_parsing.rs`, separated into UTC and Central timezone variants. Tests that depend on the current date have a TODO noting they can't assert exact output.
- The current branch (`fix-dst-changeover`) has a failing test (`test_dst_changeover`) that was added to capture a DST edge case that is not yet fixed.
