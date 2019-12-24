use super::{builtin, Plugin};
use std::collections::HashMap;

type PluginMap = HashMap<String, Plugin>;

pub struct PluginsBuilder {
    plugins: PluginMap,
}

impl PluginsBuilder {
    fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn builtin() -> Self {
        Self::new().with_plugin(builtin::uv_texture::PLUGIN)
    }

    pub fn with_plugin(mut self, plugin: Plugin) -> Self {
        let key = plugin.desc.name.into();
        self.plugins.insert(key, plugin);
        self
    }

    pub fn build(self) -> Plugins {
        Plugins::new(self.plugins)
    }
}

pub struct Plugins {
    plugins: PluginMap,
}

impl Plugins {
    pub(self) fn new(plugins: PluginMap) -> Self {
        Self { plugins }
    }

    pub fn get(&self, name: &str) -> Option<&Plugin> {
        self.plugins.get(name)
    }
}
