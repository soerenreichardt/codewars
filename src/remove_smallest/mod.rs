fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    match numbers.iter().min() {
        Some(min) => {
            let (index, _) = numbers.iter().enumerate().find(|(_i, n)| *n == min).unwrap();
            numbers.iter().enumerate().filter(|(i, _n)| *i != index).map(|(_i, &n)| n).collect::<Vec<_>>()
        },
        None => numbers.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::remove_smallest;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(a: &[u32], expected: &[u32]) {
        assert_eq!(remove_smallest(a), expected, "{ERR_MSG} with numbers = {a:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 4, 5], &[2, 3, 4, 5]);
        dotest(&[1, 2, 3, 4], &[2, 3, 4]);
        dotest(&[5, 3, 2, 1, 4], &[5, 3, 2, 4]);
        dotest(&[1, 2, 3, 1, 1], &[2, 3, 1, 1]);
    }
}