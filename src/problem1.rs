fn compute_sum_numbers(number:i128) -> i128{
    let mut count: i128 = 0;
    let mut result: i128 = 0;
    while count < number  {
        if count%3 == 0 || count %5 == 0 {
            result = result + count;
        }
        count = count +1
        
    }
    result
}
mod tests{
    use super::*;
    #[test]
    fn test_compute_sum_numbers(){
        assert_eq!(compute_sum_numbers(10),23)
    }
}

