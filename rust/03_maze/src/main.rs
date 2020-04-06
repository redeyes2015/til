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

fn visit(maze : &mut Vec<Vec<MazeCell>>, i : usize, j : usize, end_i : usize, end_j : usize) -> bool {
    if let U = maze[i][j] {
        maze[i][j] = T;
        if let U = maze[end_i][end_j] {
            if !(visit(maze, i - 1, j, end_i, end_j) ||
                 visit(maze, i + 1, j, end_i, end_j) ||
                 visit(maze, i, j + 1, end_i, end_j) ||
                 visit(maze, i, j - 1, end_i, end_j)) {
                maze[i][j] = U;
            }
        }
    }

    match maze[end_i][end_j] {
        W => false,
        U => false,
        T => true,
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
        vec![W, W, W, U, W],
        vec![W, U, U, U, W],
        vec![W, W, U, W, W],
        vec![W, U, U, W, W],
        vec![W, W, W, W, U],
    ];

    dump_maze(maze);

    println!("{}", visit(maze, 3, 1, 0, 3));
    println!("{}", visit(maze, 3, 1, 4, 4));
}
