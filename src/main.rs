use std::{env, fs, path::PathBuf};
use serde_json::{Result, Value};

fn main() -> Result<()> {
    let arg_path = env::args().nth(1).expect("No path given");
    let as_path = PathBuf::from(arg_path);

    for entry in fs::read_dir(&as_path).expect("unable to read directory") {
        let entry = entry.expect("cannot unwrap entry");
        let path = entry.path();
        let metadata = fs::metadata(&path).expect("unable to extract metadata");

        if metadata.is_file() {
            let filename = path.file_name().expect("no file name found").to_str().unwrap();

            println!("parsing file {:?}", filename);

            let f_path = PathBuf::from(&as_path).join(filename);

            // TODO: handle errors better here
            let contents = fs::read_to_string(f_path).expect("cannot read file to string");

            let j: Value = serde_json::from_str(&contents)?;
            println!("{:?}", j);
        }
    }

    Ok(())
}
