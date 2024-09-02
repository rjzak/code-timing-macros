use std::thread;
use std::time::Duration;

use timer_macro::time_function;

#[time_function]
fn sleeper() {
    thread::sleep(Duration::from_secs_f32(2.0f32));
}

#[test]
fn macro_test() {
    sleeper();
}
