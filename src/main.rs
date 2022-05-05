use std::{path::{Path, PathBuf}, error::Error};
use sdl2::{render::*, video::*, EventPump, event::*, keyboard::*, pixels::*, rect::*};
use gbi::{mainboard::Mainboard, ppu};

fn main() -> Result<(), Box<dyn Error>>
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2
    {
        println!("Please provide a ROM file.");
        std::process::exit(1);
    }
    let filename = PathBuf::from(&args[1]);
    let frontend = PCHardware::new()?;
    let mut motherboard = Mainboard::new(frontend);
    motherboard.load_game(Path::new(filename.as_path()))?;

    loop
    {
        motherboard.execute_frame();
    }
}

const COLORS:[Color; 4] =
[
    Color::RGB(0xFF, 0xFF, 0xFF),
    Color::RGB(0xAA, 0xAA, 0xAA),
    Color::RGB(0x55, 0x55, 0x55),
    Color::RGB(0x00, 0x00, 0x00),
];

struct PCHardware
{
    canvas: Canvas<Window>,
    event_pump: EventPump
}

impl PCHardware
{
    fn new() -> Result<Self, Box<dyn Error>>
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
                canvas: canvas,
                event_pump:event_pump
            }
        )
    }
}

impl gbi::Frontend for PCHardware
{
    fn receive_rom_information(&mut self, title: &str)
    {
        self.canvas.window_mut().set_title(&format!("Game Boy Inator - \"{}\"", title)).unwrap();
    }

    fn event_poll(&mut self)
    {
        for event in self.event_pump.poll_iter()
        {
            match event
            {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                {
                    std::process::exit(0);
                },
                _ => {}
            }
        }
    }

    fn video_update(&mut self, buffer: &[[u8; ppu::SCREEN_HEIGHT];ppu::SCREEN_WIDTH])
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
        self.canvas.window_mut().raise();
        self.canvas.present();
    }
}
