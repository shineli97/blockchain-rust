/*
 * @Author: 李帅帅 shineli97@163.com
 * @Date: 2023-03-22 11:45:53
 * @LastEditors: 李帅帅 shineli97@163.com
 * @LastEditTime: 2023-03-23 18:16:38
 * @FilePath: \blockchain-rust\src\part2\block.rs
 * @Description:
 */
use std::time::{ UNIX_EPOCH, SystemTime };
use crypto::{ sha2::Sha256, digest::Digest };
#[derive(Debug, Clone)]
pub struct Block {
    pub timestamp: u128,
    pub data: Vec<String>,
    pub prev_block_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(data: Vec<String>, pre_block_hash: String) -> Block {
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        

        
        block.set_hash();
        block
    }
    pub fn set_hash(&mut self) {
        let str = self.timestamp.to_string() + &self.prev_block_hash + &self.data.join("");
        println!("before digest：{}", str);
        let mut sha256 = Sha256::new();
        sha256.input_str(&str);
        println!("after digest:{}", sha256.result_str());
        self.hash = sha256.result_str();
    }
    pub fn new_genesis_block() -> Block {
        let data = vec!["Genesis Block".to_string()];
        Self::new(data, "0000".to_string())
    }
}