use clap::Parser;
use fetch::FetchError;

mod system_info;
use system_info::Sysinfo;

mod ascii;
use ascii::{entry, AsciiArt, Color};

// Simple program to fetch info about the system
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Disable ascii art
    #[arg(long, default_value_t = false)]
    no_ascii: bool,

    // Disable showing current distro
    #[arg(long, default_value_t = false)]
    no_distro: bool,

    // Disable showing the hostname
    #[arg(long, default_value_t = false)]
    no_hostname: bool,

    // Disable showing the kernel version
    #[arg(long, default_value_t = false)]
    no_kernel: bool,

    // Disable showing current desktop
    #[arg(long, default_value_t = false)]
    no_desktop: bool,

    // Disable showing current uptime
    #[arg(long, default_value_t = false)]
    no_uptime: bool,

    #[arg(long, default_value_t = AsciiArt::Owl)]
    ascii_art: AsciiArt,
}

fn could_error() -> Result<(), FetchError> {
    let args = Args::parse();
    let mut data_block: Vec<String> = Vec::new();

    if !args.no_hostname {
        data_block.push(entry("Hostname", Color::Cyan, Sysinfo::get_hostname()?))
    }
    if !args.no_desktop {
        data_block.push(entry("Desktop", Color::Green, Sysinfo::get_desktop()?))
    }
    if !args.no_distro {
        data_block.push(entry("Distro", Color::Yellow, Sysinfo::get_distro()?))
    }
    if !args.no_kernel {
        data_block.push(entry("Kernel", Color::Red, Sysinfo::get_kernel()?))
    }

    if !args.no_uptime {
        data_block.push(entry("Uptime", Color::Blue, Sysinfo::get_uptime()?))
    }

    let mut ascii_block = ascii::ascii_art(args.ascii_art);

    let mut to_print = String::new();
    ascii_block
        .split("\n")
        .into_iter()
        .zip(data_block.into_iter())
        .for_each(|(a, b)| to_print.push_str(&format!("  {}  {}\n", a, b)));

    println!("{}", to_print);

    Ok(())
}

fn main() {
    match could_error() {
        Ok(()) => (),
        Err(a) => eprintln!("An error occured: {}", a),
    }
}
