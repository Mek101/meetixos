/*! # MeetiX Userland Standard Library
 *
 * MeetiX standard library implementation
 */

#![no_std]

pub use prelude::*;

pub mod prelude {
    pub use ::core::{
        prelude::rust_2018::*,
        *
    };

    pub use ::api::*;
    pub mod api_bits {
        pub use ::api_data::*;
    }
    pub use ::bits::*;
    pub use ::helps::*;
    pub use ::rt::*;
    pub use ::symbols::*;
    pub use ::sync::*;
}
