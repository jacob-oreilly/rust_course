use rand::{thread_rng, seq::SliceRandom};

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

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let cards = deck.deal(3);
    println!("Here's your hand: {:#?}", cards);
    println!("Here's what's left in the deck: {:#?}", deck);
}
