use regex::Regex;

#[derive(Debug)]
pub struct Doc {
    tag: Vec<String>,
    def: Definition
}

impl Doc {
    pub fn new() -> Self {
        Doc {
            tag: Vec::new(),
            def: Definition::None,
        }
    }

    pub fn push(&mut self, s: String) {
        self.tag.push(s);
    }

    pub fn set_def(&mut self, def: Definition) {
        self.def = def;
    }
}

#[derive(Debug)]
pub enum Definition {
    Class(ClassDef),
    None,
}

impl Definition {
    pub fn derive(s: String) -> Option<Self> {
        // java class definition pattern
        // absolute hell
        // groupings are
        //   1. modifiers (optional)
        //   2. type (class, int, String, whatever)
        //   3. name
        //   4. arguments (optional)

        let pattern = Regex::new(r"(?i)\s*(?P<modifier>(public\s|private\s|final\s|default\s|protected\s|abstract\s|static\s|transient\s|synchronized\s|volatile\s)*)(?P<type>\S*) (?P<name>\S*)\s*[\((?P<args>.s*)\)]?.s*").expect("Failed to compile regex");
        
        let caps = match pattern.captures(&s) {
            Some(c) => c,
            None => return None
        };
        Some(match caps.name("type").unwrap().as_str() {
            "class" => Definition::Class(ClassDef::new(
                    String::from(caps.name("name").unwrap().as_str()),
                    String::from(match caps.name("modifier") {
                        Some(n) => n.as_str(),
                        None => "",
                    }))),
            _ => Definition::None,
        })
    }
}

#[derive(Debug)]
pub struct ClassDef {
    name: String,
    modifiers: String,
}

impl ClassDef {
    pub fn new(name: String, modifiers: String) -> Self {
        ClassDef {
            name,
            modifiers
        }
    }
}
