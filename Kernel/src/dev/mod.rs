/*! Kernel device drivers */

use alloc::{
    collections::BTreeMap,
    string::String,
    sync::Arc,
    vec::Vec
};

use api_data::object::device::{
    DeviceId,
    DeviceIdClass
};
use sync::SpinRwLock;

use crate::dev::{
    random::TRandomDevice,
    uart::TUartDevice
};

pub mod random;
pub mod uart;

/* <None> until <DevManager::early_init()> is called */
static mut SM_DEV_MANAGER: DevManager =
    DevManager { m_devices: SpinRwLock::const_new(BTreeMap::new()) };

/**
 * Kernel centralized `TDevice` driver manager
 */
pub struct DevManager {
    m_devices: SpinRwLock<BTreeMap<DeviceId, Arc<dyn TDevice>>>
}

impl DevManager /* Constructors */ {
    /**
     * Registers fundamentals device drivers
     */
    pub fn early_init() {
        unsafe {
            assert_eq!(SM_DEV_MANAGER.m_devices.read().len(),
                       0,
                       "Called DevManager::early_init() twice!");

            /* NOTE <DevManager::register_early_devices()> is implemented into
             * Kernel/arch/<arch_name>/dev/mod.rs.
             *
             * Each architecture must implement it and here must be registered at least
             * the random device and the serial device
             */
            SM_DEV_MANAGER.register_early_devices();
        }
    }
}

impl DevManager /* Methods */ {
    /**
     * Registers a device driver and initializes it. Returns whether the
     * driver wasn't already existing or was unable to initialize it
     */
    pub fn register_device<T>(&self, device_driver: T) -> bool
        where T: TDevice + 'static {
        let already_exists =
            self.m_devices.read().contains_key(&device_driver.device_id());

        if !already_exists {
            /* move the device driver instance to the heap */
            let heap_device_driver = Arc::new(device_driver);

            /* initialize the device driver hardware */
            if heap_device_driver.init_hw() {
                self.m_devices
                    .write()
                    .insert(heap_device_driver.device_id(), heap_device_driver)
                    .is_none()
            } else {
                false
            }
        } else {
            false
        }
    }

    /**
     * Removes a previously registered device driver and returns it
     */
    pub fn unregister_device(&self, device_id: DeviceId) -> Option<Arc<dyn TDevice>> {
        self.m_devices.write().remove(&device_id)
    }
}

impl DevManager /* Getters */ {
    /**
     * Returns the global `DevManager` instance
     */
    pub fn instance() -> &'static Self {
        unsafe { &SM_DEV_MANAGER }
    }

    /**
     * Returns the device driver which corresponds to the given `DeviceId`
     */
    pub fn device_by_id(&self, device_id: DeviceId) -> Option<Arc<dyn TDevice>> {
        self.m_devices.read().get(&device_id).map(|device| device.clone())
    }

    /**
     * Returns a `Vec` of device drivers with the same `DeviceIdClass`
     */
    pub fn enumerate_by_class(&self,
                              device_class: DeviceIdClass)
                              -> Option<Vec<Arc<dyn TDevice>>> {
        let unlocked_devices = self.m_devices.read();
        let mut devices_by_class = Vec::with_capacity(unlocked_devices.len());

        /* collect all the device drivers of the same class */
        for (device_id, device_driver) in unlocked_devices.iter() {
            if device_id.device_class() == device_class {
                devices_by_class.push(device_driver.clone())
            }
        }

        /* return the devices only if the <Vec> has elements */
        if !devices_by_class.is_empty() {
            Some(devices_by_class)
        } else {
            None
        }
    }

    /**
     * Returns the first device driver with the given `DeviceIdClass`
     */
    pub fn device_by_class(&self,
                           device_class: DeviceIdClass)
                           -> Option<Arc<dyn TDevice>> {
        for (device_id, device_driver) in self.m_devices.read().iter() {
            if device_id.device_class() == device_class {
                return Some(Arc::clone(device_driver));
            }
        }
        None
    }
}

/**
 * Base interface for all the device drivers
 */
pub trait TDevice {
    /**
     * Returns the `DeviceId` of this device driver
     */
    fn device_id(&self) -> DeviceId;

    /**
     * Returns the device driver human-readable name
     */
    fn device_name(&self) -> String;

    /**
     * Initializes the underling hardware and returns whether it is
     * available and right initialized
     */
    fn init_hw(&self) -> bool;

    /**
     * Downcast this `TDevice` to a `TRandomDevice`
     */
    fn as_random(&self) -> Option<&dyn TRandomDevice> {
        None
    }

    /**
     * Downcast this `TDevice` to a `TUartDevice`
     */
    fn as_uart(&self) -> Option<&dyn TUartDevice> {
        None
    }
}

impl TDevice for Arc<dyn TDevice> {
    fn device_id(&self) -> DeviceId {
        (**self).device_id()
    }

    fn device_name(&self) -> String {
        (**self).device_name()
    }

    fn init_hw(&self) -> bool {
        (**self).init_hw()
    }

    fn as_random(&self) -> Option<&dyn TRandomDevice> {
        (**self).as_random()
    }

    fn as_uart(&self) -> Option<&dyn TUartDevice> {
        (**self).as_uart()
    }
}
