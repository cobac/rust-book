/*  Doesn't work because all structs an enums have to have a defined size
enum List<T> {
Cons(T, List<T>),
}
 */

use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

// We can use a pointer (of fixed size) in the type declaration
#[allow(dead_code)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

#[derive(Debug)]
enum RefList<'a, T> {
    Cons(T, &'a RefList<'a, T>),
    Nil,
}

#[derive(Debug)]
enum RcList<T> {
    Cons(T, Rc<RcList<T>>),
    Nil,
}

#[derive(Debug)]
enum RcRefList<T> {
    Cons(Rc<RefCell<T>>, Rc<RcRefList<T>>),
    Nil,
}

struct MyBox<T> {
    field: T,
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox { field: x }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.field
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data)
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    // Cant be Rc because it would be a ref cycle
    // If a parent is dropped, its children will be dropped
    // If a child is dropped, the parent won't be dropped
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    {
        let b = Box::new(5);
        println!("b = {}", b);
    }
    {
        // Dereferencing works
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    {
        let m = MyBox::new(String::from("Rust"));
        /*
        &MyBox<String> => &String via Deref
        &String => &str via Deref for String in std
         */
        hello(&m);
        /*
        Without coercion we would have to write:
        hello(&(*m)[..]);
         *m Box<String> => String, then &m[..] to take the slice reference
         */
    }
    /*
        Doesn't work because we cannot call .drop() manually.
        The compiled program would still call drop after the value goes out of scope
        and that would be a double free error.
        {
        let c = CustomSmartPointer {
        data: String::from("some data"),
    };
        println!("CustomSmartPointer created.");
        c.drop();
        println!("CustomSmartPointer dropped before the end of main.");
    }
         */
    {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        drop(c); // Still executes the Drop impl
        println!("CustomSmartPointer dropped before the end of main.");
    }
    /*
        Doesn't work because common_body is passed to l1.
    {
    let common_body = List::Cons(3, Box::new(List::Cons(4, Box::new(List::Nil))));
    let l1 = List::Cons(1, Box::new(common_body));
        let l2 = List::Cons(2, Box::new(common_body));
    }
         */

    // We could refactor Cons to use normal references and lifetimes,
    // but then all elements of the list would live as long as the whole list.
    {
        let common_body = RefList::Cons(3, &RefList::Cons(4, &RefList::Nil));
        let l1 = RefList::Cons(1, &common_body);
        let l2 = RefList::Cons(2, &common_body);
        println!("{l1:?}");
        println!("{l2:?}")
    }
    {
        let common_body = Rc::new(RcList::Cons(
            3,
            Rc::new(RcList::Cons(4, Rc::new(RcList::Nil))),
        ));
        println!(
            "Ref count after creating common_body = {}",
            Rc::strong_count(&common_body)
        );
        let l1 = RcList::Cons(1, Rc::clone(&common_body));
        println!("{l1:?}");
        println!(
            "Ref count after creating l1 = {}",
            Rc::strong_count(&common_body)
        );
        {
            let l2 = RcList::Cons(2, Rc::clone(&common_body));
            println!("{l2:?}");
            println!(
                "Ref count after creating l1 = {}",
                Rc::strong_count(&common_body)
            );
            println!("Dropping l2");
        }
        println!(
            "Ref count after dropping l2 = {}",
            Rc::strong_count(&common_body)
        );
    }
    {
        let value = Rc::new(RefCell::new(5));

        let common_body = Rc::new(RcRefList::Cons(Rc::clone(&value), Rc::new(RcRefList::Nil)));

        let l1 = RcRefList::Cons(Rc::new(RefCell::new(3)), Rc::clone(&common_body));
        let l2 = RcRefList::Cons(Rc::new(RefCell::new(4)), Rc::clone(&common_body));

        println!("l1 = {l1:?}");
        println!("l2 = {l2:?}");

        *value.borrow_mut() += 10;
        println!("l1 = {l1:?}");
        println!("l2 = {l2:?}");
    }
    {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }
}
