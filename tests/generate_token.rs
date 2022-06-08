use dotenv::dotenv;

#[tokio::test]
async fn test_generate_token(){
    dotenv().ok();
    let cons_key = std::env::var("CONSUMER_KEY").unwrap();
    let cons_secret = std::env::var("CONSUMER_SECRET").unwrap();

    let fake_cons_key = std::env::var("FAKE_CONSUMER_KEY").unwrap();
    let fake_cons_secret = std::env::var("FAKE_CONSUMER_SECRET").unwrap();

    //success status if there is internet
    let client = mpesa_async::Client::new(cons_key, cons_secret,true).await.unwrap();
    assert!(client.req_secs.unwrap()>1);

    //failure should return a ClientCreationError
    match mpesa_async::Client::new(fake_cons_key,
           fake_cons_secret,true).await{
        Ok(_) => panic!("Should never reach here, as is supposed to fail"),
        Err(err) => assert!(err.error_code.len()>0)
    }
}
