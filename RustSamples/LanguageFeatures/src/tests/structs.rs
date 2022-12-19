use std::ops::Deref;

struct Mutable<T> {
    value: T,
}

impl<T> Mutable<T> {
    pub fn new(value: T) -> Self {
        Mutable { value }
    }
}

struct Immutable<T> {
    value: T,
}

impl<T> Immutable<T> {
    pub fn new(value: T) -> Self {
        Immutable { value }
    }
}

impl<T> Deref for Immutable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
#[test]
pub fn deref() {

    let mut m :Immutable<i32> = Immutable::new(42);
    m.value = 10;
    assert_eq!(10, m.value);
}