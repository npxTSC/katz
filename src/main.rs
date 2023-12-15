use tokio::time::{self, Duration};

use tokio::runtime::Runtime;
#[tokio::main]
async fn main() {
    //    tokio::spawn(async move {
    //        sleep_test().await;
    sleep_test().await;
    //    });
    //   let rt = Runtime::new().unwrap();
    //   rt.block_on(sleep_test());
}

async fn sleep_test() {
    println!("not sleep");
    time::sleep(Duration::from_secs(5)).await;
    println!("sleep");
}
