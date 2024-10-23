use std::thread;
use std::time::Duration;

use code_timing_macros::{time_function, time_snippet};

#[time_function]
pub(crate) fn sleeper() {
    thread::sleep(Duration::from_secs_f32(2.0f32));
}

#[time_function]
pub fn meaning_of_life() -> u8 {
    42
}

#[time_function]
fn function_with_args(data: &[u8]) -> usize {
    data.len()
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

#[test]
fn simple_functions_with_args() {
    let contents = std::fs::read(std::env::current_exe().expect("failed to get path to self"))
        .expect("failed to read self");
    let _contents_len = function_with_args(&contents);
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

#[tokio::test]
async fn async_snippet() {
    use tokio::time::{sleep, Duration};

    time_snippet!(
        async {
            sleep(Duration::from_millis(100)).await;
        }
        .await
    )
}

#[test]
fn snippet_result() {
    let result = time_snippet!(100 * 1000 + 20);
    assert_eq!(result, 100 * 1000 + 20);
}

mod object {
    use code_timing_macros::time_function;

    struct SomeObject {
        num: u16,
    }

    impl SomeObject {
        #[time_function(SomeObject::new())]
        pub fn new() -> Self {
            SomeObject { num: 22 }
        }

        #[time_function(SomeObject::semi_private())]
        pub(crate) fn semi_private(&self) {
            println!("Semi-private function");
            self.private();
        }

        #[time_function(SomeObject::private())]
        fn private(&self) {
            println!("Private function")
        }
    }

    impl Default for SomeObject {
        #[time_function(SomeObject::default)]
        fn default() -> Self {
            SomeObject { num: 42 }
        }
    }

    #[test]
    fn test_obj() {
        let default_version = SomeObject::default();
        let constructed_version = SomeObject::new();

        assert_ne!(default_version.num, constructed_version.num);
        default_version.semi_private();
    }
}
