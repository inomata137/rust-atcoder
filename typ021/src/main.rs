#[allow(unused)]
use proconio::{input, marker::{Chars, Usize1}};
#[allow(unused)]
use std::collections::*;
#[allow(unused)]
const USIZE_MAX: usize = 18_446_744_073_709_551_615usize;

fn main() {
    input! {
        n: usize,
        m: usize,
        nodes: [(Usize1, Usize1); m],
    }
    let nodes: Vec<(usize, usize)> = nodes;
    let mut g = vec![HashSet::new(); n];
    let mut g_rev = vec![HashSet::new(); n];

    for (a, b) in nodes {
        g[a].insert(b);
        g_rev[b].insert(a);
    }

    let mut label_asc = VecDeque::with_capacity(n);

    let mut visited = vec![false; n];
    for vertex in 0..n {
        if visited[vertex] {
            continue;
        }
        dfs(&g, vertex, &mut visited, &mut label_asc);
    }

    let mut visited = vec![false; n];
    let mut ans = 0usize;
    while let Some(vertex) = label_asc.pop_back() {
        if visited[vertex] {
            continue;
        }

        let group_size = dfs_rev(&g_rev, vertex, &mut visited);
        ans += group_size * (group_size - 1) / 2;
    }

    println!("{}", ans);
}

fn dfs (graph: &Vec<HashSet<usize>>, start: usize, visited: &mut Vec<bool>, label_asc: &mut VecDeque<usize>) {
    visited[start] = true;
    for &nex in &graph[start] {
        if !visited[nex] {
            dfs(graph, nex, visited, label_asc);
        }
    }
    label_asc.push_back(start);
}

fn dfs_rev(graph_rev: &Vec<HashSet<usize>>, start: usize, visited: &mut Vec<bool>) -> usize {
    let mut size = 1;
    visited[start] = true;
    for &nex in &graph_rev[start] {
        if !visited[nex] {
            size += dfs_rev(graph_rev, nex, visited);
        }
    }
    return size;
}
