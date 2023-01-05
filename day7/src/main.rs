use std::collections::HashMap;
use std::fs;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;


fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Expect input.txt file");

    let file_map = derive_file_system(input.as_str());
    //print_map(&file_map);
    //println!();
    let dir_map = calculate_directory_sizes(&file_map);

    //print_map(&dir_map);

    let small_dirs = find_at_most_size_dirs(&dir_map, 100_000);
    println!("total1: {}", small_dirs);

    let delete_dir = find_dir_to_delete(&dir_map, 30_000_000, 70_000_000);
    println!("total2: {}", delete_dir);
}

fn derive_file_system(input: &str) -> HashMap<String, usize> {
    lazy_static! {
        static ref LS_RE: Regex = Regex::new(r"^\$\s+ls\s*$").unwrap();
        static ref CD_RE: Regex = Regex::new(r"^\$\s+cd\s+([\w/\.]+)\s*$").unwrap();
        static ref DIR_RE: Regex = Regex::new(r"^\s*dir\s+([\w]+)\s*$").unwrap();
        static ref FILE_RE: Regex = Regex::new(r"^\s*(\d+)\s+([\w\.]+)\s*$").unwrap();
    }

    let lines: Vec<_> = input.split('\n')
        .collect();

    if lines.is_empty() {
        panic!("empty lines");
    }
    if CD_RE.captures(lines[0]).filter(|cap| &cap[1] == "/").is_none() { 
        panic!("first command must be to change to root");
    }

    let mut file_map = HashMap::new();
    let mut path_buf = PathBuf::from("/");
    
    for line in lines {
        if LS_RE.is_match(line) || DIR_RE.is_match(line) {
            continue;
        }

        if let Some(cap) = CD_RE.captures(line) {
            let fname = &cap[1];
            if fname == ".." {
                path_buf.pop();

            } else if fname.starts_with("/") {
                path_buf = PathBuf::from(fname);

            } else {
                path_buf.push(fname);
            }
            continue;
        }

        if let Some(cap) = FILE_RE.captures(line) {
            let fsize: usize = cap[1].parse().unwrap_or(0);
            let fname = &cap[2];

            let mut this_path_buf = path_buf.clone();
            this_path_buf.push(fname);

            let fpath = String::from(this_path_buf.to_str().unwrap());
            if !file_map.contains_key(&fpath) {
                file_map.insert(fpath, fsize);
            }
        }
    }
    
    return file_map;
}


fn calculate_directory_sizes(file_map: &HashMap<String, usize>) -> HashMap<String, usize> {
    let mut dir_map: HashMap<String, usize> = HashMap::new();
    for (fpath, fsize) in file_map {
        let mut path_buf = PathBuf::from(fpath);
        let fsize = fsize.to_owned();
        
        loop {
            if !path_buf.pop() {
                break;
            }
        
            let parent_path = String::from(path_buf.to_str().unwrap());
            
            if dir_map.contains_key(&parent_path) {
                *dir_map.get_mut(&parent_path).unwrap() += fsize;
            } else {
                dir_map.insert(parent_path, fsize);
            }
        }
    }

    return dir_map;
}


fn find_at_most_size_dirs(dir_map: &HashMap<String, usize>, at_most: usize) -> usize {
    let mut total = 0;
    for (_, dsize) in dir_map {
        if at_most >= *dsize {
            total += dsize;
        }
    }
    return total;
}


fn find_dir_to_delete(dir_map: &HashMap<String, usize>, at_least: usize, total: usize) -> usize {
    let total_used = dir_map["/"];
    let free_space = total - total_used;
    let space_to_free = at_least - free_space;

    println!("total={}, total_used={}, free={}, at_least={}, space_to_free={}", total, total_used, free_space, at_least, space_to_free);

    let mut min_to_delete = None;
    for (_, dsize) in dir_map {
        if space_to_free <= *dsize {
            if min_to_delete.filter(|m| *m < *dsize).is_none() {
                min_to_delete = Some(*dsize);
            }
        }
    }
    return min_to_delete.unwrap_or(0);
}


// fn print_map(map: &HashMap<String, usize>) {
//     let mut ordered_keys: Vec<_> = map.keys().collect();
//     ordered_keys.sort();

//     for key in ordered_keys {
//         let value = map[key];
//         println!("{} => {}", key, value);
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = concat!(
        "$ cd /\n",
        "$ ls\n",
        "dir a\n",
        "14848514 b.txt\n",
        "8504156 c.dat\n",
        "dir d\n",
        "$ cd a\n",
        "$ \n",
        "dir e\n",
        "29116 f\n",
        "2557 g\n",
        "62596 h.lst\n",
        "$ cd e\n",
        "$ ls\n",
        "584 i\n",
        "$ cd ..\n",
        "$ cd ..\n",
        "$ cd d\n",
        "$ ls\n",
        "4060174 j\n",
        "8033020 d.log\n",
        "5626152 d.ext\n",
        "7214296 k\n",
    );

    #[test]
    fn it_derive_file_system() {
        let actual = derive_file_system(INPUT);
        let expect = HashMap::from(
            [
                (String::from("/a/e/i"), 584),
                (String::from("/a/f"), 29116),
                (String::from("/a/g"), 2557),
                (String::from("/a/h.lst"), 62596),
                (String::from("/b.txt"), 14848514),
                (String::from("/c.dat"), 8504156),
                (String::from("/d/j"), 4060174),
                (String::from("/d/d.log"), 8033020),
                (String::from("/d/d.ext"), 5626152),
                (String::from("/d/k"), 7214296),
            ]
        );
        assert_eq!(actual, expect);
    }

    #[test]
    fn it_calculate_directory_sizes() {
        let file_map = HashMap::from(
            [
                (String::from("/a/e/i"), 584),
                (String::from("/a/f"), 29116),
                (String::from("/a/g"), 2557),
                (String::from("/a/h.lst"), 62596),
                (String::from("/b.txt"), 14848514),
                (String::from("/c.dat"), 8504156),
                (String::from("/d/j"), 4060174),
                (String::from("/d/d.log"), 8033020),
                (String::from("/d/d.ext"), 5626152),
                (String::from("/d/k"), 7214296),
            ]
        );
        let actual = calculate_directory_sizes(&file_map);
        let expect = HashMap::from(
            [
                (String::from("/a/e"), 584),
                (String::from("/a"), 584 + 29116 + 2557 + 62596),
                (String::from("/d"), 4060174 + 8033020 + 5626152 + 7214296),
                (String::from("/"),  584 + 29116 + 2557 + 62596 + 4060174 + 8033020 + 5626152 + 7214296 + 14848514 + 8504156),
            ]
        );
        assert_eq!(actual, expect);
    }

    #[test]
    fn it_find_dir_to_delete() {
        let file_map = derive_file_system(INPUT);
        let dir_map = calculate_directory_sizes(&file_map);
        let actual = find_dir_to_delete(&dir_map, 30_000_000, 70_000_000);
        let expect = 24933642;
        assert_eq!(actual, expect);
    }

}
