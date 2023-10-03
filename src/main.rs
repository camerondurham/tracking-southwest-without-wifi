use std::{env, fs, path::PathBuf, error::Error};

fn main() -> Result<(), Box<dyn Error> > {
    let arg_path = env::args().nth(1).expect("No path given");
    let as_path = PathBuf::from(arg_path);

    for entry in fs::read_dir(as_path)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;

        if metadata.is_file() {
            println!("parsing file {:?}", path.file_name().expect("no file name found"));
        }
    }

    Ok(())
}
