/*! Configuration Trait */

use core::convert::TryFrom;

/**
 * Lists the available implementations for the `ConfigMode`
 */
#[repr(u8)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(PartialOrd, Ord)]
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

impl Into<u8> for ConfigModeType {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for ConfigModeType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Create),
            1 => Ok(Self::Open),
            _ => Err(())
        }
    }
}

/**
 * Represents the base interface for the configuration modes
 */
pub trait TConfigMode {
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

impl TConfigMode for CreatMode {
    const TYPE: ConfigModeType = ConfigModeType::Create;
}

/**
 * Enables methods useful to customize the opening of an item defined by the
 * used configurator
 */
#[derive(Debug, Copy, Clone)]
pub struct OpenMode;

impl TConfigMode for OpenMode {
    const TYPE: ConfigModeType = ConfigModeType::Open;
}
