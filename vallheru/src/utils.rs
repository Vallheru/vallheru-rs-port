pub fn to_ordinal(num: u32) -> String {
    let num_mod_100 = num % 100;
    let num_mod_10 = num % 10;

    if num_mod_100 > 10 && num_mod_100 < 14 {
        num.to_string() + "th"
    } else if num_mod_10 == 1 {
        num.to_string() + "st"
    } else if num_mod_10 == 2 {
        num.to_string() + "nd"
    } else if num_mod_10 == 3 {
        num.to_string() + "rd"
    } else {
        num.to_string() + "th"
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_to_ordinal() {
        assert_eq!(String::from("1st"), super::to_ordinal(1));
        assert_eq!(String::from("2nd"), super::to_ordinal(2));
        assert_eq!(String::from("3rd"), super::to_ordinal(3));
        assert_eq!(String::from("4th"), super::to_ordinal(4));
        assert_eq!(String::from("5th"), super::to_ordinal(5));
        assert_eq!(String::from("6th"), super::to_ordinal(6));
        assert_eq!(String::from("7th"), super::to_ordinal(7));
        assert_eq!(String::from("8th"), super::to_ordinal(8));
        assert_eq!(String::from("9th"), super::to_ordinal(9));
        assert_eq!(String::from("10th"), super::to_ordinal(10));
        assert_eq!(String::from("11th"), super::to_ordinal(11));
        assert_eq!(String::from("12th"), super::to_ordinal(12));
        assert_eq!(String::from("13th"), super::to_ordinal(13));

        assert_eq!(String::from("21st"), super::to_ordinal(21));
        assert_eq!(String::from("32nd"), super::to_ordinal(32));
        assert_eq!(String::from("43rd"), super::to_ordinal(43));

        assert_eq!(String::from("99th"), super::to_ordinal(99));
        assert_eq!(String::from("100th"), super::to_ordinal(100));
        assert_eq!(String::from("101st"), super::to_ordinal(101));
        assert_eq!(String::from("102nd"), super::to_ordinal(102));
        assert_eq!(String::from("103rd"), super::to_ordinal(103));

        assert_eq!(String::from("101202nd"), super::to_ordinal(101202));
        assert_eq!(String::from("1000009th"), super::to_ordinal(1000009));
    }
}
