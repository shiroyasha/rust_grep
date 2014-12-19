use std::os;
use std::comm;
use std::io::{File, BufferedReader, fs};
use std::io::fs::PathExtensions;

fn search_in_file(text_to_find: &str, path: Path) -> bool {
    let mut file  = BufferedReader::new(File::open(&path));
    let mut found = false;

    let mut file_iterator = file.lines().filter_map(|result| result.ok());

    for line in file_iterator {
        if line.as_slice().contains(text_to_find) {
            println!("{}", path.display())
            found = true
        }
    }

    found
}

fn search_in_files(paths: fs::Directories) -> bool {
    let (tx, rx): (Sender<bool>, Receiver<bool>) = comm::channel();

    let mut number_of_files = 0u;
    let mut file_paths_iter = paths.filter(|path| path.is_file());

    for path in file_paths_iter {
        number_of_files += 1;

        let task_tx = tx.clone();
        let path_clone = path.clone();

        spawn(move || {
            let args = os::args();
            let text_to_find = args.get(1).unwrap().as_slice();

            let found = search_in_file(text_to_find, path_clone);
            task_tx.send(found);
        });
    }

    let mut found = false;

    for _ in range(0u, number_of_files) {
        let found_in_file = rx.recv();
        
        if found_in_file { found = true }
    }

    found
}

fn log_error(why: std::io::IoError) {
    println!("! {}", why.desc);
}

fn main() {
    let found = match std::io::fs::walk_dir(&Path::new(".")) {
        Err(why)  => { log_error(why); false },
        Ok(paths) => search_in_files(paths)
    };

    std::os::set_exit_status(if found { 0 } else { 1 })
}
