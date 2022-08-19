fn find_all(sum_dig: u8, digs: u8) -> Option<(usize, u64, u64)> {
    let limit = (10 as i32).pow(digs as u32);
    let start = (10 as i32).pow((digs-1) as u32);
    let mut candidates = Vec::new();
    
    for number in start..limit {
        let mut sum: u8 = 0;
        let mut last_num = std::u8::MAX;
        let mut candidate: u64 = 0;
        let mut ordered = true;
        for digit in 0..digs {
            let num = ((number / (10 as i32).pow(digit as u32)) % 10) as u8;
            sum = sum + num;
            if num > last_num {
                ordered = false;
            }
            last_num = num;
            candidate = candidate + (num as u64 * (10 as u64).pow(digit as u32));
        }

        if sum == sum_dig && ordered {
            candidates.push(candidate);
        }
    }

    match candidates.len() {
        0 => None,
        len => Some((len, *candidates.first().unwrap(), *candidates.last().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::find_all;

    #[test]
    fn sample_tests() {
        assert_eq!(find_all(10, 3), Some((8, 118, 334)));
        assert_eq!(find_all(27, 3), Some((1, 999, 999)));
        assert_eq!(find_all(84, 4), None);
        assert_eq!(find_all(35, 6), Some((123, 116999, 566666)));
    }
}
