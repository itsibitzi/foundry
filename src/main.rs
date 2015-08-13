#![feature(plugin)]
#![plugin(docopt_macros)]

//extern crate cargo;
extern crate docopt;
extern crate gl;
extern crate rustc_serialize;
extern crate sdl2;
extern crate sdl2_ttf;

use render::Window;
use session::Buffer;

mod render;
mod session;

#[macro_use]
mod utils;

docopt!(Args derive Debug, "
foundry text editor for rust

Usage:
    foundry [options] [<file>]

Options:
    -h --help       Show this help message.
    -V --version    Show version information.
    -v --verbose    Output verbose information.
    -f --font=FONT  Set the font.
", flag_font: Option<String>);

fn get_version() -> Option<String> {
    let (maj, min, pat) = (
        option_env!("CARGO_PKG_VERSION_MAJ"),
        option_env!("CARGO_PKG_VERSION_MIN"),
        option_env!("CARGO_PKG_VERSION_PAT"),
        );
    match (maj, min, pat) {
        (Some(maj), Some(min), Some(pat)) => Some(format!("foundry {}.{}.{}", maj, min, pat)),
        _ => None,
    }
}

fn main() {
    let args: Args = Args::docopt()
        .options_first(true)
        .version(get_version())
        .decode().unwrap_or_else(|e| e.exit());
    utils::set_verbose(args.flag_verbose);

    verboseln!("{:?}", args);

    sdl2_ttf::init();

    let mut buffer = Buffer::new();

    let mut window = Window::new("Foundry", 800, 600);

    loop {
        window.process_events();
        if window.should_close() {
            break;
        }

        window.write("baws");

        window.present();
    }

    sdl2_ttf::quit();
}
