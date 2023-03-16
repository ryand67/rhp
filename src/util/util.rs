use std::{
    env::args,
    fs::{read_dir, File},
    io::{BufReader, Error},
    path::PathBuf,
};

fn get_path_args() -> Option<PathBuf> {
    let file_name: String = args().nth(1).unwrap_or("".to_string());

    if file_name == "".to_string() {
        return None;
    }

    return Some(PathBuf::from(file_name));
}

pub fn open_file(path: &PathBuf) -> Result<BufReader<File>, Error> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    return Ok(reader);
}

pub fn get_reader() -> Result<Vec<PathBuf>, String> {
    let path_name = get_path_args().expect("No file path given.");

    if path_name.is_file() {
        if path_name.extension().unwrap() != "html" {
            return Err(format!(
                "Please pass an html file, you passed {:?}.",
                path_name.file_name().unwrap()
            ));
        }

        return Ok(vec![path_name]);
    } else if path_name.is_dir() {
        let mut v: Vec<PathBuf> = Vec::new();

        let e = get_paths(&path_name, &mut v);
        if e.is_err() {
            return Err(format!("{}", e.unwrap_err()));
        }

        return Ok(v);
    } else {
        return Err("idk man".to_string());
    }
}

fn get_paths(path: &PathBuf, readers: &mut Vec<PathBuf>) -> Result<(), Error> {
    let paths = read_dir(path)?;

    for p in paths {
        let entry = p?;
        let path = entry.path();

        if path.is_dir() {
            get_paths(&path, readers)?;
        } else {
            readers.push(path);
        }
    }

    Ok(())
}
