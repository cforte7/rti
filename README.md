# RTI
RTI (Rusty Time Interpreter) is command line tool for converting between unix epoch time and date/time strings. It is written and Rust and should run on any platform.

This tool was inspired by the extremely useful [epoch-echo](https://github.com/ainsleymcgrath/epoch-echo) by Ainsley Mcgrath. While the tool is fantastic and has more functionality than RTI currently, the main issues were speed and portability which inspired me to write this in Rust.

# Installation
Currently you can either clone the repo and compile binaries yourself, or download with [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html). Using cargo:
```
cargo install rti
```

This will create a globally executable command that can be used in your command line. **Note:** This has only been tested on macOS.

# Usage
RTI takes in an arbitrary number of command line arguments, either integer unix epochs or string date/time/datetimes and converts them to the opposite.
```
rti 1 1650627609 2022-04-22\ 11:40:09
1 => 01-01-1970 00:00:01
1650627609 => 04-22-2022 11:40:09
2022-04-22 11:40:09 => 1650627609
Timezone: UTC
```

There are also special keywords for `now`, `yesterday`, `tomorrow` which will give the epoch time for the current time of today, yesterday, and tomorrow respectively.

```
// ran at May 10, 2022 at 5:25PM UTC time
rti now yesterday tomorrow
now => 1652203517
yesterday => 1652117117
tomorrow => 1652289917
Timezone: UTC
```

## Timezone
RTI supports using custom timezone both by setting a persisted configuration and through environment variables.
If no timezone is set in your local config or by environment variable, UTC will be used.
```

// Set custom timezone in config
rti set-tz America/New_York

// clear timezone config
rti clear-tz

// using TIMEZONE environment variable
TIMEZONE=America/Denver rti now
```

RTI timezones are set in the following precedence:
1. Environment variable
2. Config
3. UTC if no config or Env variable is present

# Limitations (Planned Features)

Currently RTI does not have support for the following (but are planned to be added over time):
- Set custom datetime parse tokens.
- Copy output to clipboard
