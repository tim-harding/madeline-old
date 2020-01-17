use super::{builtin, Plugin};
use crate::utils::Table;

pub fn populate_builtin(plugins: &mut Table<Plugin>) {
    plugins.insert(builtin::loader::create());
    plugins.insert(builtin::merge::create());
    plugins.insert(builtin::shuffle::create());
    plugins.insert(builtin::blur::create());
}
