use std::env;
use std::fs;
use std::io;
use std::path::Path;

// Meu primeiro c[odigo de verdade em RUST. Ainda entendendo
fn main() {
    let args: Vec<String> = env::args().collect();

    let (target_name, folder_only) = parse_args(&args);
    let prefix = String::from("");
    let children_prefix = String::from("");

    let _ = print_target(
        target_name,
        &prefix,
        &children_prefix,
        false,
        true,
        folder_only,
    );
}

fn parse_args(args: &[String]) -> (&str, bool) {
    let target_name = &args[1];

    let mut folder_only = false;

    for arg in args {
        if arg == "--folder-only" {
            folder_only = true;
        }
    }

    return (target_name, folder_only);
}

fn print_target(
    target_path: &str,
    prefix: &str,
    children_prefix: &str,
    is_last: bool,
    is_root: bool,
    folder_only: bool,
) -> io::Result<()> {
    // Eu tinha que retornar o tipo do erro que pode vir do read_dir (operaçao sensivel)
    //dbg!(target_name);
    let target_path = Path::new(target_path);
    let is_target_path_dir = target_path.is_dir();

    let target_name = target_path.file_name().unwrap();
    let target_name = target_name.to_str().unwrap();
    let suffix = if is_target_path_dir { "/" } else { "" };
    let printing_indicator = if is_root {
        ""
    } else {
        if is_last { "└── " } else { "├── " }
    };

    print!(
        "{}{}{}{}\n",
        prefix, printing_indicator, target_name, suffix
    );

    if !target_path.exists() {
        panic!("path does not exist");
    }

    if is_target_path_dir {
        let entries = fs::read_dir(target_path)?;

        let mut it = entries.peekable();

        while let Some(path) = it.next() {
            let is_last = it.peek().is_none();
            let path = path?.path();
            let parsed_path = path.clone().into_os_string();
            let parsed_path = parsed_path.to_str().unwrap();
            let appending_string = if is_last { "   " } else { "│    " };
            let new_prefix = [children_prefix, appending_string].concat();

            if folder_only {
                if path.is_dir() {
                    let _ = print_target(
                        parsed_path,
                        &prefix,
                        &new_prefix,
                        is_last,
                        false,
                        folder_only,
                    );
                }
            } else {
                let _ = print_target(
                    parsed_path,
                    &children_prefix,
                    &new_prefix,
                    is_last,
                    false,
                    folder_only,
                );
            }
        }
    }

    Ok(())
}
