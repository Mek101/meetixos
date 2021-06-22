/*! Configuration Trait */

use core::marker::PhantomData;

use num_enum::{
    IntoPrimitive,
    TryFromPrimitive
};

/**
 * Lists the available implementations for the `ConfigMode`
 */
#[repr(u8)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum ConfigModeType {
    /**
     * Used as default value
     */
    Unknown,

    /**
     * Identifies the `CreatMode`
     */
    Create,

    /**
     * Identifies the `FindMode`
     */
    Find
}

/**
 * Represents the base interface for the configuration modes
 */
pub trait ConfigMode {
    /**
     * The `ConfigModeType` which the concrete type represents
     */
    const TYPE: ConfigModeType;
}

/**
 * Enables methods useful to customize the creation of an item defined by
 * the used configurator
 */
#[derive(Debug, Copy, Clone)]
pub struct CreatMode;

impl ConfigMode for CreatMode {
    const TYPE: ConfigModeType = ConfigModeType::Create;
}

/**
 * Enables methods useful to customize the search of an item defined by the
 * used configurator
 */
#[derive(Debug, Copy, Clone)]
pub struct FindMode;

impl ConfigMode for FindMode {
    const TYPE: ConfigModeType = ConfigModeType::Find;
}

/**
 * Library internal wrapper for `KrnIterator` that iterates instances of
 * type `T` that inside contains handles of type `H`
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
    type Item = T;

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
    fn next_back(&mut self) -> Option<Self::Item> {
        self.m_iter
            .find_next_back()
            .unwrap_or_default() /* returns None if Err */
            .map(|value| T::from(H::from(value)))
    }
}
