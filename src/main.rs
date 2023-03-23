/*
 * @Author: 李帅帅 shineli97@163.com
 * @Date: 2023-03-22 11:44:56
 * @LastEditors: 李帅帅 shineli97@163.com
 * @LastEditTime: 2023-03-23 14:21:20
 * @FilePath: \blockchain-rust\src\main.rs
 * @Description:
 */
mod part1;
mod part2;
use part2::proof_of_work::test_proof_of_work;
use crate::part1::blockchain::BlockChain;
fn main() {
    // test_part1()
    test_proof_of_work()
}
#[test]
fn test_part1() {
    println!("Hello, world!");
    let mut block_chain = BlockChain::new_genesis_block();
    block_chain.add_block(vec!["I 'm fine!".to_string()]);
    block_chain.add_block(vec!["thank you!".to_string()]);

    for block in block_chain.blocks.iter() {
        println!("================");
        println!("data:{:?}", block.data);
        println!("hash:{:?}", block.hash);
        println!("prev_block_hash:{:?}", block.prev_block_hash);
        println!("timestamp:{:?}", block.timestamp);
    }
}

