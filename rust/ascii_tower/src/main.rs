mod tower {
    #[derive(impl_new::New)]
    pub struct Tower {
        name: String,
        floors: i32,
    }

    mod block {
        const ALIGN: &str = "       ";

        use std::fmt::{Display, Result, Write};

        pub fn sep<W: Write>(res: &mut W) -> Result {
            writeln!(res, "{ALIGN}|------------|")
        }

        pub fn lobby<W: Write>(res: &mut W) -> Result {
            writeln!(
                res,
                "{ALIGN}|----_____---|\n{ALIGN}|----|  ,|---|\n{ALIGN}|----|   |---|"
            )
        }

        pub fn roof<W: Write, D: Display>(res: &mut W, text: D) -> Result {
            writeln!(res, "{ALIGN}|{:=^12}|", text)
        }

        pub fn floor<W: Write, D: Display>(res: &mut W, text: D) -> Result {
            writeln!(res, "({:0>3})  |-[]--[]--[]-|", text)
        }
    }

    mod display {
        use super::block::{floor, lobby, roof, sep};
        use std::fmt::{Display, Formatter, Result};

        impl Display for super::Tower {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
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
}

use tower::Tower;
fn main() -> std::fmt::Result {
    println!("{}", Tower::new("Hotel", 12));
    Ok(())
}
