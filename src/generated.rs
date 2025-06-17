#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod main {
    extern crate alloc;
    use core::borrow::Borrow;
    use lazy_static::lazy_static;
    use rasn::prelude::*;
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice)]
    pub enum FizzBuzz {
        #[rasn(tag(context, 456))]
        foo(()),
        #[rasn(tag(context, 789))]
        bar(()),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    pub struct Fizz {
        #[rasn(tag(explicit(context, 123)))]
        pub buzz: FizzBuzz,
    }
    impl Fizz {
        pub fn new(buzz: FizzBuzz) -> Self {
            Self { buzz }
        }
    }
}
