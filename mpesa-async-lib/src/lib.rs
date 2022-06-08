use reqwest::Client as WebClient;
use reqwest::Method;
use serde::{Deserialize, Serialize};

const SANDBOX_OAUTH_URL: &str = "https://sandbox.safaricom.co.ke/oauth/v1/generate";
const MAIN_OAUTH_URL: &str = "https://safaricom.co.ke/oauth/v1/generate";

pub mod c2b;

pub struct Client {
    web_client: WebClient,
    pub req_secs: Option<u32>,
    pub access_token: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
struct GeneratorReply{
    expires_in: String,
    access_token: String,
}

mod error{
    use super::*;
    #[derive(Deserialize, Serialize, Debug)]
    pub struct ClientCreationError{
        pub error_message: String,
        pub request_id: String,
        pub error_code: String
    }
}

impl Client {
    /// Create a new client Object, by using cons_key and cons_secret
    /// which are strings, that can be provided from Safaricom
    /// or use Daraja sandbox https://developer.safaricom.co.ke/ to get
    /// them
    pub async fn new(cons_key: String, cons_secret: String, test: bool)
        -> Result<Client, error::ClientCreationError>{
        let mut client = Client{ 
            web_client: WebClient::new(),
            req_secs: None,
            access_token: None
        };
        let req_builder = match test{
            true => client.web_client
                .request(Method::GET, SANDBOX_OAUTH_URL),
            false => client.web_client.request(Method::GET, MAIN_OAUTH_URL)
        };

        let mut to_encode = cons_secret;
        to_encode.insert_str(0,":");
        to_encode.insert_str(0,&cons_key);
        let mut encoded: String = base64::encode(to_encode);
        encoded.insert_str(0,"Basic ");
        let request = req_builder.header("Authorization",encoded.into_bytes())
            .header("Content-Type","application/json".as_bytes())
            .query(&[("grant_type","client_credentials")])
            .build().unwrap();

        let response  = client.web_client.execute(request).await.unwrap();
        if response.status().is_client_error(){
            match response
                .json::<std::collections::HashMap<String,String>>()
                .await{
                Ok(output) => Err(error::ClientCreationError{
                            error_message: output
                                .get("errorMessage").unwrap().clone(),
                            request_id: output
                                .get("requestId").unwrap().clone(),
                            error_code: output
                                .get("errorCode").unwrap().clone() 
                        })
                ,
                Err(_) => Err(error::ClientCreationError{
                              error_message: String::from("unkown"),
                              request_id: String::from("unkown"),
                              error_code: String::from("unkown") 
                          })
            }
        }else{
            let output = response
                .json::<GeneratorReply>()
                .await.unwrap();
            client.access_token = Some(output.access_token);
            client.req_secs = Some(output
                   .expires_in.parse::<u32>().unwrap());
            Ok(client)
        }
    }
}
