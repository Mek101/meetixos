/*! Path Management */

use core::{
    convert::TryFrom,
    fmt,
    iter::Filter,
    ops::{
        Add,
        AddAssign,
        Sub,
        SubAssign
    },
    str::Split
};

use os::{
    limits::VFS_PATH_LEN_MAX,
    str_utils,
    sysc::{
        codes::KernPathFnId,
        fn_path::KernFnPath
    }
};

use crate::{
    bits::path::PathExistsState,
    caller::KernCaller
};
use core::ops::Index;

/**
 * Implements a simple way to manage VFS paths.
 *
 * The struct allows the following operations:
 *
 * # Normalization
 * Consecutive separators, self links and parent links are removed/resolved
 * when is possible without query the Kernel (i.e
 * `/Path/To///.././Something` becomes `/Path/Something`,
 * `../Stuffs/./To/../Searched` becomes `../Stuffs/Searched`)
 *
 * # Concatenation
 * It is possible to concatenate more than one `Path`/`&str` (with
 * `Path::append()` & `Path::append_raw()`) when is not known at compile
 * time all the components of a `Path`. When a `Path`/`&str` is appended it
 * is normalized according to the first point too
 *
 * # Component iteration
 * It is possible to iterate the components of the normalized path in a
 * comfortable for-loop with `Path::components()`
 *
 * # Printing
 * As string with `Display`
 */
#[derive(Debug, Copy, Clone)]
pub struct Path {
    m_buf: [u8; VFS_PATH_LEN_MAX],
    m_len: usize,
    m_abs: bool
}

impl Path {
    /**
     * The path separator character, MeetiX uses the same path schema of
     * Unix
     */
    pub const SEPARATOR: &'static str = "/";

    /**
     * The path parent link string, MeetiX uses the same path schema of Unix
     */
    pub const PARENT_LINK: &'static str = "..";

    /**
     * The path self link string, MeetiX uses the same path schema of Unix
     */
    pub const SELF_LINK: &'static str = ".";

    /**  
     * Appends a new `Path`.
     *
     * The given `Path` is evaluated, if it is absolute overwrites the
     * current one (if contains something), otherwise it is appended to
     * this one.
     *
     * The parent links are resolved until it is possible
     */
    pub fn append(&mut self, path: &Path) {
        self.append_raw(path.as_str());
    }

    /**  
     * Appends a new raw path.
     *
     * The given raw path is evaluated, if it is absolute overwrites the
     * current one (if contains something), otherwise it is resolved then
     * appended to this one.
     *
     * The parent links are resolved until it is possible
     */
    pub fn append_raw(&mut self, raw_path: &str) {
        /* check for absolute path, it overwrites the previous one */
        if raw_path.starts_with(Self::SEPARATOR) {
            self.m_len = 0;
            self.m_abs = true;
        }

        /* special case: only the separator character is given */
        if raw_path == Self::SEPARATOR {
            self.append_unchecked(Self::SEPARATOR);
            return;
        }

        /* iterate for each component, ignoring empty values (i.e multiple contiguous
         * separators produces empty components)
         */
        for c in raw_path.split(Self::SEPARATOR).filter(|uc| !uc.is_empty()) {
            /* check whether the separator must be prepended */
            let need_sep = {
                (self.is_empty() && self.is_absolute())
                || (!self.is_empty() && !self.last_is_separator())
            };

            /* insert, remove, or ignore the component */
            match c {
                Self::SELF_LINK => continue,
                Self::PARENT_LINK if self.may_pop_last() => {
                    self.pop();
                },
                component => {
                    if need_sep {
                        self.append_unchecked(Self::SEPARATOR);
                    }
                    self.append_unchecked(component);
                }
            }
        }
    }

    /**
     * Returns `Some(Path)` until this `Path` contains elements.
     *
     * When the `Path` have no more elements return `None`
     */
    pub fn pop(&mut self) -> Option<Path> {
        self.components()
            .last()
            .map(|last| last.len())
            .map(|len| {
                let parent = self.basename();

                /* remove the last component */
                self.m_len -= len;

                /* to avoid that the next call to pop() returns an empty element remove
                 * the path separator, if any.
                 */
                if self.last_is_separator() {
                    self.m_len -= 1;
                }
                parent
            })
            .unwrap_or(None)
    }

    /**
     * Asks to the Kernel to resolve the stored path and return a
     * `PathExistsState` which tells with his variants the result
     */
    pub fn exists(&self) -> PathExistsState {
        let mut index = 0usize;
        self.kern_call_2(KernFnPath::Path(KernPathFnId::Exists),
                         self as *const _ as usize,
                         &mut index as *mut _ as usize)
            .map(|value| PathExistsState::try_from((value, index)).unwrap())
            .unwrap()
    }

    /**
     * Constructs a path component `Iterator`
     */
    pub fn components(&self) -> impl Iterator<Item = &str> {
        PathComponentIter::new(self.as_str())
    }

    /**
     * Returns the last component of the `Path`
     */
    pub fn basename(&self) -> Option<Path> {
        self.components().last().map(|last| Path::from(last))
    }

    /**
     * Returns the `Path` as string slice
     */
    pub fn as_str(&self) -> &str {
        str_utils::u8_ptr_to_str_slice(self.m_buf.as_ptr(), self.m_len)
    }

    /**
     * Returns the length of this `Path`
     */
    pub fn len(&self) -> usize {
        self.m_len
    }

    /**
     * Returns whether this `Path` is empty
     */
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /**
     * Returns whether this `Path` is absolute
     */
    pub fn is_absolute(&self) -> bool {
        self.m_abs
    }

    /**
     * Returns whether a parent link can pop the last element
     */
    fn may_pop_last(&self) -> bool {
        (self.len() > 0 || self.is_absolute())
        && self.components().last().map(|last| last != Self::PARENT_LINK).unwrap_or(true)
    }

    /**
     * Returns whether the last character is a separator
     */
    fn last_is_separator(&self) -> bool {
        self.as_str()
            .chars()
            .last()
            .map(|c| c == Self::SEPARATOR.chars().next().unwrap())
            .unwrap_or(false)
    }

    /**
     * Appends a new component without checks
     */
    fn append_unchecked(&mut self, c: &str) {
        self.m_buf[self.m_len..self.m_len + c.len()].copy_from_slice(c.as_bytes());
        self.m_len += c.len();
    }
}

impl KernCaller for Path {
    /* Nothing to implement */
}

impl Default for Path {
    fn default() -> Self {
        Self { m_buf: [0; VFS_PATH_LEN_MAX],
               m_len: 0,
               m_abs: false }
    }
}

impl From<&str> for Path {
    fn from(raw_path: &str) -> Self {
        let mut path = Self::default();
        path.append_raw(raw_path);
        path
    }
}

impl From<&Path> for Path {
    fn from(other_path: &Path) -> Self {
        let mut path = Self::default();
        path.append(other_path);
        path
    }
}

impl Add<&str> for Path {
    type Output = Path;

    fn add(self, rhs: &str) -> Self::Output {
        let mut new_path = Self::from(self);
        new_path.append_raw(rhs);
        new_path
    }
}

impl Add<Path> for Path {
    type Output = Path;

    fn add(self, rhs: Path) -> Self::Output {
        let mut new_path = Self::from(self);
        new_path.append(&rhs);
        new_path
    }
}

impl Add<&Path> for Path {
    type Output = Path;

    fn add(self, rhs: &Path) -> Self::Output {
        let mut new_path = Self::from(self);
        new_path.append(rhs);
        new_path
    }
}

impl AddAssign<&str> for Path {
    fn add_assign(&mut self, rhs: &str) {
        self.append_raw(rhs)
    }
}

impl AddAssign<Path> for Path {
    fn add_assign(&mut self, rhs: Path) {
        self.append(&rhs)
    }
}

impl AddAssign<&Path> for Path {
    fn add_assign(&mut self, rhs: &Path) {
        self.append(rhs)
    }
}

impl Sub<usize> for Path {
    type Output = Path;

    fn sub(self, rhs: usize) -> Self::Output {
        let mut new_path = Self::from(self);
        for _ in 0..rhs {
            if new_path.pop().is_none() {
                break;
            }
        }
        new_path
    }
}

impl SubAssign<usize> for Path {
    fn sub_assign(&mut self, rhs: usize) {
        for _ in 0..rhs {
            if self.pop().is_none() {
                break;
            }
        }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.as_str().eq(other.as_str())
    }
}

impl Eq for Path {
    /* No methods to implement, just a marker */
}

impl AsRef<[u8]> for Path {
    fn as_ref(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl<'a> Index<usize> for Path {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        for (i, component) in self.components().enumerate() {
            if i == index {
                return component;
            }
        }
        unreachable!();
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/**
 * Allows to iterate the components of the `Path` that have originated
 * this without empty units.
 *
 * It essentially consists in a `str::Split` and a `iter::Filter`
 */
struct PathComponentIter<'a>(Filter<Split<'a, &'a str>, fn(&&str) -> bool>);

impl<'a> PathComponentIter<'a> {
    /**
     * Constructs a new `PathComponentIter` from the given `str` slice
     */
    fn new(raw_path: &'a str) -> Self {
        Self(raw_path.split(Path::SEPARATOR).filter(|c| !c.is_empty()))
    }
}

impl<'a> Iterator for PathComponentIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
