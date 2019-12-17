use std::i32::MAX;

pub fn fuel_cost(mass: i64) -> i64 {
    if mass <= 0 {
         0
    } else {
        let cost = mass / 3 - 2;
        let added_cost = fuel_cost(cost);
        if added_cost > 0 {
            cost + added_cost
        } else {
            cost
        }
    }
}

pub fn intcode(input: &String) -> String {
    let mut code: Vec<i32> = input.split(",").map(|x| x.parse().unwrap())
        .collect();
    let mut i = 0;
    loop {
        match code[i] {
            1 => {
                let s = code[code[i + 1] as usize] + code[code[i + 2] as usize];
                let out_i = code[i + 3] as usize;
                code[out_i] = s;
                i = i + 4;
            },
            2 => {
                let s = code[code[i + 1] as usize] * code[code[i + 2] as usize];
                let out_i = code[i + 3] as usize;
                code[out_i] = s;
                i = i + 4;
            },
            99 => break,
            _ => panic!("Unknown opcode!")
        }
    }
    let mut out: String = String::new();
    for part in code {
        out.push_str(part.to_string().as_ref());
        out.push(',')
    }
    out.pop();
    out
}

pub fn new_intcode(noun: &str, verb: &str, code: &String) -> String {
    let new_code = format!("{},{},{},{}", &code[0..1], &noun,
                           &verb,  &code[6..]);
    new_code
}

pub fn create_path(wire: &str) -> Vec<(i32, i32)> {
    let mut path: Vec<(i32, i32)> = vec![(0, 0)];
    for steps in wire.split(",") {
        let direction: &str = &steps[..1];
        let length: i32 = steps[1..].parse().unwrap();
        match direction {
            "U" => {
                for _ in 0..length {
                    let mut coord = (0, 0);
                    if let Some((x, y)) = path.last() {
                        coord = (*x, *y + 1)
                    }
                    path.push(coord);
                }
            },
            "D" => {
                for _ in 0..length {
                    let mut coord = (0, 0);
                    if let Some((x, y)) = path.last() {
                        coord = (*x, *y - 1)
                    }
                    path.push(coord);
                }
            },
            "R" => {
                for _ in 0..length {
                    let mut coord = (0, 0);
                    if let Some((x, y)) = path.last() {
                        coord = (*x + 1, *y)
                    }
                    path.push(coord);
                }
            },
            "L" => {
                for _ in 0..length {
                    let mut coord = (0, 0);
                    if let Some((x, y)) = path.last() {
                        coord = (*x - 1, *y)
                    }
                    path.push(coord);
                }
            },
            _ => panic!("Unknown direction!")
        }
    };
    path
}

pub fn intersections(xs: &Vec<(i32, i32)>, ys: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut cross: Vec<(i32, i32)> = vec![];
    for x in xs.iter() {
        for y in ys.iter() {
            if x == y {cross.push(*x)}
        }
    }
    cross
}

pub fn intersection_distance(wire1: &str, wire2: &str) -> i32 {
    let path1 = create_path(wire1);
    let path2 = create_path(wire2);

    let crossings = intersections(&path1, &path2);
    let mut distance: i32 = MAX;
    for (x, y) in crossings.iter() {
        if (*x, *y) == (0, 0) {continue}
        if distance > x.abs() + y.abs() {distance = x.abs() + y.abs()}
    }
    distance
}

pub fn passes_rules(password: &i32) -> bool {
    let mut checks: Vec<bool> = vec![];
    let password_str = password.to_string();
    let double_digits = |pass: &String| -> bool {
        let mut lens = vec![];
        let mut last: char = "a".parse().unwrap();
        let mut seq: String = String::new();

        for digit in pass.chars() {

            if digit == last {
                seq.push(digit);
            } else {
                lens.push(seq.len() + 1);
                seq = String::new();
            }
            last = digit;

        }
        lens.push(seq.len() + 1);

        println!("{:?}", lens);
        for len in lens.iter() {
            if (*len > 1 as usize) & (*len % 2 != 0) {return false}
            if *len > 1 as usize {return true}
        }
        false
    };
    let increasing = |pass: &String| -> bool {
        let mut last: u8 = "0".parse().unwrap();
        for digit in pass.chars() {
            if digit as u8 >= last {
                last = digit as u8;
            } else {
                return false
            }
        }
        true
    };

    //checks.push((*password > 183564) & (*password < 657474));
    checks.push(password_str.len() == 6);
    checks.push(double_digits(&password_str));
    checks.push(increasing(&password_str));

    checks.iter().fold(true, |acc, x| acc & x)
}


#[cfg(test)]
mod tests {
    use crate::*;
    // Test 4 examples of fuel_cost
    #[test]
    fn test_fuel_cost() {
        assert_eq!(fuel_cost(12), 2);
        assert_eq!(fuel_cost(14), 2);
        assert_eq!(fuel_cost(1969), 966);
        assert_eq!(fuel_cost(100756), 50346);
    }

    #[test]
    fn test_intcode() {
        assert_eq!(intcode(&String::from("1,0,0,0,99")),
                   "2,0,0,0,99".parse::<String>().unwrap());
        assert_eq!(intcode(&String::from("2,3,0,3,99")),
                   "2,3,0,6,99".parse::<String>().unwrap());
        assert_eq!(intcode(&String::from("2,4,4,5,99,0")),
                   "2,4,4,5,99,9801".parse::<String>().unwrap());
        assert_eq!(intcode(&String::from("1,1,1,4,99,5,6,0,99")),
                   "30,1,1,4,2,5,6,0,99".parse::<String>().unwrap());
    }

    #[test]
    fn test_new_code() {
        let input = String::from("0,1,2,3");
        let new = new_intcode("99", "88", &input);
        assert_eq!(new, "0,99,88,3");
    }

    #[test]
    fn test_intersection_distance() {
        assert_eq!(intersection_distance("R75,D30,R83,U83,L12,D49,R71,U7,L72",
        "U62,R66,U55,R34,D71,R55,D58,R83"), 159);
        assert_eq!(intersection_distance("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
        "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 135);
    }

    #[test]
    fn test_create_path() {
        assert_eq!(create_path("U2,L1"), vec![(0, 0),
                                              (0, 1), (0, 2),
                                              (-1, 2)])
    }

    #[test]
    fn test_intersection() {
        let xs = vec![(0, 0), (1, 1), (-1, 1)];
        let ys = vec![(1, 1), (11, 1), (0, 0)];
        assert_eq!(intersections(&xs, &ys), vec![(0, 0), (1, 1)])
    }

    #[test]
    fn test_password_rules() {
        assert!(passes_rules(&111111));
        assert!(!passes_rules(&123));
        assert!(!passes_rules(&223450));
        assert!(!passes_rules(&123789));
        assert!(!passes_rules(&123444));
        assert!(passes_rules(&111122));
    }
}
