use std::ffi::{OsStr, OsString};
use std::{env, fs};
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let folder_path = match get_path_from_args(&args) {
        Ok(path) => path,
        Err(err) => panic!("{err}")
    };
    let folder_path = Path::new(&folder_path);
    if !folder_path.is_absolute() { panic!("Invalid path, check if is absolute!") }

    let content = fs::read_dir(folder_path).expect("cant read children");
    
    for dir_entry in content.into_iter() {
        
        let dir_entry = match dir_entry {
            Ok(de) => de,
            Err(_) => continue,
        };
        
        if !dir_entry.path().is_dir() { continue; }
        
        let path = dir_entry.path();
        let name_dir = path
            .components()
            .last()
            .expect("Invalid path")
            .as_os_str();

        let date_format = match get_date_format_from_os_string(&name_dir) {
            Ok(df) => df,
            Err(_) => continue,
        };

        let name = new_folder_name(date_format);
        let new_path = change_last_component_for(&path, name);
        fs::rename(path, new_path).expect("problem renaming");
    }
}

fn change_last_component_for(path: &PathBuf, name: OsString) -> PathBuf {
    path.parent()
        .expect("has parent")
        .join(name)
}

fn get_path_from_args(args: &Vec<String>) -> Result<String, String> {
    if args.len() >= 2 {
        return Ok(args[1].to_string());
    }
    return Err("Cant find path".to_string());
}

struct DateFormat {
    month: String,
    year: String
}

fn get_date_format_from_os_string(origin: &OsStr) -> Result<DateFormat, String> {
    let origin = match origin.to_str().ok_or("bad conv") {
        Ok(o) => o,
        Err(err) => return Err(err.to_string())
    };
    let origin: Vec<&str> = origin.split(":").collect();
    if origin.len() != 2 { return Err("not valid format".to_string());}
    return Ok( DateFormat { month: origin[0].to_string(), year: origin[1].to_string() });
}

fn new_folder_name(date_format: DateFormat) -> OsString {
    let mut new_name = String::new();
    new_name.push_str(&date_format.year);
    new_name.push('-');
    new_name.push_str(&date_format.month);

    return OsString::from(new_name);
}