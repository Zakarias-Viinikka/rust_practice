#![allow(warnings)]
use tokio::{
    process::Command,
    sync::{mpsc, oneshot},
    time::{Duration, sleep},
};

use std::sync::Arc;

/*
 * the goal of this mini project is to create a function that takes a &i32 parameter and then trying to call it when the caller wants to send in a "async safe variable"
 *
 * I also wanna check if i can get it to work for &mut i32
 */
#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<oneshot::Sender<Arc<i32>>>(32);

    let manager = tokio::task::spawn(async move {
        let arc_x = std::sync::Arc::new(42i32);

        //while let necessary to keep it alive? maybe that's what im doing with the manager_never_stops. i donät need the channel to stay alive though.
        if let Some(reply) = rx.recv().await {
            let _ = reply.send(Arc::clone(&arc_x));
        }
        manager_never_stops();
    });
    tokio::task::spawn(async_func(tx));
}

async fn async_func(tx: mpsc::Sender<oneshot::Sender<Arc<i32>>>) {
    let (resp_tx, resp_rx) = oneshot::channel();
    let arc_to_send = Arc::new(42);
    tx.send((resp_tx)).await.ok();
    let response = resp_rx.await;
    // Now you can use response
    match response {
        Ok(num) => {
            give_me_your_i32(&num);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn give_me_your_i32(num: &i32) {
    println!("compiler ain't mad.");
    println!("number: {}", num);
}

async fn manager_never_stops() {
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}
