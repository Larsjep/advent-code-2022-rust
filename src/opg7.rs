
use anyhow::{Result};
use std::borrow::{BorrowMut, Borrow};
use std::f32::consts::E;
use std::fs::read_to_string;
//use std::intrinsics::const_eval_select;
use itertools::Itertools;

use nom::AsChar;
use nom::character::complete::{ char, none_of, digit1, alpha1 };
use nom::bytes::complete::{ tag, is_not };
use nom::bytes::complete::take_while;
use nom::combinator::{ map, value };
use nom::sequence::{ pair, separated_pair };
use nom::branch::alt;
use nom::error::Error;
use nom::multi::separated_list0;
use core::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use std::fmt;
use std::cell::Ref;

#[derive(Debug)]
enum Files<'a>
{
    Directory(String),
    File(&'a str, usize),
}

#[derive(Debug)]
enum Commands<'a>
{
    Cd(&'a str),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug)]
struct Entry
{
    name: String,
    size: usize,
    dir: Option<Rc<RefCell<Dir>>>,
}

impl Entry
{
    fn get_size(&self) -> usize
    {
        match &self.dir
        {
            None => self.size,
            Some(dir) => (**dir).borrow().total_size()
        }
    }
}

#[derive(Debug)]
struct Dir
{
    pub parent: Option<Weak<RefCell<Dir>>>,
    pub files: Vec<Entry>
}

struct DirPtr(Rc<RefCell<Dir>>);

impl Dir {
    fn fmt(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        for entry in &self.files {
            entry.fmt(f, indent);
        }
        Ok(())
    }

    fn get_dir(&self, name: &str) -> Rc<RefCell<Dir>>
    {
        // println!("Getting dir: {:?}", name);
        self.files.iter().find(|f| f.name == name).unwrap().dir.as_ref().unwrap().clone()
    }

    fn total_size(&self) -> usize
    {
        self.files.iter().map(|f| f.get_size()).sum()
    }

}

fn get_dir_iter(dir: &Rc<RefCell<Dir>>) -> DirIterator
{
    DirIterator::new(dir)
}


impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f, 0)
    }
}

impl Entry
{
    fn fmt(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        match &self.dir
        {
            None => writeln!(f, "{:indent$}- {name:10} : {size:10} bytes", "", indent=indent, name=self.name, size=self.size)?,
            Some(dir) => {
                writeln!(f, "{:indent$}- {dirname}<dir>: {size:10} bytes", "", indent=indent, dirname=self.name, size=self.get_size())?;
                //let d : Ref<Dir> = (**dir).borrow();
                //d.fmt(f, ident + 4);
                (**dir).borrow().fmt(f, indent + 4)?;
            }
        }
        Ok(())
    }
}



fn parse(input: &str) -> Vec<Commands>
{
    let cd_parser = map(
        pair(tag::<_,_, Error<_>>("$ cd "), is_not("\n")),
        |result: (&str, &str)| Commands::Cd( result.1)
    );
    let ls_output_parser = alt((
        map(pair(tag::<_,_, Error<_>>("dir "), is_not("\n")),
            |result: (&str, &str)| Files::Directory(result.1.to_owned()) ),
        map(separated_pair(digit1,  char(' '), is_not("\n")),
            |result: (&str, &str)|Files::File(result.1, result.0.parse::<usize>().unwrap())
            )
    ));
    let ls_parser = map(
        pair(
            tag::<_,_, Error<_>>("$ ls\n"),
            separated_list0(tag("\n"), ls_output_parser)
        ),
        |result: (&str, Vec<Files>)| Commands::Ls(result.1)
    );
    let command_parser = alt((cd_parser, ls_parser));

    //let mut list_parser = take_while(AsChar::is_alpha);
    //let mut item_parser = alt((command_parser, list_parser));
    //let result = item_parser(input);
    let result = separated_list0(tag("\n"), command_parser)(input);
    if let Ok(list) = result
    {
        println!("{:?}", list.1);
        list.1
    }
    else
    {
        panic!("Incomplete file parsning");
    }
}

fn files_to_dir<'a>(files: &Files, parent: &Rc<RefCell<Dir>>) -> Entry
{
    match files
    {
        Files::Directory(name) => Entry { name: name.to_owned(), size:0, dir: Some(Rc::new(RefCell::new(Dir { files:vec![], parent: Some(Rc::downgrade(parent)) }))) },
        Files::File(name, size) => Entry { name: name.to_string(), size: *size, dir: None }
    }
}

pub fn opg7() -> Result<()> {
    println!("Helloo from opg7");
    let filename = "input7.txt";
    let input = read_to_string(filename).unwrap();
    let result = do_opg(&input);
    println!("Result is {:?}", result);
    let result2 = do_opg_part2(&input);
    println!("Result part 2 is {:?}", result2);
    Ok(())
}

fn value_below(size: usize) -> usize
{
    if size <= 100000
    {
        size
    }
    else {
        0
    }
}

fn total_below_100000_entry(entry: &Entry) -> usize
{
    match &entry.dir
    {
        None => 0,
        Some(dir) => value_below((**dir).borrow().total_size()) + total_below_100000(dir)
    }
}

fn total_below_100000(dir: &Rc<RefCell<Dir>>) -> usize
{
    (**dir).borrow().files.iter().map(|f| total_below_100000_entry(f)).sum()
}

fn parse_to_dir(input: &str) -> Rc<RefCell<Dir>>
{
    println!("Parsing input!");
    let commands = parse(input);
    println!("Input parsed!");
    //let root = Box::<Dir>::new(Dir { files: vec![] });
    let root = Rc::new(RefCell::new(Dir { files: vec![], parent: None }));
    let mut current = root.clone();
    for command in commands
    {
        //println!("Proccesing command: {:?}", command);
        //panic!("kdkd");
        match command {
            Commands::Cd(dir) => current = match dir {
                "/" => root.clone(),
                ".." => (*current).borrow().parent.as_ref().unwrap().upgrade().unwrap(),
                _ => (*current).borrow().get_dir(dir)
            },
            Commands::Ls(files) => {
                //println!("Adding files to dir");
                //current.borrow_mut().unwrap().files = files.iter().map(|f| files_to_dir(f) ).collect::<Vec<Entry>>();
                for x in files.iter()
                {
                    let new = files_to_dir(x, &current );
                    let mut borrowed = (*current).borrow_mut();
                    borrowed.files.push(new);
                    //borrowed.files.push(Entry{ name:"dkdk".to_string(), size:123, dir:None });
                    drop(borrowed);
                    //.files.push(new); 
                    //panic!("ddd");
                        // let mutref = &mut **(current.borrow_mut()).as_mut().unwrap();
                        // mutref.files.push(files_to_dir(x));
                        //(&mut **(current.borrow_mut()).as_mut().unwrap()).files.push(files_to_dir(x));
                }
                //RefCell::new(Option::Some(current.borrow_mut()));
            }
        }
    }
    root
} 

fn do_opg(input: &str) -> Result<usize> {
    let root = parse_to_dir(input);
    println!("Root:");
    println!("{}", (*root).borrow());
    
    //panic!("What????");
    let total = get_dir_iter(&root).filter(|x| x < &100000).sum();
    // 2104783
    //Ok(total_below_100000(&root))
    Ok(total)
}

struct DirIterator
{
    dir: Rc<RefCell<Dir>>,
    file_indencies: Vec<usize>,
    file_index: usize
}

impl DirIterator {
    fn new(dir: &Rc<RefCell<Dir>>) -> DirIterator
    {
        DirIterator { dir:dir.clone(), file_indencies: vec![], file_index:0 }
    }
}

impl Iterator for DirIterator {
    type Item = usize;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        // Increment our count. This is why we started at zero.
        if self.file_index == 0 {
            let dir = (*self.dir).borrow();
            self.file_index += 1;
            Some(dir.total_size())
        }
        else
        { 
            while self.file_index < (*self.dir).borrow().files.len() + 1
            {
                if ((*self.dir).borrow().files[self.file_index - 1].dir.is_some())
                {
                    let newdir = 
                        if let Some(sub_dir) = &(*self.dir).borrow().files[self.file_index - 1].dir
                        {
                            sub_dir.clone()
                        }
                        else { panic!("We should never get here???")
                        };
                    self.dir = newdir;
                    self.file_indencies.push(self.file_index);
                    self.file_index = 0;
                    return self.next();
                }
                self.file_index += 1;
            }
            if !self.file_indencies.is_empty()
            {
                self.file_index = self.file_indencies.pop().unwrap() + 1;
                let parent = (*self.dir).borrow().parent.as_ref().unwrap().clone();
                self.dir = parent.upgrade().unwrap();
                return self.next();
            }
            return None;
        }
    }
}


fn do_opg_part2(input: &str) -> Result<usize> {
    let root = parse_to_dir(input);
    let size_needed = 30000000 - (70000000 - (*root).borrow().total_size());
    println!("Size needed: {}", size_needed);
    let mut dir_sizes = get_dir_iter(&root).collect::<Vec<_>>();
    dir_sizes.sort();
    let candicates = dir_sizes.into_iter().filter(|x| x > &size_needed).collect::<Vec<_>>();


    println!("Directories by size: {:?}", candicates);
    Ok(candicates[0])
}


#[cfg(test)]
mod tests
{
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    use crate::opg7::*;
    #[test]
    fn should_give_correct_output()
    {
        assert_eq!(do_opg(INPUT).unwrap(), 95437);
    }

    #[test]
    fn should_give_correct_part2_result()
    {
        assert_eq!(do_opg_part2(INPUT).unwrap(), 24933642);
    }
}