/*! # HAL Interrupt Manager
 *
 * Implements the structures to handle the interrupts through an
 * architecture independent manager
 */

use core::sync::atomic::{
    AtomicUsize,
    Ordering
};

use crate::{
    arch::hal::interrupt::HwInterruptManager,
    hal::interrupt::InterruptStackFrame
};

/** # Interrupt Handler Function
 *
 * Functor expected by the [`InterruptManager`] as callback for common
 * interrupts
 *
 * [`InterruptManager`]: /hal/interrupt/struct.InterruptManager.html
 */
pub type InterruptHandler = fn(InterruptStackFrame, usize);

/** # Interrupt Exception Handler Function
 *
 * Functor expected by the [`InterruptManager`] as callback for exceptions
 *
 * [`InterruptManager`]: /hal/interrupt/struct.InterruptManager.html
 */
pub type ExceptionHandler = fn(InterruptStackFrame, InterruptManagerException) -> bool;

/** # Interrupt Manager
 *
 * Allows to manage hardware interrupts using high level functions and
 * associating safe rust callbacks to handle them when throw
 */
pub struct InterruptManager {
    m_hw_intr_man: HwInterruptManager,
    m_handlers: InterruptManagerHandlers
}

impl InterruptManager {
    /** # Constructs an uninitialized `InterruptManager`
     *
     * The returned instance must be loaded with
     * [`InterruptManager::enable_as_global()`](IG)
     *
     * [IG]: crate::hal::interrupt::manager::InterruptManager::
     * enable_as_global
     */
    pub const fn new_uninitialized() -> Self {
        Self { m_hw_intr_man: HwInterruptManager::CONST_NEW,
               m_handlers: InterruptManagerHandlers::new() }
    }

    /** # Enables this manager as global
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

    /** # Registers an `InterruptManagerException` callback
     *
     * Registers the given [`ExceptionHandler`](EH) as callback for the
     * given [`InterruptManagerException`](IM).
     *
     * Note that each registered callback may be called to handle more than
     * one hardware exception type (of the same domain of course), due to
     * the different granularity of error that each architecture
     * manages.
     *
     * Each call overwrites previously registered handler
     *
     * [EH]: crate::hal::interrupt::manager::ExceptionHandler
     * [IM]: crate::hal::interrupt::manager::InterruptManagerException
     */
    pub fn set_except_handler(&mut self,
                              exception: InterruptManagerException,
                              handler: ExceptionHandler) {
        self.m_handlers.set_except_handler(exception, handler);
    }

    /** # Registers an interrupt callback
     *
     * Registers the given [`InterruptHandler`](IH) as callback for the
     * given interrupt number.
     *
     * Each call overwrites previously registered handlers
     *
     * [IH]: crate::hal::interrupt::manager::InterruptHandler
     */
    pub fn set_intr_handler(&mut self, intr: usize, handler: InterruptHandler) {
        self.m_handlers.set_intr_handler(intr, handler);
    }

    /** # Executes a function without interrupts
     *
     * The given functor is executed without interrupts which are disabled
     * and re-enabled only if they are enabled
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

    /** Enables the hardware interrupts
     */
    pub fn enable_intr(&self) {
        self.m_hw_intr_man.enable_intr();
    }

    /** Disables the hardware interrupts
     */
    pub fn disable_intr(&self) {
        self.m_hw_intr_man.disable_intr();
    }

    /** Returns whether the hardware interrupts are enabled
     */
    pub fn intr_are_enabled(&self) -> bool {
        self.m_hw_intr_man.intr_are_enabled()
    }

    /** Returns a reference to the [`InterruptStats`]
     *
     * [`InterruptStats`]: crate::hal::interrupt::manager::InterruptStats
     */
    pub fn stats(&self) -> &InterruptManagerStats {
        &self.m_handlers.m_intr_stats
    }
}

/** # Interrupt Manager Statistics
 *
 * Contains the statistics of thrown exceptions/interrupts and, for
 * exceptions only, which was solved (i.e the [`ExceptionHandler`] have
 * returned `true`)
 *
 * [`ExceptionHandler`]: crate::hal::interrupt::manager::ExceptionHandler
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
    /** # Constructs an `InterruptManagerStats`
     *
     * The returned instance is completely zeroed
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

    /** Returns how many [`InterruptManagerException::MathDomain`](MD) was
     * handled
     *
     * [MD]: crate::hal::interrupt::manager::InterruptManagerException::
     * MathDomain
     */
    pub fn math_domain_faults(&self) -> usize {
        self.m_math_domain_faults.load(Ordering::SeqCst)
    }

    /** Returns how many [`InterruptManagerException::MathDomain`](MD) was
     * solved (i.e the [`ExceptionHandler`](EH) have returned `true`)
     *
     * [MD]: crate::hal::interrupt::manager::InterruptManagerException::
     * MathDomain [EH]: crate::hal::interrupt::manager::ExceptionHandler
     */
    pub fn math_domain_faults_solved(&self) -> usize {
        self.m_math_domain_faults_solved.load(Ordering::SeqCst)
    }

    /** Returns how many [`InterruptManagerException::MathDomain`] was
     * unsolved (i.e the [`ExceptionHandler`] have returned `false`)
     *
     * [`InterruptManagerException::MathDomain`]:
     * /hal/interrupt/enum.InterruptManagerException.html#variant.MathDomain
     * [`ExceptionHandler`]: /hal/interrupt/type.ExceptionHandler.html
     */
    pub fn math_domain_faults_unsolved(&self) -> usize {
        self.math_domain_faults() - self.math_domain_faults_solved()
    }

    /** Returns how many [`InterruptManagerException::InvalidInstr`] was
     * handled
     *
     * [`InterruptManagerException::InvalidInstr`]:
     * /hal/interrupt/enum.InterruptManagerException.html#variant.
     * InvalidInstr
     */
    pub fn invalid_instr_faults(&self) -> usize {
        self.m_invalid_instr_faults.load(Ordering::SeqCst)
    }

    /** Returns how many [`InterruptManagerException::InvalidInstr`] was
     * solved (i.e the [`ExceptionHandler`] have returned `true`)
     *
     * [`InterruptManagerException::InvalidInstr`]:
     * /hal/interrupt/enum.InterruptManagerException.html#variant.
     * InvalidInstr [`ExceptionHandler`]:
     * /hal/interrupt/type.ExceptionHandler.html
     */
    pub fn invalid_instr_faults_solved(&self) -> usize {
        self.m_invalid_instr_faults_solved.load(Ordering::SeqCst)
    }

    /** Returns how many [`InterruptManagerException::InvalidInstr`] was
     * unsolved (i.e the [`ExceptionHandler`] have returned `false`)
     *
     * [`InterruptManagerException::InvalidInstr`]:
     * /hal/interrupt/enum.InterruptManagerException.html#variant.
     * InvalidInstr
     * [`ExceptionHandler`]: /hal/interrupt/type.ExceptionHandler.html
     */
    pub fn invalid_instr_faults_unsolved(&self) -> usize {
        self.invalid_instr_faults() - self.invalid_instr_faults_solved()
    }

    /** Returns how many [`InterruptManagerException::PageFault`] was
     * handled
     *
     * [`InterruptManagerException::PageFault`]:
     * /hal/interrupt/enum.InterruptManagerException.html#variant.PageFault
     */
    pub fn page_faults(&self) -> usize {
        self.m_page_faults.load(Ordering::SeqCst)
    }

    /** Returns how many [`InterruptManagerException::PageFault`] was
     * solved (i.e the [`ExceptionHandler`] have returned `true`)
     *
     * [`InterruptManagerException::PageFault`]:
     * /hal/interrupt/enum.InterruptManagerException.html#variant.PageFault
     * [`ExceptionHandler`]: /hal/interrupt/type.ExceptionHandler.html
     */
    pub fn page_faults_solved(&self) -> usize {
        self.m_page_faults_solved.load(Ordering::SeqCst)
    }

    /** Returns how many [`InterruptManagerException::PageFault`] was
     * unsolved (i.e the [`ExceptionHandler`] have returned `false`)
     *
     * [`InterruptManagerException::PageFault`]:
     * /hal/interrupt/enum.InterruptManagerException.html#variant.PageFault
     * [`ExceptionHandler`]: /hal/interrupt/type.ExceptionHandler.html
     */
    pub fn page_faults_unsolved(&self) -> usize {
        self.page_faults() - self.page_faults_solved()
    }

    /** Returns how many [`InterruptManagerException::FloatingPoint`] was
     * handled
     *
     * [`InterruptManagerException::FloatingPoint`]:
     * /hal/interrupt/enum.InterruptManagerException.html#variant.
     * FloatingPoint
     */
    pub fn floating_point_faults(&self) -> usize {
        self.m_floating_point_faults.load(Ordering::SeqCst)
    }

    /** Returns how many [`InterruptManagerException::FloatingPoint`] was
     * solved (i.e the [`ExceptionHandler`] have returned `true`)
     *
     * [`InterruptManagerException::FloatingPoint`]:
     * /hal/interrupt/enum.InterruptManagerException.html#variant.
     * FloatingPoint
     * [`ExceptionHandler`]: /hal/interrupt/type.ExceptionHandler.html
     */
    pub fn floating_point_faults_solved(&self) -> usize {
        self.m_floating_point_faults_solved.load(Ordering::SeqCst)
    }

    /** Returns how many [`InterruptManagerException::FloatingPoint`] was
     * unsolved (i.e the [`ExceptionHandler`] have returned `false`)
     *
     * [`InterruptManagerException::FloatingPoint`]:
     * /hal/interrupt/enum.InterruptManagerException.html#variant.
     * FloatingPoint
     * [`ExceptionHandler`]: /hal/interrupt/type.ExceptionHandler.html
     */
    pub fn floating_point_faults_unsolved(&self) -> usize {
        self.floating_point_faults() - self.floating_point_faults_solved()
    }

    /** Returns how many times the given interrupt was handled
     */
    pub fn intr_throws_of(&self, intr: usize) -> usize {
        assert!(intr < HwInterruptManager::INTR_COUNT + HwInterruptManager::INTR_OFFSET);
        self.m_intr_throws[intr - HwInterruptManager::INTR_OFFSET].load(Ordering::SeqCst)
    }

    /** # Handles the given exception
     *
     * Executes the given handler and updates the statistics relatively to
     * the given [`InterruptManagerException`]
     *
     * [`InterruptManagerException`]:
     * /hal/interrupt/enum.InterruptManagerException.html
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
        };

        /* update the statistics for the given exception.
         *
         * Note that no locks are used here because i want to keep the HAL lock
         * independent, but here must be ensure a minimum of data coherence,
         * so this is why are used <AtomicUsize>
         */
        abs_counter.fetch_add(1, Ordering::SeqCst);
        if except_solved {
            solved_counter.fetch_add(1, Ordering::SeqCst);
        }
    }

    /** # Handles the given interrupt
     *
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

/** # Interrupt Manager Exceptions
 *
 * Enumerates the manageable exceptions that are assignable
 * through [`InterruptManager::set_except_handler()`]
 *
 * [`InterruptManager::set_except_handler()`]:
 * /hal/interrupt/struct.InterruptManager.html#method.set_except_handler
 */
#[repr(usize)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum InterruptManagerException {
    /** Mathematical errors, like division by zero, overflows and many
     * others, depends on the underling architecture
     */
    MathDomain,

    /** Invalid instruction code or operands
     */
    InvalidInstr,

    /** Pagination fault, like missing pages or protection violations
     */
    PageFault,

    /** Floating point, like sign exception
     */
    FloatingPoint,

    /** Amount of managed exceptions
     */
    Count
}

impl InterruptManagerException {
    pub const COUNT: usize = Self::Count as usize;
}

/** # Interrupt Manager Handlers
 *
 * Contains the pointers to the exceptions/interrupts handlers and the
 * statistics.
 *
 * This is the HAL middleware between the HAL [`InterruptManager`] and the
 * hardware interrupt manager
 *
 * [`InterruptManager`]: /hal/interrupt/struct.InterruptManager.html
 */
pub(crate) struct InterruptManagerHandlers {
    m_except_handlers: [Option<ExceptionHandler>; InterruptManagerException::COUNT],
    m_intr_handlers: [Option<InterruptHandler>; HwInterruptManager::INTR_COUNT],
    m_intr_stats: InterruptManagerStats
}

impl InterruptManagerHandlers {
    /** # Constructs an `InterruptManagerHandlers`
     *
     * The returned instance is blank and zeroed
     */
    const fn new() -> Self {
        Self { m_except_handlers: [None; InterruptManagerException::COUNT],
               m_intr_handlers: [None; HwInterruptManager::INTR_COUNT],
               m_intr_stats: InterruptManagerStats::new() }
    }

    /** # Handles `HwInterruptManager` calls
     *
     * Acts as callback for the `HwInterruptManager` and calls the
     * registered rust functors, and updates the related statistics
     */
    pub(crate) fn handle_hw_intr_callback(&mut self,
                                          stack_frame: InterruptStackFrame,
                                          opt_except: Option<InterruptManagerException>,
                                          opt_intr: Option<usize>) {
        if let Some(exception) = opt_except {
            /* obtain the exception handler from the vector */
            if let Some(except_handler) = self.m_except_handlers[exception.as_usize()] {
                /* delegate the call of the functor to the <InterruptManagerStats>
                 * which will updates the called exception statistics
                 */
                self.m_intr_stats
                    .handle_exception(except_handler, stack_frame, exception);
            }
        } else if let Some(intr_num) = opt_intr {
            /* obtain the interrupt handler from the vector */
            if let Some(intr_handler) =
                self.m_intr_handlers[intr_num - HwInterruptManager::INTR_OFFSET]
            {
                /* delegate the call of the functor to the <InterruptManagerStats>
                 * which will updates the called interrupt statistics
                 */
                self.m_intr_stats.handle_interrupt(intr_handler, stack_frame, intr_num);
            }
        } else {
            panic!("HwInterruptManager called callback with bot exception and \
                    interrupt to nullptr, report the bug");
        }
    }

    /** # Registers an `InterruptManagerException` callback
     *
     * Registers the given [`ExceptionHandler`] as callback for the given
     * [`InterruptManagerException`].
     *
     * Each call overwrites previously registered handler
     *
     * [`ExceptionHandler`]: /hal/interrupt/type.ExceptionHandler.html
     * [`InterruptManagerException`]:
     * /hal/interrupt/enum.InterruptManagerException.html
     */
    fn set_except_handler(&mut self,
                          exception: InterruptManagerException,
                          handler: ExceptionHandler) {
        self.m_except_handlers[exception.as_usize()] = Some(handler);
    }

    /** # Registers an interrupt callback
     *
     * Registers the given [`InterruptHandler`] as callback for the given
     * interrupt number.
     *
     * Each call overwrites previously registered handlers
     *
     * [`InterruptHandler`]: /hal/interrupt/type.InterruptHandler.html
     */
    fn set_intr_handler(&mut self, intr: usize, handler: InterruptHandler) {
        assert!(intr < HwInterruptManager::INTR_COUNT + HwInterruptManager::INTR_OFFSET);
        self.m_intr_handlers[intr - HwInterruptManager::INTR_OFFSET] = Some(handler);
    }
}

/** # Hardware Interrupt Manager Base Interface
 *
 * Defines the base interface of methods and constants which are used by the
 * [`InterruptManager`] and the [`InterruptManagerHandlers`]
 *
 * [`InterruptManager`]: /hal/interrupt/struct.InterruptManager.html
 * [`InterruptManagerHandlers`]:
 * /hal/interrupt/struct.InterruptManagerHandlers.html
 */
pub(crate) trait HwInterruptManagerBase {
    /** Constructs a `HwInterruptManagerBase` implementation.
     *
     * Workaround to get trait's const fn
     */
    const CONST_NEW: Self;

    /** Number of interrupts without exceptions
     */
    const INTR_COUNT: usize;

    /** Offset of the first interrupt after the reserved exceptions
     */
    const INTR_OFFSET: usize;

    /** # Enables the instance as global
     *
     * Performs the necessary operations to make the hardware implementation
     * active to handle interrupts and exceptions
     */
    unsafe fn enable_as_global(&'static mut self,
                               intr_handlers: &'static mut InterruptManagerHandlers);

    /** Enables the hardware interrupts
     */
    fn enable_intr(&self);

    /** Disables the hardware interrupts
     */
    fn disable_intr(&self);

    /** Returns whether the hardware interrupts are enabled
     */
    fn intr_are_enabled(&self) -> bool;
}
