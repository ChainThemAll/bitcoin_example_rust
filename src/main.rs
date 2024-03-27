use core::num;

use block_chain_example::{
    cli::{command, Cli},
    log::log_init,
};
use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    log_init(cli.log.unwrap());

    command(cli.clone()).await;
}

#[test]
fn test() {
    let v = vec![2, 2];

    // assert_eq!(my_sqrt(v, 2), vec![0, 1]);
    // use std::cmp::Ordering::{Equal, Greater, Less};
    let r = is_perfect_square(2147483647);
    println!("{}", r);
    pub fn is_perfect_square(num: i32) -> bool {
        use std::cmp::Ordering::*;
        let (mut left, mut right) = (0, num);
        while left <= right {
            let mid = (left + right) / 2;
            match ((mid * mid) as i64).cmp(&(num as i64)) {
                Less => left = mid + 1,
                Equal => return true,
                Greater => right = mid - 1,
            }
        }
        false
    }
}
