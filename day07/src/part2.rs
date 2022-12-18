use std::collections::VecDeque;
use std::{
    fs::File,
    io::{self, BufRead},
};

// FIXME(alvaro): I think as it is right now having FSDirectory and FSFile as
// independent structs is redundant

#[derive(Debug, Clone)]
struct FSDirectory {
    name: String,
    path: String,
    entries: Vec<Entry>,
}

impl FSDirectory {
    fn new(name: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            entries: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct FSFile {
    name: String,
    path: String,
    size: usize,
}

impl FSFile {
    fn new(name: impl Into<String>, path: impl Into<String>, size: usize) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            size,
        }
    }
}

#[derive(Debug, Clone)]
enum Entry {
    DirEntry(FSDirectory),
    FileEntry(FSFile),
}

impl Entry {
    fn get_name(&self) -> &str {
        match self {
            Entry::DirEntry(FSDirectory { name, .. }) => name.as_ref(),
            Entry::FileEntry(FSFile { name, .. }) => name.as_ref(),
        }
    }

    fn get_path(&self) -> &str {
        match self {
            Entry::DirEntry(FSDirectory { path, .. }) => path.as_ref(),
            Entry::FileEntry(FSFile { path, .. }) => path.as_ref(),
        }
    }

    fn is_dir(&self) -> bool {
        matches!(self, Entry::DirEntry(..))
    }

    fn is_file(&self) -> bool {
        matches!(self, Entry::FileEntry(..))
    }

    fn get_entry(&self, path: &str) -> Option<&Self> {
        let name = self.get_name();
        let (segment, rest) = split_path_segment(path);
        if name != segment {
            None
        } else if rest.is_empty() {
            Some(self)
        } else if let Entry::DirEntry(dir) = self {
            for entry in dir.entries.iter() {
                if let Some(result) = entry.get_entry(rest) {
                    return Some(result);
                }
            }
            None
        } else {
            None
        }
    }

    fn get_entry_mut(&mut self, path: &str) -> Option<&mut Self> {
        let name = self.get_name();
        let (segment, rest) = split_path_segment(path);
        if name != segment {
            None
        } else if rest.is_empty() {
            Some(self)
        } else if let Entry::DirEntry(dir) = self {
            for entry in dir.entries.iter_mut() {
                if let Some(result) = entry.get_entry_mut(rest) {
                    return Some(result);
                }
            }
            None
        } else {
            None
        }
    }

    /// Return the full size of this entry
    fn size(&self) -> usize {
        match self {
            Entry::DirEntry(dir) => dir.entries.iter().fold(0, |acc, e| acc + e.size()),
            Entry::FileEntry(file) => file.size,
        }
    }

    /// Helper to print the contents of the given entry
    fn print_with_level(&self, level: u32) {
        let prefix = "  ".repeat(level as usize * 2);
        match self {
            Entry::DirEntry(FSDirectory { name, entries, .. }) => {
                let dirname = if name.is_empty() { "/" } else { name };
                println!("{}- {} (dir)", prefix, dirname);
                for entry in entries {
                    entry.print_with_level(level + 1);
                }
            }
            Entry::FileEntry(FSFile { name, size, .. }) => {
                println!("{}- {} (file, size={})", prefix, name, size);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct FileSystem {
    cwd: String,
    root: Entry,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            cwd: "/".into(),
            root: Entry::DirEntry(FSDirectory::new("", "/")),
        }
    }

    /// Create the directory (if it does not exist) and set it as the current
    /// working directory
    fn cd(&mut self, path_or_name: String) {
        // TODO(alvaro): Ideally we would support having '..' in any part of
        // the path... but we already wasted too much time in this already
        if path_or_name == ".." {
            self.cwd = parent_path(&self.cwd).into();
            return;
        }
        let canonical_path = self.canonicalize(path_or_name.as_ref());
        assert!(
            self.is_dir(canonical_path.as_ref()),
            "cding into a non-existent directory"
        );
        self.cwd = canonical_path;
    }

    /// Returns `true` if a path exists and is a DirEntry::Directory
    fn is_dir(&self, path: &str) -> bool {
        self.get_entry(path)
            .filter(|e| matches!(e, &Entry::DirEntry(..)))
            .is_some()
    }

    // FIXME(alvaro): It's a shame that this does a clone always, it could
    // be better to use something like a Cow to avoid unnecessary clones
    // when possible (NOTE: impl Into<String> does not work here)
    /// Return the canonical path (absolute path) given a path that can be
    /// relative or absolute
    fn canonicalize(&self, relative_or_absolute_path: &str) -> String {
        if relative_or_absolute_path.starts_with('/') {
            relative_or_absolute_path.to_string()
        } else {
            join_path(self.cwd.clone(), relative_or_absolute_path)
        }
    }

    fn mkdir(&mut self, path: &str) {
        // Find the parent node
        let canonical_path = self.canonicalize(path);
        let parent_path = parent_path(&canonical_path);
        let parent_entry = self
            .get_entry_mut(parent_path)
            .expect("the parent to exist");
        let Entry::DirEntry(parent_dir) = parent_entry else {
            panic!("parent entry to be a directory");
        };

        let name = name_from_path(&canonical_path).to_string();
        let fsdir = FSDirectory::new(name, canonical_path);
        parent_dir.entries.push(Entry::DirEntry(fsdir));
    }

    fn write(&mut self, path: &str, size: usize) {
        // Find the parent node
        let canonical_path = self.canonicalize(path);
        let parent_path = parent_path(&canonical_path);
        let parent_entry = self
            .get_entry_mut(parent_path)
            .expect("the parent to exist");
        let Entry::DirEntry(parent_dir) = parent_entry else {
            panic!("parent entry to be a directory");
        };

        let name = name_from_path(&canonical_path).to_string();
        let fsfile = FSFile::new(name, canonical_path, size);
        parent_dir.entries.push(Entry::FileEntry(fsfile));
    }

    /// Find the entry associated to a given path and return an immutable
    /// reference to it
    fn get_entry(&self, path: &str) -> Option<&Entry> {
        let canonical_path = self.canonicalize(path);
        self.root.get_entry(&canonical_path)
    }

    /// Find the entry associated to a given path and return a mutable
    /// reference to it
    fn get_entry_mut(&mut self, path: &str) -> Option<&mut Entry> {
        let canonical_path = self.canonicalize(path);
        self.root.get_entry_mut(&canonical_path)
    }

    // TODO(alvaro): Yes... I know that making a proper iterator would be the
    // right thing to do here... but it's hurting my brain at this point
    fn bfs_references(&self) -> Vec<&Entry> {
        let mut refs = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(&self.root);
        while let Some(entry) = queue.pop_front() {
            // Add the inner entries to the queue (if any)
            if let Entry::DirEntry(dir) = entry {
                for inner in dir.entries.iter() {
                    queue.push_back(inner);
                }
            };
            // Add this entry to the refs vec
            refs.push(entry);
        }
        refs
    }

    fn size(&self) -> usize {
        self.root.size()
    }
}

fn name_from_path(path: &str) -> &str {
    if let Some(split_idx) = path.rfind('/') {
        &path[split_idx + 1..]
    } else {
        path
    }
}

fn parent_path(path: &str) -> &str {
    let rindex = path.rfind('/').expect("must be a canonical path");
    if rindex == 0 {
        "/"
    } else {
        &path[..rindex]
    }
}

fn split_path_segment(path: &str) -> (&str, &str) {
    if let Some(split_idx) = path.find('/') {
        let segment = &path[..split_idx];
        let rest = &path[split_idx + 1..];
        (segment, rest)
    } else {
        (path, "")
    }
}

fn join_path(left: String, right: &str) -> String {
    if left.ends_with('/') {
        format!("{}{}", left, right)
    } else {
        format!("{}/{}", left, right)
    }
}

fn main() -> io::Result<()> {
    let input = parse("input.txt")?;
    let solution = solve(input);
    println!("{solution}");
    Ok(())
}

fn parse(path: &str) -> io::Result<Vec<Vec<String>>> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();

    let mut parsed = Vec::new();
    for line in lines.into_iter().flatten() {
        let words: Vec<String> = line.split_whitespace().map(String::from).collect();
        parsed.push(words);
    }

    Ok(parsed)
}

fn solve(input: Vec<Vec<String>>) -> u64 {
    let mut fs = FileSystem::new();

    let mut listing = false;
    for mut line in input {
        let first = &line[0][..];
        match first {
            "$" => {
                let command = &line[1][..];
                match command {
                    "cd" => {
                        listing = false;
                        let dirname = line.pop().expect("a dirname");
                        fs.cd(dirname);
                    }
                    "ls" => {
                        assert!(!listing, "Must not be listing already");
                        listing = true;
                    }
                    _ => panic!("Unexpected command: {}", command),
                }
            }
            "dir" => {
                assert!(listing, "Found a directory while not listing");
                let name = line.pop().expect("a dirname");
                fs.mkdir(&name);
            }
            size if size.chars().all(|c| c.is_numeric()) => {
                assert!(listing, "Found a file while not listing");
                let size: usize = size.parse().unwrap();
                let name = line.pop().expect("a file name");
                fs.write(&name, size);
            }
            _ => panic!("unexpected input"),
        }
    }
    let unused = 70_000_000 - fs.size() as u64;
    let missing_unused = 30_000_000 - unused;
    fs.bfs_references()
        .into_iter()
        .filter(|e| matches!(e, Entry::DirEntry(..)))
        .map(|e| e.size() as u64)
        .filter(|s| *s > missing_unused)
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = parse("sample.txt").unwrap();
        let solution = solve(input);
        assert_eq!(solution, 24933642)
    }
}
