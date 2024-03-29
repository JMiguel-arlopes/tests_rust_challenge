fn main() {
    let order_list: [i8; 5] = [1, 2, 3, 4, 5];
    let result = find_pars(order_list, 9);   

    println!("numero de pares formados pelo alvo: {} ", result);
}

fn find_pars(order_list: [i8; 5], target: i8) -> i8 {
    let mut mount_pars = 0;
    let mut current_idx = 0;
    let mut final_idx = 4;


    while current_idx < final_idx {
        let sum = order_list[current_idx] + order_list[final_idx];

        if sum == target {
            mount_pars += 1;
            current_idx += 1;
            final_idx -= 1;
        } else if sum > target {
            final_idx -= 1;
        } else if sum < target {
            current_idx += 1;
        }
    }
    return mount_pars;
}


#[cfg(test)]
mod tests {
    use super::*;
    
    const ORDER_LIST: [i8; 5] = [1, 2, 3, 4, 5];
    
    #[test]
    fn test_find_pars() {
        assert_eq!(find_pars(ORDER_LIST, 6), 2)
    }

    #[test]
    fn test_find_pars_2() {
        assert_eq!(find_pars(ORDER_LIST, 3), 1)
    }

    #[test]
    fn test_find_pars_3() {
        assert_eq!(find_pars(ORDER_LIST, 10), 0)
    }
}
