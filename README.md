# Advent of Code 2021 

Day 5 in Rust

Originally I put the points for each line in a hashmap but due to a problem with the input data I thought my code was broken when it wasn't. I rewrote the code instead to generate vectors of line segments. By sorting this then using itertools duplicates function I can get the duplicated points and count them.


