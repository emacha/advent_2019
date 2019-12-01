
pub fn fuel_cost(mass: u64) -> u64 {
    mass
}




#[cfg(test)]
mod tests {
    use crate::fuel_cost;
    // Test 4 examples of fuel_cost
    #[test]
    fn test_fuel_cost() {
        assert_eq!(fuel_cost(12), 2);
        assert_eq!(fuel_cost(14), 2);
        assert_eq!(fuel_cost(1969), 654);
        assert_eq!(fuel_cost(100756), 33583);
    }
}
