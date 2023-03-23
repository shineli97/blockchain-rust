/*
 * @Author: 李帅帅 shineli97@163.com
 * @Date: 2023-03-22 17:35:18
 * @LastEditors: 李帅帅 shineli97@163.com
 * @LastEditTime: 2023-03-23 18:00:42
 * @FilePath: \blockchain-rust\src\part2\proof_of_work.rs
 * @Description:
 */
use std::ops::{ Shl, ShlAssign };
use super::{ block::Block, blockchain::BlockChain };
use crypto::{ sha2::Sha256, digest::Digest };

// 难度，指的是算出来的hash前24位必须是0
const TARGET_BITS: u32 = 24;

pub struct ProofOfWork {
    block: Block,
    target: u128,
}

impl ProofOfWork {
    pub fn new(block: Block) -> Self {
        let mut target = 1;
        println!("target:{}", target);
        let step = 128 - TARGET_BITS;
        target = target.shl(step);
        ProofOfWork { block, target }
    }
    pub fn run(){
        
    }
}
#[test]
pub fn test_proof_of_work() {
    let mut block_chain = BlockChain::new_genesis_block();
    block_chain.add_block(vec!["I 'm fine!".to_string()]);
    block_chain.add_block(vec!["thank you!".to_string()]);
    for block in block_chain.blocks.into_iter() {
        let proof_of_work = ProofOfWork::new(block);
        println!("{:?}", proof_of_work.block.data);
        println!("{}", proof_of_work.target);
    }
}

#[test]
fn test_shl() {
    let a: u128 = 1;
    println!("b={:08b}", a);
    let b: u128 = a.shl(127);
    println!("b={:0128b}", b);
    let c = b.swap_bytes();
    println!("c={:08b}", c);
}

#[test]
fn test() {
    let data1 = String::from("I like donuts");
    let data2 = String::from("I like Tom");
    let target: u128 = 1;
    let mut sha256 = Sha256::new();
    sha256.input_str(&data1);
    
    println!("111={}", sha256.result_str());
    
    let mut sha2562 = Sha256::new();
    sha2562.input_str(&data2);
    println!("222={}", &sha2562.result_str());
}