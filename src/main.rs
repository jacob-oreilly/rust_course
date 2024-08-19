#[derive(Debug)]
struct Deck {
    cards: Vec<String>,   
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
        let values = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];
    
        let mut deck = Deck {cards: vec![]};
    
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                deck.cards.push(card);
            }
        }
        deck
    }
}

fn main() {
    let deck = Deck::new();
    
    println!("Here's your deck: {:#?}", deck);
}
