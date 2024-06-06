mod tower {
    pub use std::fmt::{self, Result};

    #[derive(impl_new::New)]
    pub struct Tower {
        name: String,
        floors: i32,
    }

    mod block {
        use super::{
            fmt::{Display, Write},
            Result,
        };

        const ALIGN: &str = "       ";

        pub fn sep(res: &mut impl Write) -> Result {
            writeln!(res, "{ALIGN}|------------|")
        }

        pub fn lobby(res: &mut impl Write) -> Result {
            writeln!(
                res,
                "{ALIGN}|----_____---|\n{ALIGN}|----|  ,|---|\n{ALIGN}|----|   |---|"
            )
        }

        pub fn roof(res: &mut impl Write, text: impl Display) -> Result {
            writeln!(res, "{ALIGN}|{:=^12}|", text)
        }

        pub fn floor(res: &mut impl Write, text: impl Display) -> Result {
            writeln!(res, "({:0>3})  |-[]--[]--[]-|", text)
        }
    }

    mod display {
        use super::{
            block::{floor, lobby, roof, sep},
            fmt::{Display, Formatter},
            Result, Tower,
        };

        impl Display for Tower {
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

fn main() {
    println!("{}", tower::Tower::new("Hotel", 12));
}
