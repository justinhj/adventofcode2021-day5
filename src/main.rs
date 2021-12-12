use scanf::sscanf;
use std::fmt;

const INPUT: &str = include_str!("../inputs/day5.txt");
const EXAMPLE: &str = include_str!("../inputs/example.txt");

struct Vec2{
    x: i64, 
    y: i64,
}

struct Line {
    p1: Vec2,
    p2: Vec2,
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("({:?},{:?}) -> ({:?},{:?})", 
                                 self.p1.x, self.p1.y,
                                 self.p2.x, self.p2.y))
    }
}

fn parse_input(input: &str) -> Vec<Line> {

    let lines: Vec<Line> = input
            .lines()
            .map(|l| {
                let mut x1: i64 = 0;         
                let mut x2: i64 = 0;         
                let mut y1: i64 = 0;         
                let mut y2: i64 = 0;         
                sscanf!(l, "{},{} -> {},{}", x1,y1,x2,y2);    
                Line{p1:Vec2{x:x1,y:y1},p2:Vec2{x:x2,y:y2}}

            })
            .collect();

    lines
}

fn main() {

    // Parse each line 
    // 0,9 -> 5,9
    // Plot the line into a hashmap
    // Iterate over the hashmap values to get the target candidates
    
    let lines = parse_input(EXAMPLE);
    println!("{:?}", lines);

}
