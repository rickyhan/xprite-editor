#![allow(clippy::float_cmp)]

extern crate fern;
#[macro_use] extern crate log;

extern crate xprite;
extern crate glium;
extern crate imgui;
extern crate clap;
extern crate imgui_glium_renderer;

extern crate cairo;

use xprite::prelude::*;

use crate::render::cairo::CairoRenderer;
// use crate::render::imgui_cairo::ImguiCairoRenderer;
use crate::render::imgui::ImguiRenderer;

use std::sync::{Arc, Mutex};

mod hotkey;
mod consts;
mod render;
mod prelude;
mod ui;
mod state;

use clap::{App, Arg, SubCommand};
fn main() {
    let matches = App::new("xprite")
                    .version("1.0")
                    .author("Ricky Han <xprite@rickyhan.com>")
                    .about("pixel art editor with extra tools")
                    .subcommand(SubCommand::with_name("run")
                                .about("run script")
                                .version("1.0")
                                .arg(Arg::with_name("INPUT")
                                    .help("INPUT.dyon script")
                                    .required(true)
                                    .index(1)
                                )
                                .arg(Arg::with_name("debug")
                                    .short("d")
                                    .help("print debug information verbosely")))
                    .get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        let inp_file = matches.value_of("INPUT").unwrap();
        run_script(inp_file);
    } else {
        run_ui();
    }
}

fn run_script(fname: &str) {
    let xpr = Xprite::new(100., 100.);
    let cairo = CairoRenderer::new(100., 100.);
    let mut state = state::State::new(xpr, cairo);
    state.script_fname = Some(fname.to_owned());
    state.xpr.execute_script(fname).unwrap();
    state.export_png("1.png");
}


fn run_ui() {
    trace!("Starting Xprite");
    let xpr = Xprite::new(100., 100.);
    init_logger(Arc::clone(&xpr.log));
    let cairo = CairoRenderer::new(100., 100.);
    let mut state = state::State::new(xpr, cairo);


    render::run("Xprite", BGCOLOR, |ui, gl_ctx, textures| {
        let mut rdr = ImguiRenderer::new(&ui, gl_ctx, textures);
        // let mut rdr = ImguiCairoRenderer::new(&ui, gl_ctx, textures, &state);
        ui::draw(&mut rdr, &mut state, ui)
    });
}

fn init_logger(console_logger: Arc<Mutex<String>>) {
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
            console_logger
                .lock().unwrap()
                .push_str(&format!("{}\n", record.args()));
        }))
        // .chain(fern::log_file("output.log")?)
        // Apply globally
        .apply().unwrap();
}