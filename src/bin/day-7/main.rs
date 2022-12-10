// use std::fs;
//
// #[derive(Debug)]
// struct File {
//     name: String,
//     size: i32,
// }
//
// #[derive(Debug)]
// struct Directory {
//     name: String,
//     files: Vec<File>,
//     directories: Vec<Directory>,
//     parent: Option<Box<Directory>>,
// }
//
// fn main() {
//     let content = fs::read_to_string("src/bin/day-7/input.txt").unwrap().replace("\r\n", "\n");
//
//     let mut root = Directory {
//         name: String::from("/"),
//         files: vec![],
//         directories: vec![],
//         parent: None,
//     };
//
//     let mut pointer: &Directory = &root;
//
//     let commands = content.split('\n').collect::<Vec<&str>>();
//
//     commands.iter().for_each(|line| {
//         let mut line = line.to_string();
//         if line.starts_with("$ ") {
//             line = line.replace("$ ", "");
//             if line.starts_with("ls") {
//
//             }
//             if line.starts_with("cd ") {
//                 line = line.replace("cd ", "");
//                 match line.as_str() {
//                     "/" => pointer = &root,
//                     ".." => {}
//                     path => {
//                         let dir = Directory {
//                             name: String::from(path),
//                             files: vec![],
//                             directories: vec![],
//                             parent: Box::new(Some(*pointer))
//                         };
//                         pointer.directories.push(dir);
//                     }
//                 };
//             }
//         }
//     });
// }

fn main() {

}