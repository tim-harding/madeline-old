use super::{builtin, Plugin};
use std::collections::HashMap;

type PluginMap = HashMap<String, Box<dyn Plugin>>;

#[derive(Default)]
pub struct PluginsBuilder {
    plugins: PluginMap,
}

impl PluginsBuilder {
    pub fn builtin() -> Self {
        Self::default()
            .with_plugin(Box::new(builtin::Loader::default()))
            .with_plugin(Box::new(builtin::Merge::default()))
    }

    pub fn with_plugin(mut self, plugin: Box<dyn Plugin>) -> Self {
        let key = plugin.desc().name.into();
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

    pub fn get(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.get(name).map(|boxed| &**boxed)
    }
}
