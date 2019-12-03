use std::collections::HashSet;

fn main() {
    generate_coords();
}

fn generate_coords() {
    let mut x = 0;
    let mut y = 0;
    let mut coords1: HashSet<(i32, i32)> = HashSet::new();

    let data = get_data_1();

    for i in data {
        let number = i[1..].parse::<i32>().unwrap();
        let letter = &i[..1];
        match letter {
            "L" => {
                for _ in 0..number {
                    x -= 1;
                    coords1.insert((x, y));
                }
            }
            "R" => {
                for _ in 0..number {
                    x += 1;
                    coords1.insert((x, y));
                }
            }
            "U" => {
                for _ in 0..number {
                    y += 1;
                    coords1.insert((x, y));
                }
            }
            "D" => {
                for _ in 0..number {
                    y -= 1;
                    coords1.insert((x, y));
                }
            }
            _ => {}
        }
    }

    x = 0;
    y = 0;
    let mut dupes: HashSet<(i32, i32)> = HashSet::new();

    let data2 = get_data_2();

    for i in data2 {
        let number = i[1..].parse::<i32>().unwrap();
        let letter = &i[..1];
        match letter {
            "L" => {
                for _ in 0..number {
                    x -= 1;
                    dupes.insert((x, y));
                }
            }
            "R" => {
                for _ in 0..number {
                    x += 1;
                    dupes.insert((x, y));
                }
            }
            "U" => {
                for _ in 0..number {
                    y += 1;
                    dupes.insert((x, y));
                }
            }
            "D" => {
                for _ in 0..number {
                    y -= 1;
                    dupes.insert((x, y));
                }
            }
            _ => {}
        }
    }

    let mut dupes_abs: Vec<i32> = Vec::new();

    for i in dupes.intersection(&coords1) {
        dupes_abs.push(i.0.abs() + i.1.abs());
    }
    dupes_abs.sort();
    println!("{:?}", dupes_abs[0])
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
