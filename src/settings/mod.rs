// This file is part of Cicero.
//
// Cicero is free software: you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// Cicero is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
// A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// Cicero. If not, see <https://www.gnu.org/licenses/>.

#[allow(clippy::module_inception)] // Can't think of a better name than "settings"
mod settings;
pub use settings::Settings;

pub fn get_settings() -> Settings {
    match read_settings_file() {
        Ok(settings) => settings,      // Return the settings read from file
        Err(_) => Settings::default(), // Failed to read the settings file, return default settings
    }
}

use std::error;
use std::fmt;
use std::fs::File;
use std::io::Read;

const SETTINGS_FILE_PATH_COMPONENT_CONFIG: &str = ".config";
const SETTINGS_FILE_PATH_COMPONENT_CICERO: &str = "cicero";
const SETTINGS_FILE_PATH_COMPONENT_SETTINGS: &str = "settings.toml";

#[derive(Debug)]
enum Error {
    FailedToGetHomeDir,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FailedToGetHomeDir => write!(f, "Unable to get home directory"),
        }
    }
}

impl error::Error for Error {}

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn read_settings_file() -> Result<Settings> {
    let settings_path = {
        let mut home_path = dirs::home_dir().ok_or(Error::FailedToGetHomeDir)?;
        home_path.push(SETTINGS_FILE_PATH_COMPONENT_CONFIG);
        home_path.push(SETTINGS_FILE_PATH_COMPONENT_CICERO);
        home_path.push(SETTINGS_FILE_PATH_COMPONENT_SETTINGS);
        home_path
    };

    let mut file = File::open(settings_path)?;

    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    match toml::from_str(&file_content) {
        Ok(settings) => Ok(settings),
        Err(error) => Err(Box::new(error)),
    }
}
