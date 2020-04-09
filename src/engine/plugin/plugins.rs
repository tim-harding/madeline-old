use super::{builtin, Plugin};
use crate::{utils::Id};
use std::collections::HashMap;

type Create = fn() -> Plugin;

struct Context<'a> {
    plugins: &'a mut HashMap<Id, Plugin>,
    names: &'a mut HashMap<String, Id>,
}

#[allow(clippy::implicit_hasher)]
pub fn populate_builtin(plugins: &mut HashMap<Id, Plugin>, names: &mut HashMap<String, Id>) {
    let mut id = 0;
    let creates = [
        builtin::load::create,
        builtin::merge::create,
        builtin::shuffle::create,
        builtin::blur::create,
        builtin::adjust::create,
        builtin::resize::create,
        builtin::crop::create,
    ];
    let mut ctx = Context { plugins, names };
    for create in creates.iter() {
        add_plugin(&mut ctx, *create, id);
        id += 1;
    }
}

fn add_plugin(ctx: &mut Context, create: Create, id: Id) {
    let plugin = create();
    let name = plugin.desc().name.clone();
    ctx.plugins.insert(id, plugin);
    ctx.names.insert(name, id);
}
