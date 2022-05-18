mod game_window;

use std::{path::{Path, PathBuf}, error::Error};
use game_window::PCHardware;
use gbi::mainboard::Mainboard;

cfg_if::cfg_if!
{
    if #[cfg(feature = "gui")]
    {
        slint::include_modules!();
        use slint::*;
    }
}

fn main() -> Result<(), Box<dyn Error>>
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2
    {
        cfg_if::cfg_if!
        {
            if #[cfg(feature = "gui")]
            {
                let main_window = MainWindow::new();

                let paths = std::fs::read_dir("./").unwrap();
                let mut roms = Vec::new();
                for file in paths
                {
                    let path = match file.as_ref()
                    {
                        Ok(x) if x.path().is_file() => x.path(),
                        _ => continue,
                    };
                    match path.extension()
                    {
                        Some(p) if p == "gb" =>
                        {
                            println!("{:?}", path);
                            roms.push
                            (
                                GameItem
                                {
                                    name: SharedString::from(path.file_name().unwrap().to_str().unwrap()),
                                    path: SharedString::from(path.to_str().unwrap())
                                }
                            );
                        }
                        _ => {}
                    }
                }

                main_window.set_list_of_roms(ModelRc::from(std::rc::Rc::new(VecModel::from(roms))));
                main_window.on_button_pressed(move |s: SharedString|
                {
                    println!("{}", s);
                    launch_game(s.to_string()).unwrap();
                });
                main_window.run();
                Ok(())
            }
            else
            {
                println!("Please provide a ROM file.");
                std::process::exit(1);
            }
        }
    }
    else
    {
        launch_game(String::from(&args[1]))
    }
}

fn launch_game(path: String) -> Result<(), Box<dyn Error>>
{
    let filename = PathBuf::from(path);
    let frontend = PCHardware::new()?;
    let mut motherboard = Mainboard::new(frontend);
    motherboard.load_game(Path::new(filename.as_path()))?;

    loop
    {
        motherboard.execute_frame();
    }
}
