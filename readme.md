# Tree Folder Printer

This code contains an attempt of making a script in Rust that works as follows:

If I have a folder,
calling
`tree-folder-printer ./path` will print something like this

```
/path
├── folder 1
│   ├── file 1
│   ├── file 2
│   ├── folder 2
│   │   ├── file 3
│   │   ├── file 4
│   └── file 5
├── file 6

```
