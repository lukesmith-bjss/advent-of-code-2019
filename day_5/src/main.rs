// This solution isn't nice. It's possibly the worst code ever written. If you find this, leave now.

fn main() {
    opcode(get_data());
}

fn opcode(mut data: Vec<i32>) -> Vec<i32> {
    let mut i = 0;

    loop {
        if data[i] as usize == 99 {
            break;
        }
        let instruction: String = get_opcode(data[i] as usize);
        let modes = get_modes(data[i] as usize );
        let mode1 = &modes[2..3];
        let mode2 = &modes[1..2];
        let mode3 = &modes[0..1];
        let param1;
        let param2;
        let location;
        match instruction.as_str() {
            "1" => {
                if mode1 == "0" {
                    param1 = data[data[i + 1] as usize];

                } else {
                    param1 = data[i + 1];
                }

                if mode2 == "0" {
                    param2 = data[data[i + 2] as usize];
                } else {
                    param2 = data[i + 2];
                }

                if mode3 == "0" {
                    location = data[i + 3];
                } else {
                    location = data[data[i + 3] as usize];
                }
                let calc = param1 + param2;
                data[location as usize] = calc;
                i += 4;
            }
            "2" => {
                if mode1 == "0" {
                    param1 = data[data[i + 1] as usize];
                } else {
                    param1 = data[i + 1];
                }

                if mode2 == "0" {
                    param2 = data[data[i + 2] as usize];
                } else {
                    param2 = data[i + 2];
                }

                if mode3 == "0" {
                    location = data[i + 3];
                } else {
                    location = data[data[i + 3] as usize];
                }
                let calc = param1 * param2;
                data[location as usize] = calc;
                i += 4;
            }
            "3" => {
                let input = 5;
                let location = data[i + 1] as usize;
                data[location] = input;
                i += 2;
            }
            "4" => {
                println!("DIAGNOSTIC: {}", data[data[i + 1] as usize]);
                i += 2;
            }
            "5" => {
                println!("modes5: {}", modes);

                if mode1 == "0" {
                    if data[data[i + 1] as usize] != 0 {
                        if mode2 == "0" {
                            i = data[data[i+2] as usize] as usize;
                        } else {
                            i = data[i+2] as usize;
                        }
                    } else {
                        i += 3;
                    }
                } else {
                    if data[i + 1] != 0 {
                        if mode2 == "0" {
                            i = data[data[i+2] as usize] as usize;
                        } else {
                            i = data[i+2] as usize;
                        }
                    } else {
                        i += 3;
                    }
                }

            }
            "6" => {
                println!("modes6: {}", modes);

                if mode1 == "0" {
                    if data[data[i + 1] as usize] == 0 {
                        if mode2 == "0" {
                            i = data[data[i+2] as usize] as usize;
                        } else {
                            i = data[i+2] as usize;
                        }
                    } else {
                        i += 3;
                    }
                } else {
                    if data[i + 1] == 0 {
                        if mode2 == "0" {
                            i = data[data[i+2] as usize] as usize;
                        } else {
                            i = data[i+2] as usize;
                        }
                    } else {
                        i += 3;
                    }
                }
            }
            "7" => {
                if mode1 == "0" {
                    param1 = data[data[i + 1] as usize];
                } else {
                    param1 = data[i + 1];
                }

                if mode2 == "0" {
                    param2 = data[data[i + 2] as usize];
                } else {
                    param2 = data[i + 2];
                }

                if mode3 == "0" {
                    location = data[i + 3];
                } else {
                    location = data[data[i + 3] as usize];
                }

                if param1 < param2 {
                    data[location as usize] = 1;
                } else {
                    data[location as usize] = 0;
                }
                i += 4;
            }
            "8" => {
                if mode1 == "0" {
                    param1 = data[data[i + 1] as usize];
                } else {
                    param1 = data[i + 1];
                }

                if mode2 == "0" {
                    param2 = data[data[i + 2] as usize];
                } else {
                    param2 = data[i + 2];
                }

                if mode3 == "0" {
                    location = data[i + 3];
                } else {
                    location = data[data[i + 3] as usize];
                }

                if param1 == param2 {
                    data[location as usize] = 1;
                } else {
                    data[location as usize] = 0;
                }
                i += 4;
            }
            "99" => {
                break;
            }
            _ => {}
        }
        println!("i: {}", i);
        println!("instruction: {}", instruction);
    }
    data
}

fn get_opcode(opcode: usize) -> String {
    let opcode = opcode.to_string();
    let instruction: String = opcode[(opcode.chars().count() - 1)..(opcode.chars().count())].to_string();
    instruction
}

fn get_modes(opcode: usize) -> String {
    let opcode = opcode.to_string();

    if opcode.chars().count() == 1 {
        return "000".to_string();

    } else if opcode.chars().count() == 3 {
        return "00".to_string() + &opcode[0..1];
    }
    let modes: String = opcode[0..(opcode.chars().count() - 2)].to_string();
    if modes.chars().count() == 2 {
        return "0".to_string() + &modes;
    } else if modes.chars().count() == 1 {
        return "0".to_string() + &modes + "0";
    }
    modes
}

fn get_data() -> Vec<i32> {
    include_str!("../resources/input").split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}
