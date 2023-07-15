use anyhow::{Result};
use std::fs::read_to_string;


pub fn opg9() -> Result<()> {
    println!("Helloo from opg");
    let filename = "input.txt";
    let input = read_to_string(filename).unwrap();
    let result = do_opg(&input);
    println!("Result is {:?}", result);
    let result2 = do_opg_part2(&input);
    println!("Result part 2 is {:?}", result2);
    Ok(())
}



fn do_opg(input: &str) -> Result<i32> {
    Ok(17)
}


fn do_opg_part2(input: &str) -> Result<i32> {
    Ok(18)
}


#[cfg(test)]
mod tests
{
    const INPUT: &str = "";
    use crate::opg11::*;
    #[test]
    fn should_give_correct_output()
    {
        assert_eq!(do_opg(INPUT).unwrap(), 22);
    }

    #[test]
    #[ignore]
    fn should_give_correct_part2_result()
    {
        assert_eq!(do_opg_part2(INPUT).unwrap(), 22);
    }
}