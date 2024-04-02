use std::sync::{mpsc::channel};
use std::thread;
mod stock;
use stock::stockData;
use std::time::Duration;

fn main() {
    let (tx, rx) = channel();

    // Start stockData thread
    thread::spawn(move || {
        stockData(tx);
    });

    // Main thread handling
    loop {
        let stocks = rx.recv().unwrap();
        for stock in &stocks {
            println!("STOCK: {:?}", stock);
        }
        println!("____________________________________________________");
    }
}