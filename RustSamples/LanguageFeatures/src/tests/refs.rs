use std::{rc::Rc, cell::{RefCell, Cell}, panic};

#[test]
fn boxes() {
    let mut b : Box<i32> = Box::new(10);

//  assert_eq!(10, b); // <- Invalid.
    assert_eq!(10, *b);
    assert_eq!(&10, &*b); // <- Wierd looking, but valid.

    *b = 20;
    assert_eq!(20, *b);
}

struct Dropable<'a> {
    pub x:  &'a mut bool
}

impl Drop for Dropable<'_> {
    fn drop(&mut self) {
        *self.x = true;
    }
}

#[test]
fn refcount() {
    let mut b : Rc<i32> = Rc::new(10);

//  assert_eq!(10, b); // <- Invalid.
    assert_eq!(10, *b);
    assert_eq!(&10, &*b); // <- Wierd looking, but valid.

//  *b = 20; // <-  Unlike with box, this is not allowed. See RefCell

    let mut x : bool = false;
    {
        let _dropable  = Rc::new(Dropable { x: &mut x});
        assert_eq!(1, Rc::strong_count(&_dropable));
        {
            let _copy  = Rc::clone(&_dropable);
            assert_eq!(2, Rc::strong_count(&_dropable));
        }
        assert_eq!(1, Rc::strong_count(&_dropable));

        assert_eq!(&false, _dropable.x);
    }
    assert_eq!(true, x);


}

// Values of the Cell<T> and RefCell<T> types may be mutated through shared references (i.e. the common &T type)
// whereas most Rust types can only be mutated through unique (&mut T) references.
// Both Cell and RefCell are single-threaded (Sync is not implemented).

#[test]
fn refcells() {
    // RefCell's can be used to make a mutable Rc.
    {
        let x = Rc::new(RefCell::new(10));

        *x.borrow_mut() = 20;

        assert_eq!(20, *x.borrow());

        let y = &x;
        *y.borrow_mut() = 30;
        
        assert_eq!(30, *x.borrow());
    }

    // Unline Cell<T>, RefCell<T> can cause a panic at runtime.
    // Due to this, you might want to prefer Cell<T>
    {
        let result = panic::catch_unwind(|| {
            let x = RefCell::new(10);
            let _y = x.borrow_mut();
            let _z = x.borrow_mut(); // This causes a panic!
        });

        assert!(result.is_err());
    }

}

#[test]
fn cells() {
    // Where RefCell<T> implements interior mutibility via mutable references, Cell<T> implements
    // it by moving data in and out.
    let x = Cell::new(10);
    assert_eq!(10, x.get());
    x.set(20);
    assert_eq!(20, x.get());
}


trait Dog {
    fn bark(&self) -> &'static str;
}

struct Labrador { }
impl Dog for Labrador {
    fn bark(&self) -> &'static str { return &"Woof";}
}

struct Chihuahua { }
impl Dog for Chihuahua {
    fn bark(&self) -> &'static str { return &"Yap Yap";}
}

#[test]
fn boxes_for_dynamic_dispatch() {
    // Refs are required by trait-objects because the size of their
    // actual type can vary.
    let mut dog : Box<dyn Dog>;

    dog = Box::new(Labrador {});
    assert_eq!("Woof", dog.bark());

    dog = Box::new(Chihuahua {});
    assert_eq!("Yap Yap", dog.bark());
}



