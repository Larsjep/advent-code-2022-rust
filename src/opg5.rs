use anyhow::{Result};
use std::fs::read_to_string;
use itertools::Itertools;

pub fn opg5() -> Result<()> {
    println!("Helloo from opg5");
    let filename_a = "input5a.txt";
    let input_a = read_to_string(filename_a).unwrap();
    let input_b = read_to_string("input5b.txt").unwrap();
    let result = do_opg5(&input_a, &input_b);
    println!("Result is {:?}", result);
    let result2 = do_opg5_part2(&input_a, &input_b);
    println!("Result part 2 is {:?}", result2);
    Ok(())
}

fn do_opg5(input_a: &str, input_b: &str) -> Result<String> {
    let mut buckets = Vec::<Vec::<char>>::new();
    let fills = input_a.split('\n');
    for fill in fills
    {

        for (index, filler) in fill.chars().chunks(4).into_iter().enumerate()
        {
            if buckets.len() <= index
            {
                buckets.push(Vec::<char>::new());
            }
            let item = filler.into_iter().nth(1).unwrap();
            if item != ' '
            {
                buckets[index].push(item);
            }
            print!("element: {}, ", item);
        }
        println!("");
    }

    for bucket in &mut buckets
    {
        bucket.reverse();
    }
    
    let commands = input_b.split('\n');
    for command in commands
    {
        let words: Vec<_> = command.split(' ').into_iter().collect();
        let count = words[1].parse::<u32>().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let to = words[5].parse::<usize>().unwrap() - 1;
        for _ in 0..count
        {
            let item = &buckets[from].pop().unwrap();
            buckets[to].push(*item);
        }
    }

    Ok(buckets.iter().map(|x| x.last().unwrap()).collect() )
}


fn do_opg5_part2(input_a: &str, input_b: &str) -> Result<String> {
    let mut buckets = Vec::<Vec::<char>>::new();
    let fills = input_a.split('\n');
    for fill in fills
    {

        for (index, filler) in fill.chars().chunks(4).into_iter().enumerate()
        {
            if buckets.len() <= index
            {
                buckets.push(Vec::<char>::new());
            }
            let item = filler.into_iter().nth(1).unwrap();
            if item != ' '
            {
                buckets[index].push(item);
            }
            print!("element: {}, ", item);
        }
        println!("");
    }

    for bucket in &mut buckets
    {
        bucket.reverse();
    }
    
    let commands = input_b.split('\n');
    for command in commands
    {
        let words: Vec<_> = command.split(' ').into_iter().collect();
        let count = words[1].parse::<usize>().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let to = words[5].parse::<usize>().unwrap() - 1;
        for _ in 0..count
        {
            let item = &buckets[from].pop().unwrap();
            buckets[to].push(*item);
        }
        let to_len = buckets[to].len();
        buckets[to][to_len-count..].reverse();
    }

    Ok(buckets.iter().map(|x| x.last().unwrap()).collect() )
}


#[cfg(test)]
mod tests
{
    const INPUT_A: &str = "    [D]    
[N] [C]    
[Z] [M] [P]";
    const INPUT_B: &str = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    use crate::opg5::*;
    #[test]
    fn should_give_correct_output()
    {
        assert_eq!(do_opg5(INPUT_A, INPUT_B).unwrap(), "CMZ");
    }

    #[test]
    fn should_give_correct_part2_result()
    {
        assert_eq!(do_opg5_part2(INPUT_A, INPUT_B).unwrap(), "MCD");
    }
}