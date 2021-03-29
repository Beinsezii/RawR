use clap::{App, Arg, ArgGroup};
use copypasta_ext::prelude::*;
use std::io::prelude::*;
mod mock;

fn mock_range_valid(arg: String) -> Result<(), String> {
    let arg = arg.trim();
    match arg.parse::<u32>() {
        Ok(_) => return Ok(()),
        Err(_) => (),
    };

    let words: Vec<&str> = arg.split("-").collect();
    if words.len() == 2 {
        match words[0].parse::<u32>() {
            Ok(_) => {
                match words[1].parse::<u32>() {
                    Ok(_) => return Ok(()),
                    Err(_) => (),
                };
            }
            Err(_) => (),
        };
    };

    Err(String::from(
        "Invalid mock-range value. Must be number or a positive range such as 2-5",
    ))
}

fn main() {
    let mut clip = copypasta_ext::x11_bin::ClipboardContext::new().expect("Clip provider fail");

    let args = App::new("RawR")
        .author("Beinsezii")
        .version("0.2.0")
        .about("Give your text some special flavor")
        .long_about("Give your text some special flavor\nDefaults to reading/writing from stdin/stdout")
        .arg(Arg::with_name("mock")
            .help("mOCkINg/SPongEBOb TExT")
            .long("mock")
        )
        .arg(Arg::with_name("uwu")
            .long("uwu")
        )
        .group(ArgGroup::with_name("flavor")
            .args(&["mock", "uwu"])
            .required(true)
            .multiple(true)
        )
        .arg(Arg::with_name("mock-range")
            .help("Controls minimum and maximum consecutive letters of a case. Number or range")
            .long("mock-range")
            .takes_value(true)
            .default_value("1-4")
            .validator(mock_range_valid)
        )
        .arg(Arg::with_name("clip-in")
            .help("Read from system clipboard")
            .short("c")
            .long("clip-in")
        )
        .group(ArgGroup::with_name("input")
            .args(&["clip-in"])
            // .required(false)
        )
        .arg(Arg::with_name("clip-out")
            .help("Output to system clipboard")
            .short("C")
            .long("clip-out")
        )
        .group(ArgGroup::with_name("output")
            .args(&["clip-out"])
            // .required(false)
        )
        .get_matches();

    let mut buff = if args.is_present("clip-in") {
        clip.get_contents().expect("Clip get fail")
    } else {
        let mut bytes: Vec<u8> = Vec::new();
        std::io::stdin()
            .read_to_end(&mut bytes)
            .expect("Could not read stdin");
        String::from_utf8(bytes).expect("Invalid UTF-8 for stdin")
    };

    if args.is_present("mock") {
        let vals = args.value_of("mock-range").unwrap().trim();
        let (v1, v2) = match vals.parse::<u32>() {
            Ok(v) => (v, v),
            Err(_) => {
                let nums: Vec<&str> = vals.split("-").collect();
                (
                    nums[0].parse::<u32>().unwrap(),
                    nums[1].parse::<u32>().unwrap(),
                )
            }
        };
        mock::mock(&mut buff, v1, v2);
    }

    if args.is_present("uwu") {
        println!("UwU todo hehe~");
    }

    if args.is_present("clip-out") {
        clip.set_contents(buff).expect("Clip set fail");
    } else {
        std::io::stdout()
            .write(buff.as_bytes())
            .expect("Could not writie stdout");
    };
}
