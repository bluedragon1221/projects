use tagger::TaggedFile;

fn get_tag(_modlist: &[String]) {
    todo!()
}

fn logged_rename(file1: String, file2: String) {
    match std::fs::rename(&file1, &file2) {
        Ok(_) => println!("mv {} {}", file1, file2),
        Err(a) => panic!("Something went wrong when renaming files: {a}"),
    }
}

pub fn parse_args(args: Vec<String>) {
    if args.len() < 1 {
        panic!("You must specify at least one argument")
    }

    let modlist: &[String] = args.get(2..).map(|x| x).unwrap_or(&[]);
    match args.get(1).map(|x| x.as_str()) {
        Some("get") => {
            get_tag(modlist);
        }
        Some(_) => {
            if let Ok(old_file) = TaggedFile::from_filename(args.get(1).unwrap()) {
                let new_file = old_file.new_with_modlist(modlist);
                logged_rename(old_file.generate_filename(), new_file.generate_filename());
            } else {
                panic!("Failed to parse filename")
            };
        }
        None => {
            panic!("You must specify arg1")
        }
    }
}
