// ABANDON HOPE ALL YE WHO ENTER HERE
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::rc::{Rc, Weak};

use bit_set::BitSet;
use infinitable::{Finite, Infinitable, Infinity};

pub fn part_one(input: &str) -> Option<u32> {
    let h = Heightmap::from(input);
    let g = Graph::from(&h);

    let start = Rc::clone(
        g.nodes
            .iter()
            .find(|node| matches!(node.borrow().square.kind, SquareKind::Start))
            .unwrap(),
    );

    let end = Rc::clone(
        g.nodes
            .iter()
            .find(|node| matches!(node.borrow().square.kind, SquareKind::End))
            .unwrap(),
    );

    let pred = g.sssp(Rc::clone(&start), true);

    let mut path = vec![Rc::clone(&end)];

    loop {
        let prev = pred[path.last().unwrap().borrow().idx];

        if prev.is_none() {
            break;
        }

        path.push(Rc::clone(&g.nodes[prev.unwrap()]));
    }

    path.reverse();

    Some(path.len() as u32 - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let h = Heightmap::from(input);
    let g = Graph::from(&h);

    let end = Rc::clone(
        g.nodes
            .iter()
            .find(|node| matches!(node.borrow().square.kind, SquareKind::End))
            .unwrap(),
    );

    let pred = g.sssp(Rc::clone(&end), false);

    let mut cache: Vec<Option<usize>> = vec![None; g.nodes.len()];

    for idx in (0..(g.nodes.len())).filter(|&idx| g.nodes[idx].borrow().square.elevation == 0) {
        let mut path = vec![Rc::clone(&g.nodes[idx])];
        cache[idx] = Some(0);

        loop {
            let prev = pred[path.last().unwrap().borrow().idx];

            let Some(prev) = prev else {
                if !matches!(g.nodes[path.last().unwrap().borrow().idx].borrow().square.kind, SquareKind::End) {
                    cache[idx] = None;
                }
                break;
            };

            if let Some(dist) = cache[prev] {
                cache[idx] = cache[idx].map(|d| d + dist);
            } else {
                cache[idx] = cache[idx].map(|d| d + 1);
            }

            path.push(Rc::clone(&g.nodes[prev]));
        }
    }

    cache.iter().flatten().min().map(|&x| x as u32)
}

#[derive(Debug, Clone)]
enum SquareKind {
    Start,
    End,
    Ordinary,
}
type ElevationType = u32;

#[derive(Debug, Clone)]
struct Square {
    elevation: ElevationType,
    kind: SquareKind,
}

impl Square {
    fn new(elevation: ElevationType, kind: SquareKind) -> Self {
        Square { elevation, kind }
    }

    fn can_step_to(&self, other: &Square) -> bool {
        self.elevation + 1 >= other.elevation
    }
}

#[derive(Debug)]
struct Heightmap {
    grid: Vec<Vec<Square>>,
}

impl From<&str> for Heightmap {
    fn from(s: &str) -> Self {
        Heightmap {
            grid: s
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            'S' => Square::new(0, SquareKind::Start),
                            'E' => Square::new(
                                'z' as ElevationType - 'a' as ElevationType,
 /* ..ahhh.................. */ SquareKind::End,
                            ),
                            _ => Square::new(
                                c as ElevationType - 'a' as ElevationType,
                                SquareKind::Ordinary,
                            ),
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Debug)]
struct Node {
    square: Square,
    outgoing: Vec<Weak<RefCell<Node>>>,
    incoming: Vec<Weak<RefCell<Node>>>,
    idx: usize,
}

impl Node {
    fn new(square: Square, idx: usize) -> Self {
        Node {
            square,
            outgoing: vec![],
            incoming: vec![],
            idx,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct HeapItem {
    priority: Infinitable<usize>, // important that this is first for lexicographic ordering
    node_idx: usize,
}

impl HeapItem {
    fn new(priority: Infinitable<usize>, node_idx: usize) -> Self {
        HeapItem { priority, node_idx }
    }
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Rc<RefCell<Node>>>,
}

impl Graph {
    fn new() -> Self {
        Graph { nodes: vec![] }
    }

    fn add_edge(&self, from: Rc<RefCell<Node>>, to: Rc<RefCell<Node>>) {
        from.borrow_mut().outgoing.push(Rc::downgrade(&to));
        to.borrow_mut().incoming.push(Rc::downgrade(&from));
    }

    // assemble predecessor table for single-source shortest path problem using
    // Dijkstra's algorithm
    fn sssp(&self, source: Rc<RefCell<Node>>, forwards: bool) -> Vec<Option<usize>> {
        let start_idx = source.borrow().idx;

        let mut dist: Vec<Infinitable<usize>> = vec![Infinity; self.nodes.len()];
        let mut pred: Vec<Option<usize>> = vec![None; self.nodes.len()];

        dist[start_idx] = Finite(0);

        let mut todo: BinaryHeap<Reverse<HeapItem>> = BinaryHeap::with_capacity(self.nodes.len());
        let mut done: BitSet<u32> = BitSet::with_capacity(self.nodes.len());

        todo.push(Reverse(HeapItem::new(Finite(0), start_idx)));

        while !todo.is_empty() {
            let HeapItem { node_idx: v, .. } = todo.pop().unwrap().0;

            if !done.contains(v) {
                done.insert(v);

                let n = self.nodes[v].borrow();
                let it = if forwards {
                    n.outgoing.iter()
                } else {
                    n.incoming.iter()
                };

                for u in it.map(|w| w.upgrade().unwrap().borrow().idx) {
                    let dist_thru_v =
                        Infinitable::finite_or_infinity(dist[v].finite().map(|d| d + 1));

                    if dist_thru_v < dist[u] {
                        dist[u] = dist_thru_v;
                        pred[u] = Some(v);
                    }

                    todo.push(Reverse(HeapItem::new(dist[u], self.nodes[u].borrow().idx)));
                }
            }
        }

        pred
    }
}

impl From<&Heightmap> for Graph {
    fn from(h: &Heightmap) -> Self {
        let mut g = Graph::new();

        let m = h.grid.len();
        let n = h.grid[0].len();

        for i in 0..m {
            for j in 0..n {
                let new = Rc::new(RefCell::new(Node::new(h.grid[i][j].clone(), g.nodes.len())));
                g.nodes.push(Rc::clone(&new));

                if i > 0 {
                    let node = g.nodes.get(n * (i - 1) + j).unwrap();
                    if node.borrow().square.can_step_to(&new.borrow().square) {
                        g.add_edge(Rc::clone(&node), Rc::clone(&new));
                    }
                    if new.borrow().square.can_step_to(&node.borrow().square) {
                        g.add_edge(Rc::clone(&new), Rc::clone(&node));
                    }
                }

                if j > 0 {
                    let node = g.nodes.get(n * i + (j - 1)).unwrap();
                    if node.borrow().square.can_step_to(&new.borrow().square) {
                        g.add_edge(Rc::clone(&node), Rc::clone(&new));
                    }
                    if new.borrow().square.can_step_to(&node.borrow().square) {
                        g.add_edge(Rc::clone(&new), Rc::clone(&node));
                    }
                }
            }
        }

        g
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
