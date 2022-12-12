const DAY: &str = "11";

use itertools::Itertools;

use crate::utils;

#[derive(Clone, Debug)]
struct TestRule {
    modulo: u64,
    target_if_true: u64,
    target_if_false: u64,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    inspect_count: u64,
    operation: String,
    test_rule: TestRule,
}

impl Monkey {
    pub fn new() -> Monkey {
        return Monkey {
            items: vec![],
            operation: String::from(""),
            inspect_count: 0,
            test_rule: TestRule {
                modulo: 0,
                target_if_true: 0,
                target_if_false: 0,
            },
        };
    }

    pub fn act(&mut self, moved_items: &mut Vec<Vec<u64>>, lcm: u64) {
        // loop over all items
        for item in self.items.iter() {
            // inspect item
            self.inspect_count += 1;

            // calculate new value
            let new_value = self.operate(*item, lcm);
            // test new value
            let test_result = (new_value % self.test_rule.modulo) == 0;
            let target: u64;

            // decide monkey to throw the item at
            if test_result {
                target = self.test_rule.target_if_true;
            } else {
                target = self.test_rule.target_if_false;
            }

            moved_items[target as usize].push(new_value);
        }

        self.items.clear();
    }

    fn operate(&self, value: u64, lcm: u64) -> u64 {
        let actions = self.operation.split_whitespace().collect_vec();
        assert!(actions[0] == "old");

        let operand = actions[2].parse().unwrap_or(value);

        let mut new_value: u64 = match actions[1] {
            "+" => value + operand,
            "*" => value * operand,
            _ => todo!(),
        };


        if lcm == 0 {
            // for part 1, new value is always divided by 3
            new_value /= 3;
        } else {
            // thanks reddit!
            // https://www.reddit.com/r/adventofcode/comments/zizi43/comment/iztt8mx
            // for part 2, to prevent integer overflow, we take the remainder of (new value % least common multiple)
            // least common multiple value is product of all monkeys' test value - since all of it is prime
            new_value %= lcm;
        }

        return new_value;
    }
}

fn calculate_monkey_business(monkeys: &Vec<Monkey>) -> u64 {
    let mut inspection_count_arr: Vec<u64> = vec![];

    for monkey in monkeys {
        inspection_count_arr.push(monkey.inspect_count);
    }

    inspection_count_arr.sort();
    inspection_count_arr.reverse();

    return inspection_count_arr[0] * inspection_count_arr[1];
}

pub fn test_results() -> (String, String) {
    let part1 = String::from("10605");
    let part2 = String::from("2713310158");
    return (part1, part2);
}

pub fn solve(target_input: &str) -> (String, String) {
    let contents = utils::helper::read_file(DAY, target_input);

    let mut monkeys: Vec<Monkey> = vec![];
    let mut new_monkey = Monkey::new();
    let monkeys_clone: Vec<Monkey>;

    for t in contents.lines() {
        let line = t.to_string().trim().to_owned();
        if line == "" {
            monkeys.push(new_monkey);
            new_monkey = Monkey::new();
        } else if line.contains("Starting items") {
            let items: Vec<u64> = line
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(",")
                .map(|f| f.trim().parse().unwrap())
                .collect_vec();
            new_monkey.items = items;
        } else if line.contains("Operation: ") {
            let operation = line.strip_prefix("Operation: new = ").unwrap().to_owned();
            new_monkey.operation = operation
        } else if line.contains("Test: divisible by ") {
            let divisible: u64 = line
                .strip_prefix("Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap();
            new_monkey.test_rule.modulo = divisible;
        } else if line.contains("throw to monkey") {
            let target: u64 = line
                .split_whitespace()
                .collect_vec()
                .last()
                .unwrap()
                .parse()
                .unwrap();
            if line.contains("true") {
                new_monkey.test_rule.target_if_true = target;
            } else {
                new_monkey.test_rule.target_if_false = target;
            }
        } else if line.contains("Monkey") {
            // do nothing
        } else {
            dbg!(&line);
            todo!();
        }
    }

    monkeys.push(new_monkey);
    monkeys_clone = monkeys.clone();

    // part 1
    for _ in 1..=20 {
        for index in 0..monkeys.len() {
            let mut moved_items: Vec<Vec<u64>> = vec![vec![]; monkeys.len()];
            monkeys[index].act(&mut moved_items, 0);

            for i in 0..monkeys.len() {
                monkeys[i].items.append(&mut moved_items[i]);
            }
        }
    }

    let part1 = calculate_monkey_business(&monkeys);

    // part 2
    monkeys = monkeys_clone.clone();

    let mut lcm = 1;

    for monkey in monkeys.iter() {
        // get least common multiple from all monkeys' test
        lcm *= monkey.test_rule.modulo;
    }

    for _ in 1..=10000 {
        for index in 0..monkeys.len() {
            let mut moved_items: Vec<Vec<u64>> = vec![vec![]; monkeys.len()];
            monkeys[index].act(&mut moved_items, lcm);

            for i in 0..monkeys.len() {
                monkeys[i].items.append(&mut moved_items[i]);
            }
        }
    }

    let part2 = calculate_monkey_business(&monkeys);

    return (part1.to_string(), part2.to_string());
}
