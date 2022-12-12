use petgraph::Graph;
use petgraph::algo::{dijkstra};
use petgraph::prelude::*;
use std::collections::HashMap;
use std::fs;
use petgraph::graph::EdgeReference;

fn main() {
    let content = fs::read_to_string("src/bin/day-12/input.txt").unwrap().replace("\r\n", "\n");
    let rows = content.split('\n').collect::<Vec<&str>>();

    let mut end = (usize::MAX, usize::MAX);
    let mut start = (usize::MAX, usize::MAX);

    let mut grid = rows.iter().map(|row| row.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut grid_map: HashMap<(usize, usize), NodeIndex> = HashMap::new();
    let mut graph = Graph::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let node = graph.add_node((i, j));

            if grid[i][j] == 'E' {
                grid[i][j] = 'z';
                end = (i, j);
            }

            if grid[i][j] == 'S' {
                start = (i, j);
            }

            grid_map.insert((i, j), node);
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if j != 0 && is_possible_elevate(grid[i][j], grid[i][j - 1]) {
                graph.extend_with_edges(&[(grid_map[&(i, j)], grid_map[&(i, j - 1)])]);
            }

            if grid[i].get(j + 1).is_some() && is_possible_elevate(grid[i][j], grid[i][j + 1]) {
                graph.extend_with_edges(&[(grid_map[&(i, j)], grid_map[&(i, j + 1)])]);
            }

            if i != 0 && is_possible_elevate(grid[i][j], grid[i - 1][j]) {
                graph.extend_with_edges(&[(grid_map[&(i, j)], grid_map[&(i - 1, j)])]);
            }

            if grid.get(i + 1).is_some() && is_possible_elevate(grid[i][j], grid[i + 1][j]) {
                graph.extend_with_edges(&[(grid_map[&(i, j)], grid_map[&(i + 1, j)])]);
            }
        }
    }

    //Part 1
    let res = dijkstra(&graph, grid_map[&start], None, |_: EdgeReference<'_, (i32, i32)>| 1);
    println!("Part 1 answer: {:?}", res.get(&grid_map[&end]).unwrap());

    // Part 2
    let mut paths: Vec<i32> = vec![];
    for (k, _) in grid_map.iter() {
        if grid[k.0][k.1] == 'a' {
            let res = dijkstra(&graph, grid_map[k], None, |_| 1);
            if res.get(&grid_map[&end]).is_some() {
                paths.push(*res.get(&grid_map[&end]).unwrap());
            }
        }
    }

    paths.sort();

    println!("Part 2 answer: {:?}", paths.first().unwrap());
}

fn is_possible_elevate(a: char, b: char) -> bool {
    let mut a_int = a as i32;
    let mut b_int = b as i32;

    if a.is_lowercase() { a_int -= 58; }
    if b.is_lowercase() { b_int -= 58; }

    b_int -= 1;

    a_int - b_int >= 0
}