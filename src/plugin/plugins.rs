use super::{builtin, Plugin};
use crate::{
    utils::Id,
    utils::Table,
};
use std::collections::HashMap;

type Create = fn() -> Plugin;

struct Context<'a> {
    plugins: &'a mut Table<Plugin>,
    names: &'a mut HashMap<String, Id>,
}

pub fn populate_builtin(plugins: &mut Table<Plugin>, names: &mut HashMap<String, Id>) {
    let creates = [
        builtin::load::create,
        builtin::merge::create,
        builtin::shuffle::create,
        builtin::blur::create,
        builtin::correct::create,
        builtin::gamma::create,
        builtin::resize::create,
        builtin::crop::create,
    ];
    let mut ctx = Context { plugins, names };
    for create in creates.iter() {
        add_plugin(&mut ctx, *create);
    }
}

fn add_plugin(ctx: &mut Context, create: Create) {
    let plugin = create();
    let name = plugin.desc().name.clone();
    let id = ctx.plugins.insert(plugin);
    ctx.names.insert(name, id);
}
