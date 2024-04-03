extern crate crossbeam_channel;
extern crate rand;
extern crate scheduled_thread_pool;

use crossbeam_channel::unbounded;
use rand::Rng;
use scheduled_thread_pool::ScheduledThreadPool;
use std::sync::{mpsc::channel, Arc, Mutex};
use std::time::Duration;
use std::{thread, vec};

// Port of https://www.rabbitmq.com/tutorials/tutorial-one-python.html. Run this
// in one shell, and run the hello_world_publish example in another.
#[derive(Debug, Clone)]
pub struct Stock {
    id:i32,
    name: String,
    v: i32,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub(crate) broker: i32,
    pub(crate)option: String,
    pub(crate)stock_id: i32,
    pub(crate)price: i32,
    pub(crate)status: String,
}

pub fn stocking(order_list: &Arc<Mutex<Vec<Order>>>) {
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
                let inc = rng.gen_range(-20..20);
    
                // Modify the stock directly in the shared vector
                let mut stocks = stocks_arc.lock().unwrap();
                let stock = &mut stocks[r_stock];
                if stock.v + inc >0{
                    stock.v += inc;
                }
    
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
    let order_arc = Arc::clone(&order_list);
    thread::spawn(move || loop {
        let stock = brocker.recv().unwrap();
        let mut order_arcs = order_arc.lock().unwrap();
        for order in order_arcs.iter_mut() {
            if order.stock_id == stock.id && order.status=="On Going".to_string(){
                if order.option=="sells".to_string()&& order.price <= stock.v{
                    order.status = "Completed".to_string();
                    println!("Broker {:?} haved {:?} stock {:?} at price {:?}. STATUS:{:?}",order.broker,order.option,order.stock_id,stock.v,order.status);
                }
                else if order.option=="buys".to_string()&& order.price >= stock.v{
                    order.status = "Completed".to_string();
                    println!("Broker {:?} haved {:?} stock {:?} at price {:?}. STATUS:{:?}",order.broker,order.option,order.stock_id,stock.v,order.status);
                }
            }
        }
    });

    loop {
    }
}