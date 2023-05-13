pub trait Summary {
    // 必須實作
    fn summarize_author(&self) -> String;

    // 預設實作 (可以 override)
    fn summarize(&self) -> String {
        format!("(從 {} 閱讀更多...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 使用特徵作為參數
pub fn notify(item: &impl Summary) {
    println!("頭條新聞！{}", item.summarize());
}

// 回傳有實作特徵的型別
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
