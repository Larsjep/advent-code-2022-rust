use anyhow::{Result};
use itertools::Itertools;
use num::abs;
use std::collections::HashSet;
use std::fs::read_to_string;

pub fn opg11() -> Result<()> {
    println!("Helloo from opg11");
    let filename = "input11.txt";
    let input = read_to_string(filename).unwrap();
    let result = do_opg(&input, 3, None);
    println!("Result is {:?}", result);
    let result2 = do_opg(&input, 1, Some(10000));
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

#[derive(Debug)]
enum Operation
{
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation
{
    fn do_oper(&self, input: u64) -> u64
    {
        match self {
            Operation::Add(x) => input + x,
            Operation::Multiply(x) => input * x,
            Operation::Square => input * input,
        }
    }
}

#[derive(Debug)]
struct Monkey
{
    items: Vec<u64>,
    operation: Operation,
    test_divisor: u64,
    throw: [u64; 2],
    inspects: u64,
}

impl Monkey
{
    fn new<'a, I>(infos: I) -> Self
    where
        I: Iterator<Item = &'a str>
    {
        let lines = infos.collect::<Vec<_>>();
        let items = lines[1][18..].split(", ").map(|x| x.parse::<u64>().unwrap()).collect();
        let operation = &lines[2][23..];
        let operation = if operation.starts_with("* old")
        {
            Operation::Square
        } else if operation.starts_with("*")
        {
            Operation::Multiply(operation[2..].parse::<u64>().unwrap())
        } else
        {
            Operation::Add(operation[2..].parse::<u64>().unwrap())
        };

        let test_divisor = lines[3][21..].parse::<u64>().unwrap();

        let throw_true = lines[4][29..].parse::<u64>().unwrap();
        let throw_false = lines[5][30..].parse::<u64>().unwrap();

        Self {
            items,
            operation,
            test_divisor,
            throw: [throw_true, throw_false],
            inspects: 0,
        }
    }

    fn process_items(&mut self, divider: u32, modulus: u64) -> Vec<(usize, u64)>
    {
        let mut result = Vec::new();
        self.inspects += self.items.len() as u64;
        for item in &self.items
        {
            let mut new_item = self.operation.do_oper(*item) / (divider as u64);
            new_item %= modulus;
            let test = (new_item % self.test_divisor) == 0;
            let to_monkey = if test { self.throw[0] } else { self.throw[1] } as usize;
            result.push((to_monkey, new_item));
        }
        self.items.clear();

        result
    }

    fn add(&mut self, item: u64)
    {
        self.items.push(item);
    }

}


fn do_opg(input: &str, divider: u32, rounds: Option<i32>) -> Result<u64> {

    let rounds = rounds.unwrap_or(20);
    let mut monkey_info = input.split('\n');

    let mut monkeys = Vec::<Monkey>::new();

    for monkey in monkey_info.chunks(7).into_iter()
    {
        monkeys.push(Monkey::new(monkey ));
    }
    
    let modulus = monkeys.iter().map(|x|x.test_divisor).fold(1, |acc, x| acc * x);

    //dbg!(monkeys);
    for _ in 0..rounds
    {
        for i in 0..monkeys.len()
        {
            let new_items = monkeys[i].process_items(divider, modulus);
            for new_item in new_items
            {
                monkeys[new_item.0].add(new_item.1);
            }
        }
        // dbg!(&monkeys);
    }

    //dbg!(&monkeys);
    let mut inspects : Vec<_> = monkeys.iter().map(|x| x.inspects).collect();
    inspects.sort();
    inspects.reverse();
    
    Ok(inspects[0] * inspects[1])
}


fn do_opg_part2(input: &str) -> Result<String> {
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
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
        input.to_string()
    }
    use crate::opg11::*;
    #[test]
    fn should_give_correct_output()
    {
        assert_eq!(do_opg(&get_input(), 3, None).unwrap(), 10605);
    }

    #[test]
    fn should_give_correct_part2_result()
    {
        assert_eq!(do_opg(&get_input(), 1, Some(10000)).unwrap(), 2713310158);
    }
}