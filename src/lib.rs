/// Defines the functionality of displaying summaries of
/// data that is stored in any type that implements it. 
pub trait Summary {
    // Instead of providing an implementation, only the
    // signature is stated. Each type implementing the
    // trait must provide its own custom behaviour for
    // the body of the method.
    fn sumarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

/// Implementing a trait on a type is similar to implementing
/// regular methods. The difference is that after `impl`, the
/// trait being implemented is specified along with the `for`
/// keyword.
impl Summary for NewsArticle {
    // Whithin the `impl` block, the specific behaviour is defined.
    fn sumarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn sumarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}