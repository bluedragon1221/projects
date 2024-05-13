mod lib {
    #[derive(impl_new::New, getset::Getters)]
    pub struct StudentClass {
        class: String,
        #[getset(get = "pub with_prefix")]
        grade: Percentage,
        #[getset(get = "pub with_prefix")]
        absenses: i8,
    }

    #[derive(Clone, impl_new::New, getset::Getters)]
    pub struct Percentage {
        #[getset(get = "pub with_prefix")]
        percent: i32,
    }

    impl Into<Percentage> for i32 {
        fn into(self) -> Percentage {
            if self <= 100 {
                Percentage::new(self)
            } else {
                panic!("{} is not a valid percent", self)
            }
        }
    }
}

use lib::StudentClass;

fn can_exempt(student: &StudentClass) -> bool {
    student.get_grade().get_percent() >= &90 && student.get_absenses() <= &2
}

fn main() {
    let collin_aphg = StudentClass::new("AP Human Geography", 90, 0);
    println!("{}", can_exempt(&collin_aphg));
}
