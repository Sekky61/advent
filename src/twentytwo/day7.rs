use crate::util::DaySolution;

use nom::{
    bytes::complete::{tag, take_till, take_while},
    character::complete::{newline, space1},
    combinator::opt,
    multi::many0,
    sequence::tuple,
    Err, IResult,
};
use slab_tree::{NodeId, Tree, TreeBuilder};

// Types

#[derive(PartialEq, Eq, Debug)]
pub enum CdTarget {
    Root,
    Parent,
    Child(String),
}

impl CdTarget {
    fn parse(path: &str) -> Result<CdTarget, &'static str> {
        let target = match path {
            "/" => CdTarget::Root,
            ".." => CdTarget::Parent,
            path => {
                if path.chars().any(|c| c == '/' || c == ' ') {
                    return Err("Path contains / or a space");
                }
                CdTarget::Child(path.to_string())
            }
        };

        Ok(target)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum FileSystemEntryKind {
    File { size: usize },
    Directory,
}

#[derive(PartialEq, Eq, Debug)]
pub struct FileSystemEntry {
    name: String,
    kind: FileSystemEntryKind,
}

impl FileSystemEntry {
    pub fn new(name: impl Into<String>, kind: FileSystemEntryKind) -> Self {
        Self {
            name: name.into(),
            kind,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Command {
    Cd(CdTarget),
    Ls { entries: Vec<FileSystemEntry> },
}

#[derive(PartialEq, Eq, Debug)]
pub struct CommandHistory {
    commands: Vec<Command>,
}

// Parsing

fn take_word(input: &str) -> IResult<&str, &str> {
    take_till(|c: char| c.is_whitespace())(input)
}

// Parse one line of the output of ls
fn ln_line(input: &str) -> IResult<&str, FileSystemEntry> {
    let mut dir_parser = tuple((tag("dir"), space1, take_word, opt(newline)));
    let mut file_parser = tuple((
        take_while(|c: char| c.is_numeric()),
        space1,
        take_word,
        opt(newline),
    ));

    if let Ok((input, (_, _, name, _))) = dir_parser(input) {
        Ok((
            input,
            FileSystemEntry::new(name, FileSystemEntryKind::Directory),
        ))
    } else if let Ok((input, (size, _, name, _))) = file_parser(input) {
        Ok((
            input,
            FileSystemEntry::new(
                name,
                FileSystemEntryKind::File {
                    size: size.parse().unwrap(),
                },
            ),
        ))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

fn command(input: &str) -> IResult<&str, Command> {
    // Consume the $ and the spaces after it
    let (input, _) = tuple((tag("$"), space1))(input)?;
    // Consume the command name
    let (input, command_name) = take_word(input)?;

    let (input, command) = match command_name {
        "cd" => {
            // Consume spaces and then a path
            let (input, _) = space1(input)?;
            let (input, path) = take_word(input)?;
            // Consume until newline
            let (input, _) = newline(input)?;
            let target = CdTarget::parse(path).unwrap();
            (input, Command::Cd(target))
        }
        "ls" => {
            // Consume newline
            let (input, _) = newline(input)?;
            // Consume lines until a newline with a $ at the start
            let (input, entries) = many0(ln_line)(input)?;

            (input, Command::Ls { entries })
        }
        _ => {
            return Err(Err::Failure(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )))
        }
    };

    Ok((input, command))
}

impl CommandHistory {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let mut history_parser = many0(command);

        let (input_str, commands) = history_parser(input)?;

        if !input_str.is_empty() {
            panic!("Failed to parse input");
        }

        Ok((input_str, Self { commands }))
    }
}

pub struct File {
    name: String,
    size: usize,
}

pub struct Folder {
    name: String,
    files: Vec<File>,
}

impl Folder {
    pub fn get_size(&self) -> usize {
        self.files.iter().map(|f| f.size).sum()
    }
}

pub struct FileSystem {
    // Root is at index 0
    tree: Tree<Folder>,
}

impl FileSystem {
    pub fn build_from_command_history(
        history: &CommandHistory,
    ) -> Result<FileSystem, &'static str> {
        let root_dir = Folder {
            name: "/".to_string(),
            files: Vec::new(),
        };
        let mut tree = TreeBuilder::new().with_root(root_dir).build();
        let mut current_dir = tree.root_id().expect("Must exist");

        for command in &history.commands {
            let mut dir = match tree.get_mut(current_dir) {
                Some(dir) => dir,
                None => {
                    return Err("Invalid current directory");
                }
            };

            match command {
                Command::Cd(target) => match target {
                    CdTarget::Root => {
                        current_dir = tree.root_id().expect("Must exist");
                    }
                    CdTarget::Parent => match dir.parent() {
                        Some(parent) => {
                            current_dir = parent.node_id();
                        }
                        None => {
                            return Err("Tried to cd to parent of root");
                        }
                    },
                    CdTarget::Child(name) => {
                        let dir_ref = tree.get(current_dir).unwrap();
                        let child = dir_ref
                            .children()
                            .find(|child| child.data().name == *name)
                            .ok_or("Subdirectory not found")?;

                        current_dir = child.node_id();
                    }
                },
                Command::Ls { entries } => {
                    // For each entry, add it to the current directory
                    entries.iter().for_each(|entry| match entry.kind {
                        FileSystemEntryKind::Directory => {
                            let new_dir = Folder {
                                name: entry.name.clone(),
                                files: Vec::new(),
                            };
                            let mut dir = tree.get_mut(current_dir).unwrap();
                            dir.append(new_dir);
                        }
                        FileSystemEntryKind::File { size } => {
                            let new_file = File {
                                name: entry.name.clone(),
                                size,
                            };
                            let mut dir = tree.get_mut(current_dir).unwrap();
                            dir.data().files.push(new_file);
                        }
                    });
                }
            }
        }

        Ok(Self { tree })
    }

    pub fn get_folder_size_recursive(&self, folder_id: NodeId) -> usize {
        let mut size = 0;
        let folder = self.tree.get(folder_id).unwrap();

        for node in folder.traverse_pre_order() {
            size += node.data().get_size();
        }

        size
    }

    pub fn get_folders_at_most(&self, size: usize) -> Vec<NodeId> {
        let mut folders = Vec::new();
        let root = self.tree.root().unwrap();

        for node in root.traverse_pre_order() {
            if self.get_folder_size_recursive(node.node_id()) <= size {
                folders.push(node.node_id());
            }
        }

        folders
    }
}

pub struct Solution {
    year: u64,
    day: u64,
}

impl DaySolution for Solution {
    fn new(year: u64, day: u64) -> Self {
        Self { year, day }
    }

    fn part1_solution(&self) -> usize {
        let input = self.get_input().unwrap();

        let (_, history) = CommandHistory::parse(&input).unwrap();
        let fs = FileSystem::build_from_command_history(&history).unwrap();

        let max_size = 100000;
        let small_folders = fs.get_folders_at_most(max_size);

        small_folders
            .iter()
            .map(|folder_id| fs.get_folder_size_recursive(*folder_id))
            .sum::<usize>()
    }

    fn part2_solution(&self) -> usize {
        let lines = self.get_input_lines().unwrap();
        0
    }

    fn get_year(&self) -> u64 {
        self.year
    }

    fn get_day(&self) -> u64 {
        self.day
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ls() {
        let input = r#"$ ls
dir a
18 filename
"#;

        let (input, command) = command(input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            command,
            Command::Ls {
                entries: vec![
                    FileSystemEntry {
                        kind: FileSystemEntryKind::Directory,
                        name: "a".to_string()
                    },
                    FileSystemEntry {
                        kind: FileSystemEntryKind::File { size: 18 },
                        name: "filename".to_string()
                    },
                ]
            }
        );
    }

    #[test]
    fn test_cd() {
        // Newline is required after cd
        let input = r#"$ cd ..
"#;

        let (input, command) = command(input).unwrap();

        assert_eq!(input, "");
        assert_eq!(command, Command::Cd(CdTarget::Parent));
    }

    #[test]
    fn test_commands() {
        let input = r#"$ cd /
$ ls
dir a
"#;

        let (input, history) = CommandHistory::parse(input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            history.commands,
            vec![
                Command::Cd(CdTarget::Root),
                Command::Ls {
                    entries: vec![FileSystemEntry {
                        kind: FileSystemEntryKind::Directory,
                        name: "a".to_string()
                    },]
                }
            ]
        );
    }
}
