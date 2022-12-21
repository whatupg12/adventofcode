use std::fs;
use std::collections::HashSet;

fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("should have input.txt");
    let input = content.as_str();

    let starter_idx = find_starter_packet(input, 4);
    println!("Idx: {}", starter_idx);

    let starter_m_idx = find_starter_packet(input, 14);
    println!("Idx: {}", starter_m_idx);
}

fn find_starter_packet(input: &str, pack_len: usize) -> usize {
    if input.len() < pack_len {
        return input.len()+1;
    }

    let chars: Vec<_> = input.chars().collect();
    for i in 0..=(chars.len() - pack_len) {
        let mut local_set = HashSet::new();
        for v in 0..pack_len {
            let this_char = chars[i+v];
            if local_set.contains(&this_char) {
                break;
            }
            local_set.insert(this_char);
        }
        if local_set.len() >= pack_len {
            return i+pack_len;
        }
    }
    return input.len()+1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_find_starter_packet() {
        let actual = find_starter_packet("", 4);
        let expect = 1;
        assert_eq!(actual, expect);

        let actual = find_starter_packet("1234", 4);
        let expect = 4;
        assert_eq!(actual, expect);

        let actual = find_starter_packet("12", 4);
        let expect = 3;
        assert_eq!(actual, expect);

        let actual = find_starter_packet("4234", 4);
        let expect = 5;
        assert_eq!(actual, expect);

        let actual = find_starter_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4);
        let expect = 7;
        assert_eq!(actual, expect);

        let actual = find_starter_packet("bvwbjplbgvbhsrlpgdmjqwftvncz", 4);
        let expect = 5;
        assert_eq!(actual, expect);

        let actual = find_starter_packet("nppdvjthqldpwncqszvftbrmjlhg", 4);
        let expect = 6;
        assert_eq!(actual, expect);

        let actual = find_starter_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4);
        let expect = 10;
        assert_eq!(actual, expect);

        let actual = find_starter_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4);
        let expect = 11;
        assert_eq!(actual, expect);

        let actual = find_starter_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14);
        let expect = 19;
        assert_eq!(actual, expect);

        let actual = find_starter_packet("bvwbjplbgvbhsrlpgdmjqwftvncz", 14);
        let expect = 23;
        assert_eq!(actual, expect);

        let actual = find_starter_packet("nppdvjthqldpwncqszvftbrmjlhg", 14);
        let expect = 23;
        assert_eq!(actual, expect);

        let actual = find_starter_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14);
        let expect = 29;
        assert_eq!(actual, expect);

        let actual = find_starter_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14);
        let expect = 26;
        assert_eq!(actual, expect);
    }
}
