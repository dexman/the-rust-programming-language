use std::collections::HashMap;

fn main() {
    let values = vec![9, 1, 2, 9, 4, 5, 6, 7, 8, 9];
    let avg = average(&values);
    let med = median(&values);
    let m = mode(&values);
    println!("Hello, world! {:?}", values);
    println!("Average={}", avg);
    println!("Median={}", med);
    println!("Mode={:?}", m);

    let p = translate_to_pig_latin(&"apple");
    println!("apple={}", p);
    let p = translate_to_pig_latin(&"first");
    println!("first={}", p);
}

fn average(values: &[i32]) -> f64 {
    let total: i32 = values.iter().sum();
    (total as f64) / (values.len() as f64)
}

fn median(values: &[i32]) -> i32 {
    let mut sorted = values.to_vec();
    sorted.sort();
    sorted[sorted.len() / 2]
} 

fn mode(values: &[i32]) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for value in values {
        let count = counts.entry(*value).or_insert(0);
        *count += 1;
    }
    let mut sorted_counts = counts.iter().collect::<Vec<_>>();
    sorted_counts.sort_by(|a, b| a.0.cmp(b.0));
    *(sorted_counts.last().unwrap().0)
}

fn translate_to_pig_latin(s: &str) -> String {
    if s.is_empty() {
        return String::from(s)
    }
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    if vowels.contains(&s.chars().next().unwrap()) {
        format!("{}-hay", s)
    } else {
        let mut chars = s.chars();
        let first_char = chars.next().unwrap();
        let remaining = chars.as_str();
        format!("{}-{}ay", remaining, first_char)
    }
}
