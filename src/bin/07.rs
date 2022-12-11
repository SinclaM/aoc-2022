use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub fn part_one(input: &str) -> Option<u32> {
    let root = Dir::parse(input);

    Some(
        Dir::all_dirs(Rc::clone(&root))
            .filter(|dir| dir.borrow().size <= 100000)
            .map(|dir| dir.borrow().size)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let root = Dir::parse(input);

    let unused = 70000000 - root.borrow().size;

    Some(
        Dir::all_dirs(Rc::clone(&root))
            .map(|dir| dir.borrow().size)
            .filter(|size| unused + size >= 30000000)
            .min()
            .unwrap(),
    )
}

#[derive(Debug)]
struct Dir {
    name: String,
    size: u32,
    parent: Option<Weak<RefCell<Dir>>>,
    children: HashMap<String, Rc<RefCell<Dir>>>,
}

impl Dir {
    fn new(name: &str) -> Self {
        Dir {
            name: name.to_string(),
            size: 0,
            parent: None,
            children: HashMap::new(),
        }
    }

    fn parse(input: &str) -> Rc<RefCell<Dir>> {
        let root = Rc::new(RefCell::new(Dir::new("/")));
        let mut cwd = Rc::clone(&root);

        for line in input.lines().skip(1) {
            let tokens: Vec<_> = line.splitn(3, " ").collect();

            match tokens[..] {
                ["$", "cd", ".."] => {
                    let next = Weak::clone(cwd.borrow().parent.as_ref().unwrap());
                    cwd = next.upgrade().unwrap();
                }
                ["$", "cd", dirname] => {
                    Dir::add_child(Rc::clone(&cwd), Dir::new(dirname));
                    let next = Rc::clone(cwd.borrow().children.get(dirname).unwrap());
                    cwd = next;
                }
                [size, _] if size.parse::<u32>().is_ok() => {
                    cwd.borrow_mut().size += tokens[0].parse::<u32>().unwrap();
                }
                _ => {}
            }
        }

        Dir::compute_sizes(Rc::clone(&root));

        root
    }

    fn add_child(cwd: Rc<RefCell<Dir>>, mut child: Dir) {
        child.parent = Some(Rc::downgrade(&cwd));
        cwd.borrow_mut()
            .children
            .insert(child.name.clone(), Rc::new(RefCell::new(child)));
    }

    fn compute_sizes(cwd: Rc<RefCell<Dir>>) {
        let mut node = cwd.borrow_mut();
        node.children
            .iter()
            .for_each(|(_, child)| Dir::compute_sizes(Rc::clone(&child)));
        node.size += node
            .children
            .iter()
            .map(|(_, child)| child.borrow().size)
            .sum::<u32>();
    }

    fn all_dirs(cwd: Rc<RefCell<Dir>>) -> Box<dyn Iterator<Item = Rc<RefCell<Dir>>>> {
        let children = cwd
            .borrow()
            .children
            .iter()
            .map(|(_, child)| child)
            .cloned()
            .collect::<Vec<_>>();

        Box::new(
            std::iter::once(cwd).chain(
                children
                    .into_iter()
                    .map(|child| Dir::all_dirs(child))
                    .flatten(),
            ),
        )
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
