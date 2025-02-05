#[derive(Debug)]
pub struct Foo(String);

impl From<u32> for Foo {
    fn from(from: u32) -> Foo {
        Foo(format!("Converted from integer: {from}"))
    }
}

impl From<bool> for Foo {
    fn from(from: bool) -> Foo {
        Foo(format!("Converted from bool: {from}"))
    }
}

pub fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

pub fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

pub fn pair_of(x: u32) -> impl std::fmt::Debug {
    (x + 1, x - 1)
}
