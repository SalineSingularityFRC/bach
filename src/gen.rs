//
//  This Source Code Form is subject to the terms of the Mozilla Public
//  License, v. 2.0. If a copy of the MPL was not distributed with this
//  file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use crate::doc::{Doc, Definition};

pub struct Generator<'a> {
    classes: Vec<&'a Doc>,
    theme: Theme,
    title: String,
    css: String,
    content: String,
    header: String,
}

impl<'a> Generator<'a> {
    pub fn new(title: String, classes: Vec<&'a Doc>, theme: Theme) -> Self {
        Generator {
            classes,
            theme,
            title,
            ..Generator::default()
        }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    pub fn generate(&mut self) -> String {
        self.css = self.theme.get();
        self.header = format!(r#"<h1>{}</h1>"#, self.title);

        for c in self.classes.iter() {
            let d = match &c.def {
                Definition::Class(c) => c,
                _ => panic!("Not a class!"),
            };

            let tag = c.tag
                .iter()
                .map(|s| s.trim_start_matches("///"))
                .collect::<Vec<&str>>()
                .join("");

            self.content += &format!(r#"
                                    <hr/>
                                    <h3>Class {title}</h3>
                                     <p>{tag}<p>
                                     "#, 
                                     title = d.get_name(),
                                     tag = tag);
        }

        // html template
        format!(r#"
                <!DOCTYPE html>
                <html>
                    <head>
                        <meta charset="utf-8"/>
                        <title>{title}</title>
                        <style>
                            {css}
                        </style>
                    </head>
                    <body>
                        {head}
                        {content}
                    </body>
                </html>"#,
                title = self.title,
                css = self.css,
                head = self.header,
                content = self.content)
    }
}

impl Default for Generator<'_> {
    fn default() -> Self {
        Generator {
            classes: Vec::new(),
            theme: Theme::Default,
            title: String::new(),
            css: String::new(),
            content: String::new(),
            header: String::new(),
        }
    }
}

pub enum Theme {
    Default,
}

impl Theme {
    pub fn get(&self) -> String {
        match self {
            Theme::Default => String::new(),
            _ => panic!("Not implemented!"),
        }
    }
}
