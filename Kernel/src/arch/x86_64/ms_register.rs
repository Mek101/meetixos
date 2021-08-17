/*! Model Specific Register */

/**
 * Generic x86_64 Model Specific Register
 */
pub struct MsRegister {
    m_register: u32
}

impl MsRegister /* Constructors */ {
    pub const fn new_star() -> Self {
        Self::new(0xc0000081)
    }

    pub const fn new_lstar() -> Self {
        Self::new(0xc0000082)
    }

    pub const fn new_fmask() -> Self {
        Self::new(0xc0000084)
    }

    /**
     * Constructs a `MsRegister` with the given value
     */
    pub const fn new(register: u32) -> Self {
        Self { m_register: register }
    }
}

impl MsRegister /* Methods */ {
    /**
     * Reads the value of this `MsRegister`
     */
    pub unsafe fn read(&self) -> u64 {
        let (high, low): (u32, u32);
        asm!("rdmsr",
             in("ecx") self.m_register,
             out("eax") low,
             out("edx") high,
             options(nomem, nostack, preserves_flags));
        ((high as u64) << 32) | (low as u64)
    }

    /**
     * Overwrites the value of this `MsRegister`
     */
    pub unsafe fn write(&self, value: u64) {
        let (high, low) = ((value >> 32) as u32, value as u32);
        asm!("wrmsr",
             in("ecx") self.m_register,
             in("eax") low,
             in("edx") high,
             options(nostack, preserves_flags))
    }
}
