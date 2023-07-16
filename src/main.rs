use std::env;
use std::fs;
use std::io::{self, Write, Result};
use std::path::PathBuf;

enum FsEntry {
    File(PathBuf),
    Dir(MapSnapshot),
}

struct MapSnapshot {
    directory: PathBuf,
    children: Vec<FsEntry>,
    depth: u8,
}

impl MapSnapshot {
    fn new(directory: &PathBuf, max_depth: u8, depth: u8) -> Result<Self> {
        use FsEntry::*;
        if depth > max_depth {
            return Ok(Self {
                directory: directory.clone(),
                children: vec![],
                depth,
            });
        }

        let mut children = Vec::new();

        let mut dirs = Vec::new();
        let mut files = Vec::new();

        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
            } else if path.is_file() {
                files.push(path);
            }
        }

        // Sort directories and files separately
        dirs.sort();
        files.sort();

        for dir in dirs {
            children.push(Dir(Self::new(&dir, max_depth, depth + 1)?));
        }

        for file in files {
            children.push(File(file));
        }

        Ok(Self {
            directory: directory.clone(),
            children,
            depth,
        })
    }

    fn print(&self, writer: &mut impl Write) -> Result<()> {
        use FsEntry::*;
        for _ in 0..self.depth {
            write!(writer, "\t")?;
        }

        writeln!(writer, "{}", self.directory.to_string_lossy())?;

        for child in &self.children {
            match child {
                Dir(dir) => dir.print(writer)?,
                File(file) => {
                    for _ in 0..self.depth+1 {
                        write!(writer, "\t")?;
                    }
                    writeln!(writer, "{}", file.to_string_lossy())?;
                }
            }
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir()?
    };

    let max_depth = if args.len() > 2 {
        args[2].parse().unwrap_or(0)
    } else {
        10 // default max depth
    };

    let snapshot = MapSnapshot::new(&dir, max_depth, 0)?;

    // Print to a file
    let mut file = fs::File::create("output.txt")?;
    snapshot.print(&mut file)?;

    Ok(())
}


