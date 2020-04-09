struct Board {
    b: [[u8; 8]; 8],
}

impl Board {
    fn is_visitable(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < 8 && y >= 0 && y < 8 && self.b[x as usize][y as usize] == 0
    }
    fn set(&mut self, x: i32, y: i32, v: u8) {
        self.b[x as usize][y as usize] = v;
    }
    fn print(&self) {
        for row in self.b.iter() {
            for cel in row.iter() {
                print!("{:2} ", cel);
            }
            println!("");
        }
    }
}

fn gen_next_steps (board: &Board, x: i32, y: i32) -> Vec<(i32, i32)> {
    [
        (2, 1), (1, 2), (-2, 1), (-1, 2),
        (2, -1), (1, -2), (-2, -1), (-1, -2),
    ].iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|(nx, ny)| board.is_visitable(*nx, *ny))
        .collect()
}

fn next_count (board: &Board, x: i32, y: i32) -> usize {
    gen_next_steps(board, x, y).len()
}

fn knight_tour (x: i32, y: i32) {
    let mut board = Board { b: [[0; 8]; 8] };
    let mut x = x;
    let mut y = y;

    board.set(x, y, 1);

    for step in 2 .. 65 {
        let next_step = gen_next_steps(&board, x, y).iter()
            .map(|(nx, ny)| (*nx, *ny, next_count(&board, *nx, *ny)))
            .min_by_key(|(_, _, next)| *next);

        match next_step {
            Some((nx, ny, _)) => {
                board.set(nx, ny, step);

                x = nx;
                y = ny;
            },
            None => {
                break;
            }
        }
    }

    board.print();
}

fn main() {
    knight_tour(5, 6);
}
