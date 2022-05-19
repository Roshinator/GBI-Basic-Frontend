
slint::include_modules!();
use std::{path::PathBuf};

use slint::{*};

pub fn new() -> MainWindow
{
    let window = MainWindow::new();
    let weak = window.as_weak();
    window.global::<GameListData>().on_select_directory(move ||
    {
        let window = weak.upgrade().unwrap();
        let game_list_data = window.global::<GameListData>();
        let res = rfd::FileDialog::new()
            .set_directory(game_list_data.get_path().to_string())
            .pick_folder();
        if let Some(x) = res
        {
            game_list_data.set_path(SharedString::from(x.to_str().unwrap()));
            update_games(&window, &x).unwrap();
        }
    });
    let weak = window.as_weak();
    window.global::<GameListData>().on_refresh_directory(move ||
    {
        let window = weak.upgrade().unwrap();
        let game_list_data = window.global::<GameListData>();
        update_games(&window, &PathBuf::from(String::from(game_list_data.get_path()))).unwrap();
    });
    window
}

pub fn update_games(window: &MainWindow, path: &PathBuf) -> Result<(), std::io::Error>
{
    let paths = std::fs::read_dir(path)?;
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
                println!("Found file in directory {:?}", path);
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

    let game_list_data = window.global::<GameListData>();
    game_list_data.set_path(SharedString::from(std::fs::canonicalize(path).unwrap().to_str().unwrap()));
    game_list_data.set_list_of_roms(ModelRc::from(std::rc::Rc::new(VecModel::from(roms))));
    game_list_data.on_button_pressed(move |s: SharedString|
    {
        println!("Launching {}", s);
        crate::launch_game(s.to_string()).unwrap();
    });
    Ok(())
}