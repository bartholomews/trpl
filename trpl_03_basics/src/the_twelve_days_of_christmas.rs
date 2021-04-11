pub fn print_lyrics() {
    let mut gifts: [&str; 12] = [
        "A Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    print_stanza(1, gifts);
    gifts[0] = "and a Partridge in a Pear Tree";
    for day in 2..12 {
        print_stanza(day, gifts);
        println!();
    }
}

fn print_stanza(day: u8, gifts: [&str; 12]) {
    println!("On the {} day of Christmas", day);
    println!("my true love sent to me:");
    for n in (0..day).rev() {
        unsafe { println!("{}", gifts.get_unchecked(n as usize)); }
    }
}