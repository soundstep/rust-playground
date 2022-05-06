use core::fmt::Debug;
use std::cmp::min;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    approvals: u32,
}

impl Post {
    pub fn new() -> Post {
        Post {
            approvals: 0,
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn get_max_approvals(&mut self) -> u32 {
        2
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    pub fn request_review(&mut self) {
        println!("Post.request_review, state: {:?}", self.state);
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    pub fn reject(&mut self) {
        println!("Post.reject, state: {:?}", self.state);
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
    pub fn approve(&mut self) {
        self.approvals = min(self.approvals + 1, self.get_max_approvals());
        println!(
            "Post.approve, state: {:?}, approvals {}",
            self.state, self.approvals
        );

        match self.state.take() {
            Some(s) => {
                self.state = if self.approvals < 2 {
                    println!("  not yet approved, missing approvals!");
                    Some(s.request_review())
                } else {
                    println!("  Post.approve, approved!");
                    Some(s.approve())
                };
            }
            None => self.state = None,
        };
    }
}

trait State: Debug {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

#[derive(Debug)]
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

#[derive(Debug)]
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

#[derive(Debug)]
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
