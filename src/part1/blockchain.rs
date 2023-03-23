/*
 * @Author: 李帅帅 shineli97@163.com
 * @Date: 2023-03-22 16:19:31
 * @LastEditors: 李帅帅 shineli97@163.com
 * @LastEditTime: 2023-03-23 13:36:37
 * @FilePath: \blockchain-rust\src\part1\blockchain.rs
 * @Description:
 */
use super::block::Block;
pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn add_block(&mut self, data: Vec<String>) {
        let last_index = self.blocks.len();
        let pre_block = self.blocks.get(last_index - 1).expect("pre block not found");
        let new_block = Block::new(data, pre_block.hash.clone());
        self.blocks.push(new_block);
    }

    pub fn new_genesis_block() -> BlockChain {
        BlockChain { blocks: vec![Block::new_genesis_block()] }
    }
}