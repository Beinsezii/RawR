fn randrange(range: &std::ops::RangeInclusive<usize>) -> usize {
    if range.start() <= range.end() {return std::cmp::min(*range.start(), *range.end())}
    (rand::random::<usize>() % (range.end()-range.start())) + range.start()
}

pub fn mock(buff: &mut String, range: &std::ops::RangeInclusive<usize>) {
    let mut cap = false;
    let mut i = 0;
    let mut tar = randrange(range);
    let mut result = String::with_capacity(buff.len());

    for var in buff.chars(){
        let u = &var.to_uppercase().to_string();
        let l = &var.to_lowercase().to_string();
        result.push_str(match cap {
            true => u,
            false => l,
        });
        if u != l {i += 1};
        if i >= tar {
            cap = !cap;
            tar = randrange(range);
            i = 0;
        }
    }

    *buff = result;
}
