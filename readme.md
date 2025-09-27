# Tree Folder Printer

This code contains an attempt of making a script in Rust that works as follows:

If I have a folder,
calling
`tree-folder-printer ./test_folder_structure` will print something like this

```
test_folder_structure/
├── test_file1.txt
└── test_folder_2/
   └── test_file2.c
```

The root folder has an image "what-i-want-to-implement.png", which is a sort of flowchart I made to describe what I wanted to implement, as I don't know Rust Syntax well, and I wanted to first express my idea with recursion.

No LLM Used (:

## How to Run

01. [Install Rust](https://www.rust-lang.org/tools/install)
02. Clone this repository
```sh
git clone https://github.com/rafael-sant-ana/tree-folder-printer
```
To test the script, then run

03. `cargo run ./test-folder-structure`
I implemented a flag "--folder-only", in case you only want to log the folders
04. `cargo run ./test-folder-structure --folder-only`

The broader way of testing it, is using
```sh
cargo run <path>
```

What was the funnest about this project is that It was my first time actually coding in Rust. And since I got in University, it's been a long while I don't code simpler projects just for fun. So this was challenging and really fun!


