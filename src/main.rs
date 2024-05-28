use std::ffi::OsStr;

use std::fs::Permissions;
use std::path::PathBuf;
use std::process::exit;

use std::time::SystemTime;

const PADDING: usize = 15;

enum DirOrFile {
    Dir,
    File,
}
impl std::fmt::Display for DirOrFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dir => write!(f, "Directory"),
            Self::File => write!(f, "File"),
        }
    }
}

struct PrettyFile<'a> {
    filename: Option<&'a OsStr>,
    absolute_path: PathBuf,
    dir_or_file: DirOrFile,
    is_symlink: bool,
    last_modified: Option<SystemTime>,
    last_accessed: Option<SystemTime>,
    created: Option<SystemTime>,
    permissions: Permissions,
    sha256: Option<String>, // TODO: size
                          // sha256: String
}

fn pad(s: &str, len: usize) -> String {
    if s.len() > len {
        return s.to_owned();
    }
    let mut padding = "".to_owned();
    for _ in 0..len - s.len() {
        padding = padding.to_owned() + " ";
    }

    padding + s
}

impl<'a> std::fmt::Display for PrettyFile<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "====================================================================\n"
        )
        .unwrap();

        if let Some(name) = self.filename {
            let tag = pad("Name", PADDING);
            writeln!(f, "{tag}: {}\n", name.to_string_lossy()).unwrap();
        }

        let tag = pad("Absolute path", PADDING);
        writeln!(f, "{tag}: {}\n", self.absolute_path.to_string_lossy()).unwrap();

        let tag = pad("Type", PADDING);
        writeln!(f, "{tag}: {}\n", self.dir_or_file).unwrap();

        let tag = pad("Symlink", PADDING);
        writeln!(f, "{tag}: {}\n", self.is_symlink).unwrap();

        if let Some(sha256) = self.sha256.as_ref() {
            let tag = pad("sha256", PADDING);
            writeln!(f, "{tag}: {}\n", sha256).unwrap();
        }

        if let Some(created) = self.created.as_ref() {
            let tag = pad("Created", PADDING);
            writeln!(f, "{tag}: {:?}\n", created).unwrap();
        }
        if let Some(last_modified) = self.last_modified.as_ref() {
            let tag = pad("Last modified", PADDING);
            writeln!(f, "{tag}: {:?}\n", last_modified).unwrap();
        }
        if let Some(last_accessed) = self.last_accessed.as_ref() {
            let tag = pad("Last accessed", PADDING);
            writeln!(f, "{tag}: {:?}\n", last_accessed).unwrap();
        }

        let tag = pad("Permissions", PADDING);
        writeln!(f, "{tag}: {:?}\n", self.permissions).unwrap();

        writeln!(
            f,
            "===================================================================="
        )
        .unwrap();
        Ok(())
    }
}
fn main() {
    let mut args = std::env::args();
    if args.len() < 2 {
        eprintln!("Missing expected argument: file");
        exit(404);
    }
    let filepath = {
        args.next();
        PathBuf::from(args.next().unwrap())
    };
    let args = args.collect::<Vec<String>>();
    let metadata = std::fs::metadata(&filepath)
        .inspect_err(|e| {
            eprintln!("Failed to read file metadata: {e:?}");
            exit(500);
        })
        .unwrap();
    let last_accessed = metadata.accessed().ok();
    let last_modified = metadata.modified().ok();
    let created = metadata.created().ok();

    let cur_path = std::env::current_dir()
        .inspect_err(|e| {
            eprintln!("Error getting cwd: {e:?}");
            exit(500);
        })
        .unwrap();

    let permissions = metadata.permissions();
    let is_symlink = metadata.is_symlink();

    let sha256 = if args
        .iter()
        .any(|v| v == "-h" || v == "--sha256")
    {
        // TODO: calculate sha256
        Some("[unimplemented]".to_owned())
    } else {
        None
    };

    let pretty_file = PrettyFile {
        absolute_path: cur_path.join(filepath.as_os_str()).canonicalize().unwrap(),
        filename: filepath.file_name(),
        dir_or_file: if filepath.is_dir() {
            DirOrFile::Dir
        } else {
            DirOrFile::File
        },
        sha256,
        is_symlink,
        last_modified,
        last_accessed,
        created,
        permissions,
    };
    println!("{}", pretty_file);
}
