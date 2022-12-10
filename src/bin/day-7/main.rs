use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Lines;
use std::io::prelude::*;
use std::option::Option;
use std::rc::Rc;

enum FileType {
    File,
    Dir,
}

struct FileEntry {
    file_type: FileType,
    name: String,
    size: i32,
    parent: Option<Rc<RefCell<FileEntry>>>,
    children: Vec<Rc<RefCell<FileEntry>>>,
}

enum Command {
    Ls,
    ChangeDir(String),
    Output(String),
}


fn main() {
    let file = File::open("src/bin/day-7/input.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let root = Rc::new(RefCell::new(FileEntry {
        file_type: FileType::Dir,
        name: String::from("/"),
        children: Vec::new(),
        parent: None,
        size: 0,
    }));

    parse(lines, Rc::clone(&root), Rc::clone(&root));

    let dir_sizes = directory_sizes(Rc::clone(&root));

    let mut total = 0;
    let mut dir_to_free_space = 0;
    let used_space = dir_sizes.get("///").unwrap();
    let need_space = (70000000 - used_space - 30000000).abs();

    for (_, v) in dir_sizes {
        if v <= 100000 {
            total += v;
        }
        if v >= need_space && (v < dir_to_free_space || dir_to_free_space == 0) {
            dir_to_free_space = v;
        }
    }

    //Part 1
    println!("Part 1 answer: {total}");

    //Part 2
    println!("Part 2 answer: {dir_to_free_space}");
}

fn parse(mut lines: Lines<BufReader<File>>, root: Rc<RefCell<FileEntry>>, current: Rc<RefCell<FileEntry>>) {
    if let Some(Ok(line)) = lines.next() {
        let cmd = parse_line(&line);
        let new_dir = parse_command(Rc::clone(&root),Rc::clone(&current), &cmd);

        if let Some(new_current) = new_dir {
            parse(lines, Rc::clone(&root), Rc::clone(&new_current));
        } else {
            parse(lines, Rc::clone(&root), Rc::clone(&current));
        }
    }
}

fn parse_line(line: &str) -> Command {
    let parts: Vec<&str> = line.splitn(3, ' ').collect();
    match parts[..] {
        ["$", "ls"] => Command::Ls,
        ["$", "cd", _] => Command::ChangeDir(String::from(parts[2])),
        _ => Command::Output(line.to_string()),
    }
}

fn parse_command(root: Rc<RefCell<FileEntry>>, current: Rc<RefCell<FileEntry>>, cmd: &Command) -> Option<Rc<RefCell<FileEntry>>> {
    match cmd {
        Command::ChangeDir(name) => change_directory(root, current, name.as_str()),
        Command::Output(value) => {
            parse_output(current, value);
            None
        }
        _ => None
    }
}

fn change_directory(root: Rc<RefCell<FileEntry>>, current: Rc<RefCell<FileEntry>>, name: &str) -> Option<Rc<RefCell<FileEntry>>> {
    match name {
        ".." => {
            let my_current = current.borrow();
            let my_parent = my_current.parent.as_ref().unwrap();
            Some(Rc::clone(my_parent))
        },
        "/" => Some(root),
        name => Some(find_dir(current, name)),
    }
}

fn find_dir(current: Rc<RefCell<FileEntry>>, name: &str) -> Rc<RefCell<FileEntry>> {
    let mut found_dir: Option<Rc<RefCell<FileEntry>>> = None;
    let my_current = &*current.borrow();
    for fe in my_current.children.iter() {
        let match_fe = &*fe.borrow();
        found_dir = match match_fe {
            FileEntry {
                file_type: FileType::Dir,
                name: dir_name,
                size: _, parent: _, children: _
            } if dir_name == name => {
                Some(Rc::clone(fe))
            }
            _ => None
        };

        if found_dir.is_some() {
            break;
        }
    }

    if let Some(return_fe) = found_dir {
        return_fe
    } else {
        Rc::new(RefCell::new(FileEntry {
            file_type: FileType::Dir,
            name: name.to_string(),
            children: Vec::new(),
            size: 0,
            parent: Some(Rc::clone(&current)),
        }))
    }
}

fn parse_output(current: Rc<RefCell<FileEntry>>, line: &str) {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    let fe = match parts[..] {
        ["dir", name] => Some(FileEntry {
            file_type: FileType::Dir,
            name: name.to_string(),
            children: Vec::new(),
            size: 0,
            parent: Some(Rc::clone(&current)),
        }),
        [size, name] => {
            Some(FileEntry {
                file_type: FileType::File,
                name: name.to_string(),
                size: size.parse::<i32>().unwrap(),
                parent: Some(Rc::clone(&current)),
                children: Vec::new(),
            })
        }
        _ => None
    };

    if let Some(new_fe) = fe {
        let mut_current = &mut *current.borrow_mut();
        mut_current.children.push(Rc::new(RefCell::new(new_fe)));
    }
}


fn directory_sizes(root: Rc<RefCell<FileEntry>>) -> HashMap<String, i32> {
    let mut sizes: HashMap<String, i32> = HashMap::new();
    dir_sizes(&mut sizes, &root.borrow(), String::from("/"));
    sizes
}

fn dir_sizes(sizes: &mut HashMap<String,i32>, node: &FileEntry, path: String) -> i32 {
    let mut dir_sum = 0;
    let new_path = format!("{}/{}", path, node.name);
    for fe in &node.children {
        let current_fe = &*fe.borrow();
        match current_fe {
            FileEntry {
                file_type: FileType::Dir, name: _, size: _, parent: _, children: _
            } => dir_sum += dir_sizes(sizes, current_fe, new_path.clone()),
            FileEntry {
                file_type: FileType::File, size: file_size, parent: _, children: _, name: _,
            } => {
                dir_sum += file_size
            }
        }
    }
    sizes.insert(new_path, dir_sum);
    dir_sum
}
