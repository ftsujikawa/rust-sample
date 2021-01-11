use reqwest::StatusCode;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://openccpm.com/unknown.txt";
    println!("call {}", url);
    let res = reqwest::get(url).await?;
    match res.status() {
        StatusCode::OK => {
            let body = res.text().await?;
            println!("response is \n{}", body);
        },
        StatusCode::NOT_FOUND => {
            println!("error: 目的のページがありませんでした。");
        },
        _ => {
            println!("error: その他のエラーが発生しました。");
        },
    }
    Ok(())
}
