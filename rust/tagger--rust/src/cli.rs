use std::path::PathBuf;

use tagger::{TaggedFile, TaggerError};

fn case_multiarg(rest_of_args: &[String], tagged_file: &mut TaggedFile) -> Result<(), TaggerError> {
    let og_file: PathBuf = tagged_file.og_file.clone();

    for i in rest_of_args {
        tagged_file.add_tag(i.to_string())
    }

    println!("mv {:?} {}", og_file, tagged_file.generate_filename());
    std::fs::rename(og_file, tagged_file.generate_filename())?;

    Ok(())
}

pub fn parse_args(args: Vec<String>) {
    let cmd = || -> Result<(), TaggerError> {
        let filename: &String = args.get(1).ok_or(TaggerError::ArgumentError(args.len()))?;
        let mut tagged_file = TaggedFile::from_filename(&filename)?;

        let rest_of_args = args
            .get(2..)
            .ok_or(TaggerError::ArgumentError(args.len()))?;

        match rest_of_args.len() {
            0 => Ok(println!("{:#?}", tagged_file)),
            _ => case_multiarg(rest_of_args, &mut tagged_file),
        }
    };

    match cmd() {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}
