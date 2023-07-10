use crate::util::DaySolution;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{digit1, newline, space1},
    combinator::{all_consuming, map, opt},
    multi::many0,
    sequence::tuple,
    IResult,
};
use slab_tree::{NodeId, Tree, TreeBuilder};

// Types and parsing

#[derive(PartialEq, Eq, Debug)]
pub enum CdTarget {
    Root,
    Parent,
    Child(String),
}

impl CdTarget {
    fn parse(input: &str) -> IResult<&str, CdTarget> {
        let (input, path) = take_word(input)?;
        match path {
            "/" => Ok((input, CdTarget::Root)),
            ".." => Ok((input, CdTarget::Parent)),
            _ => Ok((input, CdTarget::Child(path.into()))),
        }
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

impl Command {
    /// Parse a cd command
    fn parse_cd(input: &str) -> IResult<&str, Command> {
        let (input, (_, _, _, _, target, _)) = tuple((
            tag("$"),
            space1,
            tag("cd"),
            space1,
            CdTarget::parse,
            opt(newline),
        ))(input)?;

        Ok((input, Command::Cd(target)))
    }

    /// Parse an ls command
    fn parse_ls(input: &str) -> IResult<&str, Command> {
        let (input, (_, _, _, _, entries)) = tuple((
            tag("$"),
            space1,
            tag("ls"),
            opt(newline),
            many0(Command::ln_line),
        ))(input)?;

        Ok((input, Command::Ls { entries }))
    }

    /// Parse a directory entry from the output of ls
    fn parse_dir(input: &str) -> IResult<&str, FileSystemEntry> {
        map(
            tuple((tag("dir"), space1, take_word, opt(newline))),
            |(_, _, name, _)| FileSystemEntry::new(name, FileSystemEntryKind::Directory),
        )(input)
    }

    /// Parse a file entry from the output of ls
    fn parse_file(input: &str) -> IResult<&str, FileSystemEntry> {
        map(
            tuple((digit1::<&str, _>, space1, take_word, opt(newline))),
            |(size, _, name, _)| {
                FileSystemEntry::new(
                    name,
                    FileSystemEntryKind::File {
                        size: size.parse().unwrap(),
                    },
                )
            },
        )(input)
    }

    /// Parse one line of the output of ls
    fn ln_line(input: &str) -> IResult<&str, FileSystemEntry> {
        alt((Command::parse_dir, Command::parse_file))(input)
    }

    /// Parse a command from a string
    fn parse(input: &str) -> IResult<&str, Command> {
        alt((Command::parse_cd, Command::parse_ls))(input)
    }
}

/// Take a word from the input
fn take_word(input: &str) -> IResult<&str, &str> {
    take_till(|c: char| c.is_whitespace())(input)
}

#[derive(PartialEq, Eq, Debug)]
pub struct CommandHistory {
    commands: Vec<Command>,
}

impl CommandHistory {
    /// Parse a command history from a string (challange input, one command per line)
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(all_consuming(many0(Command::parse)), |commands| Self {
            commands,
        })(input)
    }
}

// Solution

pub struct File {
    name: String,
    size: usize,
}

pub struct Folder {
    name: String,
    files: Vec<File>,
}

impl Folder {
    /// Get the size of all files directly in this folder
    pub fn get_size(&self) -> usize {
        self.files.iter().map(|f| f.size).sum()
    }
}

pub struct FileSystem {
    // Root is at index 0
    current_dir: NodeId,
    tree: Tree<Folder>,
}

impl FileSystem {
    /// Iterate over all folders in the file system
    pub fn folders(&self) -> impl Iterator<Item = NodeId> + '_ {
        let root = self.tree.root().expect("Must exist");
        root.traverse_pre_order().map(|n| n.node_id())
    }

    pub fn add_file(mut self, name: &str, size: usize) -> Self {
        // Get the current directory
        let mut current_dir = self.tree.get_mut(self.current_dir).expect("Must exist");

        // Add the new file
        current_dir.data().files.push(File {
            name: name.into(),
            size,
        });

        self
    }

    /// Add a new subdirectory to the current directory
    pub fn add_directory(mut self, name: &str) -> Self {
        // Get the current directory
        let mut current_dir = self.tree.get_mut(self.current_dir).expect("Must exist");

        // Add the new directory
        current_dir.append(Folder {
            name: name.into(),
            files: Vec::new(),
        });

        self
    }

    /// Execute a command, mutating the model of the file system
    pub fn exec_command(self, command: &Command) -> Self {
        match command {
            Command::Cd(target) => {
                let current_dir = match target {
                    CdTarget::Root => self.tree.root_id().expect("Must exist"),
                    CdTarget::Parent => self
                        .tree
                        .get(self.current_dir)
                        .expect("Must exist")
                        .parent()
                        .expect("Must exist")
                        .node_id(),
                    CdTarget::Child(name) => self
                        .tree
                        .get(self.current_dir)
                        .expect("Must exist")
                        .children()
                        .find(|n| n.data().name == *name)
                        .expect("Must exist")
                        .node_id(),
                };

                Self {
                    current_dir,
                    tree: self.tree,
                }
            }
            Command::Ls { entries } => {
                // For each entry, add it to the current directory
                entries.iter().fold(self, |fs, entry| match entry.kind {
                    FileSystemEntryKind::Directory => fs.add_directory(&entry.name),
                    FileSystemEntryKind::File { size } => fs.add_file(&entry.name, size),
                })
            }
        }
    }

    /// Reconstruction of the file system from the command history
    pub fn build_from_command_history(history: &CommandHistory) -> FileSystem {
        let root_dir = Folder {
            name: "/".to_string(),
            files: Vec::new(),
        };
        let tree = TreeBuilder::new().with_root(root_dir).build();
        let current_dir = tree.root_id().expect("Must exist");

        let fs = Self { current_dir, tree };

        history.commands.iter().fold(fs, FileSystem::exec_command)
    }

    /// Get the size of a folder and all its subfolders
    pub fn get_folder_size_recursive(&self, folder_id: NodeId) -> usize {
        let folder = self.tree.get(folder_id).unwrap();

        folder
            .traverse_pre_order()
            .fold(0, |acc, folder_node| acc + folder_node.data().get_size())
    }

    /// Get all folders that are at most the given size (including subfolders)
    pub fn get_folders_at_most(&self, size: usize) -> impl Iterator<Item = NodeId> + '_ {
        self.folders()
            .filter(move |&folder_id| self.get_folder_size_recursive(folder_id) <= size)
    }

    /// Get the smallest folder that is at least the given size (including subfolders)
    pub fn get_smallest_folder_larger_than(&self, size: usize) -> NodeId {
        self.folders()
            .filter(|&folder_id| self.get_folder_size_recursive(folder_id) >= size)
            .min_by_key(|folder_id| self.get_folder_size_recursive(*folder_id))
            .unwrap()
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
        // Parse the input into filesystem
        let input = self.get_input().unwrap();
        let (_, history) = CommandHistory::parse(&input).unwrap();
        let fs = FileSystem::build_from_command_history(&history);

        // Get all folders that are at most 100_000 bytes and sum their sizes
        let max_size = 100_000;
        fs.get_folders_at_most(max_size)
            .map(|folder_id| fs.get_folder_size_recursive(folder_id))
            .sum()
    }

    fn part2_solution(&self) -> usize {
        // Parse the input into filesystem
        let input = self.get_input().unwrap();
        let (_, history) = CommandHistory::parse(&input).unwrap();
        let fs = FileSystem::build_from_command_history(&history);

        // Define lower bound for the sought directory size
        let total_space = 70_000_000;
        let needed_space = 30_000_000;

        let root = fs.tree.root_id().unwrap();
        let used_space = fs.get_folder_size_recursive(root);

        let current_free_space = total_space - used_space;
        let space_to_free = needed_space - current_free_space;

        // Get the smallest folder that is at least the given size
        let node = fs.get_smallest_folder_larger_than(space_to_free);

        fs.get_folder_size_recursive(node)
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

        let (input, command) = Command::parse(input).unwrap();

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

        let (input, command) = Command::parse(input).unwrap();

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
