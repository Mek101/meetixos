/*! debug printing support */

use core::{
    convert::TryFrom,
    fmt,
    fmt::{
        Display,
        Write
    }
};

use sync::mutex::{
    spin_mutex::RawSpinMutex,
    Mutex
};

use crate::{
    boot_info::BootInfo,
    dev::uart::Uart
};

/* Vt100 color codes */
const C_VT100_RED: usize = 31;
const C_VT100_GREEN: usize = 32;
const C_VT100_YELLOW: usize = 33;
const C_VT100_MAGENTA: usize = 35;
const C_VT100_WHITE: usize = 37;

/* output raw-device for <dbg_println()> */
static S_DBG_OUTPUT_UART: Mutex<RawSpinMutex, Uart> = Mutex::const_new(Uart::new());

/* verbosity of the <dbg_println()> */
static mut SM_DBG_MAX_LEVEL: DbgLevel = DbgLevel::Info;

/**
 * Enumerates the `dbg_println()` levels
 */
#[repr(u8)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
pub enum DbgLevel {
    Err,
    Warn,
    Info,
    Debug,
    Trace
}

impl DbgLevel /* Getters */ {
    /**
     * Returns the VT100 color code for the current variant
     */
    pub fn as_vt100_color(&self) -> usize {
        match self {
            Self::Err => C_VT100_RED,
            Self::Warn => C_VT100_YELLOW,
            Self::Info => C_VT100_GREEN,
            Self::Debug => C_VT100_MAGENTA,
            Self::Trace => C_VT100_WHITE
        }
    }
}

impl TryFrom<&str> for DbgLevel {
    type Error = ();

    fn try_from(str_dbg_level: &str) -> Result<Self, Self::Error> {
        if str_dbg_level.eq_ignore_ascii_case("Error") {
            Ok(Self::Err)
        } else if str_dbg_level.eq_ignore_ascii_case("Warning") {
            Ok(Self::Warn)
        } else if str_dbg_level.eq_ignore_ascii_case("Info") {
            Ok(Self::Info)
        } else if str_dbg_level.eq_ignore_ascii_case("Debug") {
            Ok(Self::Debug)
        } else if str_dbg_level.eq_ignore_ascii_case("Trace") {
            Ok(Self::Trace)
        } else {
            Err(())
        }
    }
}

impl Display for DbgLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Err => write!(f, "{: <7}", "Error"),
            Self::Warn => write!(f, "{: <7}", "Warning"),
            Self::Info => write!(f, "{: <7}", "Info"),
            Self::Debug => write!(f, "{: <7}", "Debug"),
            Self::Trace => write!(f, "{: <7}", "Trace")
        }
    }
}

/**
 * Prints on the debug output the given message with the given `DbgLevel`
 */
#[macro_export]
macro_rules! dbg_println {
    ($DbgLevel:expr, $($arg:tt)*) => (
        {
            if $DbgLevel <=$crate::dbg_print::dbg_print_max_level() {
                $crate::dbg_print::dbg_do_print(format_args!($($arg)*),
                                                $DbgLevel,
                                                module_path!())
            }
        }
    )
}

/**
 * Initializes the debug printing
 */
pub fn dbg_print_init() {
    /* initialize the hardware output */
    if !S_DBG_OUTPUT_UART.lock().init() {
        panic!("Failed to initialize serial debug output");
    }

    /* search into the cmdline whether the -log-level option is given, in that
     * case try to parse it and set it as new-level. otherwise print a warning
     */
    if let Some((_, value)) = BootInfo::instance().cmd_line_find_arg("-log-level") {
        if let Some(str_dbg_level) = value {
            if let Some(_) = dbg_print_set_max_level_from_str(str_dbg_level) {
                dbg_println!(DbgLevel::Trace,
                             "Maximum allow debug printing level is DbgLevel::{}",
                             unsafe { SM_DBG_MAX_LEVEL })
            } else {
                dbg_println!(DbgLevel::Warn,
                             "Unsupported DbgLevel given: {}",
                             str_dbg_level);
            }
        }
    }
}

/**
 * Returns the global `DbgLevel`
 */
pub fn dbg_print_max_level() -> DbgLevel {
    unsafe { SM_DBG_MAX_LEVEL }
}

/**
 * Sets a new `DbgLevel` from the given string value and returns the old
 * value if the given string slice is reducible to a one of the supported
 * `DbgLevel`s
 */
pub fn dbg_print_set_max_level_from_str(str_dbg_level: &str) -> Option<DbgLevel> {
    if let Ok(new_dbg_level) = DbgLevel::try_from(str_dbg_level) {
        Some(dbg_print_set_max_level(new_dbg_level))
    } else {
        None
    }
}

/**
 * Sets a new `DbgLevel` and returns the previous one
 */
pub fn dbg_print_set_max_level(dbg_level: DbgLevel) -> DbgLevel {
    unsafe {
        let old_dbg_level = SM_DBG_MAX_LEVEL;
        SM_DBG_MAX_LEVEL = dbg_level;
        old_dbg_level
    }
}

/**
 * Performs the output to the selected debug device
 */
pub fn dbg_do_print(args: fmt::Arguments<'_>, dbg_level: DbgLevel, module_path: &str) {
    write!(S_DBG_OUTPUT_UART.lock(),
           "[\x1b[0;{}m{}\x1b[0m <> \x1b[0;{}m{: <25}\x1b[0m] \x1b[0;{}m{}\x1b[0m\n",
           dbg_level.as_vt100_color(),
           dbg_level,
           C_VT100_MAGENTA,
           module_path,
           dbg_level.as_vt100_color(),
           args).expect("Failed to print to serial debug output");
}
