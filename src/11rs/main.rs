use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;
use num_bigint::{BigUint, ToBigUint, BigInt, ToBigInt};


fn update_step(a: &mut BigInt, old_a: &mut BigInt, quotient: &BigInt) {
    // adapted from: https://github.com/TheAlgorithms/Rust/blob/master/src/math/extended_euclidean_algorithm.rs
    let temp = &a.clone();
    *a = old_a.clone() - quotient * temp;
    *old_a = temp.clone();
}


pub fn extended_euclidean_algorithm(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    // adapted from: https://github.com/TheAlgorithms/Rust/blob/master/src/math/extended_euclidean_algorithm.rs
    let (mut old_r, mut rem) = (a.clone(), b.clone());
    let (mut old_s, mut coeff_s) = (1_i32.to_bigint().unwrap(), 0_i32.to_bigint().unwrap());
    let (mut old_t, mut coeff_t) = (0_i32.to_bigint().unwrap(), 1_i32.to_bigint().unwrap());

    while &rem != &0_i32.to_bigint().unwrap() {
        let quotient = &old_r / &rem;

        update_step(&mut rem, &mut old_r, &quotient);
        update_step(&mut coeff_s, &mut old_s, &quotient);
        update_step(&mut coeff_t, &mut old_t, &quotient);
    }

    (old_r, old_s, old_t)
}


fn mod_inv(x: &BigInt, n: &BigInt) -> Option<BigInt> {
    // adapted from: https://github.com/TheAlgorithms/Rust/blob/master/src/math/chinese_remainder_theorem.rs
    let (g, x, _) = extended_euclidean_algorithm(x, n);
    if g == 1.to_bigint().unwrap() {
        Some((x % n + n) % n)
    } else {
        None
    }
}


pub fn chinese_remainder_theorem(residues: &Vec<BigInt>, modulli: &Vec<BigInt>) -> Option<BigInt> {
    // adapted from: https://github.com/TheAlgorithms/Rust/blob/master/src/math/chinese_remainder_theorem.rs
    let prod = &modulli.iter().product::<BigInt>();

    let mut sum = 0_i32.to_bigint().unwrap();

    for (residue, modulus) in residues.iter().zip(modulli) {
        let p = prod / modulus;
        sum += residue * mod_inv(&p, modulus)? * p
    }
    Some(sum % prod)
}


struct Monkey<'a> {
    inspected_count: i32,
    items: Vec<BigUint>,
    operation: Box<dyn Fn(&BigUint) -> BigUint + 'a>,
    throw_test: (i32, Box<dyn Fn(&BigUint) -> bool + 'a>),
    test_success_target: i32,
    test_fail_target: i32,
}

impl<'a> Monkey<'a> {
    fn from_lines(monkey_lines: Vec<String>) -> Self {
        Self {
            inspected_count: 0,
            items: monkey_lines[1]
                .trim()
                .split("Starting items: ")
                .last()
                .unwrap()
                .split(", ")
                .map(|x| x.parse::<BigUint>().unwrap())
                .collect_vec(),
            operation: {
                let op_line = monkey_lines[2].trim().split("Operation: ").last().unwrap();
                match op_line.split("new = ").last().unwrap().split(' ').collect_vec()[..] {
                    ["old", "*", "old"] => {
                        Box::new(|x: &BigUint| -> BigUint { x * x })
                    },
                    ["old", "*", some_val] => {
                        let some_val_int = some_val.parse::<i32>().unwrap();
                        Box::new(move |x: &BigUint| -> BigUint { x * some_val_int.to_biguint().unwrap() })
                    },
                    ["old", "+", "old"] => {
                        Box::new(|x: &BigUint| -> BigUint { x + x })
                    },
                    ["old", "+", some_val] => {
                        let some_val_int = some_val.parse::<i32>().unwrap();
                        Box::new(move |x: &BigUint| -> BigUint { x + some_val_int.to_biguint().unwrap() })
                    },
                    _ => panic!()
                }
            },
            throw_test: {
                let test_val = monkey_lines[3].trim().split("Test: divisible by ").last().unwrap();
                let test_val_int: i32 = test_val.parse::<i32>().unwrap();
                (test_val_int, Box::new(move |x: &BigUint| -> bool { x % test_val_int.to_biguint().unwrap() == 0_u32.into() }))
            },
            test_success_target: {
                let test_target = monkey_lines[4].trim().split("If true: throw to monkey ").last().unwrap();
                test_target.parse::<i32>().unwrap()
            },
            test_fail_target: {
                let test_target = monkey_lines[5].trim().split("If false: throw to monkey ").last().unwrap();
                test_target.parse::<i32>().unwrap()
            },
        }
    }

    fn business(&mut self) -> HashMap<usize, Vec<BigUint>> {
        let mut targets: HashMap<usize, Vec<BigUint>> = HashMap::new();
        for worry_level in &self.items {
            // let mut new_worry_level = (self.operation)(worry_level);
            // new_worry_level /= 3.to_biguint().unwrap();
            let new_worry_level = (self.operation)(worry_level);
            let (_, execute_test) = &self.throw_test;
            let test_result = (execute_test)(&new_worry_level);
            let target = if test_result { self.test_success_target } else { self.test_fail_target };
            let utarget = target as usize;
            if !targets.contains_key(&utarget) {
                targets.insert(utarget, Vec::new());
            }
            let target_items: &mut Vec<BigUint> = targets.get_mut(&utarget).unwrap();
            target_items.push(new_worry_level);

            self.inspected_count += 1;
        }
        self.items.clear();

        return targets
    }
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn parse(path: &Path) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    if let Ok(lines) = read_lines(path) {
        let all_lines = lines.map(|x| x.unwrap()).collect::<Vec<String>>();
        let mut lines_iter = all_lines.iter();
        loop {
            let monkey_lines: Vec<String> = lines_iter
                .take_while_ref(|&x| x != "")
                .map(|x| x.to_owned())
                .collect_vec();
            lines_iter.next();
            if monkey_lines.is_empty() {
                break;
            } else {
                monkeys.push(
                    Monkey::from_lines(monkey_lines)
                )
            }
        }
    }
    return monkeys;
}


fn run_rounds(mut monkeys: Vec<Monkey>, rounds: i32) -> BigUint {
    let mut new_targets: HashMap<usize, Vec<BigUint>> = HashMap::new();
    for i in 0..rounds {
        println!("{:?}", i);
        for (i, m) in monkeys.iter_mut().enumerate() {
            if (&new_targets).contains_key(&i) {
                m.items.append(new_targets.get_mut(&i).unwrap());
            }
            let targets = m.business();
            for (target, mut target_items) in targets {
                if new_targets.contains_key(&target) {
                    new_targets.get_mut(&target).unwrap().append(&mut target_items);
                } else {
                    new_targets.insert(target, target_items);
                }
            }
        }
        for (_, target_items) in &mut new_targets {
            let mut new_target_items: Vec<BigUint> = Vec::new();
            for ti in target_items.iter() {
                let modulli = monkeys.iter().map(|x| x.throw_test.0.to_bigint().unwrap()).collect::<Vec<BigInt>>();
                let residues = monkeys.iter().map(|x| ti.to_bigint().unwrap() % x.throw_test.0.to_bigint().unwrap()).collect::<Vec<BigInt>>();
                match chinese_remainder_theorem(&residues, &modulli) {
                    None => new_target_items.push(ti.clone()),
                    Some(x) => new_target_items.push(x.to_biguint().unwrap()),
                };
            }
            target_items.clear();
            target_items.append(&mut new_target_items);
        }
    }

    monkeys.sort_by(|a, b| a.inspected_count.partial_cmp(&b.inspected_count).unwrap());
    let business_level = monkeys[monkeys.len()-2..]
        .iter()
        .fold(1.to_biguint().unwrap(), |acc,x| acc * x.inspected_count.to_biguint().unwrap());

    return business_level;
}


fn main() {
    // let path = Path::new("src/11rs/ex.in.txt");
    let path = Path::new("src/11rs/in.txt");

    let monkeys = parse(path);
    let total = run_rounds(monkeys, 10000);

    println!("Total is: {}", total);

}
