use fix_core::kernel::Kernel;
use fix_core::plugins::Plugin;
use fix_plugin::MongoPlugin;

fn main() {

    let kernel = Kernel::new();
    println!("kernel created!");

    let plugin = Box::new(MongoPlugin::default());
    println!("plugin init!");

    plugin.install(&kernel);
    println!("plugin installed!");

    println!("kernel is running");
    kernel.run();
    println!("kernel quit!");

    plugin.uninstall();
    println!("plugin uninstalled!");

}
