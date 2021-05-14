/*! Interrupt manager */

use core::sync::atomic::{
    AtomicUsize,
    Ordering
};

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

use crate::{
    arch::interrupt::manager::HwInterruptManager,
    interrupt::stack_frame::InterruptStackFrame
};

/**
 * Callback used by the `InterruptManager` for common interrupts
 */
pub type InterruptHandler = fn(InterruptStackFrame, usize);

/**
 * Callback used by the `InterruptManager` for exception interrupts
 */
pub type ExceptionHandler = fn(InterruptStackFrame, InterruptManagerException) -> bool;

/**
 * High level interrupt manager.
 *
 * Allows to manage hardware interrupts using high level functions and
 * associating safe rust callbacks to handle them when throw
 */
pub struct InterruptManager {
    m_hw_intr_man: HwInterruptManager,
    m_handlers: InterruptManagerHandlers
}

impl InterruptManager {
    /**
     * Constructs an uninitialized `InterruptManager`, which must be
     * initialized with `InterruptManager::enable_as_global()`
     */
    pub const fn new_uninitialized() -> Self {
        Self { m_hw_intr_man: HwInterruptManager::CONST_NEW,
               m_handlers: InterruptManagerHandlers::new() }
    }

    /**  
     * Enables this manager as global
     *
     * Means that really enables the hardware implementation of the manager
     * and after this call it will start receiving interrupts and handle
     * them with the registered callbacks.
     *
     * Note that to call this method is necessary that the instance must be
     * placed as static global object.
     * Following calls to this method may panic
     */
    pub unsafe fn enable_as_global(&'static mut self) {
        self.m_hw_intr_man.enable_as_global(&mut self.m_handlers);
    }

    /**
     * Registers the given `ExceptionHandler` as callback for the given
     * `InterruptManagerException`.
     *
     * Note that each registered callback may be called to handle more than
     * one hardware exception type (of the same domain of course), due to
     * the different granularity of error that each architecture
     * manages.
     *
     * Each call overwrites previously registered handler
     */
    pub fn set_except_handler(&mut self,
                              exception: InterruptManagerException,
                              handler: ExceptionHandler) {
        self.m_handlers.set_except_handler(exception, handler);
    }

    /**
     * Registers the given `InterruptHandler` as callback for the given
     * interrupt number.
     *
     * Each call overwrites previously registered handlers
     */
    pub fn set_intr_handler(&mut self, intr: usize, handler: InterruptHandler) {
        self.m_handlers.set_intr_handler(intr, handler);
    }

    /**
     * Executes a function without interrupts
     */
    pub fn without_intr<T, F>(&self, f: F) -> T
        where F: FnOnce() -> T {
        let intr_was_enabled = self.intr_are_enabled();

        /* disable only if they are enabled */
        if intr_was_enabled {
            self.disable_intr()
        }

        /* execute the given functor */
        let res = f();

        /* re-enable the interrupts if they was enabled */
        if intr_was_enabled {
            self.enable_intr()
        }

        res
    }

    /**
     * Enables the hardware interrupts
     */
    pub fn enable_intr(&self) {
        self.m_hw_intr_man.enable_intr();
    }

    /**
     * Disables the hardware interrupts
     */
    pub fn disable_intr(&self) {
        self.m_hw_intr_man.disable_intr();
    }

    /**
     * Returns whether the hardware interrupts are enabled
     */
    pub fn intr_are_enabled(&self) -> bool {
        self.m_hw_intr_man.intr_are_enabled()
    }

    /**
     * Returns a reference to the `InterruptStats`
     */
    pub fn stats(&self) -> &InterruptManagerStats {
        &self.m_handlers.m_intr_stats
    }
}

/**  
 * Interrupt Manager Statistics
 *
 * Contains the statistics of thrown exceptions/interrupts and, for
 * exceptions only, which was solved (i.e the `ExceptionHandler` have
 * returned `true`)
 */
pub struct InterruptManagerStats {
    /* InterruptManagerException::MathDomain statistics */
    m_math_domain_faults: AtomicUsize,
    m_math_domain_faults_solved: AtomicUsize,

    /* InterruptManagerException::InvalidInstr statistics */
    m_invalid_instr_faults: AtomicUsize,
    m_invalid_instr_faults_solved: AtomicUsize,

    /* InterruptManagerException::PageFault statistics */
    m_page_faults: AtomicUsize,
    m_page_faults_solved: AtomicUsize,

    /* InterruptManagerException::FloatingPoint statistics */
    m_floating_point_faults: AtomicUsize,
    m_floating_point_faults_solved: AtomicUsize,

    /* common interrupts statistics */
    m_intr_throws: [AtomicUsize; HwInterruptManager::INTR_COUNT]
}

impl InterruptManagerStats {
    /**
     * Constructs an empty `InterruptManagerStats`
     */
    const fn new() -> Self {
        const ATOMIC_ZERO: AtomicUsize = AtomicUsize::new(0);

        Self { m_math_domain_faults: ATOMIC_ZERO,
               m_math_domain_faults_solved: ATOMIC_ZERO,
               m_invalid_instr_faults: ATOMIC_ZERO,
               m_invalid_instr_faults_solved: ATOMIC_ZERO,
               m_page_faults: ATOMIC_ZERO,
               m_page_faults_solved: ATOMIC_ZERO,
               m_floating_point_faults: ATOMIC_ZERO,
               m_floating_point_faults_solved: ATOMIC_ZERO,
               m_intr_throws: [ATOMIC_ZERO; HwInterruptManager::INTR_COUNT] }
    }

    /**
     * Returns how many `InterruptManagerException::MathDomain` was handled
     */
    pub fn math_domain_faults(&self) -> usize {
        self.m_math_domain_faults.load(Ordering::SeqCst)
    }

    /**
     * Returns how many `InterruptManagerException::MathDomain` was solved
     * (i.e the `ExceptionHandler` have returned `true`)
     */
    pub fn math_domain_faults_solved(&self) -> usize {
        self.m_math_domain_faults_solved.load(Ordering::SeqCst)
    }

    /**
     * Returns how many `InterruptManagerException::MathDomain` was unsolved
     * (i.e the `ExceptionHandler` have returned `false`)
     */
    pub fn math_domain_faults_unsolved(&self) -> usize {
        self.math_domain_faults() - self.math_domain_faults_solved()
    }

    /**
     * Returns how many `InterruptManagerException::InvalidInstr` was
     * handled
     */
    pub fn invalid_instr_faults(&self) -> usize {
        self.m_invalid_instr_faults.load(Ordering::SeqCst)
    }

    /**
     * Returns how many `InterruptManagerException::InvalidInstr` was solved
     * (i.e the `ExceptionHandler` have returned `true`)
     */
    pub fn invalid_instr_faults_solved(&self) -> usize {
        self.m_invalid_instr_faults_solved.load(Ordering::SeqCst)
    }

    /**
     * Returns how many `InterruptManagerException::InvalidInstr` was
     * unsolved (i.e the `ExceptionHandler` have returned `false`)
     */
    pub fn invalid_instr_faults_unsolved(&self) -> usize {
        self.invalid_instr_faults() - self.invalid_instr_faults_solved()
    }

    /**
     * Returns how many `InterruptManagerException::PageFault` was handled
     */
    pub fn page_faults(&self) -> usize {
        self.m_page_faults.load(Ordering::SeqCst)
    }

    /**
     * Returns how many `InterruptManagerException::PageFault` was solved
     * (i.e the `ExceptionHandler` have returned `true`)
     */
    pub fn page_faults_solved(&self) -> usize {
        self.m_page_faults_solved.load(Ordering::SeqCst)
    }

    /**
     * Returns how many `InterruptManagerException::PageFault` was unsolved
     * (i.e the `ExceptionHandler` have returned `false`)
     */
    pub fn page_faults_unsolved(&self) -> usize {
        self.page_faults() - self.page_faults_solved()
    }

    /**
     * Returns how many `InterruptManagerException::FloatingPoint` was
     * handled
     */
    pub fn floating_point_faults(&self) -> usize {
        self.m_floating_point_faults.load(Ordering::SeqCst)
    }

    /**
     * Returns how many `InterruptManagerException::FloatingPoint` was
     * solved (i.e the `ExceptionHandler` have returned `true`)
     */
    pub fn floating_point_faults_solved(&self) -> usize {
        self.m_floating_point_faults_solved.load(Ordering::SeqCst)
    }

    /**
     * Returns how many `InterruptManagerException::FloatingPoint` was
     * unsolved (i.e the `ExceptionHandler` have returned `false`)
     */
    pub fn floating_point_faults_unsolved(&self) -> usize {
        self.floating_point_faults() - self.floating_point_faults_solved()
    }

    /**
     * Returns how many times the given interrupt was handled
     */
    pub fn intr_throws_of(&self, intr: usize) -> usize {
        assert!(intr < HwInterruptManager::INTR_COUNT + HwInterruptManager::INTR_OFFSET);
        self.m_intr_throws[intr - HwInterruptManager::INTR_OFFSET].load(Ordering::SeqCst)
    }

    /**
     * Executes the given handler and updates the statistics relatively to
     * the given `InterruptManagerException`
     */
    fn handle_exception(&mut self,
                        except_handler: ExceptionHandler,
                        stack_frame: InterruptStackFrame,
                        exception: InterruptManagerException) {
        /* execute the given exception handler routine */
        let except_solved = except_handler(stack_frame, exception);

        /* obtain the mutable references to the counter of the given exception */
        let (abs_counter, solved_counter) = match exception {
            InterruptManagerException::MathDomain => {
                (&mut self.m_math_domain_faults, &mut self.m_math_domain_faults_solved)
            },
            InterruptManagerException::InvalidInstr => {
                (&mut self.m_invalid_instr_faults,
                 &mut self.m_invalid_instr_faults_solved)
            },
            InterruptManagerException::PageFault => {
                (&mut self.m_page_faults, &mut self.m_page_faults_solved)
            },
            InterruptManagerException::FloatingPoint => {
                (&mut self.m_floating_point_faults,
                 &mut self.m_floating_point_faults_solved)
            },
            _ => panic!("Requested Unknown exception handling")
        };

        /* update the statistics for the given exception.
         *
         * Note that no locks are used here because i want to keep the shared kernel
         * code lock independent, but here must be ensure a minimum of data
         * coherence, so this is why are used <AtomicUsize>
         */
        abs_counter.fetch_add(1, Ordering::SeqCst);
        if except_solved {
            solved_counter.fetch_add(1, Ordering::SeqCst);
        }
    }

    /**
     * Executes the given handler and updates the statistics relatively to
     * the given interrupt number
     */
    fn handle_interrupt(&mut self,
                        intr_handler: InterruptHandler,
                        stack_frame: InterruptStackFrame,
                        intr_num: usize) {
        intr_handler(stack_frame, intr_num);
        let intr_throws =
            &mut self.m_intr_throws[intr_num - HwInterruptManager::INTR_OFFSET];
        intr_throws.fetch_add(1, Ordering::SeqCst);
    }
}

/**
 * Enumerates the manageable exceptions that are assignable through
 * `InterruptManager::set_except_handler()`
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum InterruptManagerException {
    /**
     * Mathematical errors, like division by zero, overflows and many
     * others, depends on the underling architecture
     */
    MathDomain,

    /**
     * Invalid instruction code or operands
     */
    InvalidInstr,

    /**
     * Pagination fault, like missing pages or protection violations
     */
    PageFault,

    /**
     * Floating point, like sign exception
     */
    FloatingPoint,

    /**
     * Amount of managed exceptions
     */
    Count
}

impl InterruptManagerException {
    pub const COUNT: usize = Self::Count as usize;
}

/**
 * Collection of interrupt manager handlers.
 *
 * Contains the pointers to the exceptions/interrupts handlers and the
 * statistics.
 *
 * This is the HAL middleware between the HAL `InterruptManager` and the
 * hardware interrupt manager
 */
pub(crate) struct InterruptManagerHandlers {
    m_except_handlers: [Option<ExceptionHandler>; InterruptManagerException::COUNT],
    m_intr_handlers: [Option<InterruptHandler>; HwInterruptManager::INTR_COUNT],
    m_intr_stats: InterruptManagerStats
}

impl InterruptManagerHandlers {
    /**
     * Constructs an empty `InterruptManagerHandlers`
     */
    const fn new() -> Self {
        Self { m_except_handlers: [None; InterruptManagerException::COUNT],
               m_intr_handlers: [None; HwInterruptManager::INTR_COUNT],
               m_intr_stats: InterruptManagerStats::new() }
    }

    /**
     * Acts as callback for the `HwInterruptManager` and calls the
     * registered rust functors, and updates the related statistics
     */
    pub(crate) fn handle_hw_intr_callback(&mut self,
                                          stack_frame: InterruptStackFrame,
                                          reason: InterruptReason) {
        match reason {
            InterruptReason::Exception(exception) => {
                /* obtain the exception handler from the vector */
                let except_index: usize = exception.clone().into();
                if let Some(except_handler) = self.m_except_handlers[except_index] {
                    /* delegate the call of the functor to the <InterruptManagerStats>
                     * which will updates the called exception statistics
                     */
                    self.m_intr_stats
                        .handle_exception(except_handler, stack_frame, exception);
                }
            },
            InterruptReason::Interrupt(interrupt) => {
                /* obtain the interrupt handler from the vector */
                if let Some(intr_handler) =
                    self.m_intr_handlers[interrupt - HwInterruptManager::INTR_OFFSET]
                {
                    /* delegate the call of the functor to the <InterruptManagerStats>
                     * which will updates the called interrupt statistics
                     */
                    self.m_intr_stats
                        .handle_interrupt(intr_handler, stack_frame, interrupt);
                }
            }
        }
    }

    /**
     * Registers the given `ExceptionHandler` as callback for the given
     * `InterruptManagerException`.
     *
     * Each call overwrites previously registered handler
     */
    fn set_except_handler(&mut self,
                          exception: InterruptManagerException,
                          handler: ExceptionHandler) {
        let except_index: usize = exception.into();
        self.m_except_handlers[except_index] = Some(handler);
    }

    /**
     * Registers the given `InterruptHandler` as callback for the given
     * interrupt number.
     *
     * Each call overwrites previously registered handlers
     */
    fn set_intr_handler(&mut self, intr: usize, handler: InterruptHandler) {
        assert!(intr < HwInterruptManager::INTR_COUNT + HwInterruptManager::INTR_OFFSET);
        self.m_intr_handlers[intr - HwInterruptManager::INTR_OFFSET] = Some(handler);
    }
}

/**
 * Lists the possible reasons for which an interrupt could occur
 */
pub(crate) enum InterruptReason {
    /**
     * One of the managed `InterruptManagerException` has occurred and needs
     * handling
     */
    Exception(InterruptManagerException),

    /**
     * One of the other interrupts has occurred and needs handling
     */
    Interrupt(usize)
}

/**
 * Interface on which `InterruptManager` and `InterruptManagerHandlers`
 * relies to manage hardware interrupts
 */
pub(crate) trait HwInterruptManagerBase {
    /**
     * Constructs a `HwInterruptManagerBase` implementation.
     *
     * Workaround to get trait's const fn
     */
    const CONST_NEW: Self;

    /**
     * Number of interrupts without reserved exceptions
     */
    const INTR_COUNT: usize;

    /**
     * Offset of the first interrupt after the reserved exceptions
     */
    const INTR_OFFSET: usize;

    /**
     * Performs the necessary operations to make the hardware implementation
     * active to handle interrupts and exceptions
     */
    unsafe fn enable_as_global(&'static mut self,
                               intr_handlers: &'static mut InterruptManagerHandlers);

    /**
     * Enables the hardware interrupts
     */
    fn enable_intr(&self);

    /**
     * Disables the hardware interrupts
     */
    fn disable_intr(&self);

    /**
     * Returns whether the hardware interrupts are enabled
     */
    fn intr_are_enabled(&self) -> bool;
}
