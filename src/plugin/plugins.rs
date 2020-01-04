use super::{builtin, Plugin};
use crate::utils::Table;

pub type Plugins = Table<Box<dyn Plugin>>;

pub fn populate_builtin(plugins: &mut Plugins) {
    plugins.insert(Box::new(builtin::Loader::default()));
    plugins.insert(Box::new(builtin::Merge::default()));
}
