use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

#[derive(Debug)]
struct Folder {
    folders: Vec<Rc<RefCell<Folder>>>,
    size: u32,
}

#[derive(Debug)]
enum TerminalCommand<'b> {
    LS,
    CD(&'b str),
}

// A line in input.txt can start with $ implying
// we are in a command else we are dealing with the output
// of ls
impl TerminalCommand<'_> {
    fn parse(line: &str) -> Option<TerminalCommand> {
        let mut words_iter = line.split(' ');
        if words_iter.next().unwrap() != "$" {
            return None;
        }

        match words_iter.next().unwrap() {
            "ls" => Some(TerminalCommand::LS),
            "cd" => Some(TerminalCommand::CD(words_iter.next().unwrap())),
            _ => panic!("Unknown command"),
        }
    }
}

// This constructs a file-tree according to what we found in
// input.txt
fn build_tree(input: Vec<&str>) -> Rc<RefCell<Folder>> {
    let root_folder = Rc::new(RefCell::new(Folder {
        folders: Vec::new(),
        size: 0,
    }));

    // TODO: replace this with a parent pointer on Folder... This gave me a lot
    // of issues...
    let mut stack: Vec<Rc<RefCell<Folder>>> = Vec::new();
    let mut current_directory = root_folder.clone();

    for line in input {
        match TerminalCommand::parse(line) {
            Some(TerminalCommand::CD(dir)) => {
                if dir == ".." {
                    current_directory = stack.pop().unwrap().clone();
                } else {
                    let new = Rc::new(RefCell::new(Folder {
                        folders: Vec::new(),
                        size: 0,
                    }));
                    current_directory.borrow_mut().folders.push(new.clone());
                    stack.push(current_directory.clone());
                    current_directory = new.clone();
                }
            }
            Some(TerminalCommand::LS) => {}
            None => {
                let parts = line.split(' ').collect::<Vec<&str>>();
                if parts[0].parse::<u32>().is_ok() {
                    let size = parts[0].parse::<u32>().unwrap();
                    let new = Rc::new(RefCell::new(Folder {
                        folders: Vec::new(),
                        size,
                    }));
                    current_directory.borrow_mut().folders.push(new.clone());
                }
            }
        }
    }

    root_folder
}

fn set_folder_sizes(folder: &mut Rc<RefCell<Folder>>) {
    let mut sum = 0;
    let mut current_folder = folder.borrow_mut();

    for child in &mut current_folder.folders {
        set_folder_sizes(child);
        sum += child.borrow().size;
    }

    if current_folder.size == 0 {
        current_folder.size = sum;
    }
}

fn sum_of_filtered_directories(folder: Rc<RefCell<Folder>>, max_size: u32) -> u32 {
    let mut sum = 0;

    let sub_folders = &folder.borrow().folders;
    // If the folder is below the max_size and has some sub-folders we sum it up
    if folder.borrow().size <= max_size && !sub_folders.is_empty() {
        sum += folder.borrow().size;
    }

    // We go over all subfolders
    for child in sub_folders {
        sum += sum_of_filtered_directories(child.clone(), max_size);
    }

    sum
}

fn find_size_of_deleteable_folder(folder: Rc<RefCell<Folder>>, need_to_free: u32) -> u32 {
    let mut total_size = 70000000;

    let sub_folders = &folder.borrow().folders;
    let folder_size = folder.borrow().size;
    // If this folder has no sub-folders we have found a leaf and can safely return
    if sub_folders.is_empty() {
        return total_size;
    }

    if folder_size >= need_to_free && folder_size < total_size {
        total_size = folder_size;
    }

    for child in sub_folders {
        let num = find_size_of_deleteable_folder(child.clone(), need_to_free);
        if num >= need_to_free && num < total_size {
            total_size = num;
        }
    }

    total_size
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();

    let mut parent = build_tree(lines);
    set_folder_sizes(&mut parent);

    println!(
        "Solution 1: {} - Solution 2: {}",
        sum_of_filtered_directories(parent.clone(), 100000),
        find_size_of_deleteable_folder(
            parent.clone(),
            30000000 - (70000000 - parent.borrow().size)
        )
    );
}
