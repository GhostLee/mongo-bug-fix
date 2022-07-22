use std::ops::Deref;
use std::sync::Arc;
pub use tokio::runtime::{Builder, Runtime};

pub struct KernelInner{
    pub runtime: Runtime,
    // to simulate some event send by plugin
    rx: std::sync::mpsc::Receiver<bool>,
    tx: std::sync::mpsc::Sender<bool>,
}

impl KernelInner {

    pub fn run(&self) {
        println!("kernel listening...");
        for event in &self.rx {
            if event == true {
                return;
            }
        }
    }
}
pub struct Kernel(Arc<KernelInner>);

impl Kernel {
    pub fn new() -> Self{
        let (tx, rx) = std::sync::mpsc::channel();

        Self(Arc::new(KernelInner{
            runtime: Builder::new_multi_thread().enable_all().build().expect("build tokio runtime failed"),
            rx,
            tx
        }))
    }
}

impl Deref for Kernel {
    type Target = KernelInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}