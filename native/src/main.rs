#![allow(clippy::float_cmp)]
#![allow(non_snake_case)]

extern crate fern;
#[macro_use]
extern crate log;

#[cfg(feature = "cairo-renderer")]
extern crate cairo;
extern crate clap;
extern crate glium;
extern crate imgui;
extern crate imgui_winit_support;
extern crate nfd;
extern crate xprite;

mod consts;
mod prelude;
mod render;
mod state;
mod ui;

use self::prelude::*;
use crate::render::imgui::ImguiRenderer;
use clap::{App, Arg};
use std::sync::{Arc, Mutex};

#[allow(unused)]
fn main() -> Result<(), String> {
    let mut t = App::new("xprite")
        .version("1.0")
        .author("Ricky Han <xprite@rickyhan.com>")
        .about("pixel art editor");
    if cfg!(feature = "python-scripting") {
        t = t.arg(
            Arg::with_name("INPUT")
                .short("-p")
                .long("python")
                .value_name("PY_FILE")
                .help("Run python script"),
        );
    }

    t = t.arg(Arg::with_name("FILENAME").value_name("FILENAME").help("file to edit"));

    let matches = t.get_matches();

    if let Some(inp_file) = matches.value_of("INPUT") {
        #[cfg(feature = "python-scripting")]
        {
            run_python_script(inp_file)?;
        }
    } else {
        run_ui(matches.value_of("FILENAME"));
    }

    Ok(())
}

#[cfg(feature = "python-scripting")]
fn run_python_script(fname: &str) -> Result<(), String> {
    println!("Running Python script {}", fname);
    let xpr = xprite::scripting::python::python(fname)?;
    println!("Finished {}", fname);
    let state = State::new(xpr);
    state.xpr().save_img("1.png", 1);
    Ok(())
}

fn run_ui(fname: Option<&str>) {
    trace!("Starting Xprite");
    let art_w = DEFAULT_WIDTH;
    let art_h = DEFAULT_HEIGHT;
    let xpr = match fname {
        Some(fname) => {
            if fname.ends_with(".ase") || fname.ends_with(".aseprite") {
                Xprite::load_ase(fname)
            } else {
                Xprite::load_img(fname)
            }
        }
        None => Xprite::new("Untitled".to_owned(), art_w, art_h),
    };
    init_full_logger(Arc::clone(&xpr.log));
    let mut state = State::new(xpr);

    let system = crate::render::run::init(&format!("Sprite 3 v{}", env!("CARGO_PKG_VERSION")));
    system.main_loop(|_, ui, gl_ctx, textures| {
        let mut rdr = ImguiRenderer::new(&ui, gl_ctx, textures);
        state.load_icons(&mut rdr);
        ui::draw(&mut rdr, &mut state, ui);
    });
}

fn init_full_logger(console_logger: Arc<Mutex<String>>) {
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Debug)
        // - and per-module overrides
        .level_for("hyper", log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        .chain(fern::Output::call(move |record| {
            console_logger.lock().unwrap().push_str(&format!("{}\n", record.args()));
        }))
        // .chain(fern::log_file("output.log")?)
        // Apply globally
        .apply()
        .unwrap();
}
