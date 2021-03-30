use clap::{App, Arg, ArgGroup};

fn mock_range(arg: &str) -> Result<(u32, u32), String> {
    let arg = arg.trim();
    match arg.parse::<u32>() {
        Ok(v) => Ok((v, v)),
        Err(_) => {
            let nums: Vec<&str> = arg.split("-").collect();
            match nums.get(0) {
                Some(s1) => match s1.parse::<u32>() {
                    Ok(v1) => match nums.get(1) {
                        Some(s2) => match s2.parse::<u32>() {
                            Ok(v2) => Ok((v1, v2)),
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

fn main() {
    let args = App::new("RawR")
        .author("Beinsezii")
        .version("0.2.0")
        .about("Give your text some special flavor")
        .long_about(
            "Give your text some special flavor\nDefaults to reading/writing from stdin/stdout",
        )
        .arg(
            Arg::with_name("mock")
                .help("mOCkINg/SPongEBOb TExT")
                .long("mock"),
        )
        .arg(Arg::with_name("uwu").long("uwu"))
        .group(
            ArgGroup::with_name("flavor")
                .args(&["mock", "uwu"])
                .required(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("mock-range")
                .help("Controls minimum and maximum consecutive letters of a case. Number or range")
                .long("mock-range")
                .takes_value(true)
                .default_value("1-4")
                .validator(mock_range_valid),
        )
        .arg(
            Arg::with_name("clip-in")
                .help("Read from system clipboard")
                .short("c")
                .long("clip-in"),
        )
        .group(
            ArgGroup::with_name("input").args(&["clip-in"]), // .required(false)
        )
        .arg(
            Arg::with_name("clip-out")
                .help("Output to system clipboard")
                .short("C")
                .long("clip-out"),
        )
        .group(
            ArgGroup::with_name("output").args(&["clip-out"]), // .required(false)
        )
        .get_matches();

    let (mr1, mr2) =
        mock_range(args.value_of("mock-range").unwrap_or("")).expect("mock_range assign fail");

    rawr::rawr(
        args.is_present("mock"),
        args.is_present("uwu"),
        if args.is_present("clip-in") {
            rawr::Source::Clip
        } else {
            rawr::Source::Stdio
        },
        if args.is_present("clip-out") {
            rawr::Source::Clip
        } else {
            rawr::Source::Stdio
        },
        rawr::Args {
            mock_min: mr1,
            mock_max: mr2,
            ..Default::default()
        },
    );
}
