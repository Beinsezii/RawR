#![allow(non_snake_case)]

use copypasta_ext::prelude::*;
mod mock;
fn main() {
    let mut clip = copypasta_ext::x11_bin::ClipboardContext::new().expect("ClipContext init fail");
    let mut buff = clip.get_contents().expect("Clip get fail");
    mock::mock(&mut buff, 1, 4);
    clip.set_contents(buff).expect("Clip set fail");
}
