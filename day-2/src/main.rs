fn split(input: &str) -> (u32, u32, u32) {
    let n: Vec<u32> = input
        .split_terminator('x')
        .map(|f| f.parse().unwrap())
        .collect();

    (n[0], n[1], n[2])
}

fn area(l: u32, w: u32, h: u32) -> u32 {
    (2 * l * w) + (2 * w * h) + (2 * h * l)
}

fn extra(l: u32, w: u32, h: u32) -> u32 {
    (l * w).min(w * h).min(l * h)
}

fn perim(l: u32, w: u32, h: u32) -> u32 {
    (2 * l + 2 * w).min(2 * w + 2 * h).min(2 * l + 2 * h)
}

fn bow(l: u32, w: u32, h: u32) -> u32 {
    l * w * h
}

fn main() {
    let data = include_str!("../input.txt");

    let total: u32 = data
        .lines()
        .map(split)
        .map(|(l, w, h)| area(l, w, h) + extra(l, w, h))
        .sum();

    println!("{total}");

    let total_ribbon: u32 = data
        .lines()
        .map(split)
        .map(|(l, w, h)| perim(l, w, h) + bow(l, w, h))
        .sum();

    println!("{total_ribbon}");
}
