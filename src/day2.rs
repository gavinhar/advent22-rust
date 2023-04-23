use std::io;

pub fn solution1() -> u32 {
    let mut total: u32 = 0;

    loop {
        let mut line: String = String::new();
        let bytes: usize = io::stdin().read_line(&mut line).expect("Fail");
        if bytes == 0 {
            return total;
        }

        let opp = line.chars().nth(0).expect("A");
        let you = line.chars().nth(2).expect("B");

        if opp == 'A' {
            if you == 'X' {
                total += 4;
            } else if you == 'Y' {
                total += 8;
            } else if you == 'Z' {
                total += 3;
            }
        } else if opp == 'B' {
            if you == 'X' {
                total += 1;
            } else if you == 'Y' {
                total += 5;
            } else if you == 'Z' {
                total += 9;
            }
        } else if opp == 'C' {
            if you == 'X' {
                total += 7;
            } else if you == 'Y' {
                total += 2;
            } else if you == 'Z' {
                total += 6;
            }
        }
    }
}

pub fn solution2() -> u32 {
    let mut total: u32 = 0;

    loop {
        let mut line: String = String::new();
        let bytes: usize = io::stdin().read_line(&mut line).expect("Fail");
        if bytes == 0 {
            return total;
        }

        let opp = line.chars().nth(0).expect("A");
        let you = line.chars().nth(2).expect("B");

        if opp == 'A' {
            if you == 'X' {
                total += 3;
            } else if you == 'Y' {
                total += 4;
            } else if you == 'Z' {
                total += 8;
            }
        } else if opp == 'B' {
            if you == 'X' {
                total += 1;
            } else if you == 'Y' {
                total += 5;
            } else if you == 'Z' {
                total += 9;
            }
        } else if opp == 'C' {
            if you == 'X' {
                total += 2;
            } else if you == 'Y' {
                total += 6;
            } else if you == 'Z' {
                total += 7;
            }
        }
    }
}
