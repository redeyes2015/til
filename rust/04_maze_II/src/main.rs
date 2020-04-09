enum MazeCell {
    Wall,
    PathTaken,
    PathUntaken,
}

use MazeCell::{
    Wall as W,
    PathTaken as T,
    PathUntaken as U,
};

use std::fmt;
impl fmt::Display for MazeCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            W => write!(f, "W"),
            T => write!(f, "T"),
            U => write!(f, "U"),
        }
    }
}

fn visit(maze : &mut Vec<Vec<MazeCell>>, i : usize, j : usize, end_i : usize, end_j : usize) {
    if let U = maze[i][j] {
        maze[i][j] = T;
        if i == end_i && j == end_j {
            dump_maze(maze);
            maze[i][j] = U;
            return;
        }
        visit(maze, i - 1, j, end_i, end_j);
        visit(maze, i + 1, j, end_i, end_j);
        visit(maze, i, j + 1, end_i, end_j);
        visit(maze, i, j - 1, end_i, end_j);
        maze[i][j] = U;
    }
}

fn dump_maze(maze : & Vec<Vec<MazeCell>>) {
    for row in maze.iter() {
        for cell in row.iter() {
            print!("{}", cell);
        }
        println!("");
    }
}

fn main() {
    let maze = &mut vec![
        vec![W, W, W, W, W],
        vec![W, W, U, U, W],
        vec![W, U, U, U, W],
        vec![W, W, U, W, W],
        vec![W, U, U, W, W],
        vec![W, W, W, W, U],
    ];

    dump_maze(maze);
    println!("--------------------");

    visit(maze, 4, 1, 1, 3);
    println!("--------------------");
    visit(maze, 4, 1, 4, 4);
}
