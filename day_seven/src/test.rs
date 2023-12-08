#[cfg(test)]
mod tests {
    use crate::{split_hand, split_bid, get_type, HandType, get_value_vec, is_five_of_a_kind, is_high_card, is_one_pair, is_four_of_a_kind, is_three_of_a_kind, is_two_pair, get_card_value};
    use crate::Card::{Card2, Card3, Card5, Card6, Card7, CardA, CardJ, CardK, CardQ, CardT};

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
        assert_eq!(get_card_value('2'), 2);
        assert_eq!(get_card_value('3'), 3);
        assert_eq!(get_card_value('4'), 4);
        assert_eq!(get_card_value('5'), 5);
        assert_eq!(get_card_value('6'), 6);
        assert_eq!(get_card_value('7'), 7);
        assert_eq!(get_card_value('8'), 8);
        assert_eq!(get_card_value('9'), 9);
        assert_eq!(get_card_value('T'), 10);
        assert_eq!(get_card_value('J'), 11);
        assert_eq!(get_card_value('Q'), 12);
        assert_eq!(get_card_value('K'), 13);
        assert_eq!(get_card_value('A'), 14);
    }

    // #[test]
    // fn test_is_five_of_a_kind() {
    //     assert_eq!(is_five_of_a_kind(vec![CardQ, CardQ, CardQ, CardQ, CardQ]), true);
    //     assert_eq!(is_five_of_a_kind(vec![CardQ, CardQ, CardQ, CardJ, CardQ]), false);
    //     assert_eq!(is_five_of_a_kind(vec![CardQ, CardQ, CardQ, CardJ, Card6]), false);
    //     assert_eq!(is_five_of_a_kind(vec![CardQ, Card2, Card3, CardJ, Card6]), false);
    //     assert_eq!(is_five_of_a_kind(vec![CardT, Card2, Card3, CardJ, Card6]), false);
    // }
    //
    // #[test]
    // fn test_is_four_of_a_kind() {
    //     assert_eq!(is_four_of_a_kind(vec![CardQ, CardQ, CardQ, CardQ, CardQ]), false);
    //     assert_eq!(is_four_of_a_kind(vec![CardQ, CardQ, CardQ, CardJ, CardQ]), true);
    //     assert_eq!(is_four_of_a_kind(vec![CardQ, CardQ, CardQ, CardJ, Card6]), false);
    //     assert_eq!(is_four_of_a_kind(vec![CardQ, CardQ, Card3, CardJ, Card6]), false);
    //     assert_eq!(is_four_of_a_kind(vec![CardT, Card2, Card3, CardJ, Card6]), false);
    // }
    //
    // #[test]
    // fn test_is_three_of_a_kind() {
    //     assert_eq!(is_three_of_a_kind(vec![Card3, Card2, CardT, Card3, CardK]), false);
    //     assert_eq!(is_three_of_a_kind(vec![CardT, Card5, Card5, CardJ, Card5]), true);
    //     assert_eq!(is_three_of_a_kind(vec![CardK, CardK, Card6, Card7, Card7]), false);
    //     assert_eq!(is_three_of_a_kind(vec![CardK, CardT, CardJ, CardJ, CardT]), false);
    //     assert_eq!(is_three_of_a_kind(vec![CardQ, CardQ, CardQ, CardJ, CardA]), true);
    //
    //     //Full House
    //     assert_eq!(is_three_of_a_kind(vec![CardJ, Card5, Card5, CardJ, Card5]), false);
    // }
    //
    // #[test]
    // fn test_is_two_pair() {
    //     assert_eq!(is_two_pair(vec![Card3, Card2, CardT, Card3, CardK]), false);
    //     assert_eq!(is_two_pair(vec![CardT, Card5, Card5, CardJ, Card5]), false);
    //     assert_eq!(is_two_pair(vec![CardK, CardK, Card6, Card7, Card7]), true);
    //     assert_eq!(is_two_pair(vec![CardK, CardT, CardJ, CardJ, CardT]), true);
    //     assert_eq!(is_two_pair(vec![CardQ, CardQ, CardQ, CardJ, CardA]), false);
    //
    //     //Full House
    //     assert_eq!(is_three_of_a_kind(vec![CardJ, Card5, Card5, CardJ, Card5]), false);
    // }
    //
    // #[test]
    // fn test_is_one_pair() {
    //     assert_eq!(is_one_pair(vec![Card3, Card2, CardT, Card3, CardK]), true);
    //     assert_eq!(is_one_pair(vec![CardT, Card5, Card5, CardJ, Card5]), false);
    //     assert_eq!(is_one_pair(vec![CardK, CardK, Card6, Card7, Card7]), false);
    //     assert_eq!(is_one_pair(vec![CardK, CardT, CardJ, CardJ, CardT]), false);
    //     assert_eq!(is_one_pair(vec![CardQ, CardQ, CardQ, CardJ, CardA]), false);
    // }
    //
    // #[test]
    // fn test_is_high_card() {
    //     assert_eq!(is_high_card(vec![CardQ, CardQ, CardQ, CardQ, CardQ]), false);
    //     assert_eq!(is_high_card(vec![CardQ, CardQ, CardQ, CardJ, CardQ]), false);
    //     assert_eq!(is_high_card(vec![CardQ, CardQ, CardQ, CardJ, Card6]), false);
    //     assert_eq!(is_high_card(vec![CardQ, Card2, Card3, CardJ, Card6]), true);
    //     assert_eq!(is_high_card(vec![CardT, Card2, Card3, CardJ, Card6]), true);
    // }
}