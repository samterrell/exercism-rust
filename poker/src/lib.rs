use std::cmp::Ordering;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

#[derive(Debug, PartialEq)]
enum Suit {
    Heart,
    Club,
    Diamond,
    Spade,
}

type Rank = u8;

#[derive(Debug)]
struct Card {
    suit: Suit,
    rank: Rank,
}

#[derive(Debug, PartialEq)]
enum Score {
    RoyalFlush,
    StraightFlush(Rank),
    FourOfAKind(Rank, Rank),
    FullHouse(Rank, Rank),
    Flush(Rank, Rank, Rank, Rank, Rank),
    Straight(Rank),
    ThreeOfAKind(Rank, Rank, Rank),
    TwoPairs(Rank, Rank, Rank),
    Pair(Rank, Rank, Rank, Rank),
    High(Rank, Rank, Rank, Rank, Rank),
}

impl Into<u64> for &Score {
    fn into(self) -> u64 {
        match self {
            Score::RoyalFlush => 90000000000,
            Score::StraightFlush(a) => 80000000000 + (*a as u64),
            Score::FourOfAKind(a, b) => {
                70000000000 + (*a as u64) * 100000000 + (*b as u64) * 1000000
            }
            Score::FullHouse(a, b) => 60000000000 + (*a as u64) * 100000000 + (*b as u64) * 1000000,
            Score::Flush(a, b, c, d, e) => {
                50000000000
                    + (*a as u64) * 100000000
                    + (*b as u64) * 1000000
                    + (*c as u64) * 10000
                    + (*d as u64) * 100
                    + (*e as u64)
            }
            Score::Straight(a) => 40000000000 + (*a as u64),
            Score::ThreeOfAKind(a, b, c) => {
                30000000000 + (*a as u64) * 100000000 + (*b as u64) * 1000000 + (*c as u64) * 10000
            }
            Score::TwoPairs(a, b, c) => {
                20000000000 + (*a as u64) * 100000000 + (*b as u64) * 1000000 + (*c as u64) * 10000
            }
            Score::Pair(a, b, c, d) => {
                10000000000
                    + (*a as u64) * 100000000
                    + (*b as u64) * 1000000
                    + (*c as u64) * 10000
                    + (*d as u64) * 100
            }
            Score::High(a, b, c, d, e) => {
                (*a as u64) * 100000000
                    + (*b as u64) * 1000000
                    + (*c as u64) * 10000
                    + (*d as u64) * 100
                    + (*e as u64)
            }
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, rhs: &Score) -> Option<Ordering> {
        let lhsi: u64 = self.into();
        let rhsi: u64 = rhs.into();
        if lhsi == rhsi {
            Some(Ordering::Equal)
        } else if lhsi < rhsi {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

struct Hand {
    score: Score,
}

fn parse_suit(c: &str) -> Result<Suit, String> {
    match c {
        "H" => Ok(Suit::Heart),
        "h" => Ok(Suit::Heart),
        "C" => Ok(Suit::Club),
        "c" => Ok(Suit::Club),
        "S" => Ok(Suit::Spade),
        "s" => Ok(Suit::Spade),
        "D" => Ok(Suit::Diamond),
        "d" => Ok(Suit::Diamond),
        c => Err(format!("Invalid suit '{}'", c)),
    }
}

fn parse_rank(s: &str) -> Result<Rank, String> {
    match s {
        "2" => Ok(2),
        "3" => Ok(3),
        "4" => Ok(4),
        "5" => Ok(5),
        "6" => Ok(6),
        "7" => Ok(7),
        "8" => Ok(8),
        "9" => Ok(9),
        "10" => Ok(10),
        "j" => Ok(11),
        "J" => Ok(11),
        "q" => Ok(12),
        "Q" => Ok(12),
        "k" => Ok(13),
        "K" => Ok(13),
        "a" => Ok(14),
        "A" => Ok(14),
        _ => Err("Invalid suit".to_string()),
    }
}

fn parse_card(card: &str) -> Result<Card, String> {
    match card.len() {
        2 => Ok(Card {
            suit: parse_suit(&card[1..2])?,
            rank: parse_rank(&card[0..1])?,
        }),
        3 => Ok(Card {
            suit: parse_suit(&card[2..3])?,
            rank: parse_rank(&card[0..2])?,
        }),
        _ => Err("Invalid Card".to_string()),
    }
}

fn is_flush(cards: &[Card; 5]) -> bool {
    for i in 1..5 {
        if cards[0].suit != cards[i].suit {
            return false;
        }
    }
    true
}

fn is_royal(cards: &[Card; 5]) -> bool {
    cards[0].rank == 14
        && cards[1].rank == 13
        && cards[2].rank == 12
        && cards[3].rank == 11
        && cards[4].rank == 10
}

fn is_straight(cards: &[Card; 5]) -> bool {
    if cards[0].rank != 14
        || cards[4].rank != 2
        || cards[3].rank != 3 && cards[2].rank != 4 && cards[1].rank != 5
    {
        for i in 1..5 {
            if cards[i].rank != cards[i - 1].rank - 1 {
                return false;
            }
        }
    }
    true
}

fn get_rank_groups(cards: &[Card; 5]) -> Vec<(u8, u8)> {
    let mut result = vec![];
    let mut current = cards[0].rank;
    let mut count = 1;
    for i in 1..5 {
        if current == cards[i].rank {
            count += 1;
        } else {
            result.push((current, count));
            current = cards[i].rank;
            count = 1
        }
    }
    result.push((current, count));
    result
}

fn score(cards: &mut [Card; 5]) -> Score {
    cards.sort_by(|l, r| r.rank.partial_cmp(&l.rank).unwrap());
    if is_flush(cards) {
        if is_royal(cards) {
            Score::RoyalFlush
        } else if is_straight(cards) {
            Score::StraightFlush(cards[1].rank)
        } else {
            Score::Flush(
                cards[0].rank,
                cards[1].rank,
                cards[2].rank,
                cards[3].rank,
                cards[4].rank,
            )
        }
    } else if is_straight(cards) {
        if cards[0].rank == 14 && cards[1].rank == 5 {
            Score::Straight(5)
        } else {
            Score::Straight(cards[0].rank)
        }
    } else {
        let mut rank_groups = get_rank_groups(cards);
        if rank_groups.len() == 2 {
            let (first, first_count) = rank_groups.pop().unwrap();
            let (second, second_count) = rank_groups.pop().unwrap();
            if first_count == 4 {
                Score::FourOfAKind(first, second)
            } else if second_count == 4 {
                Score::FourOfAKind(second, first)
            } else if first_count == 3 && second_count == 2 {
                Score::FullHouse(first, second)
            } else {
                debug_assert!(second_count == 2 && first_count == 3);
                Score::FullHouse(second, first)
            }
        } else if rank_groups.len() == 3 {
            let (third, third_count) = rank_groups.pop().unwrap();
            let (second, second_count) = rank_groups.pop().unwrap();
            let (first, first_count) = rank_groups.pop().unwrap();
            if first_count == 3 {
                Score::ThreeOfAKind(first, second, third)
            } else if second_count == 3 {
                Score::ThreeOfAKind(second, first, third)
            } else if third_count == 3 {
                Score::ThreeOfAKind(third, first, second)
            } else if first_count == 2 {
                if second_count == 2 {
                    Score::TwoPairs(first, second, third)
                } else {
                    debug_assert_eq!(third_count, 2);
                    Score::TwoPairs(first, third, second)
                }
            } else {
                debug_assert_eq!(second_count, 2);
                debug_assert_eq!(third_count, 2);
                Score::TwoPairs(second, third, first)
            }
        } else if rank_groups.len() == 4 {
            let (first, first_count) = rank_groups.pop().unwrap();
            let (second, second_count) = rank_groups.pop().unwrap();
            let (third, third_count) = rank_groups.pop().unwrap();
            let (fourth, fourth_count) = rank_groups.pop().unwrap();
            if first_count == 2 {
                Score::Pair(first, second, third, fourth)
            } else if second_count == 2 {
                Score::Pair(second, first, third, fourth)
            } else if third_count == 2 {
                Score::Pair(third, first, second, fourth)
            } else {
                debug_assert_eq!(fourth_count, 2);
                Score::Pair(fourth, first, second, third)
            }
        } else {
            Score::High(
                cards[0].rank,
                cards[1].rank,
                cards[2].rank,
                cards[3].rank,
                cards[4].rank,
            )
        }
    }
}

fn parse_hand(hand: &str) -> Result<Hand, String> {
    // let cards = hand.split(" ");
    let mut cards = vec![];
    for s in hand.split(" ") {
        cards.push(parse_card(s)?)
    }
    if cards.len() != 5 {
        return Err("Wrong hand size".to_string());
    }
    let mut cards = [
        cards.pop().ok_or("Missing Card")?,
        cards.pop().ok_or("Missing Card")?,
        cards.pop().ok_or("Missing Card")?,
        cards.pop().ok_or("Missing Card")?,
        cards.pop().ok_or("Missing Card")?,
    ];
    let score = score(&mut cards);
    Ok(Hand { score: score })
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut winning: Vec<&'a str> = vec![];
    let mut best_score = Score::High(0, 0, 0, 0, 0);
    for hstr in hands {
        let hand = parse_hand(hstr).unwrap();
        if hand.score == best_score {
            winning.push(hstr);
        } else if hand.score > best_score {
            best_score = hand.score;
            winning.clear();
            winning.push(hstr);
        }
    }
    winning
}
