extern crate crossbeam_channel;
extern crate rand;
extern crate scheduled_thread_pool;

use crossbeam_channel::unbounded;
use rand::Rng;
use scheduled_thread_pool::ScheduledThreadPool;
use std::sync::{mpsc::channel, Arc, Mutex};
use std::time::Duration;
use std::{thread, vec};

#[derive(Debug, Clone)]
struct Stock {
    id:i32,
    name: String,
    v: i32,
}

pub fn stocking() {
    let (sel_s, sel_r) = unbounded();
    let (tx, rx) = channel();
    let sched = ScheduledThreadPool::new(5);
    let shared_stock = Arc::new(Mutex::new(vec![
        Stock {
            id: 1,
            name: "apl".to_string(),
            v: 100,
        },
        Stock {
            id: 2,
            name: "len".to_string(),
            v: 100,
        },
        Stock {
            id:3,
            name: "ibm".to_string(),
            v: 100,
        },
        Stock {
            id: 4,
            name: "msf".to_string(),
            v: 100,
        },
        Stock {
            id: 5,
            name: "del".to_string(),
            v: 100,
        },
    ]));

    // create a stock selector task
    let stock_arc = shared_stock.clone();

    sched.execute_at_fixed_rate(
        Duration::from_micros(0),
        Duration::from_secs(1),
        move || {
            let mut rng = rand::thread_rng();
            let r_stock = rng.gen_range(0..5);
    
            // send index of selected stock to incrementor
            sel_s.send(r_stock).unwrap();
        },
    );

    //incrementor tasks
    for _i in 1..4 {
        let tx1 = tx.clone();
        let receiver = sel_r.clone();
        let stocks_arc = shared_stock.clone(); // Need to clone the Arc inside the loop
        thread::spawn(move || {
            loop {
                let r_stock = receiver.recv().unwrap();
                let mut rng = rand::thread_rng();
                let inc = rng.gen_range(0..20);
    
                // Modify the stock directly in the shared vector
                let mut stocks = stocks_arc.lock().unwrap();
                let stock = &mut stocks[r_stock];
                stock.v += inc;
    
                // Send the index of the modified stock to the broadcaster
                tx1.send(stock.clone()).unwrap();
            }
        });
    }

    // sender / broadcaster thread

    let (broad_s, broad_r) = unbounded();
    thread::spawn(move || {
        loop {
            let stock = rx.recv().unwrap();
            // broadcast the stock
            println!("Stock Broadcast: {:?}",stock);
            broad_s.send(stock.clone()).unwrap();
        }
    });

    // brocker tasks
    let brocker = broad_r.clone();
    thread::spawn(move || loop {
        let stock = brocker.recv().unwrap();
        if stock.v >150{
            println!("BROCKER 1: buying stock {:?}", stock)
        }
    });

    loop {
    }
}