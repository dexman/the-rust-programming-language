fn main() {
    let mut current = 1;
    let mut previous = 0;
    loop {
        println!("{}", current);
        let tmp = current;
        current += previous;
        previous = tmp;
    }
}
