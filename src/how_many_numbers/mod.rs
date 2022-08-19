fn find_all(sum_dig: u8, digs: u8) -> Option<(usize, u64, u64)> {
    let limit = (10 as u64).pow(digs as u32);
    let mut start = 0;
    for digit in 0..digs {
        start = start + (10 as u64).pow(digit as u32);
    }
    let mut candidates = Vec::new();

    for number in start..limit {
        let mut sum = 0;
        let mut last_num = std::u64::MAX;
        let mut ordered = true;
        let mut increment = 1;
        for _digit in 0..digs as u32 {
            let num = (number / increment) % 10;
            increment = increment * 10;
            sum = sum + num;
            if num > last_num {
                ordered = false;
                break;
            }
            last_num = num;
        }

        if sum as u8 == sum_dig && ordered {
            candidates.push(number);
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
