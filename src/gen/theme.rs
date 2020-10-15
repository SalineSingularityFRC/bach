//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

// Read the default CSS file into a static str to use for later
pub static DEFAULT_THEME_CSS: &'static str = include_str!("html/default.css");

// Enumeration of each theme
pub enum Theme {
    Default,
}

impl Theme {
    // Read the css theme file corresponding to the theme
    pub fn get(&self) -> String {
        match self {
            Theme::Default => DEFAULT_THEME_CSS.to_owned()
        }
    }

    pub fn name(&self) -> &str {
        match &self {
            Theme::Default => "default",
            // _ => "undefined",
        }
    }
}
