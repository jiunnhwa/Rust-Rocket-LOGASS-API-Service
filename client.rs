use reqwest;
use std::error::Error;
use std::time::{Duration, Instant};

fn main(){
    let start = Instant::now();
    for n in 1..100 {
        post_items();
    }
    let duration = start.elapsed();
    println!("Time elapsed in post_items() is: {:?}", duration);    
}

//Insert items
#[tokio::main]
async fn post_items() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8000/log")
        .header("Content-Type", "application/json")
        .body("\"the exact body that is sent\"")
        .send()
        .await?
        .text()
        .await?;   
        
        println!("{:}", res);
        Ok(())
}


