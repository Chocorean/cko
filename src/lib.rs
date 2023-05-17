use std::{fs::{read_link, symlink_metadata}, os::unix::prelude::MetadataExt};
use walkdir::WalkDir;

/// Recursively inspect a directory and return a list of incorrect files.
pub fn inspect(path: &str) -> Option<Vec<String>> {
    let mut msg = vec![];

    // Skipping possible issues like permissions related
    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        let filetype = entry.file_type();
        // ignoring dicts
        if filetype.is_dir() {
            continue;
        }
        // check destination of soft links
        if filetype.is_symlink() {
            let entry_path = entry.path();
            let dest = read_link(entry_path).unwrap();
            // check if dest is inside 
            println!("{} -> {}", entry_path.display(), dest.display());
        }
        // if file, check number of inode stuff
        if filetype.is_file() {
            let entry_path = entry.path();
            let md = symlink_metadata(entry_path).unwrap();
            let nlink = md.nlink();
            if nlink != 1 {
                msg.push(format!("{} has more ({}) than one hard link", entry_path.display(), nlink));
            }
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
        assert_eq!(inspect("data/pointing_outside"), Some(vec!["data/pointing_outside/link".to_string()]));
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