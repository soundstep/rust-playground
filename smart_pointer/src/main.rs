use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::mem::drop;
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    {
        let b = Box::new(5); // data stored in the heap instead of the stack
        println!("b = {}", b);
    }
    {
        let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }
    {
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(&5, y);
        assert_eq!(5, *y);
    }
    {
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y); // *y is the same as *(y.deref())
    }
    {
        let m = MyBox::new(String::from("Rust"));
        hello(&m); // without deref, we would need to write: hello(&(*m)[..]);
    }
    {
        let _c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let _d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    }
    {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        // c.drop(); // this is not allowed, use instead the std::mem::drop
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }
    {
        // in this scenario "a" has multiple owners (b and c)
        let a = Rc::new(ListCounted::Cons(
            5,
            Rc::new(ListCounted::Cons(10, Rc::new(ListCounted::Nil))),
        ));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let _b = ListCounted::Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let _c = ListCounted::Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
    {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(ListMutated::Cons(
            Rc::clone(&value),
            Rc::new(ListMutated::Nil),
        ));

        let b = ListMutated::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = ListMutated::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10; // mutating the immutable, wait what?

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
    {
        let a = Rc::new(ListTailed::Cons(5, RefCell::new(Rc::new(ListTailed::Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(ListTailed::Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());
    }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

enum ListCounted {
    Cons(i32, Rc<ListCounted>),
    Nil,
}

#[derive(Debug)]
enum ListMutated {
    Cons(Rc<RefCell<i32>>, Rc<ListMutated>),
    Nil,
}

#[derive(Debug)]
enum ListTailed {
    Cons(i32, RefCell<Rc<ListTailed>>),
    Nil,
}

impl ListTailed {
    fn tail(&self) -> Option<&RefCell<Rc<ListTailed>>> {
        match self {
            ListTailed::Cons(_, item) => Some(item),
            ListTailed::Nil => None,
        }
    }
}
