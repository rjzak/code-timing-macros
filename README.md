[![Test](https://github.com/rjzak/code-timing-macros/actions/workflows/test.yml/badge.svg)](https://github.com/rjzak/code-timing-macros/actions/workflows/test.yml)[![Lint](https://github.com/rjzak/code-timing-macros/actions/workflows/lint.yml/badge.svg)](https://github.com/rjzak/code-timing-macros/actions/workflows/lint.yml)[![Crates.io Version](https://img.shields.io/crates/v/code-timing-macros)](https://crates.io/crates/code-timing-macros)

## code-timing-macros!
This crate aims to provide useful, easy to use macros to measure the time taken to execute some code. At present, this is **alpha** quality, and subject to changes.

Contributions welcomed!

### Macros:
* Adding `#[time_function]` to a function causes the program to print how long a function took to run when it's finished.
* Use `time_snippet!()` to report the timing for a snippet (or block) of code.

### Capabilities:
* Works on functions and code blocks with or without a return.
* Async functions and code blocks tested and seem to work. Please report any issues.
* Const functions won't work since the contents of the function won't be determinable at compile time.

### Features
The following crate features are available:
* `release`: by default, the macros will not modify the code for release builds. This feature prevents that, so release builds will report execution time.
* `tracing`: by default, the macros will print elapsed time information to standard output, but this feature instead sends information to the log using the [tracing](https://crates.io/crates/tracing) crate.

### Examples
* See the [unit tests](https://github.com/rjzak/code-timing-macros/blob/main/tests/test.rs) for examples.
