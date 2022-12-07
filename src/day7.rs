use crate::error::Error;
use std::collections::HashMap;

pub fn get_path_from_directory_stack(directories: &[String]) -> String {
    "/".to_string() + &directories.join("/")
}

pub fn get_paths_from_directory_stack(stack: &[String]) -> Vec<String> {
    let mut result = Vec::with_capacity(stack.len() + 1);
    for index in 0..stack.len() {
        result.push(get_path_from_directory_stack(&stack[0..stack.len() - index]));
    }
    result.push("/".to_string());
    result
}

pub fn compute(input: &str) -> Result<HashMap<String, usize>, Error> {
    const COMMAND_CD_ROOT: &str = "$ cd /";
    const COMMAND_CD_PARENT: &str = "$ cd ..";
    const COMMAND_CD_DIRECTORY: &str = "$ cd ";
    const COMMAND_LS: &str = "$ ls";
    const OUTPUT_DIR: &str = "dir ";
    let mut result = HashMap::new();
    let mut directory_stack = Vec::new();
    for line in input.trim_start().trim_end().lines() {
        if line.is_empty() || line.starts_with(COMMAND_LS) || line.starts_with(OUTPUT_DIR) {
            continue;
        } else if line.starts_with(COMMAND_CD_ROOT) {
            directory_stack.clear();
        } else if line.starts_with(COMMAND_CD_PARENT) {
            directory_stack.truncate(directory_stack.len() - 1);
        } else if line.starts_with(COMMAND_CD_DIRECTORY) {
            let folder = line.trim_start_matches(COMMAND_CD_DIRECTORY);
            directory_stack.push(folder.to_owned());
        } else {
            use text_io::try_scan;
            let size: usize;
            let filename: String;
            try_scan!(line.bytes() => "{} {}", size, filename);
            for directory in get_paths_from_directory_stack(&directory_stack) {
                *result.entry(directory).or_insert(0) += size;
            }
        }
    }
    Ok(result)
}

pub fn score(directories: &HashMap<String, usize>) -> usize {
    let mut total = 0;
    for v in directories.values() {
        if v <= &100_000 {
            total += v;
        }
    }
    total
}

pub fn smallest(directories: &HashMap<String, usize>) -> usize {
    let mut candidates = Vec::new();
    let currently_used = directories["/"];
    let currently_unused = 70000000 - currently_used;
    let need_more = 30000000 - currently_unused;
    for v in directories.values() {
        if v > &need_more {
            candidates.push(*v);
        }
    }
    candidates.sort();
    *candidates.first().unwrap()
}

#[test]
fn test() -> Result<(), Error> {
    let input = r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    let directories = compute(input)?;
    assert_eq!(directories["/d"], 24933642);
    assert_eq!(directories["/a/e"], 584);
    assert_eq!(directories["/"], 48381165);
    assert_eq!(score(&directories), 95437);
    assert_eq!(smallest(&directories), 24933642);

    let input = std::fs::read_to_string("input/day7")?;
    let directories = compute(&input)?;
    assert_eq!(score(&directories), 1778099);
    assert_eq!(smallest(&directories), 1623571);

    Ok(())
}
