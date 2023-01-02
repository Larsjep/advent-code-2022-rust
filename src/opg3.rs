use anyhow::{Result};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::fs::read_to_string;
use itertools::Itertools;


pub fn opg3() -> Result<()> {
    println!("Helloo from opg3");
    let filename = "input3.txt";
    let input = read_to_string(filename).unwrap();
    let result = do_opg3(&input);
    println!("Result is {:?}", result);
    let result2 = do_opg3_part2(&input);
    println!("Result part 2 is {:?}", result2);
    Ok(())
}

fn get_value(ch: char) -> i32
{
    let codepoint = ch as i32;
    match ch {
        'a' ..= 'z' => (codepoint - ('a' as i32)) + 1,
        'A' ..= 'Z' => (codepoint - ('A' as i32)) + 27,
        _ => panic!("Unknown char")
    }
}

fn do_opg3(input: &str) -> Result<i32> {
    let mut sum = 0;
    let bags = input.split('\n');
    for bag in bags
    {
        let (compartment1, compartment2)= bag.split_at(bag.len()/2);
        //println!("comp1: {:?}, comp2: {:?}", &compartment1, &compartment2);
        let hash1: HashSet<char> = HashSet::from_iter(compartment1.chars());
        for item in compartment2.chars()
        {
            if hash1.contains(&item)
            {
                //println!("Found item in both: {:?}", &item);
                sum += get_value(item);
                break;
            }
        }

    }
    Ok(sum)
}


fn do_opg3_part2(input: &str) -> Result<i32> {
    let mut sum = 0;
    for group in &input.split('\n').into_iter().chunks(3)
    {
        let group : Vec<_> = group.collect();
        let hash1: HashSet<char> = HashSet::from_iter(group[0].chars());
        let hash2: HashSet<char> = HashSet::from_iter(group[1].chars());
        let elf3 = group[2];
        for item in elf3.chars()
        {
            if hash1.contains(&item) && hash2.contains(&item)
            {
                //println!("Found item in both: {:?}", &item);
                sum += get_value(item);
                break;
            }
        }
    }
    Ok(sum)
}


#[cfg(test)]
mod tests
{
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    use crate::opg3::*;
    #[test]
    fn should_give_correct_output()
    {
        assert_eq!(do_opg3(INPUT).unwrap(), 157);
    }

    #[test]
    fn should_get_correct_value()
    {
        assert_eq!(get_value('a'), 1);
        assert_eq!(get_value('z'), 26);
        assert_eq!(get_value('A'), 27);
        assert_eq!(get_value('Z'), 52);
    }

    #[test]
    fn should_give_correct_part2_result()
    {
        assert_eq!(do_opg3_part2(INPUT).unwrap(), 70);
    }
}