use std::io;

pub fn solution() -> u32 {
    let mut total: u32 = 0;
    let mut totals: Vec<u32> = Vec::new();

    loop {
        let mut line: String = String::new();
        let bytes: usize = io::stdin().read_line(&mut line).expect("Fail");
        if bytes == 0 {
            totals.sort();
            let len: usize = totals.len();
            return totals[len - 1] + totals[len - 2] + totals[len - 3];
        }

        let cals: u32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        total += cals;
        if cals == 0 {
            totals.push(total);
            total = 0;
        }
    }
}
