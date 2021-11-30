use std::fs;

fn main() {
    let input = fs::read_to_string("../input").expect("Wrong input!");
    let input: Vec<&str> = input.split('\n')
                                .collect();
    let mut format_input: Vec<Vec<i32>> = Vec::new();
    for i in input {
        format_input.push(i.split('x')
                    .parse()
                    .unwrap_or(0)
                    .collect());
    }
    println!("{:?}", &format_input);

}
