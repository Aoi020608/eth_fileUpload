use std::{ops::Deref, rc::Rc};

pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn hello(name: &str) {
    println!("Hello, {name}");
}

pub struct CustomerSmartPointer {
data: String,
}

impl Drop for CustomerSmartPointer {
fn drop(&mut self) {
    println!("Dropping CustomerSmartPointer with data `{}`", self.data);
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box() {
        let b = Box::new(5);
        println!("b = {}", b);
    }

    #[test]
    fn test_cons_list() {
        let a = Rc::new(List::Cons(
            5,
            Rc::new(List::Cons(10, Rc::new(List::Nil)))),
        );
        let b = List::Cons(3, Rc::clone(&a));
        let c = List::Cons(4, Rc::clone(&a));

    }

    #[test]
    fn test_box_ref() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_mybox_ref() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_hello() {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }

    #[test]
    fn test_custom_smart_pointer() {
        let c = CustomerSmartPointer {
            data: String::from("my stuff"),
        };

        let d = CustomerSmartPointer {
            data: String::from("other stuff"),
        };

        println!("CustomSmartPointers created.");
    }

    #[test]
    fn test_increase_rc() {
let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
        println!("Count after creating a = {}", Rc::strong_count(&a));
        let b = List::Cons(3, Rc::clone(&a));
        println!("Count after creating b = {}", Rc::strong_count(&a));
        {
let c= List::Cons(4, Rc::clone(&a));
            println!("Count after creating c = {}", Rc::strong_count(&a));
        }

        println!("Count after c goes out of scope = {}", Rc::strong_count(&a));

    }
}
