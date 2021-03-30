use copypasta_ext::prelude::*;
use std::io::prelude::*;
mod mock;
mod uwu;

pub enum Source {
    Stdio,
    Clip,
    File,
}

impl Default for Source {
    fn default() -> Self{Source::Stdio}
}

pub struct Args {
    pub mock_min: u32,
    pub mock_max: u32,
}

impl Default for Args {
    fn default() -> Self { Args{mock_min: 1, mock_max: 3} }
}

pub fn rawr(mock: bool, uwu: bool, source_in: Source, source_out: Source, args: Args) {
    let mut clip = copypasta_ext::x11_bin::ClipboardContext::new().expect("Clip provider fail");

    let mut buff = match source_in {
        Source::Stdio => {
            let mut bytes: Vec<u8> = Vec::new();
            std::io::stdin()
            .read_to_end(&mut bytes)
            .expect("Could not read stdin");
            String::from_utf8(bytes).expect("Invalid UTF-8 for stdin")
        },
        Source::Clip => clip.get_contents().expect("Clip get fail"),
        Source::File => panic!("File reading not implemented yet"),
    };

    if uwu {
        uwu::uwu(&mut buff);
    }

    if mock {
        mock::mock(&mut buff, args.mock_min, args.mock_max);
    }

    match source_out {
        Source::Stdio => {
            std::io::stdout()
            .write(buff.as_bytes())
            .expect("Could not writie stdout");
        },
        Source::Clip => clip.set_contents(buff).expect("Clip set fail"),
        Source::File => panic!("File writing not implemented yet"),
    };
}
