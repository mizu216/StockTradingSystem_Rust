use amiquip::{Connection, Exchange, Publish, Result};
use get_user_input::{self, set_from_input};

fn main(){
    loop{
        let mut option = "".to_string();
        let mut stock_num = "".to_string();
        let mut stock_price = "".to_string();
        println!("Choose option type (1 = preorder/2 = presell):");
        set_from_input!(&mut option);
        println!("Choose stock id 1 - 60:");
        set_from_input!(&mut stock_num);
        println!("Input Price:");
        set_from_input!(&mut stock_price);
        let message_type = option.trim();
        let _ = instruction(option,stock_num,stock_price);
    }
}

fn instruction(option:String,stock_num:String,stock_price:String) -> Result<()> {
    // Open connection.
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let exchange = Exchange::direct(&channel);
    let message = format!("Stock number: {}, Sell price: {}", stock_num, stock_price);
    // Publish a message to the "hello" queue.
    exchange.publish(Publish::new(message.as_bytes(), "hello"))?;

    connection.close()
}