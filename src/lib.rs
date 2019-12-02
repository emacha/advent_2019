

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

}
