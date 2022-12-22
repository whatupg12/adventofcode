use std::fmt;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    // println!("{}", root_root);

    // let small_dir_sizes = find_small_dir_sizes(&root_root, 100_000);
    // println!("total: {}", small_dir_sizes.iter().sum::<usize>());
}


struct Directory<'a> {
    level: usize,
    dir: HashMap<&'a str, Item<'a>>,
}

impl fmt::Display for Directory<'_> {
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

enum Item<'a> {
    File(usize),
    Directory(Directory<'a>)
}

fn find_large_files(root_dir: &Directory, size: usize, starting_total: usize) -> usize {
    let mut total = starting_total;
    for (_, item) in &root_dir.dir {
        match item {
            Item::File(fsize) if fsize > &size => total += fsize,
            Item::Directory(dir) => total += find_large_files(dir, size, total),
            _ => (),
        }
    }
    return total;
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

    fn make_test_root<'a>() -> Directory<'a> {
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
        e_map.insert("i", i_file);
        let e_dir = Item::Directory(Directory {level: l3, dir: e_map});
    
        let mut a_map = HashMap::new();
        a_map.insert("e", e_dir);
        a_map.insert("f", f_file);
        a_map.insert("g", g_file);
        a_map.insert("h_lst", h_lst_file);
        let a_dir = Item::Directory(Directory {level: l2, dir: a_map});
        
        let mut d_map = HashMap::new();
        d_map.insert("j", j_file);
        d_map.insert("d.log", d_log_file);
        d_map.insert("d.ext", d_ext_file);
        d_map.insert("k", k_file);
        let d_dir = Item::Directory(Directory {level: l2, dir: d_map});
    
        let mut root_dir = HashMap::new();
        root_dir.insert("a", a_dir);
        root_dir.insert("b.txt", b_txt_file);
        root_dir.insert("c.dat", c_dat_file);
        root_dir.insert("d", d_dir);
    
    
        let mut root = HashMap::new();
        root.insert("/", Item::Directory(Directory{level: l1, dir: root_dir}));
        let root_root = Directory {level: l0, dir: root};

        return root_root;
    }

    #[test]
    fn test_find_small_dir_sizes() {
        let root_dir = make_test_root();
        let actual: usize = find_small_dir_sizes(&root_dir, 100_000).iter().sum();
        let expect = 95437;
        assert_eq!(actual, expect);
    }
    
}
