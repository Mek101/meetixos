/*! Kernel device drivers */

use alloc::{
    collections::BTreeMap,
    string::String,
    sync::Arc
};

use api_data::object::device::DeviceId;
use sync::SpinRwLock;

use crate::dev::{
    random::TRandomDevice,
    uart::TUartDevice
};

pub mod random;
pub mod uart;

static mut SM_DEV_MANAGER: Option<DevManager> = None;

pub struct DevManager {
    m_devices: SpinRwLock<BTreeMap<DeviceId, Arc<dyn TDevice>>>
}

impl DevManager /* Constructors */ {
    pub fn early_init() {
        unsafe {
            assert!(SM_DEV_MANAGER.is_none(),
                    "Called DevManager::init_instance() twice!");

            let dev_manager = Self { m_devices: SpinRwLock::const_new(BTreeMap::new()) };
            dev_manager.register_early_devices();

            SM_DEV_MANAGER = Some(dev_manager)
        }
    }
}

impl DevManager /* Methods */ {
    pub fn register_device<T>(&self, device_driver: T) -> bool
        where T: TDevice + 'static {
        let already_exists =
            self.m_devices.read().contains_key(&device_driver.device_id());

        if !already_exists {
            self.m_devices
                .write()
                .insert(device_driver.device_id(), Arc::new(device_driver))
                .is_none()
        } else {
            false
        }
    }
}

impl DevManager /* Getters */ {
    pub fn instance() -> &'static Self {
        unsafe {
            SM_DEV_MANAGER.as_ref().expect("Tried to obtain DevManager instance before \
                                            initialization")
        }
    }
}

impl TDevice for Arc<dyn TDevice> {
    fn device_id(&self) -> DeviceId {
        (**self).device_id()
    }

    fn device_name(&self) -> String {
        (**self).device_name()
    }

    fn as_random(&self) -> Option<&dyn TRandomDevice> {
        (**self).as_random()
    }

    fn as_uart(&self) -> Option<&dyn TUartDevice> {
        (**self).as_uart()
    }
}

pub trait TDevice {
    fn device_id(&self) -> DeviceId;

    fn device_name(&self) -> String;

    fn as_random(&self) -> Option<&dyn TRandomDevice> {
        None
    }

    fn as_uart(&self) -> Option<&dyn TUartDevice> {
        None
    }
}
