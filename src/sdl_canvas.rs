use std::time::Duration;

use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

use crate::game::World;

struct Config {
    window_width: usize,
    window_height: usize,
    pixel_size: usize,
}

impl Config {
    fn new(window_width: usize, window_height: usize, pixel_size: usize) -> Self {
        Self {
            window_width,
            window_height,
            pixel_size,
        }
    }

    fn pixel_count_x(&self) -> usize {
        self.window_width / self.pixel_size
    }

    fn pixel_count_y(&self) -> usize {
        self.window_height / self.pixel_size
    }
}

const CONFIG: Config = Config {
    window_width: 1200,
    window_height: 800,
    pixel_size: 5,
};

pub fn run() -> Result<(), String> {
    let mut world = World::new(CONFIG.pixel_count_x(), CONFIG.pixel_count_y());

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Game of Life",
            CONFIG.window_width as u32,
            CONFIG.window_height as u32,
        )
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .map_err(|s| s.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|s| s.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    let mut event_dump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_dump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::MouseMotion { .. } => {}
                e => {
                    println!("{e:?}");
                }
            }
        }

        world.tick();

        draw_world(&mut canvas, &world)?;

        std::thread::sleep(Duration::from_millis(50));
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
    }

    Ok(())
}

fn draw_world(canvas: &mut Canvas<Window>, world: &World) -> Result<(), String> {
    for (index, cell) in world.cells().iter().enumerate() {
        let (x, y) = world.get_pos(index);
        let rect = Rect::new(
            (x * CONFIG.pixel_size) as i32,
            (y * CONFIG.pixel_size) as i32,
            CONFIG.pixel_size as u32,
            CONFIG.pixel_size as u32,
        );
        if cell.is_alive() {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
        } else {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
        }

        canvas.fill_rect(rect)?;
    }
    canvas.present();

    Ok(())
}
