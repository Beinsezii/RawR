const CHARS: [char; 54] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '-', '\'',
];

fn title(buff: &str) -> String {
    let mut first = true;
    buff.chars()
        .map(|mut c| {
            if first {
                c.make_ascii_uppercase();
                first = false;
            } else {
                c.make_ascii_lowercase();
            };
            c
        })
        .collect::<String>()
}

fn repwace(buff: &str, from: &str, to: &str) -> String {
    // copy of str's .replace, but matches case. Needs from and to to be same len.
    if from.len() == to.len() {
        let mut result = String::new();
        let mut last_end = 0;
        for (start, _part) in buff
            .to_ascii_lowercase()
            .match_indices(&from.to_ascii_lowercase())
        {
            result.push_str(unsafe { buff.get_unchecked(last_end..start) });
            result.push_str(
                &unsafe { buff.get_unchecked(start..start + to.len()) }
                    .chars()
                    .zip(to.chars())
                    .map(|mut c| {
                        if c.0.is_ascii_uppercase() {
                            c.1.make_ascii_uppercase()
                        } else {
                            c.1.make_ascii_lowercase()
                        }
                        c.1
                    })
                    .collect::<String>(),
            );
            last_end = start + to.len();
        }
        result.push_str(unsafe { buff.get_unchecked(last_end..buff.len()) });
        result
    // 'dumb' fallback method. Checks literal, lower, UPPER, and 'Title'.
    } else {
        let mut buff = buff.to_owned();
        let mut dupes_check = Vec::<String>::new();
        for v in [
            (from.to_ascii_lowercase(), to.to_ascii_lowercase()),
            (from.to_ascii_uppercase(), to.to_ascii_uppercase()),
            (title(&from), title(&to)),
            (from.to_owned(), to.to_owned()),
        ].iter() {
            if !dupes_check.contains(&v.0) {
                buff = buff.replace(&v.0, &v.1)
            };
            dupes_check.push(v.0.clone());
        }
        buff
    }
}

fn repwend(buff: &str, from: &str, to: &str) -> String {
    if buff.len() >= from.len() {
        unsafe { buff.get_unchecked(..buff.len() - from.len()) }.to_owned()
            + &unsafe { buff.get_unchecked(buff.len() - from.len()..) }.repwace(from, to)
    } else {
        buff.to_owned()
    }
}

fn repwart(buff: &str, from: &str, to: &str) -> String {
    if buff.len() >= from.len() {
        unsafe { buff.get_unchecked(0..from.len()) }.repwace(from, to)
            + unsafe { buff.get_unchecked(from.len()..) }
    } else {
        buff.to_owned()
    }
}

trait UwU {
    fn repwace(&self, from: &str, to: &str) -> String;
    fn repwend(&self, from: &str, to: &str) -> String;
    fn repwart(&self, from: &str, to: &str) -> String;
}

impl UwU for str {
    fn repwace(&self, from: &str, to: &str) -> String {
        repwace(self, from, to)
    }
    fn repwend(&self, from: &str, to: &str) -> String {
        repwend(self, from, to)
    }
    fn repwart(&self, from: &str, to: &str) -> String {
        repwart(self, from, to)
    }
}

impl UwU for String {
    fn repwace(&self, from: &str, to: &str) -> String {
        repwace(self, from, to)
    }
    fn repwend(&self, from: &str, to: &str) -> String {
        repwend(self, from, to)
    }
    fn repwart(&self, from: &str, to: &str) -> String {
        repwart(self, from, to)
    }
}

fn process_word(word: &str) -> String {
    match word.to_lowercase().as_str() {
        "yes" => word.repwace("e", "i"),
        "no" => word.repwace("o", "u"),
        "the" | "this" | "that" => word.repwace("th", "d"),
        "think" => word.repwace("th", "t"),
        "have" => word.repwace("have", "haf"),
        "when" | "which" | "what" => word.repwace("wh", "w"),
        "your" | "you" => word.repwace("you", "u"),

        "know" | "though" => unsafe { word.get_unchecked(..3) }.to_owned(),
        "yeah" => unsafe { word.get_unchecked(0..2) }.to_owned(),

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
    .repwend("ss", "ssy")
    .repwend("ck", "cky")
    .repwend("ug", "uggy")
    .repwace("l", "w")
    .repwace("r", "w")
    .repwace("aughty", "awty")
    .repwace("ould", "ud")
    .repwace("ime", "iem")
    .repwace("ike", "iek")
    .repwace("icks", "ickies")
    .repwace("cause", "cuz")
    .repwace("some", "sum")
}

pub fn uwu(buff: &mut String) {
    let mut result = String::with_capacity(buff.len());
    let mut word = String::new();
    for c in buff.chars() {
        if CHARS.contains(&c) {
            word.push(c);
        } else {
            if !word.is_empty() {
                result.push_str(&process_word(&word));
                word.clear();
            }
            result.push(c);
        };
    }
    result.push_str(&process_word(&word));
    result.shrink_to_fit(); //99% of the time it'll be longer, but just in case.
    *buff = result;
}
