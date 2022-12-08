//! The solution for [advent of code 2022, day 7](https://adventofcode.com/2022/day/7)

use std::collections::HashMap;
use std::path::PathBuf;
use std::string::ToString;

use clap::Parser;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::{parse_error, MainResult};

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 7")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The maximum dir size to consider.
    #[clap(short, long, value_parser, default_value_t = 100000)]
    threshold: u32,

    /// The size of the file system.
    #[clap(short, long, value_parser, default_value_t = 70000000)]
    fs_size: u32,

    /// The minimum amount of free space needed.
    #[clap(short, long, value_parser, default_value_t = 30000000)]
    needed: u32,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);
    let (root_dir_size, dir_sizes) = compute_dir_sizes(input.read_lines())?;

    println!(
        "The total size of all directories with a size of at most {}, is {}",
        args.threshold,
        dir_sizes
            .iter()
            .filter(|&&size| size <= args.threshold)
            .sum::<u32>()
    );

    match find_dir_to_delete(&dir_sizes, root_dir_size, args.fs_size, args.needed) {
        Some(v) => println!(
            "The smallest directory that can be deleted to free up enough space has size {}",
            v
        ),
        None => eprintln!("Could not find any directory that, if deleted, frees up enough space"),
    }

    Ok(())
}

fn compute_dir_sizes<T>(input: T) -> Result<(u32, Vec<u32>), ParseError>
where
    T: Iterator<Item = String>,
{
    let mut cwd: Vec<String> = vec![];
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();
    let mut root_size = 0;

    for line in input {
        match line.strip_prefix("$ ") {
            Some(command) => match command.strip_prefix("cd ") {
                Some("/") => {
                    cwd.clear();
                }
                Some("..") => {
                    cwd.pop();
                }
                Some(dir) => {
                    cwd.push(dir.to_string());
                    let path = to_path(cwd.iter());
                    dir_sizes.entry(path).or_insert(0);
                }
                None => {
                    // must be ls
                    // no action required
                }
            },
            None => {
                // must be output of ls
                let i = line
                    .find(' ')
                    .ok_or_else(|| parse_error!("invalid input: {}", line))?;

                let left = &line[..i];
                if left != "dir" {
                    let file_size = left.parse::<u32>()?;
                    root_size += file_size;
                    for n in 1..=cwd.len() {
                        let path = to_path(cwd.iter().take(n));
                        *dir_sizes.entry(path).or_insert(0) += file_size;
                    }
                }
            }
        }
    }

    let dirs: Vec<u32> = dir_sizes.values().copied().collect();

    Ok((root_size, dirs))
}

fn to_path<'a, T>(cwd_iter: T) -> String
where
    T: Iterator<Item = &'a String>,
{
    cwd_iter.fold("".to_string(), |mut acc, x| {
        acc.push('/');
        acc.push_str(x);
        acc
    })
}

fn find_dir_to_delete(
    dir_sizes: &[u32],
    used_space: u32,
    fs_size: u32,
    needed: u32,
) -> Option<u32> {
    let free_space = fs_size - used_space;
    let needed = needed - free_space;

    dir_sizes
        .iter()
        .copied()
        .filter(|&size| size >= needed)
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = String> {
        InputReader::from("./src/day07/test.txt").read_lines()
    }

    #[test]
    fn test_compute_dir_sizes() {
        let (root_dir_size, mut dir_sizes) = compute_dir_sizes(test_data()).unwrap();
        dir_sizes.sort_unstable();

        assert_eq!(root_dir_size, 48381165);
        assert_eq!(dir_sizes, vec![584, 94853, 24933642]);
    }

    #[test]
    fn test_find_dir_to_delete() {
        let (root_dir_size, dir_sizes) = compute_dir_sizes(test_data()).unwrap();
        let dir_to_delete =
            find_dir_to_delete(&dir_sizes, root_dir_size, 70000000, 30000000).unwrap();

        assert_eq!(dir_to_delete, 24933642);
    }
}
