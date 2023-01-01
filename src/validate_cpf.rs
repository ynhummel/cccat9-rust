pub fn validate_cpf(cpf: &str) -> bool {
    if cpf.is_empty() {
        return false;
    }
    let parsed_cpf = parse_cpf(cpf);
    if invalid_length(&parsed_cpf) || all_digits_equal(&parsed_cpf) {
        return false;
    }
    let dig_result1 = digit_calculation(&parsed_cpf, 10);
    let dig_result2 = digit_calculation(&parsed_cpf, 11);
    let actual_digits = extract_digits(&parsed_cpf);
    let calculated_digits = format!("{}{}", dig_result1, dig_result2);
    actual_digits == calculated_digits
}

fn parse_cpf(cpf: &str) -> Vec<u32> {
    cpf.chars()
        .filter_map(|char| char.to_digit(10))
        .collect::<Vec<_>>()
}

fn invalid_length(cpf: &[u32]) -> bool {
    cpf.len() != 11
}

fn all_digits_equal(cpf: &[u32]) -> bool {
    let mut iterator = cpf.iter();
    let first_item = iterator.next().unwrap();
    iterator.all(|x| x == first_item)
}

fn extract_digits(cpf: &[u32]) -> String {
    let l = cpf.len();
    cpf[l - 2..=l - 1]
        .iter()
        .map(ToString::to_string)
        .collect::<String>()
}

fn digit_calculation(cpf: &[u32], mut factor: u32) -> u32 {
    let mut total = 0;
    for digit in cpf.iter() {
        if factor > 1 {
            total += digit * factor;
            factor -= 1;
        }
    }
    let rest = total % 11;
    if rest < 2 {
        0
    } else {
        11 - rest
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn validate_false() {
        let invalid_cpfs = [
            "111.111.111-11",
            "222.222.222-22",
            "333.333.333-33",
            "123",
            "",
            "987.654.321-10",
            "714.602.380-11",
            "313.030.210-52",
            "144.796.170-50",
        ];

        invalid_cpfs.iter().for_each(|cpf| {
            assert!(!super::validate_cpf(cpf));
        });
    }

    #[test]
    fn validate_true() {
        let valid_cpfs = [
            "987.654.321-00",
            "714.602.380-01",
            "313.030.210-72",
            "144.796.170-60",
        ];

        valid_cpfs.iter().for_each(|cpf| {
            assert!(super::validate_cpf(cpf));
        });
    }
}
