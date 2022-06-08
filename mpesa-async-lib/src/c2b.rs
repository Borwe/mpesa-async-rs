use serde::{Serialize,Deserialize};

#[allow(non_snake_case)]
#[cfg(feature = "mpesa_obj")]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ValidateReply{
    pub ResultCode: u8,
    pub ResultDesc: String
}

#[allow(non_snake_case)]
#[cfg(feature = "mpesa_obj")]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ValidatorServerRequest {
    pub TransactionType: String,
    pub TransID: String,
    pub TransTime: String,
    pub TransAmount: String,
    pub BusinessShortCode: String,
    pub BillRefNumber: String,
    pub InvoiceNumber: String,
    pub OrgAccountBalance: String,
    pub ThirdPartyTransID: String,
    pub MSISDN: String,
    pub FirstName: String,
    pub MiddleName: String,
    pub LastName: String 
}


#[cfg(feature = "mpesa_obj")]
impl ValidateReply{
    pub fn success()->Self{
        ValidateReply{
            ResultCode: 0,
            ResultDesc: String::from("Accepted")
        }
    }

    pub fn failure()->Self{
        ValidateReply{
            ResultCode: 1,
            ResultDesc: String::from("Rejected")
        }
    }
}
