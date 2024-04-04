extern crate crossbeam_channel;
extern crate rand;
extern crate scheduled_thread_pool;
extern crate bma_benchmark;
use bma_benchmark::staged_benchmark_print_for;
use bma_benchmark::staged_benchmark_finish_current;
use bma_benchmark::staged_benchmark_start;
use std::sync::{Arc, Mutex};
use std::{thread, vec};
// Port of https://www.rabbitmq.com/tutorials/tutorial-one-python.html. Run this
// in one shell, and run the hello_world_publish example in another.
use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};
#[path = "../stock.rs"]
mod stock;
use stock::{stocking, Order};

fn main(){
    let shared_order = Arc::new(Mutex::new(vec![]));
    let shared_order_clone = Arc::clone(&shared_order); // Clone shared_order
    thread::spawn(move || loop {
        receiver(&shared_order);
    });
    stocking(&shared_order_clone);
}

fn receiver(shared_order: &Arc<Mutex<Vec<Order>>>) -> Result<()> {
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
                let mut option = parts[1].parse().unwrap();
                option = if option == "1" {
                    "buys".to_string()
                } else {
                    "sells".to_string()
                };
                let stock_id = parts[2].parse().unwrap();
                let price = parts[3].parse().unwrap();
                let order = Order {
                    broker: broker_id,
                    option,
                    stock_id,
                    price,
                    status: "On Going".to_string(),
                };
                println!("Broker {:?} wants to {:?} stock {:?} at price {:?}. STATUS:{:?}",order.broker,order.option,order.stock_id,order.price,order.status);
                consumer.ack(delivery)?;
                let mut shared_orders = shared_order.lock().unwrap();
                shared_orders.push(order.clone());

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