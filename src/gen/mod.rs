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
            format!("<h5>Modifiers</h5>\n<ul>{}</ul>", 
                    $m.trim()
                        .split(" ")
                        .map(|m| format!("<li><code>{}</code></li>", m))
                        .collect::<Vec<String>>()
                        .join("\n"))
        } else {
            String::new()
        }
    };
}

// Format fields to html
// for the class template in Generator::generate()
// TODO(@monarrk): what the fuck
macro_rules! format_fields {
    ( $m:expr ) => {
        {
            let mut s = String::new();
            if $m.len() > 0 {
                s = String::from("<h5>Fields</h5>\n");
                for i in $m.iter() {
                    s += format!("<table><tr><th>Description</th</tr><td><code>{tag}</code></td><tr><th>Name</th><th>Definition</th></tr>{insert}</table><br/>",
                        insert = match &i.def {
                            Definition::Field(f) => {
                                format!(r"<tr><td><code>{name}</code></td><td><code>{definition}</code></td></tr>",
                                         name = f.name,
                                         definition = f.raw)
                            },
                            Definition::Method(m) => {
                                format!(r"<tr><td><code>{name}</code></td><td><code>{definition}</code></td></tr>",
                                        name = m.name,
                                        definition = m.raw.trim().trim_end_matches("{"))
                            },
                            _ => String::new(),
                        }, 
                        tag = i.tag.iter()
                            .map(|t| t.trim().trim_start_matches("///"))
                            .collect::<Vec<&str>>()
                            .join("<br/>")
                        ).as_str()
                }
            }
            s
        }
    };
}

// Output html for the sidebar
macro_rules! sidebar {
    ( $x:expr ) => {
        {
            let mut s = String::new();
            if $x.contains_classes() {
                s += "<h4 class=\"sidebar-head\"><a href=\"#classes\">Classes</a></h4>\n";
                s += "<ul>\n";
                for c in &$x.classes {
                    s += &format!("<li class=\"sidebar-item\"><a href=\"#class-{class}\">{class}</a></li>", class = c.name());
                }
                s += "</ul>\n";
            }
            s
        }
    };
}

// A generator type for generating the documentation
pub struct Generator<'a> {
    pub(crate) classes: Vec<&'a Doc<'a>>,
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

    pub fn contains_classes(&self) -> bool {
        self.classes.len() != 0
    }

    // Return a String of generated HTML derived from the information
    pub fn generate(&mut self) -> String {
        // Set the easy stuff
        self.css = self.theme.get();
        self.header = format!(r#"<h1>Package {}</h1>"#, self.title);

        self.content += "<h1 id=\"classes\"><a href=\"#classes\" class=\"section-head\">Classes</a></h1>\n";
        for c in self.classes.iter() {
            // Unwrap our class
            // TODO(@monarrk): Make this safe? Probably?
            let d = match &c.def {
                Definition::Class(c) => c,
                _ => panic!("Not a class!"),
            };

            let tag = c.tag
                .iter()
                .map(|s| s.trim().trim_start_matches("///"))
                .collect::<Vec<&str>>()
                .join("<br>");

            // Add a new block to the content with our class
            self.content += &format!(r#"
                                    <hr/>
                                    <div class="block">
                                    <h3>Class <span class="sub" id="class-{title}"><b><code>{title}</code></b></span></h3>
                                    <p>{tag}<p>
                                    {modifiers}
                                    {fields}
                                    <h5>Definition</h5>
                                    <p><code>{definition}</code></p>
                                    </div>
                                     "#, 
                                     title = d.get_name(),
                                     tag = tag,
                                     definition = d.raw(),
                                     modifiers = format_modifiers!(d.modifiers),
                                     fields = format_fields!(d.fields));
        }

        // html template
        format!(r#"
                <!DOCTYPE html>
                <html>
                    <head>
                        <meta charset="utf-8"/>
                        <title>Package {title}</title>
                        <style>
                            {css}
                        </style>
                    </head>
                    <body>
                        <div class="sidebar">
                            {bar}
                        </div>

                        <div class="main">
                            {head}
                            {content}

                            <br/>
                            <h6>Generated with <a href="https://github.com/SalineSingularityFRC/bach" target="_blank">Bach</a> with the {theme} theme</h6>
                        </div>
                    </body>
                </html>"#,
                title = self.title,
                css = self.css,
                head = self.header,
                content = self.content,
                bar = sidebar!(self),
                theme = self.theme.name())
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

    pub fn name(&self) -> &str {
        match &self {
            Theme::Default => "default",
            // _ => "undefined",
        }
    }
}
