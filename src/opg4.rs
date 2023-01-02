use anyhow::{Result};
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

pub fn opg4() -> Result<()> {
    println!("Helloo from opg4");
    let filename = "input4.txt";
    let input = read_to_string(filename).unwrap();
    let result = do_opg4(&input);
    println!("Result is {:?}", result);
    let result2 = do_opg4_part2(&input);
    println!("Result part 2 is {:?}", result2);
    Ok(())
}

fn do_opg4(input: &str) -> Result<i32> {
    let mut sum = 0;
    let groups = input.split('\n');
    for group in groups
    {
        let group: Vec<_> = group.split(',').collect();
        let elf1 = to_range(group[0]);
        let elf2 = to_range(group[1]);
        if range_in_range(&elf1, &elf2) || range_in_range(&elf2, &elf1)
        {
            sum += 1;
        }

    }
    Ok(sum)
}


fn do_opg4_part2(input: &str) -> Result<i32> {
    let mut sum = 0;
    let groups = input.split('\n');
    for group in groups
    {
        let group: Vec<_> = group.split(',').collect();
        let elf1 = to_range(group[0]);
        let elf2 = to_range(group[1]);
        if range_overlap(&elf1, &elf2) || range_overlap(&elf2, &elf1)
        {
            sum += 1;
        }

    }
    Ok(sum)
}


#[cfg(test)]
mod tests
{
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    use crate::opg4::*;
    #[test]
    fn should_give_correct_output()
    {
        assert_eq!(do_opg4(INPUT).unwrap(), 2);
    }

    #[test]
    fn should_give_correct_part2_result()
    {
        assert_eq!(do_opg4_part2(INPUT).unwrap(), 4);
    }
}