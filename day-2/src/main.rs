const REQUIRED: (i32, i32, i32) = (12, 13, 14);

fn is_possible(x: (i32, i32, i32), other: (i32, i32, i32)) -> bool {
    if x.0 > other.0 {
        return false;
    }
    if x.1 > other.1 {
        return false;
    }
    if x.2 > other.2 {
        return false;
    }
    true
}

fn part1(input: &str) {
    let mut id_sum: usize = 0;

    for (id, i) in input.lines().enumerate() {
        let start_offset = i.find(":").unwrap() + 1;
        let colors_group = i[start_offset..].trim();
        let mut possible = true;

        for colors in colors_group.split(";") {
            let color = get_color(colors);

            if !is_possible(color, REQUIRED) {
                possible = false;
                break;
            }
        }

        if possible {
            id_sum += id + 1;
        }
    }

    println!("{id_sum}")
}

fn get_color(input: &str) -> (i32, i32, i32) {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    for color in input.split(',') {
        let (count, color) = color.trim().split_once(" ").unwrap();
        let count: i32 = count.parse().unwrap();

        let col = match color {
            "red" => &mut r,
            "green" => &mut g,
            "blue" => &mut b,
            _ => unreachable!(),
        };

        *col += count;
    }
    (r, g, b)
}

fn part2(input: &str) {
    let mut power_set_sum: usize = 0;

    for i in input.lines() {
        let mut min_color = (0, 0, 0);

        let start_offset = i.find(":").unwrap() + 1;
        let colors_group = i[start_offset..].trim();

        for colors in colors_group.split(";") {
            let color = get_color(colors);

            if color.0 > min_color.0 {
                min_color.0 = color.0
            }
            if color.1 > min_color.1 {
                min_color.1 = color.1
            }
            if color.2 > min_color.2 {
                min_color.2 = color.2
            }
        }

        let power = (min_color.0 * min_color.1 * min_color.2) as usize;
        power_set_sum += power;
    }
    println!("{power_set_sum}");
}

fn main() {
    let input = include_str!("../input.txt");
    part1(input);
    part2(input);
}