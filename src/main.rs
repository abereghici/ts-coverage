use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::{env, io, process};

struct Extension<'a> {
    name: &'a str,
    variations: Vec<&'a str>,
}

impl<'a> Extension<'a> {
    pub fn new(name: &'a str, mut variations: Vec<&'a str>) -> Self {
        if !variations.contains(&name) {
            variations.push(name.clone())
        }
        Extension { name, variations }
    }

    pub fn r#match(&self, extension: &str) -> bool {
        self.variations.contains(&extension)
    }
}

fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide the path");
        process::exit(1);
    }

    let path = &args[1];
    let mut extensions_hash: HashMap<&str, i32> = HashMap::new();
    let mut files_count = 0;

    let extensions = [
        Extension::new("js", vec!["jsx", "cjs", "mjs", "cjsx", "mjsx"]),
        Extension::new("ts", vec!["ts", "tsx", "cts", "mts", "ctsx", "mtsx"]),
    ];

    let result = visit_dirs(&Path::new(path), &mut |file| {
        let path = file.path();
        let current_extension = path.extension().and_then(OsStr::to_str);

        if let Some(current_extension) = current_extension {
            for ext in &extensions {
                if ext.r#match(current_extension) {
                    *extensions_hash.entry(ext.name).or_insert(0) += 1;
                    files_count += 1;
                    break;
                }
            }
        }
    });

    if result.is_err() {
        eprintln!("Error occurred: {}", result.unwrap_err());
        process::exit(1);
    }

    if extensions_hash.is_empty() {
        println!("No js / ts files found in directory");
    } else {
        for (extension, number) in extensions_hash {
            let bar_size = 60.0;
            let percentage = (number as f32 * 100.0) / files_count as f32;
            let percentage_bar = (number as f32 * bar_size) / files_count as f32;
            print!(".{}(x)", extension);
            print!(" | {:#<1$}", "", percentage_bar as usize);
            print!("{:>1$} | ", "", (bar_size - percentage_bar) as usize);
            println!("{} ({:.1}%)", number, percentage);
        }
    }
}
