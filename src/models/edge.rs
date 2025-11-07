use serde::{Serialize,Deserialize};
use chrono::{DateTime,Utc};
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub struct Edge{
    pub from : String,
    pub to:String,
    pub created_at:DateTime<Utc>
}

impl Edge{
    pub fn new(start:String,to:String)->Self{
        Self{
            from:start,to,created_at:Utc::now()
        }
    }
}