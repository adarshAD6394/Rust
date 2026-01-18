fn main () {
    let tweet = Tweet {
        username: String::from("ad_adi"),
        content: String::from("hello world"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("Breaking News"),
        content: String::from("Some interesting content"),
    };

    println!("Tweet Summary: {}", tweet.summarize());
    println!("Article Summary: {}", article.summarize());
    notify(&tweet);
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}" , self.username, self.content)
    }
}

struct NewsArticle {
    author: String,
    headline: String,
    content: String,
}

impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{} by {}", self.headline, self.author)
//     }
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub fn notify (item : &impl Summary) {
    println!("breaking news! {}", item.summarize());
}

pub fn notify (item1: &impl Summary, item2: &impl Summary) {
    // 
}

pub fn notify (item1: &(impl Summary + Display), item2: &impl Summary) {
    // 
}

pub fn notify<T: Summary>(item1: &T, item2: &T) {
    //
}

pub fn notify<T: Summary + Display>( item: &T , item2: &T) {
    //
}

pub fn function<T: Summary + Clone, U: CLone + Debug>(t: &T, u: &U) -> i32 {
    // 
}

// can also be written as 

pub fn function<T, U> (t: &T, u: &U) -> i32 {
    where T: Summary + Clone,
        U: CLone :  Debug
}


// returning types that implement traits are very important and used often in closures. Resrictions apply
fn summarisable () -> impl Summary {
    // 
}

 