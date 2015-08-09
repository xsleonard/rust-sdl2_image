use std::path::Path;
use sdl2;
use sdl2_image::{self, LoadTexture, INIT_PNG, INIT_JPG};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn main(png: &Path) {

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    sdl2_image::init(INIT_PNG | INIT_JPG);
    let window = video.window("rust-sdl2 demo: Video", 800, 600)
      .position_centered()
      .build()
      .unwrap();

    let mut renderer = window.renderer().software().build().unwrap();
    let texture = renderer.load_texture(png).unwrap();

    renderer.copy(&texture, None, None);
    renderer.present();

    'mainloop: loop {
        for event in sdl.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }
    }

    sdl2_image::quit();
}
