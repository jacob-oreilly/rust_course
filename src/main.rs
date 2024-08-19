#[derive(Debug)]
struct Deck {
    cards: Vec<String>,   
}

fn main() {
    let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
    let values = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

    let mut deck = Deck {cards: vec![]};

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            deck.cards.push(card);
        }
    }
    
    println!("Here's your deck: {:#?}", deck);
}
