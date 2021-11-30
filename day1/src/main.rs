use std::fs;

fn main() {
    let input = fs::read_to_string("../input").expect("Something went wrong!");
    println!("{}", floor_counter(&input));
    println!("{}", first_basement(&input).unwrap());
}

fn floor_counter( s: &str ) -> i32 {

    let mut count = 0; 
    for c in s.chars() {
        match c {
            '(' => { count += 1; }
            ')' => { count -= 1; }
            _ => {}
        }
    }
    count
}

fn first_basement( s: &str ) -> Option<i32> {

    let mut count = 0; 
    for (i, c) in s.chars()
                   .enumerate() 
                   {
        match c {
            '(' => { count += 1; }
            ')' => { count -= 1; }
            _ => {}
        }
        if count == -1 {
            return Some(i as i32 + 1)
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(floor_counter("((("), 3);
    }
    #[test]
    fn test2() {
        assert_eq!(floor_counter("((()"), 2);
    }
    #[test]
    fn test3() {
        assert_eq!(floor_counter("((()))))"), -2);
    }
    #[test]
    fn test4() {
        assert_eq!(first_basement("((())))").unwrap(), 7);
    }
    #[test]
    fn test5() {
        assert_eq!(first_basement(")").unwrap(), 1);
    }
    #[test]
    fn test6() {
        assert_eq!(first_basement("((()))(()))").unwrap(), 11);
    }
}
