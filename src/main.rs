#![feature(slice_partition_dedup)]

use std::fmt;
use scanf::sscanf;
use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/day5.txt");
const EXAMPLE: &str = include_str!("../inputs/example.txt");

#[derive(PartialEq,Eq,Hash,Copy,Clone,Debug,PartialOrd,Ord)]
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
                // TODO fix unused result warning
                Line{p1:Vec2{x:x1,y:y1},p2:Vec2{x:x2,y:y2}}

            })
            .collect();

    lines
}

// Add all the points on the line to the map
fn create_line(line: &Line, allow_diagonals: bool) -> Vec<Vec2> {
    let mut points: Vec<Vec2> = vec!();
    if line.p1.x == line.p2.x {
        let r = if line.p1.y < line.p2.y {
                line.p1.y ..= line.p2.y
            } else {
                line.p2.y ..= line.p1.y
            };
        r.for_each(|y| {
            points.push(Vec2{x:line.p1.x,y:y})
        });
        return points;
    } 
    else if line.p1.y == line.p2.y {
        let r = if line.p1.x < line.p2.x {
                line.p1.x ..= line.p2.x
            } else {
                line.p2.x ..= line.p1.x
            };
        r.for_each(|x| {
            points.push(Vec2{x:x,y:line.p1.y})
        });
        return points;
    } else if allow_diagonals {
        let mut y_inc = 0;
        let mut y = 0;
        
        let r = if line.p1.x > line.p2.x {
                y_inc = if line.p2.y > line.p1.y {-1} else {1};
                y = line.p2.y;
                line.p2.x ..= line.p1.x
            } else {
                y_inc = if line.p2.y < line.p1.y {-1} else {1};
                y = line.p1.y;
                line.p1.x ..= line.p2.x
            };
        r.for_each(|x| {
            points.push(Vec2{x,y});
            y += y_inc    
        });

        return points;
    } else {
        return points;
    }
}

fn solve(input: &Vec<Line>, allow_diagonals: bool) -> i64 {
    let mut points: Vec<Vec2> = vec!();

    for line in input {
        points.append(&mut create_line(line, allow_diagonals))
    }
    points.sort();

    let duplicates: Vec<Vec2> = points.into_iter().duplicates().collect();

    duplicates.len().try_into().unwrap()
}

fn main() {
    let lines = parse_input(EXAMPLE);
    println!("part1 example {:?}", solve(&lines, false));
    println!("part2 example {:?}", solve(&lines, true));

    let lines_input = parse_input(INPUT);
    println!("part1 {:?}", solve(&lines_input, false));
    println!("part2 {:?}", solve(&lines_input, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_line_creation() {
        let line5 = parse_input("0,0 -> 4,0");
        let line5_vec = create_line(&line5[0],false);

        let line5r = parse_input("4,0 -> 0,0");
        let line5r_vec = create_line(&line5r[0],false);

        assert_eq!(line5_vec, line5r_vec);
        assert_eq!(line5_vec.len(), 5);
    }

    #[test]
    fn test_diagonal_line_creation() {
        let lines = parse_input("7,9 -> 9,7");
        let line_vec = create_line(&lines[0],false);
        assert_eq!(line_vec.len(), 0);

        let line_vec_diag = create_line(&lines[0],true);
        assert_eq!(line_vec_diag.len(), 3);
    }

    #[test]
    fn test_vertical_line_creation() {
        let line5 = parse_input("0,4 -> 0,0");
        let line5_vec = create_line(&line5[0],false);

        let line5r = parse_input("0,0 -> 0,4");
        let line5r_vec = create_line(&line5r[0],false);

        assert_eq!(line5_vec, line5r_vec);
        assert_eq!(line5_vec.len(), 5);
    }

    #[test]
    fn test_invalid_line_creation() {
        let line = parse_input("0,4 -> 4,0");
        let line_vec = create_line(&line[0],false);

        assert_eq!(line_vec.len(), 0);
    }

    #[test]
    fn test_overlaps() {
        let line = parse_input("1,10 -> 1,109\n1,0 -> 1,119\n");
        let line_vec1 = create_line(&line[0],false);
        let line_vec2 = create_line(&line[1],false);

        assert_eq!(line.len(), 2);
        assert_eq!(line_vec1.len(), 100);
        assert_eq!(line_vec2.len(), 120);
        
        let s = solve(&line,false);
        assert_eq!(s, 100);
    }

    #[test]
    fn test_a_square() {
        let line = parse_input("10,10 -> 40,10\n10,10 -> 10,40\n10,40 -> 40,40\n40,40 -> 40,10\n");
        
        let s = solve(&line,false);
        assert_eq!(s, 4);
    }

    #[test]
    fn eq_vec2() {
        assert_eq!(Vec2{x:2,y:1}, Vec2{x:1 + 1,y:1});
    }
}
