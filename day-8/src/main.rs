use std::{collections::HashMap, fs};

type Position = (usize, usize);
struct Grid(Vec<Vec<char>>);

impl Grid {
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn get(&self, position: Position) -> char {
        self.0[position.1][position.0]
    }
}

#[derive(Clone)]
struct Tree {
    position: Position,
    value: usize,
}

impl Tree {
    fn new(position: Position, grid: &Grid) -> Self {
        let value = grid.get(position);
        let value = value.to_string().parse::<usize>().unwrap();

        Tree { position, value }
    }
}
#[derive(Clone)]
struct TreeSlice {
    first: Tree,
    last: Tree,
    local: Vec<Tree>,
}
#[derive(Clone)]
struct TreeRow(TreeSlice);

impl TreeRow {
    fn new(y: usize, grid: &Grid) -> Self {
        let length = grid.width();
        TreeRow(TreeSlice {
            first: Tree::new((0, y), grid),
            last: Tree::new((length, y), grid),
            local: vec![],
        })
    }

    fn insert_tree(&self, tree: Tree) {}
}

type TreeFromView = HashMap<usize, TreeRow>;
struct TreesFromLeft(TreeFromView);
struct TreesFromTop(TreeFromView);
fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let content = fs::read_to_string("./day-8/input")?;

    let grid = content
        .lines()
        .map(|content| content.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let grid = Grid(grid);

    let mut visible_by_y = TreesFromLeft(HashMap::new());
    let mut visible_by_x = TreesFromTop(HashMap::new());
    let right = grid.width();
    let bottom = grid.height();

    let top = 1;
    let left = 1;

    for y in top..bottom {
        for x in left..right {
            let row = match visible_by_y.0.get(&y) {
                Some(row) => row.clone(),
                None => {
                    let row = TreeRow::new(y, &grid);
                    let value = visible_by_y.0.insert(y, row);
                    value.unwrap()
                }
            };
        }
    }

    Ok(())
}
