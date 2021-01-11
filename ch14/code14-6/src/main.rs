#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = 7;
    let url = format!("http://openccpm.com/blog/?p={}", n);
    println!("call {}", url);
    let res = reqwest::get(&url).await?;
    let body = res.text().await?;
    println!("response is \n{}", body);
    Ok(())
}
