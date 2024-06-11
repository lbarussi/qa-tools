use regex::{Captures, Regex};

use crate::helpers::RandomNumbers;

pub struct CNPJ {
    pub generate: String,
    pub validate: bool
}

impl CNPJ {
    pub fn generate() -> String {
        let mut numbers: Vec<u16> = RandomNumbers::generate( 12);
        let document: &mut Vec<u16> = Self::calculate_digits(&mut numbers, false);

        return Self::format_document(document.iter().copied().collect(), true);
    }

    pub fn validate(document: String) -> bool {
        return true;
    }

    fn calculate_digits(numbers: &mut Vec<u16>, calculate_next_digit: bool) -> &mut Vec<u16> {
        let mut weight: Vec<u16> = vec![5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

        if calculate_next_digit {
            weight.insert(0, 6);
        }

        let sum: u16 = numbers.iter()
            .enumerate()
            .map(|(i, number)| number * weight[i])
            .sum();

        let rest_of_division: u16 = sum % 11;

        if rest_of_division < 2 {
            numbers.push(0);
        } else {
            numbers.push(11 - rest_of_division);
        }

        if !calculate_next_digit {
            return Self::calculate_digits(numbers, true)
        }

        return numbers;
    }

    fn format_document(document: Vec<u16>, apply_mask: bool) -> String {
        let document_as_string: String = document.iter()
            .map(|number: &u16| number.to_string())
            .collect();

        if apply_mask {
            return format!(
                "{}.{}.{}/{}-{}",
                &document_as_string[0..2],
                &document_as_string[2..5],
                &document_as_string[5..8],
                &document_as_string[8..12],
                &document_as_string[12..14],
            )
        }

        return format!(
            "{}{}{}{}{}",
            &document_as_string[0..2],
            &document_as_string[2..5],
            &document_as_string[5..8],
            &document_as_string[8..12],
            &document_as_string[12..14],
        )
    }
}