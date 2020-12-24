use std::slice::Iter;

fn main() {
    let song: [(&str, &str); 12] = [
        ("first", "A partridge in a pear tree"), 
        ("second", "Two turtle doves and "),
        ("third", "Three French hens, "),
        ("fourth", "Four calling birds, "),
        ("fifth", "Five gold rings, "),
        ("sixth", "Six geese a laying, "),
        ("seventh", "Seven swans a swimming, "),
        ("eighth", "Eight maids a milking, "),
        ("ninth", "Nine ladies dancing, "),
        ("tenth", "Ten lords a leaping, "),
        ("eleventh", "Eleven pipers piping, "),
        ("twelfth", "Twelve drummers drumming, ")
    ];

    let mut full_song = song.iter();
    let first_couplet: (&str, &str) = *full_song.next().unwrap();

    create_song(full_song, first_couplet);
}

pub fn create_song(mut remaining_song: Iter<(&str, &str)>, current_couplet: (&str, &str)) {
    println!("On the {} day of Christmas my true love gave to me", current_couplet.0);
    println!("{}", current_couplet.1);
    println!("");

    let is_remaining = remaining_song.next();

    if is_remaining == None {
        return
    }

    let next_couplet = is_remaining.unwrap();
    let new_verse = next_couplet.1.to_owned() + "\n" + current_couplet.1;
    let new: &str = &new_verse;

    let new_couplet = (next_couplet.0, new);

    create_song(remaining_song, new_couplet)
}