use dotenv::dotenv;

#[tokio::test]
async fn test_generate_token(){
    dotenv().ok();
    let cons_key = std::env::var("CONSUMER_KEY").unwrap();
    let cons_secret = std::env::var("CONSUMER_SECRET").unwrap();
    //let client = mpesa_async::Client::new(cons_key, cons_secret,true).await.unwrap();
}
