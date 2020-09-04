fn main() {
    // 空の草稿ができる。
    let mut post = Post::new();

    // 文章を追加する。
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    // (草稿ができたので)レビューを要求する。
    post.request_review();
    assert_eq!("", post.content());

    // 承認がおりたら、コンテンツは公開される。
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

pub struct Post {
    state: Option<Box<State>>,
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
        // 値の所有権ではなく、Option内部の値への参照が欲しいから as_ref()
        // &Box<State>に対してcontentを呼び出す
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        // 今、保持しているステートの状態変更
        // takeメソッドを呼び出して、 stateフィールドからSome値を取り出し、その箇所にNoneを残します。
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// 草稿状態
struct Draft {}
impl State for Draft {
    // 型を保持するBoxに対して呼ばれた時のみ、 このメソッドが合法になる
    // Box<Self>の所有権を奪い、古い状態を無効化する
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // 草稿状態でレビュー依頼すると、レビュー結果待ち状態に変わる
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// レビュー結果待ち状態
struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // 既にPendingReview状態にある記事の査読を要求したら、 PendingReview状態に留まるべき
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // 承認がおりたら公開
        Box::new(Published {})
    }
}

struct Published {}
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
