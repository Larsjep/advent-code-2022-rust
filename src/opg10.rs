use anyhow::{Result};
use num::abs;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::RangeInclusive;


pub fn to_range(def : &str) -> RangeInclusive<i32>
{
    let parts : Vec<_> = def.split('-').collect();
    std::ops::RangeInclusive::new(
        parts[0].parse::<i32>().unwrap(),
        parts[1].parse::<i32>().unwrap())
}

pub fn range_in_range(a : &RangeInclusive<i32>, b : &RangeInclusive<i32>) -> bool
{
    // println!("Testing ranges {:?}, {:?}", a,  b);
    for x in a.clone()
    {
        if !b.contains(&x)
        {
            // println!("{:?} is not in range {:?}", x, b);
            return false;
        }
    }
    true
}

pub fn range_overlap(a : &RangeInclusive<i32>, b : &RangeInclusive<i32>) -> bool
{
    // println!("Testing ranges {:?}, {:?}", a,  b);
    for x in a.clone()
    {
        if b.contains(&x)
        {
            // println!("{:?} is not in range {:?}", x, b);
            return true;
        }
    }
    false
}

pub fn opg10() -> Result<()> {
    println!("Helloo from opg10");
    let filename = "input10.txt";
    let input = read_to_string(filename).unwrap();
    let result = do_opg(&input);
    println!("Result is {:?}", result);
    let result2 = do_opg_part2(&input, 10000000);
    println!("Result part 2 is:");
    println!("{}", result2.unwrap());
    Ok(())
}

struct Cpu
{
    sumCycles: HashSet<i32>,
    result: i32,
    X: i32,
    cycle: i32,
    crt: String,
}

impl Cpu
{
    fn new() -> Self
    {
        Self { result: 0, X:1, sumCycles: HashSet::from_iter([20, 60, 100, 140, 180, 220]), cycle:0, crt: String::new() }
    }
    fn step(&mut self, num: i32)
    {
        let mut xpos = self.cycle % 40;
        for x in 0..num
        {
            if (abs(xpos - self.X) < 2)
            {
                self.crt += "#";
            }
            else
            {
                self.crt += ".";
            }
            self.cycle += 1;
            xpos += 1;
            if (xpos == 40)
            {
                self.crt += "\n";
                xpos = 0;
            }
            if self.sumCycles.contains(&self.cycle)
            {
                self.result += self.X * self.cycle;
            }
        }
    }
}


fn do_opg(input: &str) -> Result<i32> {

    let instructions = input.split('\n');

    let mut cpu = Cpu::new();
    for instruction in instructions
    {
        if instruction.starts_with("noop")
        {
            cpu.step(1);
        } else if instruction.starts_with("addx") {
            let value = instruction[5..].parse::<i32>().unwrap();
            cpu.step(2);
            cpu.X += value;            
        }
    }
    
    Ok(cpu.result)
}


fn do_opg_part2(input: &str, inst: i32) -> Result<String> {
    let instructions = input.split('\n');

    let mut cpu = Cpu::new();
    for instruction in instructions
    {
        if instruction.starts_with("noop")
        {
            cpu.step(1);
        } else if instruction.starts_with("addx") {
            let value = instruction[5..].parse::<i32>().unwrap();
            cpu.step(2);
            cpu.X += value;            
        }
    }
    
    cpu.crt.pop();
    Ok(cpu.crt)
}


#[cfg(test)]
mod tests
{
    fn get_input() -> String
    {
        let filename = "input10test.txt";
        read_to_string(filename).unwrap()
    }
    use crate::opg10::*;
    #[test]
    fn should_give_correct_output()
    {
        assert_eq!(do_opg(&get_input()).unwrap(), 13140);
    }

    #[test]
    fn should_give_correct_part2_result()
    {
        let result = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(do_opg_part2(&get_input(), 21).unwrap(), result);
    }
}