

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





#[cfg(test)]
mod tests {
    use crate::fuel_cost;
    // Test 4 examples of fuel_cost
    #[test]
    fn test_fuel_cost() {
        assert_eq!(fuel_cost(12), 2);
        assert_eq!(fuel_cost(14), 2);
        assert_eq!(fuel_cost(1969), 966);
        assert_eq!(fuel_cost(100756), 50346);
    }
}
