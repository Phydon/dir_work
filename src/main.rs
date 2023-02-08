use colored::*;

use std::{env, fs, io};

// Colors => purple
const F7: u8 = 127;
const F8: u8 = 83;
const F9: u8 = 191;

fn main() {
    let tmp_dir_work = check_create_dir().unwrap();
    get_dirs(&tmp_dir_work).unwrap();
}

fn check_create_dir() -> io::Result<String> {
    let mut tmp_path = env::temp_dir();
    tmp_path.push("dir_work_tmp/");

    if !tmp_path.as_path().exists() {
        println!("Creating dir_work tmp path");
        fs::create_dir(&tmp_path)?;
    } else {
        println!("dir_work tmp path already exists");
    }

    let dir = tmp_path.into_os_string().into_string().unwrap();

    Ok(dir)
}

fn get_dirs(tmp_dir_work_path: &str) -> io::Result<()> {
    // for entry in fs::read_dir(env::temp_dir().as_path())? {
    for entry in fs::read_dir(tmp_dir_work_path)? {
        let entry = entry?;
        match entry.path().file_name() {
            Some(file) => {
                let content = file.to_string_lossy();
                if content.contains(&"dir_work_testfile".to_string()) {
                    println!("Deleted {}", content.italic().truecolor(F7, F8, F9));
                    // for testing
                    // FIXME TODO CAREFUL WITH REMOVING STUFF
                    // fs::remove_file(entry.path())?;
                    // fs::copy(
                    //     entry.path(),
                    //     "~\\AppData\\Local\\Temp\\dir_work_testfile2.txt",
                    // )?;
                }
            }
            None => {}
        }
    }

    Ok(())
}
