
pub(crate) struct Post
{
    msg: String,
    pub state: Option<Box<dyn State>>
}

impl Post {
    pub fn new(msg: String) -> Self
    {
        Self
        {
            msg,
            state: Some(Box::new(Draft {}))
        }
    }

    pub fn content(&self) -> String {
        return self.msg.clone();
    }

    pub fn update(&mut self, msg: String)
    {
        self.state = Some(Box::new(PendingReview {}));
        self.msg = msg;
    }

    pub fn add(&mut self, msg: String)
    {
        self.msg = format!("{}{}", self.msg, msg);
    }

    pub fn post(&mut self)
    {
        self.state = Some(Box::new( Posted {} ))
    }
}


pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

#[derive(Debug)]
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

#[derive(Debug)]
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

#[derive(Debug)]
struct Posted {}

impl State for Posted {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}
