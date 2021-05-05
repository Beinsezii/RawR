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

pub struct Rawrgs<'a> {
    pub input: Source<'a>,
    pub output: Source<'a>,
    pub uwu: bool,
    pub mock: bool,
    pub cipher: String,
    pub mock_range: std::ops::RangeInclusive<usize>,
    pub cipher_decode: bool,
}

impl Default for Rawrgs<'_> {
    fn default() -> Self { Rawrgs{
        input: Source::Stdio,
        output: Source::Stdio,
        uwu: false,
        mock: false,
        cipher: "".to_owned(),
        mock_range: 1..=4,
        cipher_decode: false,
    } }
}

pub fn get_file() {
}

pub fn set_file(){
}

pub fn get_clip() -> String {
    copypasta_ext::x11_bin::ClipboardContext::new().expect("Clip provider fail").get_contents().expect("Clip get fail")
}

pub fn set_clip(buff: String){
    copypasta_ext::x11_bin::ClipboardContext::new().expect("Clip provider fail").set_contents(buff).expect("Clip set fail")
}

pub fn rawr<'a>(rawrgs: Rawrgs<'a>) {

    let mut buff = match rawrgs.input {
        Source::Stdio => {
            let mut bytes: Vec<u8> = Vec::new();
            std::io::stdin()
            .read_to_end(&mut bytes)
            .expect("Could not read stdin");
            String::from_utf8(bytes).expect("Invalid UTF-8 for stdin")
        },
        Source::Clip => get_clip(),
        Source::File(file) => String::from_utf8(std::fs::read(file).expect("Input file read error")).expect("Input file invalid UTF-8"),
        Source::String(string) => string.to_owned(),
    };

    if !rawrgs.cipher.is_empty() && rawrgs.cipher_decode {
        cipher::cipher(&rawrgs.cipher, rawrgs.cipher_decode, &mut buff);
    }

    if rawrgs.uwu {
        uwu::uwu(&mut buff);
    }

    if rawrgs.mock {
        mock::mock(&mut buff, &rawrgs.mock_range);
    }

    if !rawrgs.cipher.is_empty() && !rawrgs.cipher_decode {
        cipher::cipher(&rawrgs.cipher, rawrgs.cipher_decode, &mut buff);
    }

    match rawrgs.output {
        Source::Stdio => {
            std::io::stdout()
            .write(buff.as_bytes())
            .expect("Could not writie stdout");
        },
        Source::Clip => set_clip(buff),
        Source::File(file) => std::fs::write(file, buff).expect("Output file write error"),
        Source::String(string) => *string = buff,
    };
}
