use clap::{App, Arg, ArgGroup};

fn mock_range(arg: &str) -> Result<std::ops::RangeInclusive<usize>, String> {
    let arg = arg.trim();
    match arg.parse::<usize>() {
        Ok(v) => Ok(v..=v),
        Err(_) => {
            let nums: Vec<&str> = arg.split("-").collect();
            match nums.get(0) {
                Some(s1) => match s1.parse::<usize>() {
                    Ok(v1) => match nums.get(1) {
                        Some(s2) => match s2.parse::<usize>() {
                            Ok(v2) => Ok(v1..=v2),
                            Err(_) => Err(String::from("Second mock-range parse fail")),
                        },
                        None => Err(String::from("Second mock-range index fail")),
                    },
                    Err(_) => Err(String::from("First mock-range parse fail")),
                },
                None => Err(String::from("First mock-range index fail")),
            }
        }
    }
}

fn mock_range_valid(arg: String) -> Result<(), String> {
    match mock_range(&arg) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn process<'a>() -> (rawr::Rawrgs<'a>, bool) {
    let args = App::new("RawR")
        .author("Beinsezii")
        .version("2.0.0")
        .about(
            "Give your text some special flavor\nDefaults to reading/writing from stdin/stdout.",
        )
        .arg(
            Arg::with_name("gui")
                .help(if cfg!(feature = "gui") {"Bring up the GUI. Default if no args present"}
                      else {"GUI has been disabled for this build and this flag will panic."})
                .short("g")
                .long("gui")
        )
        .arg(
            Arg::with_name("uwu")
                .help("UwU vewy coot and fwenwy :D text")
                .display_order(1)
                .long("uwu")
                .short("u"),
        )
        .arg(
            Arg::with_name("mock")
                .help("mOCkINg/SPongEBOb TExT")
                .display_order(2)
                .long("mock")
                .short("m"),
        )
        .arg(
            Arg::with_name("cipher")
                .help("Encrypt/decrypt text with a password")
                .long_help("Encrypt/decrypt text with a password\n\
                            I don't know anything about cryptography so don't use it for illegal shiz")
                .display_order(3)
                .long("password")
                .short("p")
                .takes_value(true)
        )
        .group(
            ArgGroup::with_name("flavor")
                .args(if cfg!(feature = "gui") {&["mock", "uwu", "cipher", "gui"]} else {&["mock", "uwu", "cipher"]})
                .multiple(true)
                .required(!cfg!(feature = "gui")),
        )
        .arg(
            Arg::with_name("mock-range")
                .help("Controls consecutive letters of a case. Number or range")
                .display_order(21)
                .long("mock-range")
                .short("R")
                .takes_value(true)
                .default_value("1-4")
                .validator(mock_range_valid),
        )
        .arg(
            Arg::with_name("cipher-decode")
            .help("Decode existing cipher")
            .display_order(22)
            .long("cipher-decode")
            .short("D"),
        )
        .arg(
            Arg::with_name("clip-in")
                .help("Read from system clipboard")
                .display_order(91)
                .short("c")
                .long("clip-in"),
        )
        .arg(
            Arg::with_name("clip-out")
                .help("Output to system clipboard")
                .display_order(92)
                .short("C")
                .long("clip-out"),
        )
        .arg(
            Arg::with_name("file-in")
                .help("Read from a file")
                .display_order(93)
                .short("f")
                .long("file-in")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file-out")
                .help("Output to a file")
                .display_order(94)
                .short("F")
                .long("file-out")
                .takes_value(true),
        )
        .group(
            ArgGroup::with_name("input").args(&["clip-in", "file-in"]),
        )
        .group(
            ArgGroup::with_name("output").args(&["clip-out", "file-out"]),
        )
        .get_matches();
    // looks like their get_matches_from clones anyway so not much point in
    // re-passing the args. I think...

    (
        rawr::Rawrgs {
            uwu: args.is_present("uwu"),
            mock: args.is_present("mock"),
            cipher: args.value_of("cipher").unwrap_or("").to_owned(),
            input: if args.is_present("clip-in") {
                rawr::Source::Clip
            } else if args.is_present("file-in") {
                rawr::Source::File(
                    args.value_of("file-in")
                        .expect("Invalid file-in arg")
                        .to_owned(),
                )
            } else {
                rawr::Source::Stdio
            },
            output: if args.is_present("clip-out") {
                rawr::Source::Clip
            } else if args.is_present("file-out") {
                rawr::Source::File(
                    args.value_of("file-out")
                        .expect("Invalid file-out arg")
                        .to_owned(),
                )
            } else {
                rawr::Source::Stdio
            },
            mock_range: mock_range(args.value_of("mock-range").unwrap_or(""))
                .expect("mock_range assign fail"),
            cipher_decode: args.is_present("cipher-decode"),
        },
        match args.is_present("gui") {
            true => true,
            false => {
                !(args.is_present("uwu") || args.is_present("mock") || args.is_present("cipher"))
            }
        },
    )
}
