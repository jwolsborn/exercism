// The &'static here means the return type has a static lifetime.
// This is a Rust feature that you don't need to worry about now.

//its hello world...
pub fn hello() -> &'static str {
    "Hello, World!"
}
