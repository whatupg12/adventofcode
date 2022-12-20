use std::fs;
use regex::Regex;
use std::slice::Iter;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let input = contents.as_str();

    let score1 = parse_cargo(input);
    println!("Score1: {}", score1);
}

fn parse_cargo(input: &str) -> String {
    let lines: Vec<_> = input.split('\n').collect();
    let instruction_idx = find_instruction_line(&lines);

    let platform_line = lines[instruction_idx-1];
    let platform_idxs = build_platform_idxs(platform_line);

    let levels = lines[0..(instruction_idx-1)].iter().rev().collect();
    let mut platforms = load_platforms(levels, platform_idxs);

    let instructions = parse_move_instructions(
        lines[(instruction_idx+1)..lines.len()].iter()
    );
    
    execute_instructions(instructions, &mut platforms);

    let mut result = String::new();
    for platform in platforms {
        result.push(platform.last().unwrap().clone());
    }
    return result;
}

fn execute_instructions(instructions: Vec<(usize, usize, usize)>, platforms: &mut Vec<Vec<char>>) {
    for (amount, source, target) in instructions {
        for _ in 0..amount {
            let cargo = platforms[source-1].pop().unwrap();
            platforms[target-1].push(cargo);
        }
    }
}

fn parse_move_instructions(lines: Iter<&str>) -> Vec<(usize, usize, usize)> {
    let re = Regex::new(r"\s*move (\d+) from (\d+) to (\d+)\s*").unwrap();
    let instructions = lines.flat_map(
        |l| 
        re.captures(l).map(
            |cap| (
                cap[1].parse().unwrap_or(0), 
                cap[2].parse().unwrap_or(0),
                cap[3].parse().unwrap_or(0),
            )
        )
    ).collect();
    return instructions;
}

fn load_platforms(levels: Vec<&&str>, platform_idxs: Vec<usize>) -> Vec<Vec<char>> {
    let mut platforms: Vec<Vec<char>> = vec![Vec::new(); platform_idxs.len()];
    levels.iter()
        .for_each(
            |l| {
            platform_idxs.iter()
                .enumerate()
                .for_each(
                    |(i, idx)|
                    match l.chars().nth(idx.clone()) {
                        Some(c) if c.is_alphabetic() => platforms[i].push(c),
                        _ => (),
                    }
                )
        });
    return platforms;
}

fn find_instruction_line(lines: &Vec<&str>) -> usize {
    return lines.iter()
        .enumerate()
        .find(|(_, l)| l.len() == 0)
        .map(|(i, _)| i)
        .unwrap_or(0);
}

fn build_platform_idxs(platform_line: &str) -> Vec<usize> {
    return platform_line.chars()
        .into_iter()
        .enumerate()
        .filter(
            |(_, c)| !c.is_whitespace()
        )
        .map(
            |(i, _)| i
        )
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = concat!(
        "    [D]    \n",
        "[N] [C]    \n",
        "[Z] [M] [P]\n",
        " 1   2   3 \n",
        "\n",
        "move 1 from 2 to 1\n",
        "move 3 from 1 to 3\n",
        "move 2 from 2 to 1\n",
        "move 1 from 1 to 2\n"
    );

    #[test]
    fn it_parse_cargo() {
        let actual = parse_cargo(TEST_INPUT);
        let expect = String::from("CMZ");
        assert_eq!(actual, expect);
    }

    #[test]
    fn it_find_instruction_line() {
        let lines: Vec<_> = TEST_INPUT.split('\n').collect();
        let actual = find_instruction_line(&lines);
        let expect = 4;
        assert_eq!(actual, expect);
    }

    #[test]
    fn it_build_platform_idxs() {
        let lines: Vec<_> = TEST_INPUT.split('\n').collect();
        let actual = build_platform_idxs(lines[3]);
        let expect = Vec::from([1, 5, 9]);
        assert_eq!(actual, expect);
    }

    #[test]
    fn it_load_platforms() {
        let levels = Vec::from([    
            &"[Z] [M] [P]",    
            &"[N] [C]    ",
            &"[D]        ",
        ]);
        let platform_idxs = Vec::from([1, 5, 9]); 
        let actual = load_platforms(levels, platform_idxs);
        let expect = Vec::from([
            Vec::from(['Z', 'N', 'D']),
            Vec::from(['M', 'C']),
            Vec::from(['P']),    
        ]);
        assert_eq!(actual, expect);
    }

    #[test]
    fn it_parse_move_instructions() {
        let lines = [
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ].iter();
        let actual = parse_move_instructions(lines);
        let expect = Vec::from([
            (1, 2, 1),
            (3, 1, 3),
            (2, 2, 1),
            (1, 1, 2),
        ]);
        assert_eq!(actual, expect);
    }

    #[test]
    fn it_execute_instructions() {
        let instructions = Vec::from([
            (1, 2, 1),
            (3, 1, 3),
            (2, 2, 1),
            (1, 1, 2),
        ]);
        let mut platforms = Vec::from([
            Vec::from(['Z', 'N']),
            Vec::from(['M', 'C', 'D']),
            Vec::from(['P']),
        ]);
        execute_instructions(instructions, &mut platforms);
        let expect = Vec::from([
            Vec::from(['C']),
            Vec::from(['M']),
            Vec::from(['P', 'D', 'N', 'Z']),
        ]);
        assert_eq!(platforms, expect);
    }

}
