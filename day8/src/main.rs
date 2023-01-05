use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("expected input.txt");

    let forest = parse_forest(input.as_str());
    // println!("{:?}", forest);

    let visible_trees = count_visible_trees(&forest);
    println!("Visible: {}", visible_trees);
}

fn parse_forest(input: &str) -> Vec<Vec<u32>> {
    let mut grid = Vec::new();

    for line in input.split('\n') {
        let mut row = Vec::new();
        for c in line.chars() {
            let tree = c.to_digit(10).unwrap();
            row.push(tree);
        }
        if !row.is_empty() {
            grid.push(row);
        }
    }

    return grid;
}

fn count_visible_trees(forest: &Vec<Vec<u32>>) -> usize {
    let height = forest.len();
    let width = forest[0].len();
    let mut seen_trees = vec![vec![0; width]; height];

    let mut core = |vis_height: Option<u32>, i: usize, j: usize| -> Option<u32> {
        let mut next_height = vis_height;
        let tree = forest[i][j];
        if vis_height.filter(|h| *h >= tree).is_none() {
            next_height = Some(tree);
            seen_trees[i][j] = 1;
        }
        return next_height;
    };

    for i in 0..height {
        let mut vis_height = None;
        for j in 0..width {
            vis_height = core(vis_height, i, j);
        }

        let mut vis_height = None;
        for j in (0..width).rev() {
            vis_height = core(vis_height, i, j);
        }
    }

    for j in 0..width {
        let mut vis_height = None;
        for i in 0..height {
            vis_height = core(vis_height, i, j);
        }

        let mut vis_height = None;
        for i in (0..height).rev() {
            vis_height = core(vis_height, i, j);
        }
    }

    // println!("{:?}", seen_trees);

    let mut total = 0;
    for row in seen_trees {
        for seen in row {
            total += seen;
        }
    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parse_forest() {
        let actual = parse_forest(concat!(
            "30373\n",
            "25512\n",
            "65332\n",
            "33549\n",
            "35390\n"
        ));
        let expect = Vec::from([
            Vec::from([3, 0, 3, 7, 3]),
            Vec::from([2, 5, 5, 1, 2]),
            Vec::from([6, 5, 3, 3, 2]),
            Vec::from([3, 3, 5, 4, 9]),
            Vec::from([3, 5, 3, 9, 0]),
        ]);
        assert_eq!(actual, expect);
    }

    #[test]
    fn it_count_visible_trees() {
        let forest = Vec::from([
            Vec::from([3, 0, 3, 7, 3]),
            Vec::from([2, 5, 5, 1, 2]),
            Vec::from([6, 5, 3, 3, 2]),
            Vec::from([3, 3, 5, 4, 9]),
            Vec::from([3, 5, 3, 9, 0]),
        ]);
        let actual = count_visible_trees(&forest);
        let expect = 21;
        assert_eq!(actual, expect);
    }
}

