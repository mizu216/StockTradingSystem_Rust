use std::option;
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
use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};

struct Order {
    broker: i32,
    option: i32,
    stock_id: i32,
    price: i32,
    status: i32,
}
fn main(){
    let order_list: Vec<Order> = vec![

    ];
    let shared_order_list = Arc::new(Mutex::new(order_list));
    receiver();
    stocking(&shared_order_list);
}

fn receiver() -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;
    let queue = channel.queue_declare("hello", QueueDeclareOptions::default())?;
    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Waiting for messages. Press Ctrl-C to exit.");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                let parts: Vec<&str> = body.split(',').map(|s| s.trim()).collect();
                let broker_id = parts[0].parse().unwrap();
                let option = parts[1].parse().unwrap();
                let stock_id = parts[2].parse().unwrap();
                let price = parts[3].parse().unwrap();
                let order = Order {
                    broker: broker_id,
                    option,
                    stock_id,
                    price,
                    status: 1,
                };

                println!("Received Order: {:?}", order.stock_id);
                consumer.ack(delivery)?;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
    connection.close()?;
    Ok(())
}


#[derive(Debug, Clone)]
struct Stock {
    id:i32,
    name: String,
    v: i32,
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
        for ordering in order_list{
            println!("{:?}",ordering);
        }
        if stock.v >150{
            println!("BROCKER 1: buying stock {:?}", stock)
        }
    });

    loop {
    }
}