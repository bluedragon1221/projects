mod tower {
    #[derive(impl_new::New)]
    pub struct Tower {
        name: String,
        floors: i32,
    }

    mod block {
        const ALIGN: &str = "       ";

        use std::fmt::{Result, Write};

        pub fn sep(res: &mut impl Write) -> Result {
            writeln!(res, "{ALIGN}|------------|")
        }

        pub fn roof(res: &mut impl Write, name: String) -> Result {
            writeln!(res, "{ALIGN}|{:=^12}|", name)
        }

        pub fn floor(res: &mut impl Write, number: i32) -> Result {
            writeln!(res, "({:0>3})  |-[]--[]--[]-|", number)
        }

        pub fn lobby(res: &mut impl Write) -> Result {
            writeln!(
                res,
                "{ALIGN}|----_____---|\n{ALIGN}|----|  ,|---|\n{ALIGN}|----|   |---|"
            )
        }
    }

    mod display {
        use super::block::{floor, lobby, roof, sep};
        use std::fmt::{Display, Formatter, Result};

        impl Display for super::Tower {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
                roof(fmt, self.name.clone())?;
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
