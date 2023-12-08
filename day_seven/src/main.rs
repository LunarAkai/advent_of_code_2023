mod test;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::ops::Deref;
use std::path::Path;
use crate::Card::{Card2, Card3, Card4, Card5, Card6, Card7, Card8, Card9, CardA, CardJ, CardK, CardQ, CardT};

#[derive(Eq, Debug)]
struct HandStruct {
    hand: String,
    bid: i32,
    hand_type: HandType
}

impl HandStruct {
    fn new(hand: String, bid: i32, hand_type: HandType) -> HandStruct {
        Self { hand: hand.to_string(), bid, hand_type}
    }

    fn get_hand(&self) -> &str {
        &self.hand
    }

    fn get_bid(&self) -> &i32 {
        &self.bid
    }

    fn get_hand_type(&self) -> &HandType {
        &self.hand_type
    }
}

impl PartialEq for HandStruct {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand &&
            self.hand_type == other.hand_type
    }
    fn ne(&self, other: &Self) -> bool {
        get_card_value(self.hand.chars().into_iter().next().unwrap()) ==
            get_card_value(other.hand.chars().into_iter().next().unwrap())
    }
}


impl PartialOrd for HandStruct {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hand_type.partial_cmp(&other.hand_type)
    }
}


impl Ord for HandStruct {
    fn cmp(&self, other: &Self) -> Ordering {

        let mut s_chars = &mut self.hand.chars();
        let mut o_chars = &mut other.hand.chars();



        get_card_value(self.hand.chars().next().unwrap()).cmp(&get_card_value(other.hand.chars().next().unwrap()))
    }
}



#[derive(Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
    Invalid,
}

#[derive(Debug, PartialEq, Hash, Eq)]
enum Card {
    CardA,
    CardK,
    CardQ,
    CardJ,
    CardT,
    Card9,
    Card8,
    Card7,
    Card6,
    Card5,
    Card4,
    Card3,
    Card2
}

fn main() {
    let mut vec_hand_bid = Vec::new();

    let mut hand_number: i32 = 0;

    if let Ok(lines) = get_lines() {
        for line in lines {
            let new_line: String = line.unwrap();
            let hand = &*split_hand(new_line.clone());
            let bid = split_bid(new_line.to_owned());
            hand_number = hand_number + 1;

            let v_vec = get_value_vec(hand);

            let hand_type = get_type(&v_vec);

            vec_hand_bid.push(HandStruct::new(hand.to_string(), bid, hand_type))

        }
    }

    vec_hand_bid.sort();

    println!("{:?}", vec_hand_bid);

    let mut total = 0;

    for i in 0..hand_number {
        total += vec_hand_bid.iter().next().unwrap().bid * (hand_number-i);
    }

    println!("{:?}", total);



}

fn get_lines() -> io::Result<Lines<BufReader<File>>> {
    read_lines("src/input.txt")
}

fn split_hand (line_string: String) -> String {
    let return_hand = line_string.split_at(5).0;

    return return_hand.to_string()
}

fn split_bid(line_string: String) -> i32{
    return line_string.split_at(5).1.trim().parse().unwrap();
}

fn get_type(cards: &Vec<Card>) -> HandType {

    match true {
        _ if is_five_of_a_kind(cards) => HandType::FiveOfAKind,
        _ if is_four_of_a_kind(cards) => HandType::FourOfAKind,
        _ if is_full_house(cards) => HandType::FullHouse,
        _ if is_three_of_a_kind(cards) => HandType::ThreeOfAKind,
        _ if is_two_pair(cards) => HandType::TwoPair,
        _ if is_one_pair(cards) => HandType::OnePair,
        _ if is_high_card(cards) => HandType::HighCard,
        _ => HandType::Invalid
    }
}

fn get_value_vec(hand_string: &str) -> Vec<Card> {
    let mut v = Vec::new();

    for c in hand_string.chars() {
        match c {
            '2' => v.push(Card2),
            '3' => v.push(Card3),
            '4' => v.push(Card4),
            '5' => v.push(Card5),
            '6' => v.push(Card6),
            '7' => v.push(Card7),
            '8' => v.push(Card8),
            '9' => v.push(Card9),
            'T' => v.push(CardT),
            'J' => v.push(CardJ),
            'Q' => v.push(CardQ),
            'K' => v.push(CardK),
            'A' => v.push(CardA),
            _ => {}
        }
    }
    return v;
}


fn is_five_of_a_kind(cards: &Vec<Card>) -> bool {
    let card_slices = cards.as_slice();

    return card_slices[0].eq(&card_slices[1]) && card_slices[0].eq(&card_slices[2]) && card_slices[0].eq(&card_slices[3]) && card_slices[0].eq(&card_slices[4])
}

fn is_four_of_a_kind(cards: &Vec<Card>) -> bool {
    let res = count_matching_card(&cards);
    let v: Vec<_> = res.values().collect();

    if v.contains(&&4) {
        return true;
    }
    return false;
}

fn is_full_house(cards: &Vec<Card>) -> bool {
    let res = count_matching_card(&cards);
    let v: Vec<_> = res.values().collect();

    if v.contains(&&3) && v.contains(&&2) {
        return true;
    }
    return false;
}

fn is_three_of_a_kind(cards: &Vec<Card>) -> bool {
    let res = count_matching_card(&cards);
    let v: Vec<_> = res.values().collect();

    if v.contains(&&3) && !v.contains(&&2) {
        return true;
    }
    return false;
}

fn is_two_pair(cards: &Vec<Card>) -> bool {
    let res = count_matching_card(&cards);
    let v: Vec<_> = res.values().collect();

    if v.len() == 3 && !v.contains(&&3) {
        return true;
    }

    return false;
}

fn is_one_pair(cards: &Vec<Card>) -> bool {
    let res = count_matching_card(&cards);
    let v: Vec<_> = res.values().collect();

    if v.len() == 4 && !v.contains(&&3) {
        return true;
    }

    return false;
}

fn is_high_card(cards: &Vec<Card>) -> bool{
    let card_slices = cards.as_slice();
    return
        card_slices[0].ne(&card_slices[1]) &&
        card_slices[0].ne(&card_slices[2]) &&
        card_slices[0].ne(&card_slices[3]) &&
        card_slices[0].ne(&card_slices[4])
}

fn count_matching_card(cards: &Vec<Card>) -> HashMap<&Card, i32> {
    let mut card_count = HashMap::new();

    for card in cards {
        *card_count.entry(card).or_insert(0) += 1;
    }

    card_count
}

fn get_card_value(c: char) -> i32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0
    }
}


// from: rust-by-example
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}