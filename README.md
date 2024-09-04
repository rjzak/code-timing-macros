## code-timing-macros
As mentioned, this crate aims to provide easy to use macros to make it easier to measure the time taken to execute some code. At present, this is **alpha** quality, and subject to changes.

Contributions welcomed!

### Macros:
* Adding `#[time_function]` to a function causes the program to print how long a function took to run when it's finished.
* Use `time_snippet!()` to report the timing for a snippet of code, doesn't yet handle a data return.

### Capabilities:
* Works on functions with or without a return.
* Async functions tested and seem to work. Please report any issues.
* Const functions won't work since the contents of the function won't be determinable at compile time.

### Features
The following crate features are available:
* `release`: by default, the macros will not modify the code for release builds. This feature prevents that, so release builds will report execution time.
* `tracing`: by default, the macros will print elapsed time information to standard output, but this feature instead sends information to the log using the [tracing](https://crates.io/crates/tracing) crate.
