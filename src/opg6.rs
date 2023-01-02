use anyhow::{Result};
use std::fs::read_to_string;
use itertools::Itertools;


pub fn opg6() -> Result<()> {
    println!("Helloo from opg6");
    let filename = "input6.txt";
    let input = read_to_string(filename).unwrap();
    let result = do_opg(&input);
    println!("Result is {:?}", result);
    let result2 = do_opg_part2(&input);
    println!("Result part 2 is {:?}", result2);
    Ok(())
}

fn do_opg(input: &str) -> Result<i32> {
    for x in 4..input.len()
    {
        let lastfour = &input[x-4..x];
        if lastfour.len() == lastfour.chars().unique().count()
        {
            return Ok(x as i32);
        }
    }
    panic!("What????");
}


fn do_opg_part2(input: &str) -> Result<i32> {
    for x in 14..input.len()
    {
        let lastfour = &input[x-14..x];
        if lastfour.len() == lastfour.chars().unique().count()
        {
            return Ok(x as i32);
        }
    }
    panic!("What????");
}


#[cfg(test)]
mod tests
{
    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    use crate::opg6::*;
    #[test]
    fn should_give_correct_output()
    {
        assert_eq!(do_opg(INPUT).unwrap(), 7);
    }

    #[test]
    fn should_give_correct_part2_result()
    {
        assert_eq!(do_opg_part2(INPUT).unwrap(), 19);
    }
}