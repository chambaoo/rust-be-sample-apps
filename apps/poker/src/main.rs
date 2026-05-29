#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    // Diamond,
    // Heart,
    // Spade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32,
}

fn main() {
    let card = Card {
        suit: Suit::Club,
        rank: 1,
    };
    println!("{:?}", card);
}
