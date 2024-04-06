fn main() {
    let s = String::from("hello");

    takes_ownership(s); // S is no longer valid from here on

    let s2 = String::from("Testing giving.");

    let s2 = takes_and_gives(s2);

    println!("{}", s2);

    let s3 = String::from("Testing references");

    let len = calc_len(&s3);

    println!("{} length: {}", s3, len); //S3 string was use in calc_len function, but is still valid, unlike S used above.

    let mut s4 = String::from("Testing mutable reference."); //Because s4 is mutable, only one reference to it can exist at any given time.

    println!("{}", s4);
    change(&mut s4);
    println!("{}", s4);
}

fn takes_ownership(some: String) {
    println!("{}", some);
}

fn takes_and_gives(some: String) -> String {
    println!("{}", some);
    String::from("Testing return.")
}

fn calc_len(some: &String) -> usize {
    some.len() // Attempting to modify "Some" will cause an error because the reference is immutable;
}

fn change(some: &mut String) {
    some.push_str(" ADDED TEXT"); //Reference was passed as immutable, no error will occur here.
}
