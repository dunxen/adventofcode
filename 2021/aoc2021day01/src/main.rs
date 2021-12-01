use itertools::Itertools;

fn main() {
    println!(
        "{}",
        include_str!("./input.txt")
            .split('\n')
            .filter_map(|s| s.parse::<i32>().ok())
            .tuple_windows::<(_, _, _)>()
            .tuple_windows()
            .filter(|(a, b)| b.2 > a.0)
            .count()
    )
}
