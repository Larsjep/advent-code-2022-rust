#[macro_use] extern crate nom;


use anyhow::{Context, Result};
use std::io::{BufRead};
use std::io;
use std::fs::{File};
use std::path::Path;

pub mod opg3;
pub mod opg4;
pub mod opg5;
pub mod opg6;
pub mod opg7;
pub mod opg8;
pub mod opg9;

pub fn opg1() -> Result<()> {
    println!("Hello, opg 1!");
    let filename = "input1.txt";
    let contents = read_lines("input1.txt")
        .with_context(|| format!("{filename}"))?;

    let groups = split(contents,|line| line.as_ref().expect("Line should be ok").len() == 0);
    // for group in groups
    // {
    //     let mut sum = 0;
    //     for line in group
    //     {
    //         if let Ok(line) = line
    //         {
    //             sum += line.parse::<i32>().unwrap();
    //         }
    //         //let line = line?;
    //         //println!("Line {line}");
    //     }
    //     println!("Sum {sum}");
    // }
    let sums = groups.map(|group|
        {

        let mut sum = 0;
        for line in group
        {
            if let Ok(line) = line
            {
                sum += line.parse::<i32>().unwrap();
            }
        }
        sum
        });

    let mut sums : Vec<i32> = sums.into_iter().collect();
    // for sum in sums
    // {
    //     println!("Sum: {sum}");
    // }

    //println!("Max item: {:?}", sums.max());
    sums.sort();

    for top in sums.iter().rev().take(3)
    {
        println!("Top: {top}")
    }
    println!("Top sum: {:?}", sums.iter().rev().take(3).sum::<i32>());
    //content.split

        Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>
{
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}


struct Split<I, P> {
    iter: I,
    pred: P,
}

impl<I, P> Iterator for Split<I,P> where I: Iterator, P: Fn(&I::Item) -> bool {
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Vec<I::Item>> {
        let ref p = self.pred;
        let not_p = |x: &I::Item| !p(x);
        let mut i = self.iter.by_ref().skip_while(p).take_while(not_p);
        match i.next() {
            None    => None,
            Some(x) => {
                let mut v = Vec::new();
                v.push(x);
                v.extend(i);
                Some(v)
            }
        }
    }
}

fn split<I: Iterator, P: Fn(&I::Item) -> bool>(iter: I, pred: P) -> Split<I, P> {
    Split { iter: iter, pred: pred}
}

fn get_my_value(letter: &str) -> i32
{
    match letter
    {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("Unknown letter {letter}")
    }
}

fn get_their_value(letter: &str) -> i32
{
    match letter
    {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("Unknown letter {letter}")
    }
}

fn value_to_lose_draw_win(other: i32) -> &'static [i32]
{
    match other
    {
        1 => &[3,1,2],
        2 => &[1,2,3],
        3 => &[2,3,1],
        _ => panic!("Unknown inpue {other}")
    }
}

pub fn opg2() -> Result<()>
{
    println!("Opgave 2");
    let filename = "input1.txt";
    let contents = read_lines("input2.txt")
        .with_context(|| format!("{filename}"))?;
    let mut sum = 0;
    let mut sum2 = 0;
    for line in contents
    {
        let line = line?;
        let parts: Vec<&str> = line.split(" ").collect();
        let letter_my = parts[1];
        let letter_their = parts[0];

        let my_value = get_my_value(letter_my);
        let their_value = get_their_value(letter_their);

        let score = if my_value == their_value
        {
            3
        }
        else if my_value == 1 && their_value == 3
        {
            6
        }
        else if my_value == 2 && their_value == 1
        {
            6
        }
        else if my_value == 3 && their_value == 2
        {
            6
        }
        else
        {
            0
        };

        let mut score2 = value_to_lose_draw_win(their_value)[(my_value-1) as usize];
        if my_value == 2
        {
            score2 += 3;
        } else if my_value == 3
        {
            score2 += 6;
        }

        sum += my_value + score;     
        sum2 += score2;

    }
    println!("Total: {sum}");
    println!("Total: {sum2}");
    Ok(())
}