/*! Programmable interrupt controller */

use crate::arch::x86_64::x64_port::X64Port;

/* <None> until <PicManager::init_instance()> is called */
static mut SM_PIC_MANAGER: Option<PicManager> = None;

/**
 * Programmable Interrupt Controller manager.
 *
 * This old structure is used when the underling hardware doesn't support
 * the Advanced Programmable Interrupt Controller (APIC)
 */
pub struct PicManager {
    m_master_pic: Pic,
    m_slave_pic: Pic
}

impl PicManager /* Constructors */ {
    /**
     * Initialize the global `PicManager` instance
     */
    pub fn init_instance() {
        unsafe {
            assert!(SM_PIC_MANAGER.is_none(), "Called PicManager::init_instance() twice");

            /* constructs the <PicManager> instance */
            let pic_manager =
                Self { m_master_pic: Pic { m_offset: 32,
                                           m_command: X64Port::new(0x20),
                                           m_data: X64Port::new(0x21) },
                       m_slave_pic: Pic { m_offset: 40,
                                          m_command: X64Port::new(0xa0),
                                          m_data: X64Port::new(0xa1) } };

            /* We need to add a delay between writes to our PICs, especially on older
             * motherboards.
             * But we don't necessarily have any kind of timers yet, because most of
             * them require interrupts.
             * Various older versions of Linux and other PC operating systems have
             * worked around this by writing garbage data to port 0x80, which allegedly
             * takes long enough to make everything work on most hardware
             */
            let write_wait = || X64Port::<u8>::new(0x80).write(0);

            /* send to the PICs the init command and tell them which we will write soon
             * into their data registry the 3 bytes sequence for initialization
             */
            pic_manager.m_master_pic.m_command.write(0x11);
            write_wait();
            pic_manager.m_slave_pic.m_command.write(0x11);
            write_wait();

            /* byte-1: Setup the base interrupt offsets */
            pic_manager.m_master_pic.m_data.write(pic_manager.m_master_pic.m_offset);
            write_wait();
            pic_manager.m_slave_pic.m_data.write(pic_manager.m_slave_pic.m_offset);
            write_wait();

            /* byte-2: Configure chaining between PIC-1 & PIC-2 */
            pic_manager.m_master_pic.m_data.write(0x04);
            write_wait();
            pic_manager.m_slave_pic.m_data.write(0x02);
            write_wait();

            /* byte-3: Set 8086-mode */
            pic_manager.m_master_pic.m_data.write(0x01);
            write_wait();
            pic_manager.m_slave_pic.m_data.write(0x01);
            write_wait();

            /* enable now all the interrupts */
            pic_manager.set_interrupts_masks(0, 0);

            /* initialize the global instance */
            SM_PIC_MANAGER = Some(pic_manager);
        }
    }
}

impl PicManager /* Methods */ {
    /**
     * Returns whether the given `interrupt_num` is handled by the
     * `PicManager`
     */
    pub fn can_handle_interrupt(&self, interrupt_num: u8) -> bool {
        [&self.m_master_pic, &self.m_slave_pic].iter()
                                               .any(|pic| {
                                                   pic.can_handle_interrupt(interrupt_num)
                                               })
    }

    /**
     * Notifies the end of the given `interrupt_num`
     */
    pub unsafe fn end_of_interrupt(&self, interrupt_num: u8) {
        if self.can_handle_interrupt(interrupt_num) {
            if self.m_master_pic.can_handle_interrupt(interrupt_num) {
                self.m_master_pic.end_of_interrupt()
            } else {
                self.m_slave_pic.end_of_interrupt()
            }
        }
    }

    /**
     * Disables the `PicManager`
     */
    pub unsafe fn disable(&self) {
        self.set_interrupts_masks(u8::MAX, u8::MAX)
    }
}

impl PicManager /* Getters */ {
    /**
     * Returns the global `PicManager` instance
     */
    pub fn instance() -> &'static Self {
        unsafe { SM_PIC_MANAGER.as_ref().expect("") }
    }

    /**
     * Returns the current interrupt mask for the two PICs
     */
    pub unsafe fn interrupt_masks(&self) -> [u8; 2] {
        [self.m_master_pic.interrupt_mask(), self.m_slave_pic.interrupt_mask()]
    }
}

impl PicManager /* Setters */ {
    /**
     * Overwrites the current interrupt mask for the two PICs
     */
    pub unsafe fn set_interrupts_masks(&self, master_mask: u8, slave_mask: u8) {
        self.m_master_pic.set_interrupt_mask(master_mask);
        self.m_slave_pic.set_interrupt_mask(slave_mask)
    }
}

/**
 * Programmable Interrupt Controller chip
 */
struct Pic {
    m_offset: u8,
    m_command: X64Port<u8>,
    m_data: X64Port<u8>
}

impl Pic /* Methods */ {
    /**
     * Returns whether it handles the `interrupt_num` given
     */
    fn can_handle_interrupt(&self, interrupt_num: u8) -> bool {
        self.m_offset <= interrupt_num && interrupt_num < self.m_offset + 8
    }

    /**
     * Notifies the end-of-interrupt
     */
    unsafe fn end_of_interrupt(&self) {
        self.m_command.write(0x20)
    }
}

impl Pic /* Getters */ {
    /**
     * Returns the current interrupt mask
     */
    unsafe fn interrupt_mask(&self) -> u8 {
        self.m_data.read()
    }
}

impl Pic /* Setters */ {
    /**
     * Overwrites the current interrupt mask
     */
    unsafe fn set_interrupt_mask(&self, interrupt_mask: u8) {
        self.m_data.write(interrupt_mask)
    }
}
