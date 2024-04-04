use amiquip::{Connection, Exchange, Publish, Result};
use get_user_input::{self, set_from_input};

fn main(){
    //broker preferences setting
    let _ = instruction("1".to_string(),"1".to_string(),"5".to_string(),"100".to_string());
    let _ = instruction("2".to_string(),"2".to_string(),"2".to_string(),"110".to_string());
}

fn instruction(broker_id:String,option:String,stock_num:String,stock_price:String) -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);
    let message = format!("{},{},{},{}",broker_id,option, stock_num, stock_price);
    exchange.publish(Publish::new(message.as_bytes(), "hello"))?;
    connection.close()
}