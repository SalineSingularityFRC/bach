//
//  This Source Code Form is subject to the terms of the Mozilla Public
//  License, v. 2.0. If a copy of the MPL was not distributed with this
//  file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

mod theme;

use crate::doc::{Doc, Definition};

// Format modifiers
// for the class template in Generator::generate()
macro_rules! format_modifiers {
    ( $m:expr ) => {
        if $m.len() != 0 {
            format!("<h5>Modifiers</h5>\n{}", 
                    $m.split(" ")
                        .map(|m| format!("<ul><code>{}</code></ul>", m))
                        .collect::<Vec<String>>()
                        .join("\n"))
        } else {
            String::new()
        }
    };
}


// A generator type for generating the documentation
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

    // allow dead code for now because we don't use this yet
    #[allow(dead_code)]
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    // Return a String of generated HTML derived from the information
    pub fn generate(&mut self) -> String {
        // Set the easy stuff
        self.css = self.theme.get();
        self.header = format!(r#"<h1>{}</h1>"#, self.title);

        for c in self.classes.iter() {
            // Unwrap our class
            // TODO(@monarrk): Make this safe? Probably?
            let d = match &c.def {
                Definition::Class(c) => c,
                _ => panic!("Not a class!"),
            };

            let tag = c.tag
                .iter()
                .map(|s| s.trim_start_matches("///"))
                .collect::<Vec<&str>>()
                .join("");

            // Add a new block to the content with our class
            self.content += &format!(r#"
                                    <hr/>
                                    <div class="block">
                                    <h3>Class <span class="sub"><b><code>{title}</code></b></span></h3>
                                    <p>{tag}<p>
                                    {modifiers}
                                    <h5>Definition</h5>
                                    <p><code>{definition}</code></p>
                                    </div>
                                     "#, 
                                     title = d.get_name(),
                                     tag = tag,
                                     definition = d.raw(),
                                     modifiers = format_modifiers!(d.modifiers));
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

                        <br/>
                        <h6>Generated with <a href="https://github.com/SalineSingularityFRC/bach" target="_blank">Bach</a></h6>
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

// Enumeration of each theme
pub enum Theme {
    Default,
}

impl Theme {
    // Read the css theme file corresponding to the theme
    pub fn get(&self) -> String {
        match self {
            Theme::Default => theme::DEFAULT_THEME_CSS.to_owned()
        }
    }
}
