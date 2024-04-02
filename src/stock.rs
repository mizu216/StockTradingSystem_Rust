extern crate rand;
extern crate scheduled_thread_pool;

use rand::Rng;
use scheduled_thread_pool::ScheduledThreadPool;
use std::sync::{mpsc::channel, Arc, Mutex};
use std::time::Duration;
use std::thread;

#[derive(Debug, Clone)]
pub struct Stock {
    name: String,
    v: i32,
}

pub fn stockData(tx: std::sync::mpsc::Sender<Vec<Stock>>) {
    let sched = ScheduledThreadPool::new(5);
    let shared_stock = Arc::new(Mutex::new(vec![
        Stock {
            name: "apl".to_string(),
            v: 100,
        },
        Stock {
            name: "len".to_string(),
            v: 100,
        },
        Stock {
            name: "ibm".to_string(),
            v: 100,
        },
        Stock {
            name: "msf".to_string(),
            v: 100,
        },
        Stock {
            name: "del".to_string(),
            v: 100,
        },
    ]));
    
    loop{
        for i in 1..6 {
            let tx = tx.clone();
            let stock_arc = shared_stock.clone();
            sched.execute_at_fixed_rate(Duration::from_secs(0), Duration::from_secs(1), move || {
                let mut rng = rand::thread_rng();
                let r_stock = rng.gen_range(0..5);
                let inc = rng.gen_range(-50..50);
                let mut stocks = stock_arc.lock().unwrap();
                let stock = &mut stocks[r_stock];

                if stock.v + inc >= 0 {
                    stock.v += inc;
                } else {
                    stock.v = 0;
                }
                tx.send(stocks.clone()).unwrap();
            });
        }
    }
}