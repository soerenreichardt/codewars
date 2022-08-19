fn descending_order(x: u64) -> u64 {
    let mut numbers: Vec<u8> = Vec::new();
    let mut stride = 1;
    while x / stride != 0 {
        numbers.push(((x / stride) % 10) as u8);
        stride = stride * 10;
    }
    numbers.sort();

    let mut result_number = 0;
    stride = 1;
    for number in numbers {
        result_number = result_number + (number as u64 * stride);
        stride = stride * 10;
    }

    result_number
}

#[test]
fn returns_expected() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}