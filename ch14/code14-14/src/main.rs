#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = "masuda";
    let age = 50;
    let url = format!("http://172.18.221.110/get.php?name={}&age={}", name, age);
    println!("call {}", url);
    let res = reqwest::get(&url).await?;
    let body = res.text().await?;
    println!("response is \n{}", body);
    Ok(())
}
