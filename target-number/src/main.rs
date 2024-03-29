fn main() {
    let result = target_in_array([1, 2, 3, 4, 5], 2);
    println!("o número procurado existe na array? {}", result);
}

fn target_in_array(array: [i8; 5], target: i8) -> bool {

    for num in array {
        if num == target {
            return true;
        } 
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARRAY: [i8; 5] = [2, 3, 5, 8, 13];
    
    #[test]
    fn test_target_in_array() {
        assert_eq!(target_in_array(ARRAY, 5), true);
    }

    #[test]
    fn test_target_in_array_2() {
        assert_eq!(target_in_array(ARRAY, 16), false);
    }

    #[test]
    fn test_target_in_array_3() {
        assert_eq!(!target_in_array(ARRAY, 7), true);
    }
}

