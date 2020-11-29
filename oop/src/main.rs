pub struct AvgContainer {
    list: Vec<i32>,
    avg: f64,
}

impl AvgContainer {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_avg();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_avg();
                Some(value)
            }
            None => None,
        }
    }

    pub fn avg(&self) -> f64 {
        self.avg
    }

    fn update_avg(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.avg = total as f64 / self.list.len() as f64;
    }
}

fn base_oop_test() {
    let mut avc = AvgContainer {
        list: vec![3, 4, 5],
        avg: 0.0,
    };

    avc.add(6);

    println!("list avg is {}", avc.avg());

    avc.remove().unwrap();

    println!("list avg after remove is {}", avc.avg());
}

mod gui {
    // trait object
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    //  trait objects allow for multiple concrete types to fill in for the trait object at RUNTIME
    //  while generic type paramter is at COMPLILE TIME
    impl Screen {
        pub fn run(&self) {
            println!("**********screen print start**********");
            for component in self.components.iter() {
                component.draw();
            }

            println!("**********screen print end**********");
        }
    }

    // concrete component type

    pub struct Buttom {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Buttom {
        fn draw(&self) {
            println!(
                "this is buttom component with w = {}, h = {}, label = {}",
                self.width, self.height, self.label
            );
        }
    }

    pub struct SelectBox {
        pub width: u32,
        pub height: u32,
        pub options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!(
                "this is select box component with w = {}, h = {}, options.len() = {}",
                self.width,
                self.height,
                self.options.len()
            );
        }
    }
}

use gui::{Buttom, Screen, SelectBox};

fn gui_test() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 55,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("maybe"),
                    String::from("nop"),
                ],
            }),
            Box::new(Buttom {
                width: 50,
                height: 12,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

mod blog {
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }

        // state transfer  for request review function
        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        // state transfer for approve function
        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }

    // Abstract State
    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }

    //Draft State
    struct Draft {}

    // state transfer  for Draft
    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    struct PendingReview {}

    // state transfer for PendingReview
    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }

    struct Published {}

    // state transfer for Published
    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }
}

use blog::Post;

fn blog_test() {
    let mut post = Post::new();
    post.add_text("I ate a sala for lunch today");
    println!("draft:  {}", post.content());

    post.request_review();
    println!("request_review: {}", post.content());

    post.approve();
    println!("approve: {}", post.content());
}

fn main() {
    base_oop_test();
    gui_test();
    blog_test();
}
