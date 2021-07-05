/*! Configuration Trait */

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
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
#[derive(IntoPrimitive, TryFromPrimitive)]
pub enum ConfigModeType {
    /**
     * Identifies the `CreatMode`
     */
    Create,

    /**
     * Identifies the `OpenMode`
     */
    Open
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
 * Enables methods useful to customize the opening of an item defined by the
 * used configurator
 */
#[derive(Debug, Copy, Clone)]
pub struct OpenMode;

impl ConfigMode for OpenMode {
    const TYPE: ConfigModeType = ConfigModeType::Open;
}
