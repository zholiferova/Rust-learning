use std::{pin::Pin, fmt, ptr::NonNull, marker::PhantomPinned};

#[derive(Debug)]
struct Unmovable {
    data: Vec<i32>,
    _pin: PhantomPinned,
}
impl Unmovable {
    fn new () -> Pin<Box<Self>> {
        let res = Unmovable {
            data: vec![1,2,3,5,7],
            _pin: PhantomPinned,
        };
        let boxed = Box::new(res);
        let pin = Box::into_pin(boxed);
        pin
    }
}
trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}
impl<T: ?Sized + fmt::Debug> SayHi for T {
    fn say_hi(self: Pin<&Self>) {
        let t = self.get_ref();
        println!("Hi from {:?}", t);
    }
}

fn main() {
    let unmovable: Pin<Box<Unmovable>> = Unmovable::new();
    let a1 = NonNull::from(&unmovable);
    println!("Before: {:?}", a1);
    unmovable.as_ref().say_hi();
    let a2 = NonNull::from(&unmovable);
    println!("After: {:?}", a2);
    let my_string = String::from("Howdy");
    let pinned_string = Pin::new(my_string);
    let a3 = NonNull::from(&pinned_string);
    println!("Before: {:?}", a3);
    pinned_string.as_ref().say_hi();
    let a4 = NonNull::from(&pinned_string);
    println!("Before: {:?}", a4);
}

