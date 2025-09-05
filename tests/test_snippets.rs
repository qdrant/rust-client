// Skip formatting for this file
#[rustfmt::skip]

mod snippet_tests {
    include!(concat!(env!("OUT_DIR"), "/tests/snippet_tests/mod.rs"));
}

// WARNING: Snippet tests are not checking if the request is executing correctly.
// The only thing it is checking is if the code compiles and runs without panicking.
