use std::fs;
fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file.");
    let input = input.trim().lines();

    let mut grid: Vec<Vec<bool>> = Vec::new();

    for _i in 0..1000 {
        let mut inner: Vec<bool> = Vec::new();

        for _j in 0..1000 {
            inner.push(false);
        }
        grid.push(inner);
    }

    for i in input {
        let j: Vec<&str> = i.split(" ").filter(|x| x.contains(",")).collect();
        let x0: usize = j[0].split(",").nth(0).unwrap().parse().unwrap();
        let x1: usize = j[1].split(",").nth(0).unwrap().parse().unwrap();
        let y0: usize = j[0].split(",").nth(1).unwrap().parse().unwrap();
        let y1: usize = j[1].split(",").nth(1).unwrap().parse().unwrap();

        println!("{},{} - {},{}", x0, y0, x1, y1);
        for k in x0..=x1 {
            for l in y0..=y1 {
                if i.contains("turn on") {
                    grid[k][l] = true;
                } else if i.contains("turn off") {
                    grid[k][l] = false;
                } else if grid[k][l] == false {
                    grid[k][l] = true;
                } else {
                    grid[k][l] = false;
                }
            }
        }
    }

    let mut num = 0;
    for i in grid {
        for j in i {
            if j { num += 1; }
        }
    }

    println!("{}", num);
}
