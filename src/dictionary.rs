use std::path::Path;

pub trait Dictionary {
    fn load_dictionary(&mut self, path: &Path) -> Result<(), std::io::Error>;
}
