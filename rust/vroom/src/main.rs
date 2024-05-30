use savefile;
use std::env;
use std::path::Path;
use vroom::{List, Measure, Vroomfile};

fn main() {
    let args: Vec<String> = env::args().collect();

    let path_str: String = if let Ok(vroomfile_home) = env::var("VROOMFILE") {
        vroomfile_home
    } else if let Ok(xdg_data_home) = env::var("XDG_DATA_HOME") {
        format!("{}/vroomfile", xdg_data_home)
    } else {
        format!("{}/.local/share/vroomfile", env::var("HOME").unwrap())
    };

    let mut vroomfile = if Path::new(&path_str).exists() {
        savefile::load_file("save.bin", 0).unwrap()
    } else {
        Vroomfile::new()
    };

    match args.get(1).and_then(|x| Some(x.as_str())) {
        Some("delete") => match args.get(2) {
            Some(list_name) if vroomfile.get_all_names().contains(&list_name) => {
                let list: &mut List = vroomfile.get_mut_list(list_name).unwrap();
                match args.get(3) {
                    Some(measure_name) if list.get_all_names().contains(&measure_name) => {
                        println!("Deleting '{}' from '{}'", measure_name, list_name);
                        list.rm_measure(measure_name)
                    }
                    Some(b) => {
                        eprintln!("There is no measure in '{}' named '{}'", list_name, b)
                    }
                    None => {
                        eprintln!("Deleting list '{}'", list_name);
                        vroomfile.rm_list(list_name)
                    }
                }
            }
            Some(a) => {
                eprintln!("you don't have a list named '{}'", a)
            }
            None => {
                eprintln!("Please specify the list from which to delete")
            }
        },
        Some("all") => {
            println!("{}", vroomfile)
        }
        Some(list_name) if vroomfile.get_all_names().contains(&list_name.to_string()) => {
            let list: &mut List = vroomfile.get_mut_list(list_name).unwrap();
            match args.get(2) {
                Some(measure_name) if list.get_all_names().contains(&measure_name) => {
                    let measure: &mut Measure = list.get_mut_measure(measure_name).unwrap();
                    println!("{}", measure.get_value())
                }
                Some(fut_measure_name) => match args.get(3) {
                    Some(value) => {
                        println!("adding new measure to '{}'", list_name);
                        list.add_measure(fut_measure_name, value)
                    }
                    None => eprintln!("Please specify the value of you're new measure"),
                },
                None => {
                    println!("{}", list)
                }
            }
        }
        Some(fut_list_name) => {
            println!("creating new list '{}'", fut_list_name);
            vroomfile.add_list(List::new(&fut_list_name))
        }
        None => {
            println!("{}", vroomfile.fmt_overview())
        }
    }

    savefile::save_file(&path_str, 0, &vroomfile).unwrap();
}
