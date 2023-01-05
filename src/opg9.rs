use anyhow::{Result};
use num::abs;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::RangeInclusive;
use std::cmp::max;


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

pub fn opg9() -> Result<()> {
    println!("Helloo from opg9");
    let filename = "input9.txt";
    let input = read_to_string(filename).unwrap();
    let result = do_opg(&input);
    println!("Result is {:?}", result);
    let result2 = do_opg_part2(&input);
    println!("Result part 2 is {:?}", result2);
    Ok(())
}


fn get_dir(dir: &str) -> (i32, i32)
{
    match dir {
    
        "L" => (-1, 0),
        "R" => (1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!("Unexpected direction {dir}")
    }
}

fn move_tail(head: (i32, i32), mut tail: (i32,i32), xdir: i32, ydir: i32) -> (i32,i32)
{
    if abs(head.0 - tail.0) > 1
    {
        tail = (tail.0 + xdir, head.1);
    } else if abs(head.1 - tail.1) > 1
    {
        tail = (head.0, tail.1 + ydir);
    }
    tail
}

fn do_opg(input: &str) -> Result<i32> {
    let mut positions = HashSet::<(i32,i32)>::new();
    let commands = input.split('\n');
    let mut head = (0,0);
    let mut tail = (0,0);
    for command in commands
    {
        let parts : Vec<_> = command.split(' ').collect();
        let (xdir, ydir) = get_dir(parts[0]);
        let len = parts[1].parse::<i32>().unwrap();
        println!("Moving {len}, ({xdir},{ydir})");
        for _ in 0..len
        {
            head = (head.0 + xdir, head.1 + ydir);
            tail = move_tail(head, tail, xdir, ydir);
            println!("Head ({}, {}), Tail ({}, {})", head.0, head.1, tail.0, tail.1);
            positions.insert(tail);
        }
    }
    

    Ok(positions.len() as i32)
}


fn do_opg_part2(input: &str) -> Result<i32> {
    let mut positions = HashSet::<(i32,i32)>::new();
    let commands = input.split('\n');
    let mut head = (0,0);
    let mut tail = (0,0);
    for command in commands
    {
        let parts : Vec<_> = command.split(' ').collect();
        let (xdir, ydir) = get_dir(parts[0]);
        let len = parts[1].parse::<i32>().unwrap();
        println!("Moving {len}, ({xdir},{ydir})");
        for _ in 0..len
        {
            head = (head.0 + xdir, head.1 + ydir);
            tail = move_tail(head, tail, xdir, ydir);
            println!("Head ({}, {}), Tail ({}, {})", head.0, head.1, tail.0, tail.1);
            positions.insert(tail);
        }
    }
    

    Ok(positions.len() as i32)
}


#[cfg(test)]
mod tests
{
    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    use crate::opg9::*;
    #[test]
    fn should_give_correct_output()
    {
        assert_eq!(do_opg(INPUT).unwrap(), 13);
    }

    #[test]
    fn should_give_correct_part2_result()
    {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(do_opg_part2(input).unwrap(), 36);
    }
}