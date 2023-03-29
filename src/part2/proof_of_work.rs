/*
 * @Author: 李帅帅 shineli97@163.com
 * @Date: 2023-03-22 17:35:18
 * @LastEditors: 李帅帅 shineli97@163.com
 * @LastEditTime: 2023-03-29 15:06:59
 * @FilePath: \blockchain-rust\src\part2\proof_of_work.rs
 * @Description:
 */
use bincode::{ serialize };
use super::{ block::Block, blockchain::BlockChain };
use crypto::{ sha2::Sha256, digest::Digest };

// 难度，指的是算出来的hash前24位必须是0
const TARGET_BITS: usize = 5;
const MAX_NONCE: i32 = 1000000000;
/**
 * 工作量证明
 * 原理：哈希与目标进行比较：先把哈希转换成一个大整数，然后检测它是否小于目标。
 *  当前实现的是比较前n位与目标值是否一致
 */
pub struct ProofOfWork {
    // 当前需要计算的区块
    block: Block,
    // 目标值
    target: String,
}

impl ProofOfWork {
    /**
     * 创建
     * 生成一个区块和它的目标值
     */
    pub fn new(block: Block) -> Self {
        let mut target = Vec::new();
        target.resize(TARGET_BITS, '0' as u8);
        let target = String::from_utf8(target).unwrap();
        println!("target为{}", target);
        ProofOfWork { block, target }
    }
    /**
     * 准备hash的数据
     */
    pub fn prepare_hash_data(&self) -> Vec<u8> {
        let res = (
            self.block.prev_block_hash.clone(),
            self.block.data.clone(),
            self.block.timestamp,
            TARGET_BITS,
            self.block.nonce,
        );
        let bytes = serialize(&res).unwrap();
        bytes
    }
    /*
     * 开始工作量计算
     */
    pub fn run(&mut self) {
        println!("Mining the block containing ：{}", self.block.data[0]);
        while !self.validate() {
            self.block.nonce += 1;
        }
    }
    /**
     * 验证计算
     * 仅寻找前n位一直的hash
     */
    pub fn validate(&mut self) -> bool {
        let current_data = self.prepare_hash_data();
        let mut current_hash = Sha256::new();
        current_hash.input(&current_data[..]);
        let res = &current_hash.result_str()[0..TARGET_BITS] == self.target;
        res
    }
}
#[test]
pub fn test_proof_of_work() {
    let mut block_chain = BlockChain::new_genesis_block();
    block_chain.add_block(vec![String::from("I like donuts")]);
    block_chain.add_block(vec![String::from("thank you!")]);
    for block in block_chain.blocks.into_iter() {
        let mut proof_of_work = ProofOfWork::new(block);
        proof_of_work.run();
        println!("数据为：{:?}", proof_of_work.block.data);
        println!("目标值：{}", proof_of_work.target);
        println!("计算次数：{}",proof_of_work.block.nonce)
    }
}