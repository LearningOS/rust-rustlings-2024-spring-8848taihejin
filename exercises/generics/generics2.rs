// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.


struct Wrapper {
    value: u32,
    str_value: String,
}

impl Wrapper {
    pub fn new(value: u32) -> Self {
        Wrapper { value, str_value: String::new() }
    }
    pub fn new_str(str_value: &str) -> Self {
        Wrapper {value: 0,str_value: str_value.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new_str("Foo").str_value, "Foo");
    }
}
