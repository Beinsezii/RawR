use copypasta_ext::prelude::*;
use std::io::prelude::*;
mod mock;
mod uwu;
mod cipher;

pub enum Source <'a> {
    Stdio,
    Clip,
    File(String),
    String(&'a mut String),
}

impl Default for Source<'_> {
    fn default() -> Self{Source::Stdio}
}

pub struct Args {
    pub mock_min: u32,
    pub mock_max: u32,
    pub cipher_decode: bool,
}

impl Default for Args {
    fn default() -> Self { Args{
        mock_min: 1,
        mock_max: 3,
        cipher_decode: false,
    } }
}

pub fn rawr(uwu: bool, mock: bool, cipher: &str, source_in: Source, source_out: Source, args: Args) {
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
        Source::File(file) => String::from_utf8(std::fs::read(file).expect("Input file read error")).expect("Input file invalid UTF-8"),
        Source::String(string) => string.to_owned(),
    };

    if !cipher.is_empty() && args.cipher_decode {
        cipher::cipher(cipher, args.cipher_decode, &mut buff);
    }

    if uwu {
        uwu::uwu(&mut buff);
    }

    if mock {
        mock::mock(&mut buff, args.mock_min, args.mock_max);
    }

    if !cipher.is_empty() && !args.cipher_decode {
        cipher::cipher(cipher, args.cipher_decode, &mut buff);
    }

    match source_out {
        Source::Stdio => {
            std::io::stdout()
            .write(buff.as_bytes())
            .expect("Could not writie stdout");
        },
        Source::Clip => clip.set_contents(buff).expect("Clip set fail"),
        Source::File(file) => std::fs::write(file, buff).expect("Output file write error"),
        Source::String(string) => *string = buff,
    };
}
