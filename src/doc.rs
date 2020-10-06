//
//  This Source Code Form is subject to the terms of the Mozilla Public
//  License, v. 2.0. If a copy of the MPL was not distributed with this
//  file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use regex::Regex;

// Something that is documented
#[derive(Debug)]
pub struct Doc {
    pub tag: Vec<String>,
    pub def: Definition,
}

macro_rules! is {
    ( $s:expr, $t:pat ) => {
        match $s.def {
            $t => true,
            _ => false,
        }
    };
}

impl Doc {
    pub fn new() -> Self {
        Doc {
            tag: Vec::new(),
            def: Definition::None,
        }
    }

    // Push a new string to the tag
    pub fn push(&mut self, s: String) {
        self.tag.push(s);
    }

    pub fn push_field(&mut self, f: Definition) {
        match &mut self.def {
            Definition::Class(c) => c.push_field(f),
            _ => panic!("Not a class"),
        }
    }

    // Set the definition 
    pub fn set_def(&mut self, def: Definition) {
        self.def = def;
    }

    pub fn is_class(&self) -> bool {
        is!(&self, Definition::Class(_))
    }

    pub fn is_field(&self) -> bool {
        is!(&self, Definition::Field(_))
    }

    pub fn name(&self) -> &str {
        match &self.def {
            Definition::Class(c) => &c.name,
            _ => ""
        }
    }
}

// An actual definition
#[derive(Debug)]
pub enum Definition {
    Class(ClassDef),
    Field(FieldDef),
    None,
}

impl Definition {
    // Derive a definition from regex
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

        // Return the correct type of data
        Some(match caps.name("type").unwrap().as_str() {
            // A class
            // TODO(@monarrk): clean this please dear god
            "class" => Definition::Class(ClassDef::new(
                    // Get the name
                    String::from(caps.name("name").unwrap().as_str()),
                    // Get the modifiers
                    String::from(match caps.name("modifier") {
                        Some(n) => n.as_str(),
                        None => "",
                    }),
                    // I spent so much time adding the infastructure to add arguments to classes
                    // which is stupid but I'm keeping this piece here so I can refer to it when I
                    // do methods
                    //match caps.name("args") {
                    //    Some(a) => a.as_str().split(",").map(|s| Variable::from_str(s)).collect::<Vec<Variable>>(),
                    //    None => Vec::new(),
                    //},
                    // Get the raw string with `{` taken off the end
                    s.trim_end_matches("{").to_string(),
                    Vec::new())),

            // Any other type, probably a variable?
            // TODO(@monarrk): can we ensure it is a var?
            _ => Definition::Field(FieldDef::new(
                // Get the name
                String::from(caps.name("name").unwrap().as_str()),
                // Get the modifiers
                String::from(match caps.name("modifier") {
                    Some(m) => m.as_str(),
                    None => "",
                }),
                // Raw definition string
                s.to_string())),
        })
    }
}

// A field definition
#[derive(Debug)]
pub struct FieldDef {
    pub name: String,
    pub modifiers: String,
    pub raw: String,
}

impl FieldDef {
    pub fn new(name: String, modifiers: String, raw: String) -> Self {
        FieldDef {
            name,
            modifiers,
            raw
        }
    }
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub ty: String,
}

impl Variable {
    pub fn new(name: String, ty: String) -> Self {
        Variable {
            name,
            ty
        }
    }

    pub fn from_str(s: &str) -> Self {
        let splt = s.split(" ").collect::<Vec<&str>>();
        Variable {
            name: splt[1].to_string(),
            ty: splt[0].to_string(),
        }
    }
}

// A class definition
#[derive(Debug)]
pub struct ClassDef {
    name: String,
    pub modifiers: String,
    raw: String,
    pub fields: Vec<Definition>,
}

impl ClassDef {
    pub fn new(name: String, modifiers: String, raw: String, fields: Vec<Definition>) -> Self {
        ClassDef {
            name,
            modifiers,
            raw,
            fields
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    // Return the raw definition line straight from the source code
    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn push_field(&mut self, f: Definition) {
        self.fields.push(f);
    }
}
