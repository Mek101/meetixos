/*! Context wide limit constants */

/**
 * Maximum length in for a filesystem path managed by the VFS kernel module
 */
pub const VFS_PATH_LEN_MAX: usize = 1024;

/**
 * Maximum length in bytes for a single name in a filesystem path
 */
pub const VFS_NAME_LEN_MAX: usize = 256;

/**
 * Maximum amount of threads that could call `api::objs::Object::watch()` at
 * the same time for the same object
 */
pub const OBJ_WATCHERS_COUNT_MAX: usize = 64;

/**
 * Maximum amount of open `api::objs::Object` instance for each process
 */
pub const OBJ_OPENED_COUNT_MAX: usize = 1024;

/**
 * Maximum length in bytes for an `api::ents::OSEntity` name
 */
pub const ENTITY_NAME_LEN_MAX: usize = 64;

/**
 * Maximum amount of `api::ents::impls::OSGroup`s for each
 * `api::ents::impls::OSUser`
 */
pub const OSUSER_GROUPS_COUNT_MAX: usize = 64;

/**
 * Maximum length in bytes for a `api::tasks::impls::Thread` name
 */
pub const THREAD_NAME_LEN_MAX: usize = 32;

/**
 * Maximum amount of single arguments for a process
 */
pub const PROC_ARG_COUNT_MAX: usize = 32;

/**
 * Maximum length in bytes for each process argument
 */
pub const PROC_ARG_LEN_MAX: usize = 64;

/**
 * Maximum length in bytes for the error message in `api::errors::Error`
 */
pub const ERROR_MESSAGE_LEN_MAX: usize = 64;
