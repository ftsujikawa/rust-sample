#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://unknown.openccpm.com/blog";
    println!("call {}", url);
    if let Ok(res) = reqwest::get(url).await {
        let body = res.text().await?;
        println!("response is \n{}", body);
    }
    else {
        println!("error: Webサーバーが見つかりませんでした。");
    }
    Ok(())
}
