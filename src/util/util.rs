use std::{
    env::args,
    fs::File,
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

fn open_file(path: &PathBuf) -> Result<BufReader<File>, Error> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    return Ok(reader);
}

pub fn get_reader() -> Result<Vec<BufReader<File>>, String> {
    let path_name = get_path_args().expect("No file path given.");

    if path_name.is_file() {
        if path_name.extension().unwrap() != "html" {
            return Err(format!(
                "Please pass an html file, you passed {:?}.",
                path_name.file_name().unwrap()
            ));
        }

        return Ok(vec![open_file(&path_name).expect("Error reading file")]);
    } else if path_name.is_dir() {
        println!("lol next time");
        todo!();
    } else {
        todo!();
    }
}
