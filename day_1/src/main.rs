fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn part_1() -> u32 {
    get_data().iter()
        .map(calculate_fuel)
        .sum::<u32>()
}

fn part_2() -> u32 {
    get_data().iter()
        .map(fuel_part_2)
        .sum::<u32>()
}

fn calculate_fuel(mass: &u32) -> u32 {
    std::cmp::max(0, (mass / 3) as i32 - 2) as u32
}

fn fuel_part_2(mass: &u32) -> u32 {
    let mut sum = 0;
    let mut fuel = *mass;

    while fuel > 0 {
        fuel = calculate_fuel(&fuel);
        sum += fuel;
    }

    sum
}

fn get_data() -> Vec<u32> {
    include_str!("../resources/input").split("\n").filter(|l| l.len() > 0)
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}
