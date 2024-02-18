mod types;

use rusty_money::{iso, Money};
use reqwest::{Client,Error};
use types::{CSFloatReturn, CSFloatParams};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let csfloat = CSFloatParams{category: 1, max_price: 550, limit: 10};
    println!("calling csfloat..");
    
    let base_url = format!("https://csfloat.com/api/v1/listings?type=buy_now&category={}&sort_by=highest_discount&max_price={}&limit={}", csfloat.category, csfloat.max_price, csfloat.limit);
    let csfloat = Client::new().get(base_url).send().await?.json().await?;

    let response: Vec<CSFloatReturn> = serde_json::from_value(csfloat).unwrap();
    for item in response  {

        let floatprice = item.price as f64;
        let floatpredicted = item.reference.predicted_price as f64;

        let discount = ((floatpredicted - floatprice) / floatpredicted) * (100 as f64);
        let formatted_discount = format!("{:.2}", discount);
        let mon = Money::from_minor(item.price, iso::USD); 
        println!("{}, Price: {}, Discount: {}% | LINK: https://csfloat.com/item/{}", item.item.market_hash_name, mon.to_string(), formatted_discount, item.id);
    }
    Ok(())
}



