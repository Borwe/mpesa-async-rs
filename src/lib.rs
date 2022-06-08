use reqwest::Client as WebClient;
use reqwest::Method;

const SANDBOX_URL: &str = "https://sandbox.safaricom.co.ke/";
const MAIN_URL: &str = "https://safaricom.co.ke/";

struct Client {
    web_client: WebClient
}

impl Client {
    /// Create a new client Object
    async fn new(cons_key: String, cons_secret: String, test: bool) {
        let client = Client{ web_client: WebClient::new() };
        let req_builder = match test{
            true => client.web_client.request(Method::GET, SANDBOX_URL),
            false => client.web_client.request(Method::GET, MAIN_URL)
        };

        //(cons_key+String::from(":")+cons_secret).as_bytes().to_
    }
}
