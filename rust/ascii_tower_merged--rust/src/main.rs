pub struct Tower {
    name: String,
    floors: i32,
}

impl Tower {
    fn new(name: &str, floors: i32) -> Self {
        Self {
            name: name.into(),
            floors,
        }
    }
}

mod display {
    use std::fmt::{Display, Formatter, Result, Write};

    impl Display for super::Tower {
        fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
            const ALIGN: &'static str = "       ";

            let roof = |res: &mut Formatter<'_>, text| writeln!(res, "{ALIGN}|{:=^12}|", text);

            let sep = |res: &mut Formatter<'_>| writeln!(res, "{ALIGN}|------------|");

            let floor =
                |res: &mut Formatter<'_>, text| writeln!(res, "({:0>3})  |-[]--[]--[]-|", text);

            let lobby = |res: &mut Formatter<'_>| {
                writeln!(
                    res,
                    "{ALIGN}|----_____---|\n{ALIGN}|----|  ,|---|\n{ALIGN}|----|   |---|"
                )
            };

            roof(fmt, &self.name)?;
            sep(fmt)?;
            for i in 0..self.floors {
                floor(fmt, self.floors - i)?;
                sep(fmt)?;
            }
            lobby(fmt)?;
            Ok(())
        }
    }
}

fn main() {
    println!("Hello, world!");
}
