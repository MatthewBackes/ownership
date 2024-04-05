fn main() {
    let s = String::from("hello");

    takes_ownership(s); // S is no longer valid from here on

    let s2 = String::from("Testing giving.");

    let s2 = takes_and_gives(s2);

    println!("{}", s2);
}

fn takes_ownership(some: String) {
    println!("{}", some);
}

fn takes_and_gives(some: String) -> String {
    println!("{}", some);
    String::from("Testing return.")
}
