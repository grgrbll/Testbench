
fn say_hello(name : String) -> String {
    format!("Hello {}.", name)
}

#[test]
fn function_pointers() {
    let fi = say_hello;
    // This is a "function item", not a function pointer.
    // Function items are size-0, they do not exist at run-time.
    assert_eq!(0, std::mem::size_of_val(&fi));
    
    // function items can be coerced into function pointers
    let fp : fn (String) -> String = fi;
    assert_eq!(8, std::mem::size_of_val(&fp));
}

fn foo<F>(f: &F) -> String where F : Fn(String) -> String {
    return f(String::from("World"));
}

fn foo_mut<F>(f: &mut F) -> String where F : FnMut(String) -> String {
    return f(String::from("World"));
}

#[test]
fn function_traits() {

    let mut c = |s : String| {format!("Hello {}", s)};
    foo(&c);
    foo_mut(&mut c);
}