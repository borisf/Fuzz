use std::env;
use std::io::stdin;
use std::io::{self, IsTerminal};

use fuzz::fuzzyfinder::ui::Picker;
use walkdir::DirEntry;
use walkdir::WalkDir;

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn main() {
    let mut picks: Vec<String> = vec![];

    if io::stdin().is_terminal() {
        // Read the current directory ("./")
        //picks = fs::read_dir("/home/boris/Development/Fuzz/target/debug")
        //    .unwrap()
        //    .filter_map(|e| e.ok())
        //    .map(|e| e.path().to_string_lossy().into_owned())
        //    .collect::<Vec<_>>();

        let m = env::current_dir()
            .expect("Failed to get current directory")
            .to_string_lossy()
            .into_owned();

        let walker = WalkDir::new(&m).into_iter();
        for entry in walker.filter_entry(|e| !is_hidden(e)) {
            //println!("{}", entry?.path().display());
            if let Some(path) = entry.ok().map(|e| e.path().to_string_lossy().into_owned()) {
                picks.push(path);
            }
        }
    } else {
        for line in stdin().lines() {
            let line = line.expect("Error reading stdin").trim().to_string();
            picks.push(line);
        }
    }

    let mut picker = Picker::new(picks);

    picker.render();
    while !picker.finished() {
        picker.read_char();
    }

    match picker.get_selection() {
        Some(result) => match result.as_str() {
            "" => {}
            x => println!("{}", x),
        },
        None => {}
    }
    // println!("{}", picker.get_selection());
}
