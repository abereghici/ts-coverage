use std::collections::HashMap;
use std::{env, process};

fn is_js(ext: &str) -> bool {
    ["js", "jsx", "cjs", "mjs", "cjsx", "mjsx"].contains(&ext)
}

fn is_ts(ext: &str) -> bool {
    ["ts", "tsx", "cts", "mts", "ctsx", "mtsx"].contains(&ext)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide the path");
        process::exit(1);
    }

    let path = &args[1];

    let files = vec![
        "index.js",
        "index.ts",
        "index.jsx",
        "index.tsx",
        "index.cjs",
        "index.cts",
        "index.mjs",
        "index.mts",
        "index.cjsx",
        "index.ctsx",
        "index.mjsx",
        "index.mtsx",
    ];

    let mut extensions_hash: HashMap<&str, i32> = HashMap::new();

    for file in files {
        let ext = file.split(".").last();

        if let Some(ext) = ext {
            let mut increase_count = |parent: &'static str| {
                *extensions_hash.entry(parent).or_insert(0) += 1;
            };

            if is_js(ext) {
                increase_count("js")
            }

            if is_ts(ext) {
                increase_count("ts")
            }
        }
    }

    for (extension, number) in extensions_hash {
        println!("{}, {}", extension, number);
    }

    dbg!(path);
}
