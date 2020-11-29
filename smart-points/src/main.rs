#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::ops::Deref;

#[derive(Debug)]
enum ListRc {
    Cons(i32, Rc<ListRc>),
    NilRc,
}
use crate::ListRc::{Cons as ConsRc, NilRc};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum ListRefCell {
    Cons(Rc<RefCell<i32>>, Rc<ListRefCell>),
    NilRefCell,
}

use crate::ListRefCell::{Cons as ConsRefCell, NilRefCell};

#[derive(Debug)]
enum ListC {
    Cons(i32, RefCell<Rc<ListC>>),
    NilC,
}

impl ListC {
    fn tail(&self) -> Option<&RefCell<Rc<ListC>>> {
        match self {
            ConsC(_, item) => Some(item),
            NilC => None,
        }
    }
}

use crate::ListC::{Cons as ConsC, NilC};

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping csm with data {}", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

use std::rc::Weak;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn weak_ref_test() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // creating a weak ref

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}

fn main() {
    let b = Box::new(5);
    println!("b is {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list is {:?}", list);

    let mut x = 5;
    // let y = &x; borrow from x
    let yb = Box::new(x); // copy from x
    println!("x is {}", x);
    // println!("y is {}", y);
    x = 8;
    println!("x is {}", x);
    println!("yb is {}", yb);
    println!("yb is {}", *yb);
    // println!("y is {}", y);
    let my = MyBox::new(x);
    println!("my is {:?}", my);
    println!("my is {:?}", *my); // implement deref trait
    println!("my is {:?}", *(my.deref())); // implement deref trait

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);

    let c = CustomSmartPointer {
        data: String::from("my stuff c"),
    };

    let d = CustomSmartPointer {
        data: String::from("my stuff d"),
    };

    println!("CustomSmartPointer created");
    // c.drop();
    drop(d);
    println!("CustomSmartPointer dropped in advance {}", c.data);

    // RC
    let rc1 = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(NilRc)))));
    println!("after creating rc1 count = {:?}", Rc::strong_count(&rc1));

    let rc2 = ConsRc(3, Rc::clone(&rc1));
    println!("after creating rc2 count = {:?}", Rc::strong_count(&rc1));

    {
        let rc3 = ConsRc(4, Rc::clone(&rc1));
        println!("after creating rc3 count = {:?}", Rc::strong_count(&rc1));
    }

    println!("rc3 goes out of scope count = {:?}", Rc::strong_count(&rc1));

    // RefCell
    let xx = 5;
    // let yy = &mut xx;

    let value = Rc::new(RefCell::new(5));
    let aa = Rc::new(ConsRefCell(Rc::clone(&value), Rc::new(NilRefCell)));
    let bb = ConsRefCell(Rc::new(RefCell::new(3)), Rc::clone(&aa));
    let cc = ConsRefCell(Rc::new(RefCell::new(3)), Rc::clone(&aa));

    *value.borrow_mut() += 10;

    println!("aa after = {:?}", aa);
    println!("bb after = {:?}", bb);
    println!("cc after = {:?}", cc);

    // ref cycles
    let ca = Rc::new(ConsC(5, RefCell::new(Rc::new(NilC))));

    println!("a initial rc count is {}", Rc::strong_count(&ca));
    println!("a next item is {:?}", ca.tail());

    let cb = Rc::new(ConsC(10, RefCell::new(Rc::clone(&ca))));

    println!("a rc count after creating b is {}", Rc::strong_count(&ca));
    println!("b  initial rc count is {}", Rc::strong_count(&cb));
    println!("b next item is {:?}", cb.tail());

    if let Some(link) = ca.tail() {
        *link.borrow_mut() = Rc::clone(&cb);
    }

    println!("b rc count after changing a is {}", Rc::strong_count(&cb));
    println!("a rc count after changing a is {}", Rc::strong_count(&ca));

    // Uncomment the next line to see that we have a cycle
    // it will overflow the stack
    // println!("a next item is {:?}", ca.tail());

    weak_ref_test();
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let per_of_max = self.value as f64 / self.max as f64;

        if per_of_max >= 1.0 {
            self.messenger.send("Error: you are over your quota");
        } else if per_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: you are using up over 90%");
        } else if per_of_max >= 0.75 {
            self.messenger.send("Warning: you are using up over 75%");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Debug)]
    struct MockMessenger {
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // sent_messages: vec![],
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // self.sent_messages.push(String::from(message));
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_per_warning_msg() {
        let mock_msg = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_msg, 100);

        limit_tracker.set_value(80);
        // println!("mock_msg is {:?}", mock_msg.sent_messages);
        // println!("mock_msg len is {}", mock_msg.sent_messages.len());

        let mut a = vec![];
        a.push(String::from("Warning: you are using up over 75%"));
        assert_eq!(a, *mock_msg.sent_messages.borrow());

        assert_eq!(1, mock_msg.sent_messages.borrow().len());
    }
}
