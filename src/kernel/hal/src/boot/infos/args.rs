/*! # HAL Command Line Arguments
 *
 * Implements the structures that hold the kernel command line arguments
 */

use os::str_utils;

/** Maximum amount of command line arguments that [`BootInfosInner`] could
 * store
 *
 * [`BootInfosInner`]: /hal/boot/struct.BootInfosInner.html
 */
pub(crate) const BOOT_CMDLINE_ARGS_COUNT_MAX: usize = 32;

/** Maximum length in bytes that each command line arguments could have
 */
pub(crate) const BOOT_CMDLINE_ARGS_LEN_MAX: usize = 64;

/** # Command Line Arguments
 *
 * Represents a collection of tokenized arguments given to the kernel by the
 * currently used bootloader
 */
#[derive(Debug)]
pub struct CmdLineArgs {
    m_args: [CmdLineArg; BOOT_CMDLINE_ARGS_COUNT_MAX],
    m_args_count: usize
}

impl CmdLineArgs {
    /** # Constructs a `CmdLineArgs`
     *
     * The given `raw_cmdline` is tokenized and parsed into sub
     * [`CmdLineArg`] object(s)
     *
     * [`CmdLineArg`]: /hal/boot/infos/struct.CmdLineArg.html
     */
    pub(crate) fn new(raw_cmdline: &str) -> Self {
        /* split and count the raw string arguments */
        let mut count = 0;
        let mut args_buf = [CmdLineArg::default(); BOOT_CMDLINE_ARGS_COUNT_MAX];
        for (i, arg) in raw_cmdline.split_whitespace().enumerate() {
            args_buf[i] = CmdLineArg::new(arg);
            count += 1;
        }

        Self { m_args: args_buf,
               m_args_count: count }
    }

    /** # Finds an argument key
     *
     * Returns the reference to a [`CmdLineArg`] if contains the given key
     * `to_find` ignoring the case.
     *
     * The method evaluates whether the current argument is a key value
     * argument (i.e -key=Value) or not.
     *
     * In the first case evaluates only the `-key` part, otherwise all the
     * word
     *
     * [`CmdLineArg`]: /hal/boot/infos/struct.CmdLineArg.html
     */
    pub fn find_key(&self, to_find: &str) -> Option<&CmdLineArg> {
        self.iter().find(|arg| {
                       if arg.is_key_value() {
                           arg.key().eq_ignore_ascii_case(to_find)
                       } else {
                           arg.as_str().eq_ignore_ascii_case(to_find)
                       }
                   })
    }

    /** # Constructs an arguments iterator
     *
     * The returned iterator allows to iterate through the arguments of this
     * object
     */
    pub fn iter(&self) -> impl Iterator<Item = &CmdLineArg> {
        CmdLineIter::from(self)
    }

    /** Returns the amount of arguments in this command line
     */
    pub fn count(&self) -> usize {
        self.m_args_count
    }

    /** Returns whether this command line is empty
     */
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }
}

/** # Command Line Argument
 *
 * Represent a single tokenized argument of the kernel's command line
 */
#[derive(Debug, Copy, Clone)]
pub struct CmdLineArg {
    m_arg: [u8; BOOT_CMDLINE_ARGS_LEN_MAX],
    m_len: usize,
    m_kv_eq_index: Option<usize>
}

impl CmdLineArg {
    /** # Constructs a `CmdLineArg`
     *
     * The raw argument string slice given is tokenized again into key and
     * value if necessary
     */
    pub(crate) fn new(arg: &str) -> Self {
        let mut arg_buf = [0; BOOT_CMDLINE_ARGS_LEN_MAX];
        str_utils::copy_str_to_u8_buf(&mut arg_buf, arg);

        Self { m_arg: arg_buf,
               m_len: arg.len(),
               m_kv_eq_index: arg.find("=") }
    }

    /** Returns this argument as string slice
     */
    pub fn as_str(&self) -> &str {
        str_utils::u8_ptr_to_str_slice(self.m_arg.as_ptr(), self.m_len)
    }

    /** Returns whether this argument have the form `-key=value`
     */
    pub fn is_key_value(&self) -> bool {
        self.m_kv_eq_index.is_some()
    }

    /** Returns the `-key` part of the `-key=value` if
     * [`CmdLineArg::is_key_value()`] returns true, same as
     * [`CmdLineArg::as_str()`] otherwise
     *
     * [`CmdLineArg::is_key_value()`]:
     * /hal/boot/infos/struct.CmdLineArg.html#method.is_key_value
     * [`CmdLineArg::as_str()`]:
     * /hal/boot/infos/struct.CmdLineArg.html#method.as_str
     */
    pub fn key(&self) -> &str {
        if self.is_key_value() {
            &self.as_str()[..self.m_kv_eq_index.unwrap()]
        } else {
            self.as_str()
        }
    }

    /** Returns the `value` part of the `-key=value` if
     * [`CmdLineArg::is_key_value()`] returns true, same as
     * [`CmdLineArg::as_str()`] otherwise
     *
     * [`CmdLineArg::is_key_value()`]:
     * /hal/boot/infos/struct.CmdLineArg.html#method.is_key_value
     * [`CmdLineArg::as_str()`]:
     * /hal/boot/infos/struct.CmdLineArg.html#method.as_str
     */
    pub fn value(&self) -> &str {
        if self.is_key_value() {
            &self.as_str()[self.m_kv_eq_index.unwrap() + 1..]
        } else {
            self.as_str()
        }
    }
}

impl Default for CmdLineArg {
    /** Returns the "default value" for a type.
     */
    fn default() -> Self {
        Self { m_arg: [0; BOOT_CMDLINE_ARGS_LEN_MAX],
               m_len: 0,
               m_kv_eq_index: None }
    }
}

/** # Command Line Iterator
 *
 * Allows to iterate sequentially the arguments stored into the given
 * command line
 */
struct CmdLineIter<'a> {
    m_cmdline_args: &'a CmdLineArgs,
    m_current_idx: usize
}

impl<'a> Iterator for CmdLineIter<'a> {
    /** The type of the elements being iterated over.
     */
    type Item = &'a CmdLineArg;

    /** Advances the iterator and returns the next value.
     */
    fn next(&mut self) -> Option<Self::Item> {
        if self.m_current_idx < self.m_cmdline_args.m_args_count {
            let current_arg = &self.m_cmdline_args.m_args[self.m_current_idx];
            self.m_current_idx += 1;
            Some(current_arg)
        } else {
            None
        }
    }
}

impl<'a> From<&'a CmdLineArgs> for CmdLineIter<'a> {
    /** Performs the conversion.
     */
    fn from(args: &'a CmdLineArgs) -> Self {
        Self { m_cmdline_args: args,
               m_current_idx: 0 }
    }
}
