use rand::Rng;
const CHARS: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

pub fn cipher(password: &str, decode: bool, buff: &mut String) {
    let mut seed = 0u64;
    for c in password.chars() {
        seed += c as u64;
    }

    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);

    if decode {
        *buff = buff
            .chars()
            .map(|c| match CHARS.iter().position(|&v| v == c) {
                Some(mut i) => {
                    i += CHARS.len();
                    i -= rng.gen::<usize>() % CHARS.len();
                    if i >= CHARS.len() {
                        i -= CHARS.len()
                    };
                    CHARS[i]
                }
                None => c,
            })
            .collect();
    } else {
        *buff = buff
            .chars()
            .map(|c| match CHARS.iter().position(|&v| v == c) {
                Some(mut i) => {
                    i += rng.gen::<usize>() % CHARS.len();
                    if i >= CHARS.len() {
                        i -= CHARS.len()
                    };
                    CHARS[i]
                }
                None => c,
            })
            .collect();
    }
}
