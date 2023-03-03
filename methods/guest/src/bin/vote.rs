// TODO: Rename this file to change the name of this method from METHOD_NAME

#![no_main]
#![no_std]  // std support is experimental, but you can remove this to try it

use types::PointVote;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let mut vote = PointVote::new(0, 0, 0);
    let mut next: u32 = env::read();

    while next != 0 {
        let (x, y, weight) = env::read();
        /*
        let x = env::read();
        let y = env::read();
        let weight = env::read();
        */
        vote = vote.add(PointVote::new(x, y, weight));
        next = env::read();
    }

    env::commit(&(vote.get_x(), vote.get_y(), vote.get_weight()));

}
