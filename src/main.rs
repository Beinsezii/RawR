mod cli;

#[cfg(feature = "gui")]
mod gui;

fn main() {
    #[allow(unused)] // will warn if built without gui. idk how to put it into the gui block.
    let args = std::env::args().collect::<Vec<String>>();
    let (rawrgs, gui) = cli::process();

    #[cfg(feature = "gui")]
    // run if no args, only arg is the binary path, or '--gui' passed
    if gui {
        eframe::run_native(Box::new(gui::App {
            input: match rawrgs.input {
                rawr::Source::Clip => rawr::get_clip(),
                _ => String::new(),
            },
            uwu: rawrgs.uwu,
            mock: rawrgs.mock,
            cipher: rawrgs.cipher,
            mock_min: *rawrgs.mock_range.start(),
            mock_max: *rawrgs.mock_range.end(),
            cipher_decode: rawrgs.cipher_decode,
            ..Default::default()
        }));
        //returns automatically here
    }

    if gui {
        panic!("GUI has not been enabled for this build");
    }

    rawr::rawr(rawrgs);
}
