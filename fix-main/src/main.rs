use libloading::Library;
use fix_core::kernel::Kernel;
use fix_core::plugins::{Plugin, PluginInitFn};

fn main() {
    let library = unsafe {
        Library::new("fix_plugin").expect("failed to load plugin")
    };
    println!("shared library loaded!");

    let kernel = Kernel::new();
    println!("kernel created!");

    let plugin = unsafe {
        let plugin_init_fn = library.get::<PluginInitFn>(b"_plugin_init").expect("failed to get _plugin_init function");
        Box::from_raw(plugin_init_fn())
    };
    println!("plugin init!");

    plugin.install(&kernel);
    println!("plugin installed!");

    println!("kernel is running");
    kernel.run();
    println!("kernel quit!");

    plugin.uninstall();
    println!("plugin uninstalled!");

    drop(library);
    println!("shared library dropped!");
}
