// トレート(インターフェースみたいなもの)関数や定数を持たせる
trait Fruits {
    fn price(&self) -> u32;
}
struct Apple;
impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}
struct Banana;
impl Fruits for Banana {
    fn price(&self) -> u32 {
        5
    }
}

trait Summary {
    // 標準の処理を記述できる(構造体でsummarizeを記述しないとこの処理が走る)
    fn summarize(&self) -> String {
        String::from("read more...")
    }
}
trait Message {
    fn message(&self) -> String {
        String::from("message")
    }
}
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    _content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} {} {}", self.headline, self.author, self.location)
    }
}
// NewsArticleは2つのトレートを保有する
impl Message for NewsArticle {}

struct Tweet {
    username: String,
    content: String,
    _reply: bool,
    _retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} {}", self.username, self.content)
    }
}

pub fn run() {
    let apple = Apple {};
    let banana = Banana {};
    get_price(apple);
    get_price(banana);

    let tweet1 = Tweet {
        username: String::from("taiyou"),
        content: String::from("うんち"),
        _reply: false,
        _retweet: false,
    };
    println!("{}", tweet1.summarize());

    let news = NewsArticle {
        headline: String::from("うおお"),
        location: String::from("japan"),
        author: String::from("taiyou"),
        _content: String::from("rust勉強なう"),
    };
    println!("{}", news.summarize());
    notify(&news);
    notify(&tweet1);
    notify_another(&news);
    // notify_another(&tweet1); //Messageトレートを含んでいないのでエラー
}

// Tはfruitsトレートを含む型のこと = fruitトレートのprice関数を必ず持つ
fn get_price<T: Fruits>(fruits: T) {
    println!("{}", fruits.price());
}

// Summaryトレートを含むデータ型はitemに渡せる
fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}
// SummaryとMessageトレート両方を持つデータ型のみ(NewsArticleのみ)
fn notify_another(item: &(impl Summary + Message)) {
    println!("{}", item.summarize());
    println!("{}", item.message());
}
