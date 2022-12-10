const DAY: &str = "10";

use itertools::Itertools;

use crate::utils;

#[derive(Debug)]
struct Process {
    delay: i32,
    value: i32,
}

pub fn test_results() -> (String, String) {
    let part1 = String::from("13140");
    let part2 = String::from("0");
    return (part1, part2);
}

pub fn solve(target_input: &str) -> (String, String) {
    let contents = utils::helper::read_file(DAY, target_input);

    let mut part1 = 0;
    let part2 = 0; // part 2 is visual answer

    let mut process_stacks: Vec<Process> = vec![];

    let mut value = 1;
    let mut iteration = 1;

    let mut line_iterations = contents.lines().peekable();
    let mut screen: Vec<Vec<String>> = vec![];
    let mut pixel_per_line: Vec<String> = vec![];

    while iteration <= 241 {
        if iteration % 40 == 20 {
            part1 += calculate_value(&iteration, &value)
        }

        if iteration % 40 == 1 && iteration > 1 {
            screen.push(pixel_per_line);
            pixel_per_line = vec![];
        }

        let mut pixel_value = " ".to_string();

        if value.abs_diff((iteration - 1) % 40) <= 1 {
            pixel_value = "■".to_string();
        }
        pixel_per_line.push(pixel_value);

        // if there is no running process and there is still item in input
        if process_stacks.len() == 0 && line_iterations.peek().is_some() {
            let line = line_iterations.next().unwrap();

            let new_process: Process;
            if line == "noop" {
                // if noop, add by 0
                new_process = Process { delay: 1, value: 0 }
            } else {
                // else add by value
                let (_, new_value) = line.split_whitespace().collect_tuple().unwrap();
                new_process = Process {
                    delay: 2,
                    value: new_value.parse::<i32>().unwrap(),
                };
            }
            process_stacks.push(new_process);
        }

        for process in &mut process_stacks {
            process.delay -= 1;

            if process.delay == 0 {
                value += process.value;
            }
        }

        process_stacks.retain(|f| f.delay > 0); // delete item that has delay < 0

        iteration += 1;
    }

    for s in screen {
        println!("{}", &s.join(""));
    }

    return (part1.to_string(), part2.to_string());
}

fn calculate_value(iteration: &i32, value: &i32) -> i32 {
    // for debugging purpose
    let return_value = iteration * value;
    return return_value;
}
