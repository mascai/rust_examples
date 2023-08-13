
#[derive(Debug)]
enum BitcoinBlock {
    Block(i32, Box<BitcoinBlock>),
    Genesis
}

use crate::BitcoinBlock::{Block, Genesis};


fn main() {
    let genesis = Genesis;
    let block1 = Block(1, Box::new(genesis)); 
    let block2 = Block(2, Box::new(block1)); 

    print_block(block2);
}


fn print_block(block: BitcoinBlock) {
    match block {
        Genesis => println!("genesis block"),
        Block(id, prev_block) => {
            println!("block {id}");
            print_block(*prev_block);
        }
    }
}
