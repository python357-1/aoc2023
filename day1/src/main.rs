use std::fs;

fn convert(input_line: String) -> String {
    input_line.re
    String::from("")
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut accumulator: i32 = 0;
    file
        .lines()
        .map(|x| x
            .chars()
            .filter(|y| y.is_digit(10))
            .map(|z| z.to_string().parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        )
        .for_each(|x| {
            accumulator += x.first().unwrap() * 10;
            accumulator += x.last().unwrap();
        });

    println!("{:?}", accumulator);
}
