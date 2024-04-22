use console::{style, Style};

#[derive(Clone, Debug)]
pub enum AsciiArt {
    Bunny,
    Owl,
}

impl ToString for AsciiArt {
    fn to_string(&self) -> String {
        match self {
            AsciiArt::Owl => "owl".to_string(),
            AsciiArt::Bunny => "bunny".to_string(),
        }
    }
}

impl From<String> for AsciiArt {
    fn from(input: String) -> AsciiArt {
        match &*input {
            "owl" => AsciiArt::Owl,
            "bunny" => AsciiArt::Bunny,
            _ => AsciiArt::Bunny,
        }
    }
}

pub fn ascii_art(art: AsciiArt) -> String {
    match art {
        AsciiArt::Bunny => {
            let paw = style("\"").red().bold();
            let eye = style(".").blue().bold();
            format!(
                // 7 spaces
                "       
(\\_(\\  
( {} {}) 
c({})({})
       ",
                eye, eye, paw, paw
            )
        }
        AsciiArt::Owl => {
            let eye = style("0").blue().bold();
            let wing = style("./").red().bold();
            format!(
                "       
{{{},{}}}  
{})_)  
 \" \"   ",
                eye, eye, wing
            )
        }
    }
}

pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
    Cyan,
    Magenta,
}

pub fn entry(name: &str, color: Color, data: String) -> String {
    let num_of_dots = 11 - name.len();

    let mut dots = String::new();
    for _ in 0..num_of_dots {
        dots.push_str("⋅")
    }

    let bold = || Style::new().bold();
    let style = match color {
        Color::Red => bold().red(),
        Color::Yellow => bold().yellow(),
        Color::Green => bold().green(),
        Color::Blue => bold().blue(),
        Color::Cyan => bold().cyan(),
        Color::Magenta => bold().blue(),
    };

    format!("{}{} {}", name, dots, style.apply_to(data))
}
