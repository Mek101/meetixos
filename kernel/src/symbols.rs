/*! Kernel symbols support */

use shared::{
    info::descriptor::LoaderInfo,
    logger::{
        error,
        warn
    }
};
use symbols::{
    list::CodeSymbolsList,
    trace::StackBackTrace
};

/* parsed code symbols from the hh_loader */
static mut CODE_SYMBOLS: CodeSymbolsList = CodeSymbolsList::new_uninitialized();

extern "C" {
    static __text_begin: usize;
    static __text_end: usize;
}

/**
 * Initializes the global kernel symbols
 */
pub fn symbols_init(loader_info: &LoaderInfo) {
    let kern_symbols = loader_info.kernel_symbols_slice();

    /* load the core symbols from the loader information */
    if !unsafe { CODE_SYMBOLS.load_from_raw(kern_symbols) } {
        warn!("No kernel symbols available");
    }
}

/**
 * Prints into the kernel error logger the current stack-trace
 */
pub fn symbols_print_backtrace() {
    let text_begin_ptr = unsafe { &__text_begin as *const _ as usize };
    let text_end_ptr = unsafe { &__text_end as *const _ as usize };

    /* initialize stack back trace printer */
    let stack_back_trace =
        StackBackTrace::new(unsafe { &CODE_SYMBOLS }, text_begin_ptr, text_end_ptr);

    error!("Kernel Stack Backtrace:");
    error!("\n{}", stack_back_trace);
}
