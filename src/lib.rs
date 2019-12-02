

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

pub fn intcode(input: String) -> String {
    "".parse().unwrap()
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
        assert_eq!(intcode("1,0,0,0,99".parse().unwrap()),
                   "2,0,0,0,99".parse::<String>().unwrap());
        assert_eq!(intcode("2,3,0,3,99".parse().unwrap()),
                   "2,3,0,6,99".parse::<String>().unwrap());
        assert_eq!(intcode("2,4,4,5,99,0".parse().unwrap()),
                   "2,4,4,5,99,9801".parse::<String>().unwrap());
        assert_eq!(intcode("1,1,1,4,99,5,6,0,99".parse().unwrap()),
                   "30,1,1,4,2,5,6,0,99".parse::<String>().unwrap());
    }
}
