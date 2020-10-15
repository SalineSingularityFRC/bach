//
//  This Source Code Form is subject to the terms of the Mozilla Public
//  License, v. 2.0. If a copy of the MPL was not distributed with this
//  file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use std::marker::PhantomData;

use regex::Regex;

// Something that is documented
#[derive(Debug, Clone)]
pub struct Doc<'a> {
    pub tag: Vec<String>,
    pub def: Definition<'a>,
    pub pkg: String,
    _marker: PhantomData<&'a ()>,
}

macro_rules! is {
    ( $s:expr, $t:pat ) => {
        match $s.def {
            $t => true,
            _ => false,
        }
    };
}

impl<'a> Doc<'a> {
    pub fn new(pkg: String) -> Self {
        Doc {
            tag: Vec::new(),
            def: Definition::None,
            pkg,
            _marker: PhantomData,
        }
    }

    // Push a new string to the tag
    pub fn push(&mut self, s: String) {
        self.tag.push(s);
    }

    pub fn push_field(&mut self, f: Doc<'a>) {
        match &mut self.def {
            Definition::Class(c) => c.push_field(f),
            _ => panic!("Not a class"),
        }
    }

    // Set the definition 
    pub fn set_def(&mut self, def: Definition<'a>) {
        self.def = def;
    }

    pub fn is_class(&self) -> bool {
        is!(&self, Definition::Class(_))
    }

    /*
    pub fn is_field(&self) -> bool {
        is!(&self, Definition::Field(_))
    }

    pub fn is_method(&self) -> bool {
        is!(&self, Definition::Method(_))
    }
    */

    pub fn name(&self) -> &str {
        match &self.def {
            Definition::Class(c) => &c.name,
            _ => ""
        }
    }
}

// An actual definition
#[derive(Debug, Clone)]
pub enum Definition<'a> {
    Class(ClassDef<'a>),
    Field(FieldDef),
    Method(MethodDef),
    None,
}

impl<'a> Definition<'a> {
    // Derive a definition from regex
    pub fn derive(s: String) -> Option<Self> {
        // java class definition pattern
        // absolute hell
        // groupings are
        //   1. modifiers (optional)
        //   2. type (class, int, String, whatever)
        //   3. name
        //   4. arguments (optional)

        let pattern = Regex::new(r"(?i)\s*(?P<modifier>(public\s|private\s|final\s|default\s|protected\s|abstract\s|static\s|transient\s|synchronized\s|volatile\s)*)(?P<type>\S*) (?P<name>[[:alnum:]]*)(\((?P<args>[^\(\)]*)\))?.s*").expect("Failed to compile regex");
        
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
                    caps.name("name").unwrap().as_str().to_owned(),
                    // Get the modifiers
                    match caps.name("modifier") {
                        Some(n) => n.as_str().to_owned(),
                        None => String::new(),
                    },
                    // Get the raw string with `{` taken off the end
                    s.trim_end_matches("{").to_owned(),
                    Vec::new())),

            // Any other type without args, probably a variable?
            // TODO(@monarrk): can we ensure it is a var?
            _ if caps.name("args").is_none() => Definition::Field(FieldDef::new(
                // Get the name
                caps.name("name").unwrap().as_str().to_owned(),
                // Get the modifiers
                match caps.name("modifier") {
                    Some(m) => m.as_str().to_owned(),
                    None => String::new(),
                },
                // Raw definition string
                s.clone())),


            // Hopefully this is a method lol
            _ if caps.name("args").is_some() => Definition::Method(MethodDef::new(
                // Get the name
                caps.name("name").unwrap().as_str().to_owned(),
                // Get the modifiers
                match caps.name("modifier") {
                    Some(n) => n.as_str().to_owned(),
                    None => String::new(),
                },
                // Get the arguments and put them into a Vec<Variable>
                match caps.name("args") {
                    Some(a) => {
                        if a.as_str().len() != 0 {
                            Some(a.as_str().split(",").map(|s| Variable::from_str(s)).collect::<Vec<Variable>>())
                        } else {
                            None
                        }
                    },
                    None => None,
                },
                // Get the raw string for the decl
                s.trim_end_matches("{").to_owned())),

            // Fallback if something really weird happens
            _ => Definition::None,
        })
    }
}

// A variable wrapper struct
#[derive(Debug, Clone)]
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
        Self::new(splt[1].to_owned(), splt[0].to_owned())
    }
}

// A method definition
#[derive(Debug, Clone)]
pub struct MethodDef {
    pub name: String,
    pub modifiers: String,
    pub args: Option<Vec<Variable>>,
    pub raw: String,
}

impl MethodDef {
    pub fn new(name: String, modifiers: String, args: Option<Vec<Variable>>, raw: String) -> Self {
        MethodDef {
            name,
            modifiers,
            args,
            raw
        }
    }
}

// A field definition
#[derive(Debug, Clone)]
pub struct FieldDef {
    pub name: String,
    pub modifiers: String,
    pub raw: String,
}

impl FieldDef {
    pub fn new(name: String, modifiers: String, raw: String) -> Self {
        FieldDef {
            name: name.clone(),
            modifiers: modifiers.clone(),
            raw: raw.clone()
        }
    }
}

// A class definition
#[derive(Debug, Clone)]
pub struct ClassDef<'a> {
    name: String,
    pub modifiers: String,
    raw: String,
    pub fields: Vec<Doc<'a>>,
}

impl<'a> ClassDef<'a> {
    pub fn new(name: String, modifiers: String, raw: String, fields: Vec<Doc<'a>>) -> Self {
        ClassDef {
            name: name.clone(),
            modifiers: modifiers.clone(),
            raw: raw.clone(),
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

    pub fn push_field(&mut self, f: Doc<'a>) {
        self.fields.push(f);
    }
}
