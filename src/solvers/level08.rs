use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use anyhow::Error;
use itertools::Itertools;
use regex::Regex;
use crate::api::Solver;

pub struct LevelSolver {
    instructions: String,
    node_map: HashMap<String, TreeNodeRef>,
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn lcmm(numbers: Vec<u64>) -> u64 {
    let mut result = numbers[0];

    for &num in &numbers[1..] {
        result = lcm(result, num);
    }

    result
}

#[derive(PartialEq, Clone)]
pub struct TreeNode {
    name: String,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

impl Debug for TreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TreeNode: {}", self.name).expect("TODO: panic message");
        if self.left.is_some() {
            write!(f, "{}", self.left.as_ref().unwrap().borrow().name).expect("TODO: panic message");
        }
        if self.right.is_some() {
            write!(f, "{}", self.right.as_ref().unwrap().borrow().name).expect("TODO: panic message");
        }
        Ok(())
    }
}

type TreeNodeRef = Rc<RefCell<TreeNode>>;

fn traverse_tree(instructions: &str, start_node: TreeNodeRef, mut stop_f: impl FnMut(&TreeNodeRef) -> bool) -> usize {
    let mut instuction_pointer = 0;
    let mut stack = vec![start_node];

    while !stack.is_empty() {
        let current: Rc<RefCell<TreeNode>> = stack.pop().unwrap();
        println!("{:?}", current);

        if stop_f(&current) {
            break;
        }

        let inst = instructions.as_bytes()[instuction_pointer % instructions.len()] as char;
        instuction_pointer += 1;
        if inst.eq(&'L') {
            if let Some(left) = &current.borrow().left {
                stack.push(left.to_owned());
            };
        } else if inst.eq(&'R') {
            if let Some(right) = &current.borrow().right {
                stack.push(right.to_owned());
            };
        } else {
            panic!("invalid instruction");
        }
    }

    instuction_pointer
}

impl Solver for LevelSolver {
    fn new(input: String) -> Self {
        let (instructions, nodes_input) = parse(&input).unwrap();

        let mut node_map = HashMap::new();
        for (node_name, (_, _)) in nodes_input.clone() {
            node_map.insert(node_name.to_string(), Rc::new(RefCell::new(TreeNode { name: node_name.to_string(), left: None, right: None })) as TreeNodeRef);
        }

        for (node_name, (node_name_l, node_name_r)) in nodes_input.clone() {
            let node_l = node_map.get(node_name_l).unwrap();
            let node_r = node_map.get(node_name_r).unwrap();
            let node = node_map.get(node_name).unwrap();
            node.borrow_mut().left = Some(node_l.to_owned());
            node.borrow_mut().right = Some(node_r.to_owned());
        }

        LevelSolver {
            instructions: instructions.to_string(),
            node_map: node_map.clone(),
        }
    }

    fn solve_a(&self) -> Result<String,Error> {
        let node_map = &self.node_map;

        let start_node = node_map.get("AAA").unwrap().to_owned();
        let stop_node = node_map.get("ZZZ").unwrap().to_owned();

        let instuction_pointer = traverse_tree(
            self.instructions.as_str(),
            start_node,
            |current| current.borrow().name == stop_node.borrow().name
        );

        Ok(instuction_pointer.to_string())
    }

    fn solve_b(&self) -> Result<String,Error> {
        let node_map = &self.node_map;

        let ps: Vec<(RefCell<usize>, TreeNodeRef)> = node_map
            .values()
            .filter(|&n| n.borrow().name.ends_with("A"))
            .map(|n| (RefCell::new(0),n.to_owned()))
            .collect_vec();

        for (_, (n,p)) in ps.iter().enumerate() {
            println!("## {:?}", p);
            let mut z_node_name: Cell<String> = Cell::new(String::new());
            let mut z_count = 0;
            let mut z_step_count = 0;

            traverse_tree(
                self.instructions.as_str(),
                p.clone(),
                |current| {
                    if z_count > 1 && &current.borrow().name == z_node_name.get_mut() {
                        return true
                    }

                    if current.borrow().name.ends_with("Z") {
                        z_node_name.set(current.borrow().name.to_string());
                        z_count += 1;
                        if z_count > 1 { return true }
                        z_step_count = 0;
                    }

                    z_step_count += 1;
                    false
                }
            );

            n.replace(z_step_count);
        }
        for (i, (n,p)) in ps.iter().enumerate() {
            println!("{}: {} | {:?}", i, n.borrow(), p.borrow());
        }
        let common_lcm = lcmm(ps.iter().map(| (n,_)| n.borrow().clone()  as u64 ).collect_vec());
        println!("== {}", common_lcm);
        Ok(common_lcm.to_string())
    }
}


fn parse(input: &str) -> Result<(&str, Vec<(&str,(&str,&str))>), Error> {
    let re = Regex::new(r"(?m)^([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)$").unwrap();
    let instructions = input.lines().next().unwrap();

    let nodes = input
        .lines()
        .skip(2)
        .map(|line| {
            let (_,[name, node_l,node_r]) = re.captures(line).unwrap().extract();
            (name,(node_l,node_r))
        })
        .collect_vec();

    Ok((instructions, nodes))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn input() -> String {
        "
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
        ".trim().to_string()
    }

    fn input2() -> String {
        "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
        ".trim().to_string()
    }

    fn input3() -> String {
        "
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
        ".trim().to_string()
    }

    #[test]
    fn test_parse() -> Result<(),Error> {
        let input = input();
        let solution = parse(&input)?;


        assert_eq!(solution.0, "RL");
        assert_eq!(solution.1, vec![
            ("AAA",("BBB", "CCC")),
            ("BBB",("DDD", "EEE")),
            ("CCC",("ZZZ", "GGG")),
            ("DDD",("DDD", "DDD")),
            ("EEE",("EEE", "EEE")),
            ("GGG",("GGG", "GGG")),
            ("ZZZ",("ZZZ", "ZZZ")),
        ]);
        Ok(())
    }

    #[test]
    fn test_a() -> Result<(),Error> {
        let input = input();
        let solution = LevelSolver::new(input).solve_a()?;

        assert_eq!(solution, 2.to_string());
        Ok(())
    }

    #[test]
    fn test_a_2() -> Result<(),Error> {
        let input = input2();
        let solution = LevelSolver::new(input).solve_a()?;

        assert_eq!(solution, 6.to_string());
        Ok(())
    }

    #[test]
    fn test_b() -> Result<(),Error> {
        let input = input3();
        let solution = LevelSolver::new(input).solve_b()?;

        assert_eq!(solution, 6.to_string());
        Ok(())
    }
}