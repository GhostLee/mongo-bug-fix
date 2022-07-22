use std::any::Any;
use crate::kernel::Kernel;

#[allow(dead_code)]
pub type PluginInitFn = unsafe fn() -> *mut FFISafePlugin;

pub trait Plugin: Any + Send + Sync{
    fn install(&self, kernel: &Kernel);
    fn uninstall(&self);
}
pub struct FFISafePlugin(Box<dyn Plugin>);

impl FFISafePlugin {
    pub fn new<T>(v: T) -> Box<Self> where T: Plugin {
        Box::new(Self(Box::new(v)))
    }
}

impl Plugin for FFISafePlugin {

    fn install(&self, kernel: &Kernel) {
        self.0.install(kernel)
    }

    fn uninstall(&self) {
        self.0.uninstall()
    }
}
