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

/// Instead of a concrete type for the `item` parameter, the `impl`
/// keyword is specified along with the trait name. This parameter
/// accepts any type that implements the specified trait.
pub fn notify(item: &impl Summary) {
    // Any methods derived from the `Summary` trait can be called.
    println!("Breaking news! {}", item.sumarize());
}

/// By using `impl Summary` or the return type, we specify that the
/// function returns some type that implements the `Summary` trait
/// without namig the concrete type.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as your probably already know, people"
        ),
        reply: false,
        retweet: false
    }
}