mod api;
mod model;
mod utils;
mod endpoint;

use std::io;

#[tokio::main]
async fn main() {

    let mut user_coin_name = String::new();
    let api_key = utils::get_api_key();
    
    println!("enter the name/id of a coin");
    


    match io::stdin().read_line(&mut user_coin_name) {
        Ok(_) => {
            let coin_name = user_coin_name.trim();
            match api::get_coin_price(coin_name, &api_key).await {
                Ok(price) => println!("The price of {} is USD: ${}", coin_name, price),
                Err(e) => eprint!("Error fetching data {}", e),
            }
        }
        Err(e) => {
            eprint!("Issue: {}", e);
        }
    }

   

}