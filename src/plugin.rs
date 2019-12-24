mod plugin;
pub use plugin::Plugin;

mod desc;
pub use desc::Desc;

mod builtin;

use std::collections::HashMap;

type PluginMap = HashMap<String, Plugin>;

pub struct PluginsBuilder {
    plugins: PluginMap,
}

impl PluginsBuilder {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
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

impl Default for PluginsBuilder {
    fn default() -> Self {
        Self::new().with_plugin(builtin::uv_texture::PLUGIN)
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
