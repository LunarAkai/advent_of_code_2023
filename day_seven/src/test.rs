#[cfg(test)]
mod tests {
    use crate::{split_hand, split_bid, get_type, HandType, get_value_vec, is_five_of_a_kind, is_high_card, is_one_pair, is_four_of_a_kind, is_three_of_a_kind, is_two_pair, get_card_value, day_seven_part_one, day_seven_part_two};
    use crate::Card::{Card2, Card3, Card5, Card6, Card7, CardA, CardJ, CardK, CardQ, CardT};

    #[test]
    fn test_test_input_part_one() {
        assert_eq!(day_seven_part_one("src/test_input.txt"), 6592);
    }

    #[test]
    fn test_test_input_part_two() {
        assert_eq!(day_seven_part_two("src/test_input.txt"), 6839);
    }

    #[test]
    fn test_get_hand() {
        assert_eq!(split_hand("32T3K 765".to_string()), "32T3K");
        assert_eq!(split_hand("T55J5 684".to_string()), "T55J5");
        assert_eq!(split_hand("KK677 28".to_string()), "KK677");
        assert_eq!(split_hand("KTJJT 220".to_string()), "KTJJT");
        assert_eq!(split_hand("QQQJA 483".to_string()), "QQQJA");
    }

    #[test]
    fn test_get_bid() {
        assert_eq!(split_bid("32T3K 765".to_string()), 765);
        assert_eq!(split_bid("T55J5 684".to_string()), 684);
        assert_eq!(split_bid("KK677 28".to_string()), 28);
        assert_eq!(split_bid("KTJJT 220".to_string()), 220);
        assert_eq!(split_bid("QQQJA 483".to_string()), 483);
    }

    #[test]
    fn test_get_type() {
        // Provided
        assert_eq!(get_type(&vec![Card3, Card2, CardT, Card3, CardK]), HandType::OnePair);
        assert_eq!(get_type(&vec![CardT, Card5, Card5, CardJ, Card5]), HandType::ThreeOfAKind);
        assert_eq!(get_type(&vec![CardK, CardK, Card6, Card7, Card7]), HandType::TwoPair);
        assert_eq!(get_type(&vec![CardK, CardT, CardJ, CardJ, CardT]), HandType::TwoPair);
        assert_eq!(get_type(&vec![CardQ, CardQ, CardQ, CardJ, CardA]), HandType::ThreeOfAKind);

        // Extra
        assert_eq!(get_type(&vec![Card6, Card2, Card3, CardJ, CardT]), HandType::HighCard);
        assert_eq!(get_type(&vec![CardT, CardT, CardJ, CardJ, CardT]), HandType::FullHouse);
        assert_eq!(get_type(&vec![CardT, CardT, CardT, CardJ, CardT]), HandType::FourOfAKind);
        assert_eq!(get_type(&vec![CardT, CardT, CardT, CardT, CardT]), HandType::FiveOfAKind);

    }

    #[test]
    fn test_parse_hand() {
        assert_eq!(get_value_vec("32T3K"), vec![Card3, Card2, CardT, Card3, CardK]);
        assert_eq!(get_value_vec("T55J5"), vec![CardT, Card5, Card5, CardJ, Card5]);
        assert_eq!(get_value_vec("KK677"), vec![CardK, CardK, Card6, Card7, Card7]);
        assert_eq!(get_value_vec("KTJJT"), vec![CardK, CardT, CardJ, CardJ, CardT]);
        assert_eq!(get_value_vec("QQQJA"), vec![CardQ, CardQ, CardQ, CardJ, CardA]);
    }

    #[test]
    fn test_get_card_value() {
        assert_eq!(get_card_value('2'),  Some(2));
        assert_eq!(get_card_value('3'), Some(3));
        assert_eq!(get_card_value('4'), Some(4));
        assert_eq!(get_card_value('5'), Some(5));
        assert_eq!(get_card_value('6'), Some(6));
        assert_eq!(get_card_value('7'), Some(7));
        assert_eq!(get_card_value('8'), Some(8));
        assert_eq!(get_card_value('9'), Some(9));
        assert_eq!(get_card_value('T'), Some(10));
        assert_eq!(get_card_value('J'), Some(11));
        assert_eq!(get_card_value('Q'), Some(12));
        assert_eq!(get_card_value('K'), Some(13));
        assert_eq!(get_card_value('A'), Some(14));
    }
}