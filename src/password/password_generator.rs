use rand::prelude::*;

pub fn generate_password(
    length: u8,
    include_numbers: bool,
    include_special_characters: bool,
    include_spaces: bool,
) -> String {
    let letters = "abcdefghijklmnoprstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "1234567890";
    let special_characters = "!@#$%^&*()-_[]{}.";

    let mut password_seed: String = letters.to_string();
    if include_numbers {
        password_seed.push_str(numbers);
    }
    if include_special_characters {
        password_seed.push_str(special_characters);
    }
    if include_spaces {
        password_seed.push_str(" ");
    }
    let password_seed: Vec<char> = password_seed.chars().collect::<Vec<char>>();

    let mut rng = rand::thread_rng();
    // password_seed.shuffle(&mut rng);

    (0..length)
        .map(|_| password_seed.choose(&mut rng).unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::generate_password;
    // use rayon::prelude::*;
    use std::collections::HashSet;

    #[test]
    fn random_confirmation_epoch() {
        let mut visited = HashSet::<String>::new();
        // (0..100000).into_par_iter().for_each(|_| {
        //     assert!(visited.insert(generate_password(16, true, true, true)));
        // });

        for _ in 0..1000000000 {
            assert!(visited.insert(generate_password(16, true, true, true)));
        }
    }
}
