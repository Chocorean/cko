use std::env::current_dir;
use std::path::PathBuf;
use std::{fs::{canonicalize, read_link, symlink_metadata}, os::unix::prelude::MetadataExt};
use walkdir::WalkDir;


/// Recursively inspect a directory and return a list of incorrect files.
pub fn inspect(path: &str) -> Option<Vec<String>> {
    let current_path = if ! path.starts_with("/") {
        canonicalize(current_dir().unwrap().join(path)).unwrap()
    } else {
        let mut p = PathBuf::new();
        p.push(path);
        p
    };
    let mut msg = vec![];

    // Skipping possible issues like permissions related
    for entry in WalkDir::new(&current_path).into_iter().filter_map(|e| e.ok()) {
        println!("debug: inspecting {}", entry.file_name().to_str().unwrap());
        let filetype = entry.file_type();
        // ignoring dicts
        if filetype.is_dir() {
            continue;
        }
        // check destination of soft links
        else if filetype.is_symlink() {
            let entry_path = entry.path();
            let dest_path = read_link(entry_path).unwrap();
            println!("\tcurrent {}\n\tentry {}\n\tdest {}", current_path.display(), entry_path.display(), dest_path.display());

            // make dest_path absolute and canocical
            let mut full_dest_path = if ! dest_path.to_str().unwrap().starts_with("/") {
                entry_path.parent().unwrap().join(dest_path.clone())
            } else {
                dest_path.clone()
            };
            println!("debug: symlink {}", full_dest_path.display());
            full_dest_path = canonicalize(full_dest_path).unwrap();
            println!("debug: caconical symlink {}", full_dest_path.display());

            // check if dest is inside the dir we are exploring
            if ! full_dest_path.starts_with(&current_path) {
                msg.push(format!("{} is pointing outsite to {}", entry_path.display(), dest_path.display()));
                continue;
            }

            // then, check if the file actually exists

            // then, check if it does not loop
        }
        // if file, check number of inode stuff
        else if filetype.is_file() {
            let entry_path = entry.path();
            let md = symlink_metadata(entry_path).unwrap();
            let nlink = md.nlink();
            if nlink != 1 {
                msg.push(format!("{} has more ({}) than one hard link", entry_path.display(), nlink));
            }
        }
        // special files I guess ?
        else {
            msg.push(format!("WARING: unknown file type: {}", entry.path().display()));
        }
    }

    if msg.len() == 0 {
        None
    } else {
        Some(msg)
    }
}

#[cfg(test)]
mod test {
    use crate::inspect;

    #[test]
    fn test_inspect_good() {
        assert_eq!(inspect("data/valid"), None);
    }

    #[test]
    fn test_inspect_pointing_outside() {
        assert_eq!(inspect("data/pointing_outside"), Some(vec!["data/pointing_outside/link is pointing outside to ../valid_dir".to_string()]));
    }

    #[test]
    fn test_inspect_loop() {
        assert_eq!(inspect("data/loop"), Some(vec!["data/loop/link0".to_string()]));
    }

    #[test]
    fn test_inspect_hardlink() {
        assert_eq!(inspect("data/hardlink").unwrap().sort(), vec!["data/hardlink/file has more (2) than one hard link".to_string(), "data/hardlink/hardlink has more (2) than one hard link".to_string()].sort());
    }

    #[test]
    fn test_inspect_broken_softlink() {
        assert_eq!(inspect("data/broken_softlink"), Some(vec!["data/broken_softlink/link".to_string()]));
    }

    #[test]
    fn test_inspect_absolute_softlink() {
        assert_eq!(inspect("data/absolute_softlink"), Some(vec!["data/absolute_softlink/link".to_string()]));
    }

    #[test]
    fn test_inspect_many_issues() {
        assert_eq!(inspect("data/many_issues"), Some(vec!["data/many_issues/absolute".to_string(), "data/many_issues/broken".to_string(), "data/many_issues/loop".to_string()]));
    }
}