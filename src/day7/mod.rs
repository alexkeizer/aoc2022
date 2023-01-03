use std::collections::HashMap;
use std::mem;
use anyhow::bail;
use pest::Parser;
use pest_derive::Parser;

// *************************************************************************************************

#[derive(Parser)]
#[grammar = "day7/commands.pest"]
struct CmdParser;


struct DirEntryLine<'a> {
    filename: &'a str,
    /// if `size == None`, this entry is a (sub)-directory
    size: Option<usize>,
}

enum Cmd<'a> {
    Cd(String),
    Ls(DirEntryLine<'a>),
}

// *************************************************************************************************


impl<'a> Cmd<'a> {
    fn parse(input: &'a str) -> impl Iterator<Item=Cmd<'a>> {
        todo!()
    }
}

// *************************************************************************************************


enum DirEntry<'a> {
    File{
        size: usize
    },
    Dir(Box<Directory<'a>>)
}

#[derive(Default)]
struct Directory<'a> {
    children: HashMap<&'a str, DirEntry<'a>>,
}

struct Cursor<'a> {
    parents: Vec<(Box<Directory<'a>>, &'a str)>,
    current: Box<Directory<'a>>
}


// *************************************************************************************************

impl<'a> Cursor<'a> {
    /// Tries to go up a single directory
    /// Returns `false` if the current dir is already the root
    fn go_up(&mut self) -> bool {
        if let Some((mut dir, name)) = self.parents.pop() {
            mem::swap(&mut dir, &mut self.current);
            self.current.children.insert(name, DirEntry::Dir(dir));
            true
        } else {
            false
        }
    }

    /// Go to the root directory
    fn go_to_root(&mut self) {
        while self.go_up() {}
    }

    /// Enter the dir with given name
    /// Supports going up a level with `..`
    fn enter(&mut self, dirname: &str) -> anyhow::Result<()> {
        match dirname {
            ".." => self.go_up(),
            "/" => self.go_to_root(),
            _ => {
                let mut child =
                    match self.current.children.remove(dirname) {
                        Some(DirEntry::Dir(dir)) => {
                            dir
                        },
                        None => {
                            Box::default()
                        }

                        Some(entry@DirEntry::File(_)) => {
                            self.current.children.insert(dirname, entry);
                            bail!("{dirname} exists, but is a file, expected a directory")
                        }
                    };
                mem::swap(&mut child, &mut self.current);
                self.parents.push((child, dirname));
            }
        }
        Ok(())
    }
}

// *************************************************************************************************


pub fn main() {}