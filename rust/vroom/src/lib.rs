use savefile_derive::Savefile;

#[derive(Savefile)]
pub struct Measure {
    name: String,
    value: String,
}

impl Measure {
    fn new(name: &str, value: &str) -> Measure {
        Measure {
            name: name.into(),
            value: value.into(),
        }
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}

impl std::fmt::Display for Measure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.value)
    }
}

#[derive(Savefile)]
pub struct List {
    name: String,
    contents: Vec<Measure>,
}

impl List {
    pub fn new(name: &str) -> List {
        List {
            name: name.into(),
            contents: Vec::new(),
        }
    }
    pub fn add_measure(&mut self, name: &str, value: &str) {
        self.contents.push(Measure::new(name, value))
    }

    pub fn rm_measure(&mut self, name: &str) {
        self.contents.retain(|x| x.name != name)
    }

    pub fn get_all_names(&self) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();

        for i in self.contents.iter() {
            res.push(i.name.to_string())
        }

        res
    }

    pub fn get_mut_measure(&mut self, name: &str) -> Option<&mut Measure> {
        self.contents.iter_mut().find(|x| x.name == name)
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        for (index, item) in self.contents.iter().enumerate() {
            if index == self.contents.len() - 1 {
                write!(f, "    {item}")?;
            } else {
                writeln!(f, "    {item}")?;
            }
        }

        Ok(())
    }
}

#[derive(Savefile)]
pub struct Vroomfile {
    contents: Vec<List>,
}

impl Vroomfile {
    pub fn new() -> Vroomfile {
        Vroomfile {
            contents: Vec::new(),
        }
    }
    pub fn add_list(&mut self, list: List) {
        self.contents.push(list)
    }

    pub fn get_mut_list(&mut self, list_name: &str) -> Option<&mut List> {
        self.contents.iter_mut().find(|x| x.name == list_name)
    }

    pub fn get_all_names(&mut self) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();

        for i in self.contents.iter() {
            res.push(i.name.to_string())
        }

        res
    }

    pub fn rm_list(&mut self, list_name: &str) {
        self.contents.retain(|x| x.name != list_name)
    }

    pub fn fmt_overview(&self) -> String {
        let mut res: Vec<String> = Vec::new();
        for i in self.contents.iter() {
            let item_count = i.contents.len();
            res.push(format!("{} ({})", i.name, item_count));
        }

        res.join("\n")
    }
}

impl std::fmt::Display for Vroomfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.contents.iter() {
            writeln!(f, "{}", i)?;
        }

        Ok(())
    }
}
