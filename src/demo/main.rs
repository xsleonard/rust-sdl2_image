#![crate_type = "bin"]
#![feature(path, os, env, core)]

extern crate sdl2;
extern crate sdl2_image;

use std::env;

mod video;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./demo image.[png|jpg]")
    } else {
        video::main(&Path::new(args[1].to_string_lossy().into_owned()));
    }
}
