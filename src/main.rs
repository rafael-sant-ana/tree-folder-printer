use std::env;
use std::fs;
use std::io;
use std::path::Path;

// Meu primeiro c[odigo de verdade em RUST. Ainda entendendo
fn main() {
    let args: Vec<String> = env::args().collect();

    let target_name = parse_args(&args);
    let prefix = String::from("");

    let _ = print_target(target_name, &prefix, false, true);
}

fn parse_args(args: &[String]) -> &str {
    let target_name = &args[1];

    return target_name;
}

fn print_target(target_path: &str, prefix: &str, is_last: bool, is_root: bool) -> io::Result<()> {
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
            let path = path?.path().into_os_string();
            let parsed_path = path.to_str().unwrap();
            let new_prefix = [prefix, "   "].concat();

            let _ = print_target(parsed_path, &new_prefix, is_last, false);
        }
    }

    Ok(())
}
