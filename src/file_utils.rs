use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind, Write};
use std::path::{Component, Path, PathBuf};

/// Writes all bytes to a file.
pub fn file_write_all_bytes(path: PathBuf, bytes: &[u8], overwrite: bool) -> io::Result<usize> {
    if path.exists() && !overwrite {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            "The specified file already exists.",
        ));
    }
    let mut file = File::create(path)?;
    file.set_len(0)?;
    file.write(bytes)
}

/// Returns a relative path from one path to another.
pub(crate) fn make_relative_path<P1: AsRef<Path>, P2: AsRef<Path>>(
    root: P1,
    current: P2,
) -> PathBuf {
    let mut result = PathBuf::new();
    let root_components = root.as_ref().components().collect::<Vec<Component<'_>>>();
    let current_components = current.as_ref().components().collect::<Vec<_>>();
    for i in 0..current_components.len() {
        let current_path_component: Component<'_> = current_components[i];
        if i < root_components.len() {
            let other: Component<'_> = root_components[i];
            if other != current_path_component {
                break;
            }
        } else {
            result.push(current_path_component);
        }
    }
    result
}
