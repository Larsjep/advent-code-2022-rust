use anyhow::{Result};
use std::{fs::read_to_string, collections::HashSet, cell::RefCell};

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

pub fn opg12() -> Result<()> {
    println!("Helloo from opg12");
    let filename = "input12.txt";
    let input = read_to_string(filename).unwrap();
    //let result = do_opg(&input);
    //println!("Result is {:?}", result);
    let result2 = do_opg_part2(&input);
    println!("Result part 2 is {:?}", result2);
    Ok(())
}

struct Grid
{
    grid: Vec<Vec<i32>>,
    min_moves: RefCell<Vec<Vec<i32>>>,
    start: Point,
    end: Point,
    size: Point,
}

impl Grid
{
    fn scan(&self, pos: Point, elevation: i32, moves: i32, visited: &HashSet<Point>) -> Vec<i32>
    {
        let mut new_vec = vec![];
        if (visited.contains(&pos))
        {
            return new_vec;
        }

        { 
            let mut min_moves = self.min_moves.borrow_mut();
            if min_moves[pos.y as usize][pos.x as usize] <= moves
            {
                return new_vec;
            }
            min_moves[pos.y as usize][pos.x as usize] = moves;
        }
        // println!("Test position: {:?}", pos);
        // if pos.x < 0 || pos.x >= self.size.x ||
        //    pos.y < 0 || pos.y >= self.size.y
        // {
        //     return new_vec;
        // }

        if pos == self.end
        {
            new_vec.push(moves);
            return new_vec;
        }

        let mut my_visited = visited.clone();
        my_visited.insert(pos.clone());
    
        for dir in [Point::new(0,1), Point::new(1,0), Point::new(0,-1), Point::new(-1,0)]
        {
            let new_pos = Point::new(pos.x + dir.x, pos.y + dir.y);
            if new_pos.x >= 0 && new_pos.x < self.size.x &&
                new_pos.y >= 0 && new_pos.y < self.size.y
            {
                let new_elevation = self.grid[new_pos.y as usize][new_pos.x as usize];
                if new_elevation < elevation ||
                new_elevation <= elevation+1
                {
                    let mut result = self.scan(new_pos, new_elevation, moves + 1, &my_visited);
                    new_vec.append(&mut result);
                }
            }

        }
    
        new_vec
    }

    fn scan_down(&self, pos: Point, elevation: i32, moves: i32, visited: &HashSet<Point>) -> Vec<i32>
    {
        let mut new_vec = vec![];
        if visited.contains(&pos)
        {
            return new_vec;
        }

        { 
            let mut min_moves = self.min_moves.borrow_mut();
            if min_moves[pos.y as usize][pos.x as usize] <= moves
            {
                return new_vec;
            }
            min_moves[pos.y as usize][pos.x as usize] = moves;
        }
        // if pos.x < 0 || pos.x >= self.size.x ||
        //    pos.y < 0 || pos.y >= self.size.y
        // {
        //     return new_vec;
        // }

        if elevation == 1
        {
            new_vec.push(moves);
            return new_vec;
        }

        let mut my_visited = visited.clone();
        my_visited.insert(pos.clone());
    
        for dir in [Point::new(0,1), Point::new(1,0), Point::new(0,-1), Point::new(-1,0)]
        {
            let new_pos = Point::new(pos.x + dir.x, pos.y + dir.y);
            if new_pos.x >= 0 && new_pos.x < self.size.x &&
            new_pos.y >= 0 && new_pos.y < self.size.y
            {
                let new_elevation = self.grid[new_pos.y as usize][new_pos.x as usize];
                if new_elevation == elevation ||
                   new_elevation >= elevation-1
                {
                    let mut result = self.scan_down(new_pos, new_elevation, moves + 1, &my_visited);
                    new_vec.append(&mut result);
                }
            }

        }
    
        new_vec
    }
    
    fn new(input: &str) -> Self
    {
        let mut grid = vec![];
        let mut min_moves = RefCell::new(vec![]);

        let mut start = Point::new(0,0);
        let mut end = Point::new(0,0);
    
        for (y, line) in input.split('\n').enumerate()
        {
            let mut ld = Vec::<i32>::new();
            let mut min_moves_line = Vec::<i32>::new();
            for (x, c) in line.chars().enumerate()
            {
                let elevation = match c {
                    'S' => { start.x = x as i32; start.y = y as i32; 1 },
                    'E' => { end.x = x as i32; end.y = y as i32; 26 },
                    x => x as i32 - ('a' as i32) + 1,
                };
                ld.push(elevation);
                min_moves_line.push(i32::MAX);
            }
            grid.push(ld);
            min_moves.borrow_mut().push(min_moves_line);
        }
        let size = Point { x:*&grid[0].len() as i32, y: *&(grid.len()) as i32 };
        println!("Data size: {:?}", size);
        Grid { grid, end, start, size, min_moves }
    }
}


fn do_opg(input: &str) -> Result<i32> {
    let grid = Grid::new(input);

    let solutions = grid.scan(grid.start.clone(), 1, 0, &HashSet::new());
    println!("{:?}", solutions);
    Ok(*solutions.iter().min().unwrap())
}


fn do_opg_part2(input: &str) -> Result<i32> {
    let grid = Grid::new(input);
    
    let solutions = grid.scan_down(grid.end.clone(), 26, 0, &HashSet::new());
    println!("{:?}", solutions);
    Ok(*solutions.iter().min().unwrap())
}


#[cfg(test)]
mod tests
{
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    use crate::opg12::*;
    #[test]
    fn should_give_correct_output()
    {
        assert_eq!(do_opg(INPUT).unwrap(), 31);
    }

    #[test]
    fn should_give_correct_part2_result()
    {
        assert_eq!(do_opg_part2(INPUT).unwrap(), 29);
    }
}