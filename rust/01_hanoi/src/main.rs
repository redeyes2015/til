fn hanoi (to_move: i32, from : char, to: char, via: char) {
    if to_move == 0 {
        return
    }
    hanoi(to_move - 1, from, via, to);
    println!("Move {} from {} to {}", to_move, from, to);
    hanoi(to_move - 1, via, to, from);
}
fn main() {
    println!("Hello, world!");
    hanoi(10, 'A', 'C', 'B');
}
