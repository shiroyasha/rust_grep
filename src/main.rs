use std::os;
use std::io::File;
use std::io::BufferedReader;
use std::io::fs;
use std::io::fs::PathExtensions;

fn search_string(text_to_find: &str, path: Path) {

    let mut file = BufferedReader::new(File::open(&path));

    for line in file.lines().filter_map(|result| result.ok()) {
        if line.as_slice().contains(text_to_find) {
            println!("{}", path.display());
        }
    }
}

fn search_in_files(text_to_find: &str, mut paths: fs::Directories) {
    for path in paths {
        if path.is_file() {
            search_string(text_to_find, path);
        }
    }
}

fn main() {
    let args = os::args();
    let text_to_find = args.get(1).unwrap().as_slice();
   
    match std::io::fs::walk_dir(&Path::new(".")) {
        Err(why) => println!("! {}", why.kind),
        Ok(paths) => search_in_files(text_to_find, paths)
    }
}
