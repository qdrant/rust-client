#[macro_export]
macro_rules! qdrant_test_snippet {
    {$code:expr} => {
        #[tokio::test]
        async fn snippet() {
            let _ = async {
                $code
                Result::<(), Box<dyn std::error::Error>>::Ok(())
            }.await;
        }
    };
}

// WARNING: Snippet tests are not checking if the request is executing correctly.
// The only thing it is checking is if the code compiles and runs without panicking.

mod snippets;
