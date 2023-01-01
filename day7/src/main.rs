use std::fmt;
use std::collections::HashMap;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;
use std::ffi::OsStr;


fn main() {
    println!("Hello, world!");
    // println!("{}", root_root);

    // let small_dir_sizes = find_small_dir_sizes(&root_root, 100_000);
    // println!("total: {}", small_dir_sizes.iter().sum::<usize>());
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Line {
    List,
    ChangeDirectory(String),
    Directory(String),
    File(usize, String),
    Nothing,
}

fn derive_file_system(input: &str) -> Directory {
    let mut lines = input.split('\n')
        .map(parse_line);

    if lines.next() != Some(Line::ChangeDirectory(String::from("/"))) { 
        panic!("first command must be to root");
    }

    let mut root_map = HashMap::new();

    let fname: String;
    let new_dir: String;

    let mut file_map = HashMap::new();
    let mut relative_path = Path::new("/");
    for line in lines {
        match line {
            Line::File(fsize, fname) => {
                let path = relative_path.join(Path::new(OsStr::new(fname.as_str()))).as_path();
                file_map.insert(
                    path,
                    Item::File(fsize)
                );
                ();
            },
            Line::ChangeDirectory(new_dir) => {
                let mut new_path = Path::new(OsStr::new(new_dir.as_str()));
                if new_path.is_relative() {
                    relative_path = relative_path.join(new_path).as_path();
                } else {
                    relative_path = new_path;
                }
                ();
            },
            _ => (),
        }
    }
    
    return Directory { level: 0, dir: root_map };
}



fn parse_line(line: &str) -> Line {
    lazy_static! {
        static ref LS_RE: Regex = Regex::new(r"^\$\s+ls\s*$").unwrap();
        static ref CD_RE: Regex = Regex::new(r"^\$\s+cd\s+([\w/\.]+)\s*$").unwrap();
        static ref DIR_RE: Regex = Regex::new(r"^\s*dir\s+([\w]+)\s*$").unwrap();
        static ref FILE_RE: Regex = Regex::new(r"^\s*(\d+)\s+([\w\.]+)\s*$").unwrap();
    }

    if LS_RE.is_match(line) {
        return Line::List;
    }
    if let Some(cap) = CD_RE.captures(line) {
        return Line::ChangeDirectory(String::from_str(&cap[1]).unwrap());
    }
    if let Some(cap) = DIR_RE.captures(line) {
        return Line::Directory(String::from_str(&cap[1]).unwrap());
    }
    if let Some(cap) = FILE_RE.captures(line) {
        return Line::File(cap[1].parse().unwrap_or(0), String::from_str(&cap[2]).unwrap());
    }
    return Line::Nothing;
}


struct Directory {
    level: usize,
    dir: HashMap<String, Item>,
}

impl fmt::Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = "  ";

        let mut margin = String::new();
        for _ in 0..self.level {
            margin.push_str(indent);
        }

        let mut res: fmt::Result = fmt::Result::Ok(());
        for (name, item) in &self.dir {
            let next = match item {
                Item::File(s) => write!(f, "{} - {} (file, size={})\n", margin, name, s),
                Item::Directory(d) => write!(f, "{} - {} (dir)\n{}", margin, name, d),
            };
            res = res.and(next);
        }

        return res;
    }
}

enum Item {
    File(usize),
    Directory(Directory)
}


fn calculate_dir_sizes(root_dir: &Directory) -> (usize, Vec<usize>) {
    let mut total = 0;
    let mut dir_sizes = Vec::new();
    for (_, item) in &root_dir.dir {
        match item {
            Item::File(fsize) => total += fsize,
            Item::Directory(dir) => {
                let (dir_size, sub_dir_sizes) = calculate_dir_sizes(dir);
                total += dir_size;
                dir_sizes.extend(sub_dir_sizes);
            }
        }
    }
    dir_sizes.push(total);
    return (total, dir_sizes);
}

fn find_small_dir_sizes(root_dir: &Directory, lsize: usize) -> Vec<usize> {
    let mut small_dir_sizes = Vec::new();
    let (_, dir_sizes) = calculate_dir_sizes(root_dir);
    for dir_size in dir_sizes {
        if dir_size <= lsize {
            small_dir_sizes.push(dir_size);
        }
    }
    return small_dir_sizes;
}

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

    fn make_test_root() -> Directory {
        let i_file = Item::File(584);
        let f_file = Item::File(29116);
        let g_file = Item::File(2557);
        let h_lst_file = Item::File(62596);
        let b_txt_file = Item::File(14848514);
        let c_dat_file = Item::File(8504156);
        let j_file = Item::File(4060174);
        let d_log_file = Item::File(8033020);
        let d_ext_file = Item::File(5626152);
        let k_file = Item::File(7214296);
    
        let (l0, l1, l2, l3) = (0, 1, 2, 3);
    
        let mut e_map = HashMap::new();
        e_map.insert(String::from("i"), i_file);
        let e_dir = Item::Directory(Directory {level: l3, dir: e_map});
    
        let mut a_map = HashMap::new();
        a_map.insert(String::from("e"), e_dir);
        a_map.insert(String::from("f"), f_file);
        a_map.insert(String::from("g"), g_file);
        a_map.insert(String::from("h_lst"), h_lst_file);
        let a_dir = Item::Directory(Directory {level: l2, dir: a_map});
        
        let mut d_map = HashMap::new();
        d_map.insert(String::from("j"), j_file);
        d_map.insert(String::from("d.log"), d_log_file);
        d_map.insert(String::from("d.ext"), d_ext_file);
        d_map.insert(String::from("k"), k_file);
        let d_dir = Item::Directory(Directory {level: l2, dir: d_map});
    
        let mut root_dir = HashMap::new();
        root_dir.insert(String::from("a"), a_dir);
        root_dir.insert(String::from("b.txt"), b_txt_file);
        root_dir.insert(String::from("c.dat"), c_dat_file);
        root_dir.insert(String::from("d"), d_dir);
    
    
        let mut root = HashMap::new();
        root.insert(String::from("/"), Item::Directory(Directory{level: l1, dir: root_dir}));
        let root_root = Directory {level: l0, dir: root};

        return root_root;
    }

    #[test]
    fn it_find_small_dir_sizes() {
        let root_dir = make_test_root();
        let actual: usize = find_small_dir_sizes(&root_dir, 100_000).iter().sum();
        let expect = 95437;
        assert_eq!(actual, expect);
    }
    
    #[test]
    fn it_parse_line() {
        let actual = parse_line("$ ls");
        let expect = Line::List;
        assert_eq!(actual, expect);

        let actual = parse_line("$ cd ..");
        let expect = Line::ChangeDirectory(String::from(".."));
        assert_eq!(actual, expect);

        let actual = parse_line("$ cd /");
        let expect = Line::ChangeDirectory(String::from("/"));
        assert_eq!(actual, expect);

        let actual = parse_line("dir a");
        let expect = Line::Directory(String::from("a"));
        assert_eq!(actual, expect);

        let actual = parse_line("1234 a.td");
        let expect = Line::File(1234, String::from("a.td"));
        assert_eq!(actual, expect);
    }
}
