/*! debug printing support */

use core::{
    fmt,
    fmt::{
        Display,
        Write
    }
};

use sync::mutex::{
    spin::RawSpinMutex,
    Mutex
};

use crate::dev::uart::Uart;

const VT100_RED: usize = 31;
const VT100_GREEN: usize = 32;
const VT100_YELLOW: usize = 33;
const VT100_MAGENTA: usize = 35;
const VT100_WHITE: usize = 37;

/* output raw-device for <dbg_println()> */
static S_DBG_OUTPUT_UART: Mutex<RawSpinMutex, Uart> = Mutex::const_new(Uart::new());

/* false until <dbg_do_print()> is called for the first time */
static mut SM_DBG_OUTPUT_INITIALIZED: bool = false;

/**
 * Enumerates the `dbg_println()` levels
 */
pub enum DbgLevel {
    Err,
    Warn,
    Info,
    Debug,
    Trace
}

impl DbgLevel {
    /**
     * Returns the VT100 color code for the current variant
     */
    pub fn as_vt100_color(&self) -> usize {
        match self {
            Self::Err => VT100_RED,
            Self::Warn => VT100_YELLOW,
            Self::Info => VT100_GREEN,
            Self::Debug => VT100_MAGENTA,
            Self::Trace => VT100_WHITE
        }
    }
}

impl Display for DbgLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Err => write!(f, "Error"),
            Self::Warn => write!(f, "Warning"),
            Self::Info => write!(f, "Info"),
            Self::Debug => write!(f, "Debug"),
            Self::Trace => write!(f, "Trace")
        }
    }
}

/**
 * Performs the output to the selected debug device
 */
pub fn dbg_do_print(dbg_level: DbgLevel, args: fmt::Arguments<'_>, module_path: &str) {
    let mut dbg_uart = unsafe {
        /* initialize the serial debug output if not already done */
        if !SM_DBG_OUTPUT_INITIALIZED {
            if !S_DBG_OUTPUT_UART.lock().init() {
                panic!("Failed to initialize serial debug output");
            }

            /* we don't want to initialize it anymore */
            SM_DBG_OUTPUT_INITIALIZED = true;
        }

        /* ensure that anyone else writes to the debug output */
        S_DBG_OUTPUT_UART.lock()
    };

    /* write out the output to the debug line */
    write!(dbg_uart,
           "[\x1b[0;{}m{: >5}\x1b[0m <> \x1b[0;{}m{: <25}\x1b[0m] \x1b[0;{}m{}\x1b[0m\n",
           dbg_level.as_vt100_color(),
           dbg_level,
           VT100_MAGENTA,
           module_path,
           dbg_level.as_vt100_color(),
           args).expect("Failed to print to serial debug output")
}

/**
 * Prints on the debug output the given message with the given `DbgLevel`
 */
#[macro_export]
macro_rules! dbg_println {
    ($DbgLevel:expr, $($Args:tt)*) => {
        $crate::dbg::print::dbg_do_print($DbgLevel, format_args!($($Args)*), module_path!())
    };
}
