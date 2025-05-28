use std::{cmp::Ordering, fs::DirEntry};

pub fn default_sorter(a: &DirEntry, b: &DirEntry) -> Ordering {
    a.file_name().cmp(&b.file_name())

    // Folders last
    // match (
    //     a.file_type().unwrap().is_dir(),
    //     b.file_type().unwrap().is_dir(),
    // ) {
    //     (true, false) => Ordering::Greater,
    //     (false, true) => Ordering::Less,
    //     _ => a.file_name().cmp(&b.file_name()),
    // }
}
