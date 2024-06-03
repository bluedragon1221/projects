mod util {
    pub struct Percentage {
        percent: i32,
    }

    impl Into<Percentage> for i32 {
        fn into(self) -> Percentage {
            if self <= 100 {
                Percentage { percent: self }
            } else {
                panic!("{} is not a valid percent", self)
            }
        }
    }

    impl Percentage {
        pub fn as_num(&self) -> &i32 {
            &self.percent
        }
    }
}

mod lib {
    use super::util::Percentage;

    #[derive(impl_new::New)]
    pub struct Class {
        subject: String,
        teacher_name: String,
    }

    pub struct StudentClass<'a> {
        class: &'a mut Class,
        grade: Percentage,
        absenses: i8,
    }

    impl<'a> StudentClass<'a> {
        pub fn new(
            class: impl Into<&'a mut Class>,
            grade: impl Into<Percentage>,
            absenses: impl Into<i8>,
        ) -> Self {
            Self {
                class: class.into(),
                grade: grade.into(),
                absenses: absenses.into(),
            }
        }

        pub fn get_grade(&self) -> &Percentage {
            &self.grade
        }
    }

    pub struct Student<'a> {
        name: String,
        classes: Vec<StudentClass<'a>>,
    }

    impl<'a> Student<'a> {
        pub fn new(name: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                classes: Vec::new(),
            }
        }

        pub fn add_class(&mut self, class: StudentClass<'a>) {
            self.classes.push(class)
        }

        pub fn get_classes(&self) -> Vec<&StudentClass<'a>> {
            self.classes.iter().collect()
        }
    }
}

fn grade_average(student: &Student) -> i32 {
    let classes: &Vec<&StudentClass<'_>> = &student.get_classes();
    classes.iter().map(|x| x.get_grade().as_num()).sum::<i32>() / classes.len() as i32
}

use lib::{Class, Student, StudentClass};

fn main() {
    let mut collin = Student::new("collin");
    let mut aphg = Class::new("AP Human Geogramhy", "James Long");
    collin.add_class(StudentClass::new(&mut aphg, 90, 0));
}
