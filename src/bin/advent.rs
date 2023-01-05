use advent_rust::{opg1, opg2};
use std::env;
use anyhow::{Result, anyhow};
extern crate num;
extern crate num_derive;
use advent_rust::opg3::opg3;
use advent_rust::opg4::opg4;
use advent_rust::opg5::opg5;
use advent_rust::opg6::opg6;
use advent_rust::opg7::opg7;
use advent_rust::opg8::opg8;
use advent_rust::opg9::opg9;

const FUNCS : &[fn() -> Result<()>] = &[ opg1, opg2, opg3, opg4, opg5, opg6, opg7, opg8, opg9 ];

fn main() -> Result<()> {
    println!("Starting");
    let args: Vec<String> = env::args().collect();
    let execise_num = args[1].parse::<usize>()? - 1;
    if execise_num >= FUNCS.len()
    {
        return Err(anyhow!("Invalid exercise number: {:?}", args[1]));
    }
    FUNCS[execise_num]()
}
