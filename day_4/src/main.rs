fn main() {
    let data = get_data();
    let mut count = 0;

    for i in data[0]..data[1] {
        if contains_consecutive_matches(i) && numbers_increase(i) {
            count += 1;
        }
    }
    println!("Part 1 {:?}", count);

    count = 0;

    for i in data[0]..data[1] {
        if numbers_increase(i)
            && contains_consecutive_matches(i)
            && contains_one_pair_no_more(i) {
            count += 1;
        }
    }

    println!("Part 2 {:?}", count);
}

fn contains_one_pair_no_more(number: u32) -> bool {
    let mut prev = 0;
    let mut count = 0;
    let mut consecutive_groups = Vec::<i32>::new();
    let number_vec = number_to_vec(number);

    for i in number_vec {
        if i == prev {
            count += 1;
        } else {
            consecutive_groups.push(count);
            count = 1;
        }

        prev = i;
    }

    consecutive_groups.push(count);

    for i in consecutive_groups {
        if i == 2 {
            return true;
        }
    }

    false
}

fn contains_consecutive_matches(number: u32) -> bool {
    let number_vec = number_to_vec(number);

    let mut last = 0;

    for i in number_vec {
        if i == last {
            return true;
        }
        last = i;
    }

    return false;
}

fn numbers_increase(number: u32) -> bool {

    let number_vec = number_to_vec(number);

    let mut prev = 0;

    for i in number_vec {
        let current = i;

        if current < prev {
            return false;
        }
        prev = current;
    }

    return true;
}

fn number_to_vec(number: u32) -> Vec<u32> {

    let new_vec = number.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    new_vec
}

fn get_data() -> Vec<u32> {
    include_str!("../resources/input").split("-").filter(|l| l.len() > 0)
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}