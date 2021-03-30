const PUNCTUATION: &[char] = &['.', ',', '!', '?', '\'', '"'];

fn repwace(buff: &str, from: &str, to: &str, ) -> String {
    let res = buff
        .replace(from, to)
        .replace(&from.to_lowercase(), &to.to_lowercase())
        .replace(&from.to_uppercase(), &to.to_uppercase());

    if from.len() >= 2 && to.len() >= 2 {
        res.replace(
            &(from[0..1].to_uppercase() + &from[1..].to_lowercase()),
            &(to[0..1].to_uppercase() + &to[1..].to_lowercase()),
        )
    } else {
        res
    }
}

fn extwend(buff: &str, suffix: &str, extension: &str) -> String {
    if buff.ends_with(&suffix.to_uppercase()) {
        buff.to_owned() + &extension.to_uppercase()
    } else if buff.ends_with(&suffix.to_lowercase()) {
        buff.to_owned() + &extension.to_lowercase()
    } else {
        buff.to_owned()
    }
}

// fn repwend(buff: &str, from: &str, to: &str) -> String {
//     let (start, end) = buff.split_at(buff.to_lowercase().rfind(from).unwrap_or(0));
// }

trait UwU {
    fn repwace(&self, from: &str, to: &str) -> String;

    fn extwend(&self, suffix: &str, extension: &str) -> String;
}

impl UwU for str {
    fn repwace(&self, from: &str, to: &str) -> String {
        repwace(self, from, to)
    }
    fn extwend(&self, suffix: &str, extension: &str) -> String {
        extwend(self, suffix, extension)
    }
}

impl UwU for String {
    fn repwace(&self, from: &str, to: &str) -> String {
        repwace(self, from, to)
    }
    fn extwend(&self, suffix: &str, extension: &str) -> String {
        extwend(self, suffix, extension)
    }
}

pub fn uwu(buff: &mut String) {
    *buff = buff
        .split_ascii_whitespace()
        .map(|word| {
            let mut p_start = String::new();
            let mut p_end = String::new();
            for c in word.chars() {
                if PUNCTUATION.contains(&c) {
                    p_start.push(c)
                } else {
                    break;
                }
            }
            for c in word.chars().rev() {
                if PUNCTUATION.contains(&c) {
                    p_end.insert(0, c)
                } else {
                    break;
                }
            }

            let word = word
                .strip_prefix(&p_start)
                .expect("Bad uwu word prefix")
                .strip_suffix(&p_end)
                .expect("Bad uwu word suffix");

            let mut word = match word.to_lowercase().as_str() {
                "yes" => word.repwace("e", "i"),
                "no" => word.repwace("o", "u"),
                "the" | "this" | "that" => word.repwace("th", "d"),
                "think" => word.repwace("th", "t"),
                "have" => word.repwace("have", "haf"),
                "when" | "which" | "what" => word.repwace("wh", "w"),
                "your" | "you" => word.repwace("you", "u"),

                "know" | "though" => word[..3].to_owned(),
                "yeah" => word[0..2].to_owned(),

                "give" => word.repwace("give", "gib"),

                "sad" | "depressed" => word.to_owned() + " UnU",
                "happy" | "excited" => word.to_owned() + " ^-^",
                "sick" | "ill" => word.to_owned() + " >~<",
                "pleased" | "satisfied" | "nice" => word.to_owned() + " UwU",

                "fuck" => word.repwace("fuck", "fucky-wucky"),
                "gross" => word.repwace("gross", "icky-wicky"),
                "disgusting" => word.repwace("disgusting", "icky-wicky >~<"),
                "wet" => word.repwace("wet", "moist"),
                "soaked" => word.repwace("soaked", "moist OwO"),
                _ => word.to_owned(),
            }
            .extwend("ss", "y")
            .extwend("ck", "y")
            .extwend("ug", "gy")
            .repwace("l", "w")
            .repwace("r", "w")
            .repwace("aughty", "awty")
            .repwace("ould", "ud")
            .repwace("ime", "iem")
            .repwace("ike", "iek")
            .repwace("icks", "ickies")
            .repwace("cause", "cuz")
            .repwace("some", "sum")
            ;

            word.insert_str(0, &p_start);
            word.push_str(&p_end);
            word
        })
        .collect::<Vec<String>>()
        .join(" ");
}
