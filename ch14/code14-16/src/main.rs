#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = [("name", "masuda"), ("age", "50")];
    let client = reqwest::Client::new();
    let url = "http://172.18.221.110/post.php";
    println!("call {}", url);
    let res = client.post(url).form(&params).send().await?;
    let body = res.text().await?;
    println!("response is \n{}", body);
    Ok(())
}
