/*! # Configuration Trait
 *
 * Implements various markers useful for the various configurations like the
 * [`ObjConfig`], [`TaskConfig`] and [`OSEntityConfig`]
 *
 * [`ObjConfig`]: /api/objs/struct.ObjConfig.html
 * [`TaskConfig`]: /api/tasks/struct.TaskConfig.html
 * [`OSEntityConfig`]: /api/ents/struct.OSEntityConfig.html
 */

use core::marker::PhantomData;

use crate::objs::{impls::KrnIterator, ObjId};

c_handy_enum! {
    /** # Configuration Type
     *
     * Lists the available implementations for the [`ConfigMode`]
     *
     * [`ConfigMode`]: /api/config/trait.ConfigMode.html
     */
    pub enum ConfigModeType : u8 {
        /** No real uses, used as default value
         */
        Unknown = 0,

        /** Identifies the [`CreatMode`]
         *
         * [`CreatMode`]: /api/config/struct.CreatMode.html
         */
        Create = 1,

        /** Identifies the [`FindMode`]
         *
         * [`FindMode`]: /api/config/struct.FindMode.html
         */
        Find = 2,

    }
}

/** # Config Mode Base
 *
 * Represents the base interface for the configuration modes
 */
pub trait ConfigMode {
    /** The [`ConfigModeType`] which the concrete type represents
     *
     * [`ConfigModeType`]: /api/config/enum.ConfigModeType.html
     */
    const TYPE: ConfigModeType;
}

/** # Creation Mode
 *
 * Enables methods useful to customize the creation of an item defined by
 * the used configurator
 */
#[derive(Debug, Copy, Clone)]
pub struct CreatMode;

impl ConfigMode for CreatMode {
    const TYPE: ConfigModeType = ConfigModeType::Create;
}

/** # Find Mode
 *
 * Enables methods useful to customize the search of an item defined by the
 * used configurator
 */
#[derive(Debug, Copy, Clone)]
pub struct FindMode;

impl ConfigMode for FindMode {
    const TYPE: ConfigModeType = ConfigModeType::Find;
}

/** # Configuration Finder Iterator
 *
 * Library internal wrapper for [`KrnIterator`] that iterates instances of
 * type `T` that inside contains handles of type `H`
 *
 * [`KrnIterator`]: /api/objs/impls/struct.KrnIterator.htm
 */
pub(crate) struct ConfigFinderIter<H, T>
    where H: From<usize>,
          T: From<H> {
    m_iter: KrnIterator,
    _handle_type: PhantomData<H>,
    _wrapper_type: PhantomData<T>
}

impl<H, T> From<usize> for ConfigFinderIter<H, T>
    where H: From<usize>,
          T: From<H>
{
    /** Performs the conversion
     */
    fn from(iter_id: usize) -> Self {
        Self { m_iter: KrnIterator::from(ObjId::from(iter_id)),
               _handle_type: Default::default(),
               _wrapper_type: Default::default() }
    }
}

impl<H, T> Iterator for ConfigFinderIter<H, T>
    where H: From<usize>,
          T: From<H>
{
    /** The type of the elements being iterated over.
     */
    type Item = T;

    /** Advances the iterator and returns the next value
     */
    fn next(&mut self) -> Option<Self::Item> {
        self.m_iter
            .find_next::<usize>()
            .unwrap_or_default() /* returns None if Err */
            .map(|value| T::from(H::from(value)))
    }
}

impl<H, T> DoubleEndedIterator for ConfigFinderIter<H, T>
    where H: From<usize>,
          T: From<H>
{
    /** Removes and returns an element from the end of the iterator.
     */
    fn next_back(&mut self) -> Option<Self::Item> {
        self.m_iter
            .find_next_back()
            .unwrap_or_default() /* returns None if Err */
            .map(|value| T::from(H::from(value)))
    }
}
