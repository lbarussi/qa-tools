use regex::{Captures, Regex};

use crate::helpers::RandomNumbers;

pub struct CPF {
    pub generate: String,
    pub validate: bool
}

impl CPF {
    pub fn generate() -> String {
        let mut numbers: Vec<u16> = RandomNumbers::generate( 9);
        let document: &mut Vec<u16> = Self::calculate_digits(&mut numbers, false);

        return Self::format_document(document.iter().copied().collect(), true);
    }

    pub fn validate(document: String) -> bool {
        if document.len() != 11 {
            return false;
        }

        let regex: Regex = Regex::new(r"\D").unwrap();
        let has_invalid_characters: Option<Captures> = regex.captures(&document);

        if has_invalid_characters.iter().len() > 0 {
            return false;
        }

        let mut numbers: Vec<u16> = vec![];

        for document in document.split("") {
            if document.is_empty() {
                continue;
            }

            let number_as_integer: u16 = document.parse::<u16>().unwrap();

            if numbers.iter().len() < 9 {
                numbers.push(number_as_integer);
                continue;
            }

            break;
        }

        let full_document: String =
                Self::format_document(
                    Self::calculate_digits(&mut numbers, false).iter().copied().collect(),
                    false
                );

        return full_document == document;
    }

    fn calculate_digits(numbers: &mut Vec<u16>, next: bool) -> &mut Vec<u16> {
        let mut base_numbers: Vec<u16> = vec![];
        let mut base_number_calculator: u16 = 11;

        if next {
            base_number_calculator = 12;
        }

        for (i, &number) in numbers.iter().enumerate() {
            let index: u16 = i as u16;
            base_numbers.push(number * (base_number_calculator - (index + 1)));
        }

        let sum_of_first_digit = base_numbers.iter().copied().reduce(|a: u16, b: u16| a + b);

        let rest_of_division = sum_of_first_digit.unwrap() % 11;

        if rest_of_division < 2 {
            numbers.push(0);
        } else {
            numbers.push(11 - rest_of_division);
        }

        if !next {
            return Self::calculate_digits(numbers, true)
        }

        return numbers;
    }

    fn format_document(document: Vec<u16>, apply_mask: bool) -> String {
        let document_as_string: String = document.iter().map(|number: &u16| {
            return number.to_string()
        }).collect();

        if apply_mask {
            return format!(
                "{}.{}.{}-{}",
                &document_as_string[0..3],
                &document_as_string[3..6],
                &document_as_string[6..9],
                &document_as_string[9..11],
            )
        }

        return format!(
            "{}{}{}{}",
            &document_as_string[0..3],
            &document_as_string[3..6],
            &document_as_string[6..9],
            &document_as_string[9..11],
        )
    }
}