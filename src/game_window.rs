use std::error::Error;

use sdl2::{render::*, video::*, EventPump, event::*, keyboard::*, pixels::*, rect::*};
use gbi::ppu;

const COLORS:[Color; 4] =
[
    Color::RGB(0xFF, 0xFF, 0xFF),
    Color::RGB(0xAA, 0xAA, 0xAA),
    Color::RGB(0x55, 0x55, 0x55),
    Color::RGB(0x00, 0x00, 0x00),
];

pub struct PCHardware
{
    canvas: Canvas<Window>,
    event_pump: EventPump,
    game_title: String
}

impl PCHardware
{
    pub fn new() -> Result<Self, Box<dyn Error>>
    {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem.window("Game Boy Inator", 810, 730)
            .position_centered()
            .resizable()
            .build()?;
        let mut canvas = window.into_canvas().accelerated().build()?;
        canvas.set_logical_size(ppu::SCREEN_WIDTH as u32, ppu::SCREEN_HEIGHT as u32)?;
        canvas.set_integer_scale(true)?;
        let event_pump = sdl_context.event_pump()?;
        Ok
        (
            PCHardware
            {
                canvas,
                event_pump,
                game_title: String::new()
            }
        )
    }
}

impl gbi::Frontend for PCHardware
{
    fn receive_rom_information(&mut self, title: &str)
    {
        self.game_title = format!("Game Boy Inator - \"{}\"", title);
        self.canvas.window_mut().set_title(&self.game_title).unwrap();
    }

    fn event_poll(&mut self) -> bool
    {
        for event in self.event_pump.poll_iter()
        {
            match event
            {
                Event::Window { win_event: WindowEvent::Close, .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                {
                    return true;
                },
                Event::Quit {..} =>
                {
                    std::process::exit(0);
                },
                _ => {}
            }
        }
        false
    }

    fn video_update(&mut self, buffer: &[[u8; ppu::SCREEN_HEIGHT];ppu::SCREEN_WIDTH], frame_count: u64)
    {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        for x_coord in 0..ppu::SCREEN_WIDTH
        {
            for y_coord in 0..ppu::SCREEN_HEIGHT
            {
                self.canvas.set_draw_color(COLORS[buffer[x_coord][y_coord] as usize]);
                self.canvas.draw_point(Point::new(x_coord as i32, y_coord as i32)).unwrap();
            }
        }
        let window = self.canvas.window_mut();
        window.set_title(&format!("{g} | Frame {frame_count}", g = self.game_title)).unwrap();
        self.canvas.present();
    }
}