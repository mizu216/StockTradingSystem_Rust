use amiquip::{Connection, Exchange, Publish, Result};
use get_user_input::{self, set_from_input};

fn main(){
    loop{
        let mut option = "".to_string();
        let mut stock_num = "".to_string();
        let mut stock_price = "".to_string();
        let mut broker_id = "".to_string();
        println!("Input Broker ID:");
        set_from_input!(&mut broker_id);
        println!("Choose option type (1 = preorder/2 = presell):");
        set_from_input!(&mut option);
        println!("Choose stock id 1 - 60:");
        set_from_input!(&mut stock_num);
        println!("Input Price:");
        set_from_input!(&mut stock_price);
        if (broker_id.parse::<i32>().is_ok()==false||broker_id.parse::<i32>().unwrap()<1){
            println!("ERROR BROKER ID INPUT!!!!!")
        }
        else if(option.parse::<i32>().is_ok()==false||option.parse::<i32>().unwrap()<1||option.parse::<i32>().unwrap()>2){
            println!("ERROR OPTION INPUT!!!!!")
        }
        else if (stock_num.parse::<i32>().is_ok()==false||stock_num.parse::<i32>().unwrap()<1||stock_num.parse::<i32>().unwrap()>60) {
            println!("ERROR STOCK ID INPUT!!!!!")
        }
        else if (stock_price.parse::<i32>().is_ok()==false||stock_price.parse::<i32>().is_ok()==false||stock_price.parse::<i32>().unwrap()<0) {
            println!("ERROR PRICE INPUT!!!!!")
        }
        else{
            let _ = instruction(broker_id,option,stock_num,stock_price);
        }
        println!("_________________________________________________________")
    }
}

fn instruction(broker_id:String,option:String,stock_num:String,stock_price:String) -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);
    let message = format!("{},{},{},{}",broker_id,option, stock_num, stock_price);
    exchange.publish(Publish::new(message.as_bytes(), "hello"))?;
    connection.close()
}