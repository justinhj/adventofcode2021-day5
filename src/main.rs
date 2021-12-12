use scanf::sscanf;
use std::fmt;
use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day5.txt");
const TEST1: &str = include_str!("../inputs/test1.txt");
const EXAMPLE: &str = include_str!("../inputs/example.txt");

#[derive(PartialEq,Eq,Hash,Copy,Clone,Debug)]
struct Vec2{
    x: i64, 
    y: i64,
}

#[derive(PartialEq,Eq,Hash)]
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

// Add all the points on the line to the map
fn map_line(line: &Line, map: &mut HashMap<Vec2,i64>) {
    // println!("{:?} {:?}", line.p1, line.p2);
    // Vertical line
    if line.p1.x == line.p2.x {
        let r = if line.p1.y < line.p2.y {
                line.p1.y ..= line.p2.y
            } else {
                line.p2.y ..= line.p1.y
            };
        // println!("range r {:?}", r);
        r.for_each(|y| {
            let point = Vec2{x:line.p1.x, y:y};
            let count: i64 = map.get(&point).cloned().unwrap_or(0i64);
            map.insert(point, count + 1);
        })
    } else if line.p1.y == line.p2.y {
        let r = if line.p1.x < line.p2.x {
                line.p1.x ..= line.p2.x
            } else {
                line.p2.x ..= line.p1.x
            };
        // println!("range r {:?}", r);
        r.for_each(|x| {
            let point = Vec2{x:x, y:line.p1.y};
            let count: i64 = map.get(&point).cloned().unwrap_or(0i64);
            map.insert(point, count + 1);
        })
    } 
    // else {
    //     println!("skip {:?}", line);
    // }
}

fn solve(input: &Vec<Line>) -> i64 {
    let mut map: HashMap<Vec2,i64> = HashMap::new();

    for line in input {
        map_line(line, &mut map);
    }
    // println!("{:?}", map);
    println!("{:?}", map.values());
    map.values().filter(|v| **v > 1i64).count().try_into().unwrap()
}

fn main() {

    let lines = parse_input(EXAMPLE);
    // println!("{:?}", &lines);
    println!("{:?}", solve(&lines));

    let lines_input = parse_input(INPUT);
    // println!("{:?}\n{:?}", &lines_input, lines_input.len());
    println!("{:?}", solve(&lines_input));

    let lines_test1 = parse_input(TEST1);
    // println!("{:?}\n{:?}", &lines_test1, lines_input.len());
    println!("{:?}", solve(&lines_test1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_vec2() {
        assert_eq!(Vec2{x:2,y:1}, Vec2{x:1 + 1,y:1});
    }
}




