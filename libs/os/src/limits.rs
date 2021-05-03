/*! # System Limits
 *
 * Lists the various limits constants valid across the system
 */

/** Represents the maximum length in bytes for a filesystem path managed by
 * the VFS kernel module
 */
pub const VFS_PATH_LEN_MAX: usize = 1024;

/** Represents the maximum length in bytes for a single name in a filesystem
 * path
 */
pub const VFS_NAME_LEN_MAX: usize = 256;

/** Represents the maximum amount of threads that could use
 * [`Object::watch()`] at the same time for the same object
 *
 * [`Object::watch()`]: api::objs::object::Object::watch
 */
pub const OBJ_WATCHERS_COUNT_MAX: usize = 64;

/** Represents the maximum amount of [`Object`]'s instance each process can
 * open simultaneously
 *
 * [`Object`]: api::objs::object::Object
 */
pub const OBJ_OPENED_COUNT_MAX: usize = 1024;

/** Represents the maximum length in bytes for an [`OSEntity`] name
 *
 * [`OSEntity`]: api::ents::entity::OSEntity
 */
pub const ENTITY_NAME_LEN_MAX: usize = 64;

/** Represents the maximum amount of [`OSGroup`]s for each [`OSUser`]
 *
 * [`OSGroup`]: api::ents::group::OSGroup
 * [`OSUser`]: api::ents::user::OSUser
 */
pub const OSUSER_GROUPS_COUNT_MAX: usize = 64;

/** Represents the maximum length in bytes for a [`Task`] name
 *
 * [`Task`]: api::tasks::task::Task
 */
pub const TASK_NAME_LEN_MAX: usize = 32;

/** Represents the maximum amount of single arguments for a process
 */
pub const PROC_ARG_COUNT_MAX: usize = 32;

/** Represents the maximum length in bytes for each process argument
 */
pub const PROC_ARG_LEN_MAX: usize = 64;

/** Represents the maximum length in bytes for the formatted error message in
 * [`Error`] struct
 *
 * [`Error`]: api::errors::error::Error
 */
pub const ERROR_MESSAGE_LEN_MAX: usize = 64;
