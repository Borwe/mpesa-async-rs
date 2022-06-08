use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ValidateReply{
    ResultCode: u8,
    ResultDesc: String
}

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
