use std::collections::{BTreeMap};
use std::fmt::{Display, Formatter};
use std::mem;
use anyhow::bail;
use pest::iterators::{Pairs};
use pest::Parser;
use pest_derive::Parser;
use once_cell::sync::OnceCell;

// *************************************************************************************************

#[derive(Parser)]
#[grammar = "day7/commands.pest"]
struct CmdParser;


struct LsLine<'a> {
    filename: &'a str,
    /// if `size == None`, this entry is a (sub)-directory
    size: Option<usize>,
}

struct LsIter<'a> {
    parse: Pairs<'a, Rule>,
}

enum Cmd<'a> {
    Cd(&'a str),
    Ls(LsIter<'a>),
}

// *************************************************************************************************

impl<'a> Iterator for LsIter<'a> {
    type Item = LsLine<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let ls_line = self.parse.next()?;
        Some(match ls_line.as_rule() {
            Rule::ls_dir => {
                let filename = ls_line.into_inner().next().unwrap().as_str();
                LsLine {
                    filename,
                    size: None,
                }
            }
            Rule::ls_file => {
                let mut ls_line = ls_line.into_inner();
                let size = Some(ls_line.next().unwrap().as_str().parse().unwrap());
                let filename = ls_line.next().unwrap().as_str();
                LsLine {
                    filename,
                    size,
                }
            }
            _ => {
                panic!("Expected `ls_dir` or `ls_file`, found {ls_line:?}")
            }
        })
    }
}


impl<'a> Cmd<'a> {
    fn parse(input: &'a str) -> impl Iterator<Item=Cmd<'a>> {
        CmdParser::parse(Rule::file, input).expect("Failed to parse input")
            .next().unwrap()
            .into_inner()
            .filter(|cmd| cmd.as_rule() != Rule::EOI)
            .map(|cmd| {
                debug_assert_eq!(cmd.as_rule(), Rule::cmd);
                let cmd = cmd.into_inner().next().unwrap();

                let rule = cmd.as_rule();
                match rule {
                    Rule::cd_cmd => {
                        let dirname = cmd.into_inner().next().unwrap().as_str();
                        Cmd::Cd(dirname)
                    }
                    Rule::ls_cmd => {
                        Cmd::Ls(LsIter {
                            parse: cmd.into_inner()
                        })
                    }
                    other => {
                        panic!("Unexpected rule ({other:?}) in {cmd:?}")
                    }
                }
            })
    }
}

// *************************************************************************************************


enum DirEntry<'a> {
    File {
        size: usize
    },
    Dir(Box<Directory<'a>>),
}

#[derive(Default)]
struct Directory<'a> {
    children: BTreeMap<&'a str, DirEntry<'a>>,
    size: OnceCell<usize>,
}

struct Cursor<'a> {
    parents: Vec<(Box<Directory<'a>>, &'a str)>,
    current: Box<Directory<'a>>,
}


// *************************************************************************************************

impl<'a> Cursor<'a> {
    /// Tries to go up a single directory
    /// Returns `false` if the current dir is already the root
    fn go_up(&mut self) -> bool {
        if let Some((mut dir, name)) = self.parents.pop() {
            self.current.size.take();
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
    fn enter(&mut self, dirname: &'a str) -> anyhow::Result<()> {
        match dirname {
            ".." => {
                self.go_up();
            }
            "/" => {
                self.go_to_root();
            }
            _ => {
                let mut child =
                    match self.current.children.remove(dirname) {
                        Some(DirEntry::Dir(dir)) => {
                            dir
                        }
                        None => {
                            Box::default()
                        }

                        Some(DirEntry::File { size }) => {
                            self.current.children.insert(dirname, DirEntry::File { size });
                            bail!("{dirname} exists, but is a file, expected a directory")
                        }
                    };
                mem::swap(&mut child, &mut self.current);
                self.parents.push((child, dirname));
            }
        }
        Ok(())
    }

    fn into_dir(mut self) -> Box<Directory<'a>> {
        self.go_to_root();
        self.current
    }
}

impl<'a> From<Cursor<'a>> for Box<Directory<'a>> {
    fn from(value: Cursor<'a>) -> Self {
        value.into_dir()
    }
}

// *************************************************************************************************

impl<'a> LsLine<'a> {
    fn dir_entry<'b>(&'b self) -> DirEntry<'a> {
        match self.size {
            Some(size) => {
                DirEntry::File { size }
            }
            None => {
                DirEntry::Dir(Box::default())
            }
        }
    }
}

impl<'a> Cursor<'a> {
    fn new() -> Self {
        Self {
            parents: vec![],
            current: Box::default(),
        }
    }

    fn build(input: &'a str) -> Self {
        let mut cursor = Self::new();
        for cmd in Cmd::parse(input) {
            match cmd {
                Cmd::Cd(dir) => {
                    cursor.enter(dir).unwrap();
                }
                Cmd::Ls(entries) => {
                    for entry in entries {
                        cursor.current.children.insert(
                            entry.filename,
                            entry.dir_entry(),
                        );
                    }
                }
            }
        }
        cursor
    }
}

// *************************************************************************************************

struct DirectoryDisplay<'a, 'b> {
    dir: &'a Directory<'b>,
    indent: usize,
}

impl Directory<'_> {
    fn display(&self, indent: usize) -> impl Display + '_ {
        DirectoryDisplay {
            dir: self,
            indent,
        }
    }
}

impl Display for DirectoryDisplay<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let indent = " ".repeat(self.indent);
        for (name, entry) in &self.dir.children {
            write!(f, "{indent}- {name} ")?;
            match entry {
                DirEntry::File { size } => {
                    writeln!(f, "(file, size={size})")?;
                }
                DirEntry::Dir(dir) => {
                    write!(f, "(dir")?;
                    if let Some(size) = dir.size.get() {
                        write!(f, ", size={size}")?;
                    }
                    write!(f, ")\n{}", dir.display(self.indent + 2))?;
                }
            }
        }
        Ok(())
    }
}

impl Display for Directory<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(dir")?;
        if let Some(size) = self.size.get() {
            write!(f, ", size={size}")?;
        }
        write!(f, ")\n{}", self.display(2))?;
        Ok(())
    }
}

// *************************************************************************************************

impl<'a, 'b> DirEntry<'a> {
    fn as_dir(&'b self) -> Option<&'b Directory<'a>> {
        if let DirEntry::Dir(dir) = self {
            Some(dir)
        } else {
            None
        }
    }
}

impl<'a> Directory<'a> {
    fn size(&self) -> usize {
        *self.size.get_or_init(|| {
            self.children.values()
                .map(|entry| {
                    match entry {
                        DirEntry::File { size } => *size,
                        DirEntry::Dir(dir) => dir.size(),
                    }
                })
                .sum()
        })
    }

    /// Flatten the directory structure, returning an iterator over all (nested) sub directories
    fn sub_dirs_flatten<'b>(&'b self) -> Box<dyn Iterator<Item=(&'a str, &'b Directory<'a>)> + 'b> {
        let iter =
            self.children.iter()
                .flat_map(|(k, v)| {
                    Some((k, v.as_dir()?))
                })
                .flat_map(|(name, dir)| {
                    [(*name, dir)].into_iter().chain(
                        dir.sub_dirs_flatten()
                    )
                });
        Box::new(iter)
    }

    /// Sum the size of all (nested) sub-directories (strictly) smaller than `threshold`
    fn sum_sizes(&self, threshold: usize) -> usize {
        self.sub_dirs_flatten()
            .map(|(_, d)| d.size())
            .filter(|&s| s < threshold)
            .sum()
    }

    /// Find the size of the smallest (nested) sub-directory that is at least as big as `threshold`
    fn find_smallest<'b>(&'b self, threshold: usize) -> usize {
        self.sub_dirs_flatten()
            .map(|(_, d)| d.size())
            .filter(|&s| s >= threshold)
            .min()
            .unwrap_or(0)
    }
}

// *************************************************************************************************

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let d = Cursor::build(input).into_dir();
    println!("{d}");

    d.sum_sizes(100_000)
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let d = Cursor::build(input).into_dir();

    let used = d.size();
    let free = 70_000_000 - used;
    let needed = 30_000_000 - free;

    println!("{d}");
    println!("need {needed}");

    for (name, sub) in d.sub_dirs_flatten() {
        println!("{name}: {}", sub.size())
    }

    d.find_smallest(needed)
}

// *************************************************************************************************


pub fn main() {
    println!("{}", part2(include_str!("test.txt")))
}