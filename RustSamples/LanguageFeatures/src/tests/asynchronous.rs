use std::cell::{Cell, RefCell};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Waker, RawWaker};
use futures::executor::block_on;
use futures::pin_mut;

async fn hello() -> String {
    return String::from("Hello World");
}

#[test]
fn basic() {
    let res = block_on(hello());
    assert_eq!("Hello World", res);
}

async fn add(x : &RefCell<Vec<String>>, s : String) {
    x.borrow_mut().push(s);
}
async fn add_group(x : &RefCell<Vec<String>>) {
    add(x, String::from("1")).await;
    x.borrow_mut().push(String::from("2"));
    add(x, String::from("3")).await;
}

#[test]
fn chained() {
    let x = RefCell::new(Vec::new());
    block_on(add_group(&x));

    let vec = x.borrow();
    for (i, s) in vec.iter().enumerate() {
        assert_eq!(&(i+1).to_string() , s);
    }
}
