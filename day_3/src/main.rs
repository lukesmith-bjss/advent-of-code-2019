use indexmap::set::IndexSet;

fn main() {
    let wire1_coords: IndexSet<(i32, i32)> = calc_coords_for_input(get_data_1());
    let wire2_coords: IndexSet<(i32, i32)> = calc_coords_for_input(get_data_2());

    let mut dupes_coord: IndexSet<(i32, i32)> = IndexSet::new();
    let mut dupes_abs: Vec<i32> = Vec::new();

    for i in wire2_coords.intersection(&wire1_coords) {
        dupes_coord.insert(*i);
        dupes_abs.push(i.0.abs() + i.1.abs());
    }
    dupes_abs.sort();

    println!("Part 1 answer: {:?}", dupes_abs[0]);

    let mut coords_as_summed_indices: IndexSet<usize> = IndexSet::new();

    for i in &dupes_coord {
        let wire_1_index = wire1_coords.get_full(i).unwrap().0;
        let wire_2_index = wire2_coords.get_full(i).unwrap().0;

        coords_as_summed_indices.insert(wire_1_index + 1 + wire_2_index + 1);
    }


    coords_as_summed_indices.sort();

    println!("Part 2 answer: {:?}", coords_as_summed_indices.get_index(0).unwrap());

}

fn calc_coords_for_input(data: Vec<String>) -> IndexSet<(i32, i32)> {
    let mut x = 0;
    let mut y = 0;
    let mut coord_set: IndexSet<(i32, i32)> = IndexSet::new();

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
