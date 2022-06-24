mod arbiter_pool;
mod client;
mod connection;
mod connection_mgr;
mod connection_pool;
mod protocol;

use crate::client::Client;
use actix_web;

#[actix_web::main]
async fn main() {
    let client = Client::new("47.95.1.251:8890".to_string());
    let a = client
        .get_candles_until_last("HUOBI:BTCUSDT.1d".to_string(), 2)
        .await
        .unwrap();
    println!("Result: {:?};", a);
    let b = a.get(0).unwrap();
    let c = client
        .get_candles_since("HUOBI:BTCUSDT.1d".to_string(), b.ts, 1)
        .await;
    println!("C: {:?}", c.unwrap());
}
