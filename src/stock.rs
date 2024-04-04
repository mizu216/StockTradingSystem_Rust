extern crate crossbeam_channel;
extern crate rand;
extern crate scheduled_thread_pool;
extern crate bma_benchmark;
use bma_benchmark::staged_benchmark_print_for;
use bma_benchmark::staged_benchmark_finish_current;
use bma_benchmark::staged_benchmark_start;
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
        Stock { id: 1, name: "apl".to_string(), v: 100 },
        Stock { id: 2, name: "len".to_string(), v: 100 },
        Stock { id: 3, name: "ibm".to_string(), v: 100 },
        Stock { id: 4, name: "msf".to_string(), v: 100 },
        Stock { id: 5, name: "del".to_string(), v: 100 },
        Stock { id: 6, name: "goog".to_string(), v: 100 },
        Stock { id: 7, name: "amzn".to_string(), v: 100 },
        Stock { id: 8, name: "tsla".to_string(), v: 100 },
        Stock { id: 9, name: "nflx".to_string(), v: 100 },
        Stock { id: 10, name: "fb".to_string(), v: 100 },
        Stock { id: 11, name: "bac".to_string(), v: 100 },
        Stock { id: 12, name: "jpm".to_string(), v: 100 },
        Stock { id: 13, name: "wfc".to_string(), v: 100 },
        Stock { id: 14, name: "brk".to_string(), v: 100 },
        Stock { id: 15, name: "ge".to_string(), v: 100 },
        Stock { id: 16, name: "crm".to_string(), v: 100 },
        Stock { id: 17, name: "orcl".to_string(), v: 100 },
        Stock { id: 18, name: "intc".to_string(), v: 100 },
        Stock { id: 19, name: "amd".to_string(), v: 100 },
        Stock { id: 20, name: "nvda".to_string(), v: 100 },
        Stock { id: 21, name: "aapl".to_string(), v: 100 },
        Stock { id: 22, name: "msft".to_string(), v: 100 },
        Stock { id: 23, name: "adbe".to_string(), v: 100 },
        Stock { id: 24, name: "pypl".to_string(), v: 100 },
        Stock { id: 25, name: "v".to_string(), v: 100 },
        Stock { id: 26, name: "ma".to_string(), v: 100 },
        Stock { id: 27, name: "axp".to_string(), v: 100 },
        Stock { id: 28, name: "crm".to_string(), v: 100 },
        Stock { id: 29, name: "cost".to_string(), v: 100 },
        Stock { id: 30, name: "hd".to_string(), v: 100 },
        Stock { id: 31, name: "low".to_string(), v: 100 },
        Stock { id: 32, name: "tgt".to_string(), v: 100 },
        Stock { id: 33, name: "wmt".to_string(), v: 100 },
        Stock { id: 34, name: "amzn".to_string(), v: 100 },
        Stock { id: 35, name: "ibm".to_string(), v: 100 },
        Stock { id: 36, name: "hpq".to_string(), v: 100 },
        Stock { id: 37, name: "csco".to_string(), v: 100 },
        Stock { id: 38, name: "orcl".to_string(), v: 100 },
        Stock { id: 39, name: "crm".to_string(), v: 100 },
        Stock { id: 40, name: "qcom".to_string(), v: 100 },
        Stock { id: 41, name: "twtr".to_string(), v: 100 },
        Stock { id: 42, name: "snap".to_string(), v: 100 },
        Stock { id: 43, name: "uber".to_string(), v: 100 },
        Stock { id: 44, name: "lyft".to_string(), v: 100 },
        Stock { id: 45, name: "ebay".to_string(), v: 100 },
        Stock { id: 46, name: "etsy".to_string(), v: 100 },
        Stock { id: 47, name: "sq".to_string(), v: 100 },
        Stock { id: 48, name: "paypal".to_string(), v: 100 },
        Stock { id: 49, name: "shop".to_string(), v: 100 },
        Stock { id: 50, name: "etsy".to_string(), v: 100 },
        Stock { id: 51, name: "msi".to_string(), v: 100 },
        Stock { id: 52, name: "cat".to_string(), v: 100 },
        Stock { id: 53, name: "de".to_string(), v: 100 },
        Stock { id: 54, name: "utx".to_string(), v: 100 },
        Stock { id: 55, name: "hon".to_string(), v: 100 },
        Stock { id: 56, name: "mmm".to_string(), v: 100 },
        Stock { id: 57, name: "ge".to_string(), v: 100 },
        Stock { id: 58, name: "ba".to_string(), v: 100 },
        Stock { id: 59, name: "lmt".to_string(), v: 100 },
        Stock { id: 60, name: "gd".to_string(), v: 100 },        
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

    //stock incrementor
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

    //broadcaster stock
    let (broad_s, broad_r) = unbounded();
    thread::spawn(move || {
        loop {
            let stock = rx.recv().unwrap();
            // broadcast the stock
            println!("Stock Broadcast: {:?}",stock);
            broad_s.send(stock.clone()).unwrap();
        }
    });

    //order manager
    let brocker = broad_r.clone();
    let order_arc = Arc::clone(&order_list);
    thread::spawn(move || {
        loop{
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
        }
    }); 

    loop{
    }
}