use anyhow::{Result};
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

pub fn opg8() -> Result<()> {
    println!("Helloo from opg8");
    let filename = "input8.txt";
    let input = read_to_string(filename).unwrap();
    let result = do_opg(&input);
    println!("Result is {:?}", result);
    let result2 = do_opg_part2(&input);
    println!("Result part 2 is {:?}", result2);
    Ok(())
}

fn visiable(grid: &Vec<Vec<u32>>, mut x: i32, mut y: i32, xdir: i32, ydir: i32) -> bool
{
    let treesize = grid[x as usize][y as usize];
    let xsize = grid[0].len() as i32;
    let ysize = grid.len() as i32;
    while x > 0 && x < xsize-1 && y > 0 && y < ysize-1 {
        x += xdir;
        y += ydir;
        if grid[x as usize][y as usize] >= treesize
        {
            return false;
        }
    }
    true
}

fn scenic(grid: &Vec<Vec<u32>>, mut x: i32, mut y: i32, xdir: i32, ydir: i32) -> i32
{
    let treesize = grid[x as usize][y as usize];
    let xsize = grid[0].len() as i32;
    let ysize = grid.len() as i32;
    let mut score = 0;
    while x > 0 && x < xsize-1 && y > 0 && y < ysize-1 {
        x += xdir;
        y += ydir;
        score += 1;
        if grid[x as usize][y as usize] >= treesize
        {
            return score;
        }
    }
    score
}

fn do_opg(input: &str) -> Result<i32> {
    let mut sum = 0;
    let mut grid = vec![];
    let rows = input.split('\n');
    for (i, row) in rows.enumerate()
    {
        let mut line = vec![];
        for c in row.chars()
        {
            line.push(c.to_digit(10).unwrap() as u32);
        }
        grid.push(line)
    }
    let xsize = grid[0].len();
    let ysize = grid.len();
    dbg!("Xsize: {xsize}, Ysize: {ysize}");

    for y in 0..ysize as i32
    {
        for x in 0..xsize as i32
        {
            if visiable(&grid, x, y, 1, 0) ||
                visiable(&grid, x, y, -1, 0) ||
                visiable(&grid, x, y, 0, 1) ||
                visiable(&grid, x, y, 0, -1)
            {
                sum += 1;
            }

        }
    }

    Ok(sum)
}


fn do_opg_part2(input: &str) -> Result<i32> {
    let mut sum = 0;
    let mut grid = vec![];
    let rows = input.split('\n');
    for (i, row) in rows.enumerate()
    {
        let mut line = vec![];
        for c in row.chars()
        {
            line.push(c.to_digit(10).unwrap() as u32);
        }
        grid.push(line)
    }
    let xsize = grid[0].len();
    let ysize = grid.len();
    dbg!("Xsize: {xsize}, Ysize: {ysize}");

    for y in 0..ysize as i32
    {
        for x in 0..xsize as i32
        {
            let scenic_score = scenic(&grid, x, y, 1, 0) *
                scenic(&grid, x, y, -1, 0) *
                scenic(&grid, x, y, 0, 1) *
                scenic(&grid, x, y, 0, -1);
            sum = max(scenic_score, sum);
        }
    }

    Ok(sum)
}


#[cfg(test)]
mod tests
{
    const INPUT: &str = "30373
25512
65332
33549
35390";
    use crate::opg8::*;
    #[test]
    fn should_give_correct_output()
    {
        assert_eq!(do_opg(INPUT).unwrap(), 21);
    }

    #[test]
    fn should_give_correct_part2_result()
    {
        assert_eq!(do_opg_part2(INPUT).unwrap(), 8);
    }
}