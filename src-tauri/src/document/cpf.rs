use rand::Rng;
use rand::rngs::ThreadRng;

pub struct CPF {
    pub generate_document: String,
    pub validate_document: bool
}

impl CPF {
    pub fn generate_document() -> String {
        let mut numbers: Vec<u32> = Self::generate_numbers();
        let document = Self::calculate_digits(&mut numbers, false);

        return Self::format_document(document.iter().copied().collect());
    }

    pub fn validate_document(document: String) -> bool {
        let document_divided = document.split("");

        return true;
    }

    fn generate_numbers() -> Vec<u32> {
        let mut rng: ThreadRng = rand::thread_rng();
        let mut numbers: Vec<u32> = vec![];

        for _ in 0..9 {
            let random_number: u32 = rng.gen_range(0..9);
            numbers.push(random_number)
        }

        return numbers;
    }

    fn calculate_digits(numbers: &mut Vec<u32>, next: bool) -> &mut Vec<u32> {
        let mut base_numbers: Vec<u32> = vec![];
        let mut base_number_calculator: u32 = 11;

        if next {
            base_number_calculator = 12;
        }

        for (i, &number) in numbers.iter().enumerate() {
            let index: u32 = i as u32;
            base_numbers.push(number * (base_number_calculator - (index + 1)));
        }

        let sum_of_first_digit = base_numbers.iter().copied().reduce(|a, b| a + b);

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

    fn format_document(document: Vec<u32>) -> String {
        let document_as_string: String = document.iter().map(|number: &u32| {
            return number.to_string()
        }).collect();

        return format!(
            "{}.{}.{}-{}",
            &document_as_string[0..3],
            &document_as_string[3..6],
            &document_as_string[6..9],
            &document_as_string[9..11],
        )
    }
}