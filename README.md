# RTI
RTI (Rusty Time Interpreter) is command line tool for parsing between unix epoch time and date/time strings. It is written and Rust and should run on any platform.

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
1 => 1970-01-01 00:00:01
1650627609 => 2022-04-22 11:40:09
2022-04-22 11:40:09 => 1650627609
```

There are also special keywords for `now`, `yesterday`, `tomorrow` which will give the epoch time for the current time of today, yesterday, and tomorrow respectively.

```
// run at April 22, 2022 at 8:11AM America/Chicago
rti now yesterday tomorrow
now => 1650633061
yesterday => 1650546661
tomorrow => 1650719461
```

# Limitations (Planned Features)

Currently RTI does not have support for the following (but are planned to be added over time).
- Set custom time zone. Currently RTI only uses your computer's local timezone.
- Set custom datetime parse tokens.
- Copy output to clipboard
