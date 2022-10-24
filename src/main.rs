use std::cell::Cell;
use std::ffi::OsStr;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::{env, io, process};

struct Extension<'a> {
    name: &'a str,
    variations: Vec<&'a str>,
    files_count: Cell<i32>,
}

impl<'a> Extension<'a> {
    pub fn new(name: &'a str, mut variations: Vec<&'a str>) -> Self {
        if !variations.contains(&name) {
            variations.push(name.clone())
        }
        Extension {
            name,
            variations,
            files_count: Cell::new(0),
        }
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

    let dir = Path::new(&args[1]);

    if !dir.is_dir() {
        eprintln!("The provided path is not a directory");
        process::exit(1);
    }

    let extensions = [
        Extension::new("js", vec!["jsx", "cjs", "mjs", "cjsx", "mjsx"]),
        Extension::new("ts", vec!["tsx", "cts", "mts", "ctsx", "mtsx"]),
    ];
    let mut total_files_count = 0;

    let result = visit_dirs(&dir, &mut |file| {
        let path = file.path();
        let current_extension = path.extension().and_then(OsStr::to_str);

        for ext in &extensions {
            if ext.r#match(current_extension.unwrap_or_default()) {
                ext.files_count.set(ext.files_count.get() + 1);
                total_files_count += 1;
                break;
            }
        }
    });

    if result.is_err() {
        eprintln!("Error occurred: {}", result.unwrap_err());
        process::exit(1);
    }

    if total_files_count == 0 {
        println!("No js / ts files found");
    } else {
        println!("");
        for extension in extensions {
            let current_count = extension.files_count.get();
            let bar_size = 60.0;
            let percentage = (current_count as f32 * 100.0) / total_files_count as f32;
            let percentage_bar = (current_count as f32 * bar_size) / total_files_count as f32;

            print!(".{}(x)", extension.name);
            print!(" | {:▓<1$}", "", percentage_bar as usize);
            print!("{:░>1$} | ", "", (bar_size - percentage_bar) as usize);
            println!("{} ({:.1}%)\n", current_count, percentage);
        }
    }
}
