use std::io;
use std::fs;
use std::path::Path;

use crate::routes::Route;

fn write_host_files(routes: Vec<Route>) {
    for route in routes{
        fs::create_dir_all(&route.route).expect("Failed creating directory");

        let file_path = route.route + "/filename.txt";
        let path = Path::new(&file_path);

        match path.exists(){
            true => todo!(),
            false => todo!(),
        }
    }
}