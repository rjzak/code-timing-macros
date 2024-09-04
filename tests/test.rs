use std::thread;
use std::time::Duration;

use code_timing_macros::{time_function, time_snippet};

#[time_function]
fn sleeper() {
    thread::sleep(Duration::from_secs_f32(2.0f32));
}

#[time_function]
fn meaning_of_life() -> u8 {
    42
}

#[time_function]
async fn test_async() -> Option<u16> {
    let handle = tokio::spawn(async { 10 });

    let out = handle.await.unwrap();
    Some(out)
}

#[test]
fn simple_functions() {
    sleeper();

    assert_eq!(meaning_of_life(), 42);
}

#[tokio::test]
async fn async_function() {
    assert_eq!(test_async().await.unwrap(), 10);
}

#[test]
fn snippet() {
    time_snippet!({
        let bytes = std::fs::read(std::env::current_exe().unwrap()).unwrap();
        let mut avg = 0.0f32;
        for b in &bytes {
            avg += *b as f32;
        }
        avg /= bytes.len() as f32;
        println!("Avg: {avg}");
    });
}
