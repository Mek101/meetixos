/*! x86_64 programmable interrupt timer */

use crate::arch::x86_64::io_port::IoPort;

static mut SM_PIT_MANAGER: Option<PitManager> = None;

pub struct PitManager {
    m_control_port: IoPort<u8>,
    m_data_port: IoPort<u8>
}

impl PitManager /* Constants */ {
    pub const BASE_FREQUENCY: u64 = 1193182;
}

impl PitManager /* Constructors */ {
    pub fn init_instance() {
        unsafe {
            SM_PIT_MANAGER = Some(PitManager { m_control_port: IoPort::new(0x43),
                                               m_data_port: IoPort::new(0x40) });
        }
    }
}

impl PitManager /* Methods */ {
    pub fn enable_periodic(&self, divisor: u16) {
        unsafe {
            self.m_control_port.write(0x00 | 0x30 | 0x4 | 0x00);
            self.m_data_port.write((divisor & 0xff) as u8);
            self.m_data_port.write((divisor >> 8) as u8);
        }
    }

    pub fn read_counter(&self) -> u32 {
        unsafe {
            self.m_control_port.write(0x00);
            let mut left = self.m_data_port.read() as u32;
            left |= (self.m_data_port.read() as u32) << 8;
            left
        }
    }
}

impl PitManager /* Getters */ {
    pub fn instance() -> &'static Self {
        unsafe {
            SM_PIT_MANAGER.as_ref().expect("Called PitManager::instance() before \
                                            PitManager::init_instance()")
        }
    }
}
