struct Student {
    name: String,
    grades: Vec<i32>,
}

impl Student {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn avg_grade(&self) -> f32 {
        self.grades.iter().sum::<i32>() as f32 / self.grades.len() as f32
    }

    fn fmt_report(&self) -> String {
        format!("{}: {}", self.name, self.avg_grade())
    }

    fn grade_to_letter(&self) -> char {
        match self.avg_grade() as i32 {
            90..=100 => 'A',
            80..=89 => 'B',
            70..=79 => 'C',
            60..=69 => 'D',
            0..=59 => 'F',
            e => panic!("Invalid grade: {}", e),
        }
    }
}

struct Class {
    teacher: String,
    students: Vec<Student>,
}

impl Class {
    fn fmt_report(&self) -> String {
        let mut res = Vec::<String>::new();

        for i in self.students.iter() {
            res.push(i.fmt_report())
        }

        res.join("\n")
    }
}
