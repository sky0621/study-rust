use futures::{executor, Future};

fn main() {
    executor::block_on(say());
}

async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

async fn say() -> i32 {
    let ans = async_add(3, 4).await;
    println!("{}", ans);
    ans
}

fn move_to_async_block() -> impl Future<Output = ()> {
    let outside_var = "this is outside".to_string();
    async move {
        println!("{}", outside_var);
    }
}
