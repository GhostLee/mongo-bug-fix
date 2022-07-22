use futures::TryStreamExt;
use mongodb::Client;
use mongodb::options::ClientOptions;
use fix_core::kernel::{Builder, Kernel};
use fix_core::plugins::Plugin;

#[no_mangle]
pub extern "C" fn _plugin_init() -> *mut fix_core::plugins::FFISafePlugin {
    let object = MongoPlugin::default();
    let boxed: Box<fix_core::plugins::FFISafePlugin> = fix_core::plugins::FFISafePlugin::new(object);
    Box::into_raw(boxed)
}

#[derive(Default, Debug)]
pub struct MongoPlugin{}

impl Plugin for MongoPlugin {
    fn install(&self, kernel: &Kernel) {
        println!("install mongo plugin!");
        let rtm = Builder::new_multi_thread().enable_all().build().expect("create temporary tokio runtime failed");
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
        println!("run mongo api with temporary tokio runtime created in current shared library");
        rtm.block_on(async {
            println!("now we are in async function");
            // Parse a connection string into an options struct.
            let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await.unwrap();
            println!("now we are in async function: client_options created");

            // Manually set an option.
            client_options.app_name = Some("mongo-bug-fix-client".to_string());
            println!("now we are in async function: client_options.app_name set");

            // Get a handle to the deployment.
            let client = Client::with_options(client_options).unwrap();
            println!("now we are in async function: client created");
            let database = client.database("local");
            println!("now we are in async function: database created");
            // List the names of the collections in that database.
            for collection_name in database.list_collection_names(None).await.expect("cannot list collection names") {
                println!("{}", collection_name);
            }
            println!("now we are in async function: collection_name list finished");
        });
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
        println!("run mongo api with existing tokio runtime created in main executable");
        kernel.runtime.block_on(async {
            println!("now we are in async function");
            // Parse a connection string into an options struct.
            let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await.unwrap();
            println!("now we are in async function: client_options created");

            // Manually set an option.
            client_options.app_name = Some("mongo-bug-fix-client".to_string());
            println!("now we are in async function: client_options.app_name set");

            // Get a handle to the deployment.
            let client = Client::with_options(client_options).unwrap();
            println!("now we are in async function: client created");
            let database = client.database("local");
            println!("now we are in async function: database created");
            // List the names of the collections in that database.
            for collection_name in database.list_collection_names(None).await.expect("cannot list collection names") {
                println!("{}", collection_name);
            }
            println!("now we are in async function: collection_name list finished");
        });
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    }

    fn uninstall(&self) {
        println!("uninstall mongo plugin!");
    }
}
