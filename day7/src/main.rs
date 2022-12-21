use std::fmt;
use std::collections::HashMap;

fn main() {
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

    

    let (l1, l2, l3) = (0, 1, 2);

    let mut e_map = HashMap::new();
    e_map.insert("i", i_file);
    let e_dir = Item::Directory(Directory {level: l3, dir: e_map});

    let mut a_map = HashMap::new();
    a_map.insert("e", e_dir);
    a_map.insert("f", f_file);
    a_map.insert("g", g_file);
    a_map.insert("h_lst", h_lst_file);
    let a_dir = Item::Directory(Directory {level: l2, dir: a_map});
    
    // let mut d_dir = HashMap::new();

    let mut root_dir = HashMap::new();
    root_dir.insert("a", a_dir);
    let root = Directory{level: l1, dir: root_dir};

    println!("Hello, world!");
    println!("{}", root_dir);
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
                _ => write!(f, "\n"),
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
