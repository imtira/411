/*
 * Copyright (C) 2020 github.com/t1ra
 *
 * This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a
 * copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{error, fmt};

#[derive(Clone)]
pub enum Too {
    Few,
    Many,
}

#[derive(Clone)]
pub struct ArgsError {
    pub reason: Too,
}

impl fmt::Debug for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to parse args, too {}.",
            match &self.reason {
                Too::Few => "few arguments. Provide a file?".to_string(),
                Too::Many => format!(
                    "many arguments? There should be under {}",
                    usize::max_value()
                ),
            }
        )
    }
}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to parse args, too {}.",
            match &self.reason {
                Too::Few => "few arguments. Provide a file?".to_string(),
                Too::Many => format!(
                    "many arguments? There should be under {}",
                    usize::max_value()
                ),
            }
        )
    }
}

impl error::Error for ArgsError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
