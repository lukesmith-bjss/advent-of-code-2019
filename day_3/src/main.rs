use std::collections::HashSet;

fn main() {
    generate_coords();
}

fn generate_coords() {
    let wire1_coords: HashSet<(i32, i32)> = calc(get_data_1());
    let wire2_coords: HashSet<(i32, i32)> = calc(get_data_2());

    let mut dupes_abs: Vec<i32> = Vec::new();

    for i in wire2_coords.intersection(&wire1_coords) {
        dupes_abs.push(i.0.abs() + i.1.abs());
    }
    dupes_abs.sort();
    println!("{:?}", dupes_abs[0])
}

fn calc(data: Vec<String>) -> HashSet<(i32, i32)> {
    let mut x = 0;
    let mut y = 0;
    let mut coord_set: HashSet<(i32, i32)> = HashSet::new();

    for i in data {
        let number = i[1..].parse::<i32>().unwrap();
        let letter = &i[..1];
        match letter {
            "L" => {
                for _ in 0..number {
                    x -= 1;
                    coord_set.insert((x, y));
                }
            }
            "R" => {
                for _ in 0..number {
                    x += 1;
                    coord_set.insert((x, y));
                }
            }
            "U" => {
                for _ in 0..number {
                    y += 1;
                    coord_set.insert((x, y));
                }
            }
            "D" => {
                for _ in 0..number {
                    y -= 1;
                    coord_set.insert((x, y));
                }
            }
            _ => {}
        }
    }
    coord_set
}

fn get_data_1() -> Vec<String> {
    include_str!("../resources/wire1").split(',')
        .map(|i| i.parse::<String>().unwrap())
        .collect()
}

fn get_data_2() -> Vec<String> {
    include_str!("../resources/wire2").split(',')
        .map(|i| i.parse::<String>().unwrap())
        .collect()
}
