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


fn get_dir(dir: &str) -> Point
{
    match dir {
    
        "L" => Point::new(-1, 0),
        "R" => Point::new(1, 0),
        "U" => Point::new(0, 1),
        "D" => Point::new(0, -1),
        _ => panic!("Unexpected direction {dir}")
    }
}

#[derive(Debug, PartialEq, Hash, Clone)]
struct Point
{
    x: i32,
    y: i32,
}

impl Eq for Point {}

impl Point
{
    fn new(x: i32, y: i32) -> Self
    {
        Self { x:x, y:y}
    }
}

fn move_tail(head: &Point, mut tail: Point, dir: &Point) -> Point
{
    if abs(head.x - tail.x) > 1
    {
        tail = Point { x: tail.x + dir.x, y: head.y + dir.y };
    }
    if abs(head.y - tail.y) > 1
    {
        tail = Point { x: head.x + dir.x, y: tail.y + dir.y };
    }
    tail
}

fn get_move(head: &Point, tail: &Point, dir: &Point) -> Point
{
    if (abs(head.x - tail.x) > 1) && (abs(head.y - tail.y) > 1)
    {
        return Point { x: dir.x, y: dir.y }
    }

    if abs(head.x - tail.x) > 1
    {
        return Point { x: dir.x, y: head.y - tail.y }
    }
    if abs(head.y - tail.y) > 1
    {
        return Point { x: head.x - tail.x, y: dir.y }
    }
    Point::new(0,0)
}

fn do_opg(input: &str) -> Result<i32> {
    let mut positions = HashSet::<Point>::new();
    let commands = input.split('\n');
    let mut head = Point {x:0,y:0};
    let mut tail = Point {x:0,y:0};
    for command in commands
    {
        let parts : Vec<_> = command.split(' ').collect();
        let dir = get_dir(parts[0]);
        let len = parts[1].parse::<i32>().unwrap();
        println!("Moving {len}, {dir:?}");
        for _ in 0..len
        {
            head = Point{ x: head.x + dir.x, y: head.y + dir.y};
            //tail = move_tail(&head, tail.clone(), &dir);
            let m = get_move(&head, &tail, &dir);
            tail = Point::new(tail.x + m.x, tail.y + m.y);
            //{
            //    tail = Point { x:tail.x + dir.x, y:tail.y + dir.y }
            // }
            println!("Head ({}, {}), Tail ({}, {})", head.x, head.y, tail.x, tail.y);
            positions.insert(tail.clone());
        }
    }
    
    Ok(positions.len() as i32)
}


fn do_opg_part2(input: &str) -> Result<i32> {
    let mut positions = HashSet::<Point>::new();
    let commands = input.split('\n');
    //let mut head = Point {x:0,y:0};
    //let mut tail = Point {x:0,y:0};
    let mut rope = vec![Point::new(0,0); 10];
    let tail = 0;
    let head = rope.len() - 1;

    for command in commands.into_iter()
    {
        let parts : Vec<_> = command.split(' ').collect();
        let dir = get_dir(parts[0]);
        let len = parts[1].parse::<i32>().unwrap();
        println!("Moving {len}, {dir:?}");
        for _ in 0..len
        {
            rope[head]  = Point{ x: rope[head].x + dir.x, y: rope[head].y + dir.y};
            //let mut last_pos = rope[head - 1].clone();
            //rope[head - 1] = move_tail(&rope[head], rope[head -1].clone(), &dir);
            let mut m = dir.clone();
            for rope_pos in 0..head
            {
                let next_m = get_move(&rope[head - rope_pos], &rope[head - 1 - rope_pos], &m);
                rope[head - 1 - rope_pos] = Point::new(rope[head - 1 - rope_pos].x + next_m.x, rope[head - 1 - rope_pos].y + next_m.y);

                m = next_m;
            }
            //tail = move_tail(&head, tail.clone(), &dir);
            positions.insert(rope[tail].clone());
            println!("Head ({}, {}), Tail ({}, {})", rope[head].x, rope[head].y, rope[tail].x, rope[tail].y);
            for x in &rope
            {
                print!("({},{})", x.x, x.y);
            }
            println!();
            //dbg!(&rope);
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