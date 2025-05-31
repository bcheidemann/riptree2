use std::path::Path;

pub fn assert_snapshot(path: impl AsRef<Path>, actual: impl AsRef<str>) {
    let update = !path.as_ref().exists() || std::env::var("UPDATE").is_ok();

    if update {
        std::fs::write(&path, actual.as_ref()).expect(&format!(
            "Failed to update snapshot file at {}",
            path.as_ref().to_string_lossy()
        ));
        return;
    }

    let expected = std::fs::read_to_string(&path).expect(&format!(
        "Failed to read snapshot file at {}",
        path.as_ref().to_string_lossy()
    ));

    pretty_assertions::assert_eq!(expected, actual.as_ref());
}
