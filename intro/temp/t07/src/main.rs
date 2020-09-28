use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::{executor, future::join_all};

fn main() {
    let cd1 = CountDown(10);
    let cd2 = CountDown(20);
    let cd_set = join_all(vec![cd1, cd2]);
    let res = executor::block_on(cd_set);
    for (i, s) in res.iter().enumerate() {
        println!("{}: {}", i, s);
    }
}

struct CountDown(u32);

impl Future for CountDown {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        if self.0 == 0 {
            Poll::Ready("Zero!!!".to_string())
        } else {
            println!("{}", self.0);
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
