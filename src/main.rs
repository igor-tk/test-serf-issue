use surf;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let req = surf::get("https://httpbin.org/get");
    let resp = surf::client().send(req).await;
    println!("Resp: {:?}", resp);
    Ok(())
}
