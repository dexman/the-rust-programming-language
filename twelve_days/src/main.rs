fn main() {
    // The gift to give on each day.
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four colly birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    // The english name of each day.
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];

    // Outer loop: loop over each day.
    for i in 0..12 {
        println!("On the {} day of Christmas", days[i]);
        println!("My true love sent to me");

        // Inner loop: print gifts for this day.
        for i in (0..i + 1).rev() {
            println!("{}", gifts[i]);
        }

        // Line break between each verse.
        println!("");
    }
}
