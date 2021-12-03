use std::fs;
use std::mem::swap;


fn main() {

    // This is probably really dumb, but I couldn't find another way.
    // When i tried to do all trimming and parsing and so on on the same input, i got
    // "creates a temporary which is freed while still in use"
    let input = fs::read_to_string("input")
        .expect("Couldn't read file");
    let input: Vec<&str> = input
        .trim()
        .lines()
        .collect();
    let input = {
        let mut vec = vec!();
        for i in input {
            let x: Vec<i32> = i
                .split('x')
                .map(|n| n.parse().unwrap_or(0))
                .collect();
            vec.push(x);
        }
        vec
    };
    println!("{:?}", &input);

}

fn mantel_area( cube: Vec<i32> ) -> i32 {
    2 * cube[0] * cube[1] + 2 * cube[1] * cube[2] + 2 * cube[0] * cube[2]
}

fn find_smallest_square( cube: Vec<i32> ) -> i32 {
    let mut cube = cube.clone();
    if cube[0] > cube[1] {cube.swap(0,1)}
    if cube[1] > cube[2] {cube.swap(1,2)}


    cube[0] * cube[1]
}

#[cfg(test)]
mod test {
    use crate::mantel_area;
    use super::find_smallest_square;

    #[test]
    fn test1() { assert_eq!(find_smallest_square(vec!(44,2,3)), 6)}

    #[test]
    fn test2() { assert_eq!(mantel_area(vec!(1,2,3)), 22)}
}
