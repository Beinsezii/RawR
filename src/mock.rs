fn randrange(min: u32, max: u32) -> u32 {
    if max <= min {return std::cmp::min(min, max)}
    (rand::random::<u32>() % (max-min)) + min
}

pub fn mock(buff: &mut String, min: u32, max: u32) {
    let mut cap = false;
    let mut i = 0;
    let mut tar = randrange(min, max);
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
            tar = randrange(min, max);
            i = 0;
        }
    }

    *buff = result;
}
