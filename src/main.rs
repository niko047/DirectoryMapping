use std::env;
use std::fs;
use std::io::{self, Write, Result};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dirmap", about = "A basic directory mapping tool.")]
struct Opt {
    /// Set starting path
    #[structopt(short = "p", long = "path", parse(from_os_str), default_value = ".")]
    path: PathBuf,

    /// Set output file
    #[structopt(short = "o", long = "output", parse(from_os_str), default_value = "output.txt")]
    output: PathBuf,

    /// Set max depth
    #[structopt(short = "d", long = "depth", default_value = "10")]
    depth: u8,
}


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

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    let snapshot = MapSnapshot::new(&opt.path, opt.depth, 0)?;

    // Print to a file
    let mut file = fs::File::create(opt.output)?;
    snapshot.print(&mut file)?;

    Ok(())
}



