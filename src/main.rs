use glob::glob;
use dotenv::dotenv;
use std::env;

// Lists .wav file names //
fn main() {

    dotenv().ok();

    let my_env = env::var("THE_PATH").expect("Path Is Set.");

        for entry in glob(&my_env).expect("Path Is Set.") {
            match entry {
                Ok(path) => {
                 println!("Name: {:?}", path.file_name());
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }

// Move .wav files to mySql database //