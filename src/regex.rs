use regex::{Regex, RegexSet};

fn main() {
    let emp_pattern = r"\s*";
    let comment_pattern = r"//.*";
    let num_pattern = r"(?P<num>[0-9]+)";
    let id_pattern = r"(?P<id>[A-Z_a-z][A-Z_a-z0-9]*|==|<=|>=|&&|\|\||\p{Punct})";
    let str_pattern = "\"(\\n|[^\"])*";

    let set = RegexSet::new(&[
        emp_pattern, // 0
        comment_pattern, // 1
        num_pattern, // 2
        str_pattern, // 3
        id_pattern, // 4
    ]).unwrap();

    let target_num = 12345.to_string();
    let target_str = "sample";

    let num_matches: Vec<_> = set.matches(&target_num).into_iter().collect();
    assert_eq!(num_matches, vec![0, 2, 3]);
    let re1 = Regex::new(num_pattern).unwrap();
    let cap_num = re1.captures(&target_num).unwrap();
    assert_eq!("12345", &cap_num["num"]);

    let str_matches: Vec<_> = set.matches(&target_str).into_iter().collect();
    assert_eq!(str_matches, vec![0, 3, 4]);
    let re2 = Regex::new(str_pattern).unwrap();
    let cap_str = re2.captures(&target_str).unwrap();
    assert_eq!("sample", &cap_str[0]);
}
