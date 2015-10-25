//#![feature(alloc, dir_builder, heap_api, oom, plugin, unique)]
#![feature(alloc, dir_builder, plugin)]
#![plugin(docopt_macros)]

extern crate alloc;

extern crate cargo;
extern crate chrono;
extern crate crypto;
extern crate docopt;
#[macro_use]
extern crate glium;
extern crate glium_text;
extern crate rustc_serialize;

use std::fs::File;
use std::sync::mpsc;
use std::thread;

use glium::{DisplayBuild, glutin, Surface};

use project::Project;
use render::{RenderMessage};

mod gap_buffer;
mod project;
mod render;

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
    -f --font=FONT  Set the font. [default: fonts/NotoSans/NotoSans-Regular.ttf]
");

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

    let project = Project::open(args.flag_verbose);
    println!("{:?}", project.config.values());

    let display = glutin::WindowBuilder::new()
        .with_title("Foundry".to_string())
        .with_dimensions(800, 600)
        .with_vsync()
        .build_glium()
        .unwrap();

    // Text
    let text_system = glium_text::TextSystem::new(&display);
    let font = glium_text::FontTexture::new(&display, File::open(args.flag_font).unwrap(), 24).unwrap();
    let text = glium_text::TextDisplay::new(&text_system, &font, "This is a tribute!");

    // Async rendering stuff
    let (tx, rx) = mpsc::channel();
    let test_tx = tx.clone();

    thread::spawn(move || {
        let mut redness = 0.0;

        loop {
            test_tx.send(RenderMessage {redness: redness});
            if redness >= 1.0 {
                redness = 0.0;
            }
            redness += 0.01;
        }
    });

    loop {
        let mut target = display.draw();
        target.clear_color(1.0, 0.0, 0.0, 1.0);

        match rx.try_recv() {
            Ok(msg) => target.clear_color(msg.redness, 0.0, 0.0, 1.0),
            Err(why) => {},
        }

        glium_text::draw(&text, &text_system, &mut target, render::matrix::IDENTITY, (1.0, 1.0, 1.0, 1.0));

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glutin::Event::Closed => {println!("BAIL"); return },// return,
                _ => {},
            }
        }
    }
}
