use serde::{Serialize, Deserialize};
use rust_decimal::prelude::*;
use crate::core::func;

#[derive(Debug,Serialize, Deserialize, Clone,PartialEq)]
pub struct Tx{
    pub address: String,
    pub token: String,
    pub amount: Decimal,
    pub time: u64,
}

impl Tx {
    pub fn new(address: String, token: String, amount: Decimal) -> Tx {
        let time:u64 = func::timestamp();
        Tx{
            address,
            token,
            amount,
            time
        }
    }

    pub fn to_string(&self) ->String{
        let res = serde_json::to_string(&self).unwrap();
        res
    }
}
