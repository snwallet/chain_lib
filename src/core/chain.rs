use serde::{Serialize, Deserialize};
use std::fs;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use crate::core::block::Block;
use crate::core::utxo::Utxo;
use crate::core::tx::Tx;
use crate::core::func;

#[derive(Debug, Serialize, Deserialize, Clone,PartialEq)]
pub struct Chain {
    pub blocks: Vec<Block>
}

impl Chain {
    fn genesis() -> Block{
        let utxo:Utxo = Utxo {input: Tx { address: "00000000000000000000000000000000".to_string(), token: "0000000000000000000000000000000000000000000000000000000000000000".to_string(), amount: dec!(0.0), time: 0 }, output: Tx { address: "00000000000000000000000000000000".to_string(), token: "0000000000000000000000000000000000000000000000000000000000000000".into(), amount: dec!(0.0), time: 0 } };
        let height = 1;
        let prehash = "00000000000000000000000000000000".to_string();
        let time: u64 = 0;
        let transaction = utxo;
        let str = format!("{:?}{:?}{:?}{:?}", time.to_string(), height.to_string(), transaction.to_string(), prehash);
        let hash = func::sha256(&str);
        let block = Block{
            height,
            hash,
            time,
            transaction,
            prehash,
        };
        block.write();
        block
    }



    pub fn new() -> Self {
        let paths = fs::read_dir("./data").unwrap();
        let mut count:u128 = 0;
        let mut blocks = Vec::new();
        for _path in paths {
            count = count+1;
            let block = Block::read_block(&format!("./data/{}.block",&count));
            println!("load...{:?}.{:?}",count,block);
            blocks.push(block);
        }
        println!("{:?}",blocks.len());
        if blocks.len()==0 {
            blocks.push(Self::genesis());
        }
        Chain { blocks }
    }
    pub fn add_block(&mut self,utxo:Utxo)->String{
        let last_block = Self::last_block( self);
        let block = Block::new(utxo,&last_block.hash,last_block.height);
        let hash_str = block.clone().hash;
        self.blocks.push(block.clone());
        block.write();
        return hash_str;
    }



    pub fn last_block(&mut self)->&Block{
        &self.blocks[self.blocks.len()-1]
    }

    pub fn to_string(&mut self) -> String {
        let res = serde_json::to_string(&self).unwrap();
        res
    }

    pub fn check_chain(&self)->bool{
        if self.blocks.len()>1 {
            for (index, block) in self.blocks.iter().enumerate() {
                println!("blocks at index {}: {:?}", index, block);
                if index > 0 {
                    if self.blocks[index].prehash == self.blocks[index - 1].hash && block.check_block() {
                        continue;
                    } else {
                        return false
                    }
                }else { continue; }
            }
            return true
        }else{
            Block::check_block(&self.blocks[0])
        }
    }

    pub fn get_balance(&mut self,_address:&str,_token:&str)->Decimal{
        let blocks =  self.blocks.clone();
        let mut in_total:Decimal = dec!(0);
        let mut out_total:Decimal = dec!(0);
        for block in blocks.iter() {
            if _address == block.transaction.input.address && _token == block.transaction.input.token {
                in_total = in_total + block.transaction.input.amount;
            }
            if _address == block.transaction.output.address && _token == block.transaction.output.token {
                out_total = out_total + block.transaction.output.amount;
            }
        }
        println!("in total:{:?}",in_total);
        println!("out total:{:?}",out_total);
        out_total - in_total
    }

    pub fn get_address_tx(&mut self,_address:&str,_token:&str)->Vec<Tx>{
        let blocks =  self.blocks.clone();
        let mut txs:Vec<Tx> = vec![];
        for block in blocks.iter() {
            if _address == block.transaction.input.address {
                txs.push(block.clone().transaction.input);
            }
            if _address == block.transaction.output.address {
                txs.push(block.clone().transaction.output);
            }
        }
        txs
    }

    pub fn get_token_total(&mut self, token:&str) -> (Decimal, Decimal) {
        let _address = "00000000000000000000000000000000";
        let mut in_total:Decimal = dec!(0);
        let mut out_total:Decimal = dec!(0);
        let blocks =  self.blocks.clone();
        for block in blocks.iter() {
            if _address == block.transaction.input.address && token == block.transaction.input.token {
                in_total = in_total + block.transaction.input.amount;
            }
            if _address == block.transaction.output.address && token == block.transaction.output.token {
                out_total = out_total + block.transaction.output.amount;
            }
        }
        (in_total,out_total)
    }

    pub fn get_block_by_height(&mut self, height:u128)->String{
        for block in self.blocks.iter(){
            if block.height == height {
                return block.to_string();
            }
        }
        return "".to_string()
    }

    pub fn get_block_by_hash(&mut self, hash:&str)->String{
        for block in self.blocks.iter(){
            if block.hash == hash {
                return block.to_string();
            }
        }
        return "".to_string()
    }


    //
    // pub fn get_address_tx(&mut self,address:&str,token:&str){
    //
    // }
    //

}
