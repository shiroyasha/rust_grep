use std::os;
use std::io::{File, BufferedReader};
use std::io::fs;
use std::io::fs::PathExtensions;

fn search_in_file(text_to_find: &str, path: Path) -> bool {
    let mut file  = BufferedReader::new(File::open(&path));
    let mut found = false;

    for line in file.lines().filter_map(|result| result.ok()) {
        if line.as_slice().contains(text_to_find) {
            println!("{}", path.display())
            found = true
        }
    }

    found
}

fn search_in_files(text_to_find: &str, paths: fs::Directories) -> bool {
    let mut found = false;

    for path in paths.filter(|path| path.is_file()) {
        if search_in_file(text_to_find, path) { found = true }
    }

    found
}

fn main() {
    let args = os::args();
    let text_to_find = args.get(1).unwrap().as_slice();

    let found = match std::io::fs::walk_dir(&Path::new(".")) {
        Err(why) => {
            println!("! {}", why.kind);
            false
        },
        Ok(paths) => search_in_files(text_to_find, paths)
    };

    std::os::set_exit_status(if found { 0 } else { 1 })
}
