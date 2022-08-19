use std::collections::HashMap;

fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    if list_art.is_empty() || list_cat.is_empty() {
        return "".to_string();
    }

    let mut category_count: HashMap<String, u32> = HashMap::new();
    list_art.iter()
    .map(|s| s.split(" ").collect::<Vec<_>>())
    .map(|split| (split.get(0).unwrap().chars().next().unwrap(), split.get(1).unwrap().parse::<u32>().unwrap()))
    .for_each(|(category, amount)| {
        let existing_amount = *category_count.entry(category.to_string()).or_insert(0);
        category_count.insert(category.to_string(), existing_amount + amount);
    });

    let result_strings = list_cat.iter()
    .map(|category| (category, category_count.get(&category.to_string()).unwrap_or(&0)))
    .map(|(category, amount)| format!("({} : {})", category, amount))
    .collect::<Vec<_>>();
    
    result_strings.join(" - ")
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(list_art: Vec<&str>, list_cat: Vec<&str>, exp: &str) -> () {
        println!("list_art: {:?};", list_art);
        println!("list_cat: {:?};", list_cat);
        let ans = stock_list(list_art, list_cat);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let mut c = vec!["A", "B", "C", "D"];
        dotest(b, c, "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");

        b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
        c = vec!["A", "B"];
        dotest(b, c, "(A : 200) - (B : 1140)");

    }
}