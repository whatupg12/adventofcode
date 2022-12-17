//use regex::Regex;

fn main() {
    parse_cargo("Hello, world!");
}

fn parse_cargo(input: &str) -> usize {
    let lines: Vec<_> = input.split('\n').collect();
    let instruction_idx = find_instruction_line(&lines);

    let platform_line = lines[instruction_idx-1];
    let platform_idxs = build_platform_idxs(platform_line);
    let platforms: Vec<Vec<char>> = vec![Vec::new(); platform_idxs.len()];

    lines[0..(instruction_idx-2)]
        .iter()
        .rev()
        .enumerate()
        .

    return instruction_idx;
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
}
