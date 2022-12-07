#[allow(unused_imports)]
use aoc::prelude::*;

use std::sync::{Arc, Weak, Mutex};

type Input = Vec<TerminalLine>;

enum TerminalLine {
    Cmd(Command),
    LsEntry(LsEntry),
}

enum Command {
    CdIn(String),
    CdUp,
    CdRoot,
    Ls,
}

enum LsEntry {
    File(File),
    Dir(String),
}

fn parser(input_file: &aoc::InputFile<'_>) -> anyhow::Result<Input> {
    let mut input = Vec::new();
    for line in input_file.lines()? {
        let line = line?;
        let tl = if let Some(line) = line.strip_prefix("$ ") {
            let cmd = if line == "ls" {
                Command::Ls
            } else if let Some(whre) = line.strip_prefix("cd ") {
                match whre {
                    ".." => Command::CdUp,
                    "/" => Command::CdRoot,
                    w => Command::CdIn(w.to_owned()),
                }
            } else {
                panic!()
            };
            TerminalLine::Cmd(cmd)
        } else if let Some(name) = line.strip_prefix("dir ") {
            TerminalLine::LsEntry(LsEntry::Dir(name.to_owned()))
        } else {
            let (size, name) = line.split_once(' ').unwrap();
            let size = size.parse().unwrap();
            let name = name.to_owned();
            TerminalLine::LsEntry(LsEntry::File(File { name, size }))
        };
        input.push(tl);
    }
    Ok(input)
}

struct Dir {
    name: String,
    files: Mutex<Vec<File>>,
    dirs: Mutex<HashMap<String, Arc<Dir>>>,
    up: Mutex<Weak<Dir>>,
}

#[derive(Clone)]
struct File {
    name: String,
    size: i64,
}

impl Dir {
    fn new_root() -> Arc<Dir> {
        let d = Dir {
            name: "/".to_owned(),
            files: Mutex::new(Vec::new()),
            dirs: Mutex::new(HashMap::new()),
            up: Mutex::new(Weak::new()),
        };
        let d = Arc::new(d);
        *d.up.lock().unwrap() = Arc::downgrade(&d);
        d
    }
}

fn to_dir(input: &Input) -> Arc<Dir> {
    let root = Dir::new_root();
    let mut cwd = root.clone();

    for line in input {
        match line {
            TerminalLine::Cmd(Command::CdRoot) => cwd = root.clone(),
            TerminalLine::Cmd(Command::CdUp) => {
                let old_cwd = cwd.clone();
                cwd = old_cwd.up.lock().unwrap().upgrade().unwrap();
            }
            TerminalLine::Cmd(Command::CdIn(subdir)) => {
                let old_cwd = cwd.clone();
                cwd = old_cwd.dirs.lock().unwrap().get(subdir).unwrap().clone();
            }
            TerminalLine::Cmd(Command::Ls) => (),
            TerminalLine::LsEntry(LsEntry::File(f)) => {
                cwd.files.lock().unwrap().push(f.clone());
            }
            TerminalLine::LsEntry(LsEntry::Dir(name)) => {
                let new_dir = Arc::new(Dir {
                    name: name.clone(),
                    files: Mutex::new(Vec::new()),
                    dirs: Mutex::new(HashMap::new()),
                    up: Mutex::new(Weak::new()),
                });
                *new_dir.up.lock().unwrap() = Arc::downgrade(&cwd);
                if let Some(d) = cwd.dirs.lock().unwrap().insert(name.clone(), new_dir) {
                    panic!("dup? {}", d.name);
                }
            }
        }
    }

    root
}

fn size_totaller<F>(d: &Arc<Dir>, f: &mut F) -> i64 
where F: for<'a> FnMut(&'a str, i64)
{
    let mut size = 0;
    for file in d.files.lock().unwrap().iter() {
        size += file.size;
    }
    for dir in d.dirs.lock().unwrap().values() {
        size += size_totaller(dir, f);
    }
    f(&d.name, size);
    size
}

fn part_a(input: &Input) -> i64 {
    let root = to_dir(input);
    let mut dirs_under = Vec::new();
    size_totaller(&root, &mut |name, size| {
        if size <= 100000 {
            dirs_under.push((name.to_owned(), size));
        }
    });
    //println!("=> {:?}", dirs_under);
    dirs_under.iter().map(|(_, s)| s).sum()
}

fn part_b(input: &Input) -> i64 {
    let disk_size = 70000000;
    let need_space = 30000000;
    let root = to_dir(input);
    let used = size_totaller(&root, &mut |name, size| {});
    let free = disk_size - used;
    let must_free = need_space - free;
    assert!(must_free > 0);

    let mut dirs = Vec::new();
    size_totaller(&root, &mut |name, size| {
        dirs.push((name.to_owned(), size));
    });
    dirs.sort_by_key(|(_, size)| *size);
    for (_d, sz) in dirs {
        if must_free <= sz {
            return sz;
        }
    }
    panic!()
}

aoc::aoc!(parser, part_a, part_b, Some(95437), Some(24933642));

#[cfg(test)]
mod tests {
    use super::*;
}
