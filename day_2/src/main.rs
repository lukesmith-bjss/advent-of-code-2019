fn main() {
    println!("Answer = {:?}", opcode(get_data()));
    println!("Part 2 = {:?}", part_2(get_data()));
}

fn opcode(mut data: Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    loop {
        match data[i] {
            1 => {
                let index = data[i+3];
                let calc = data[data[i+1]] + data[data[i +2]];
                data[index] = calc;
            }
            2 => {
                let index = data[i+3];
                let calc = data[data[i+1]] * data[data[i +2]];
                data[index] = calc;
            }
            99 => {
                break
            }
            _ => {

            }
        }
        i += 4;
    }
    data
}

fn part_2(data: Vec<usize>) -> usize {
    for noun in 1..99 {
        for verb in 1..99 {
            let mut data = data.clone();
            data[1] = noun;
            data[2] = verb;
            if opcode(data)[0] == 19690720 {
                return noun * 100 + verb;
            }
        }
    }
    0
}

fn get_data() -> Vec<usize> {
    include_str!("../resources/input").split(',')
        .map(|i| i.parse::<usize>().unwrap())
        .collect()
}
