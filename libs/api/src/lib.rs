/*! # API library
 *
 * Implements an object oriented interface to interact with the MeetiX's
 * kernel.
 *
 * The crate is mainly divided in two pieces, [`Objects`] and [`Tasks`].
 *
 * # Objects
 * A kernel's abstraction based on [`ObjId`] which is representable into the
 * filesystem.
 *
 * The [`ObjId`] is an opaque handle that takes the place of the old style
 * file descriptor integer, used by all the Unix-like OS to keep reference
 * to an open resource (commonly files, directories, links, sockets, and so
 * on), used as argument to C functions to perform operations on the
 * resource referenced by the file descriptor.
 *
 * Obviously this implementation doesn't change the world, but introduces
 * pretty stuffs, like:
 * * [`RAII`] support for **all** the [`ObjId` based objects], so no more
 *   `close()`, `delete()`, `detach()` calls, because are implicit in the
 *   [`Drop`] of the object
 * * [`ObjConfig`] - the common and flexible way to open/create new
 *   [`ObjId`] based objects
 * * Common representation - all the objects are representable (can be
 *   unnamed) into the filesystem as nodes with a name and a parent, so they
 *   can be referenced with a path using the [`ObjConfig`]
 * * Common [`Object`] interface which gives to each implementation a set of
 *   common methods to [delete the object], [obtain informations] about it
 *   (like a Unix-like [`stat()`]) and to [watch it]
 * * Shareability - with to the [`Object`] trait each object is [sendable]
 *   and [receivable]. This simplifies a lot (for the userspace) resource
 *   sharing among different thread/processes. So share a [`File`] among
 *   different processes is exactly the same to share a [`Mutex`] or
 *   [`MMap`]
 *
 * # Tasks
 * A kernel's abstraction based on [`TaskId`] to reference an active task.
 *
 * The [`TaskId`] is an opaque handle, like the [`ObjId`], that takes the
 * place of the old style [`pid_t`], [`pthread_t`], used by all the
 * Unix-like OS to identify processes and threads.
 *
 * Like for objects this implementation doesn't change the world, but
 * introduces pretty stuffs, like:
 * * [`TaskConfig`] - the common and flexible way to spawn thread and
 *   processes with a variable length of parameters and customizations that
 *   can be given before run the new task
 *
 * [`Objects`]: /api/objs/index.html
 * [`Tasks`]: /api/tasks/index.html
 * [`ObjId`]: crate::objs::ObjId
 * [`RAII`]: https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization
 * [`ObjId` based objects]: /api/objs/impls/index.html
 * [`Drop`]: core::ops::Drop
 * [`ObjConfig`]: crate::objs::ObjConfig
 * [`Object`]: crate::objs::Object
 * [delete the object]: crate::objs::Object::drop_name
 * [obtain informations]: crate::objs::Object::infos
 * [`stat()`]: https://en.wikipedia.org/wiki/Stat_(system_call)
 * [watch it]: crate::objs::Object::watch
 * [sendable]: crate::objs::Object::send
 * [receivable]: crate::objs::Object::recv
 * [`File`]: crate::objs::impls::File
 * [`Mutex`]: crate::objs::impls::Mutex
 * [`MMap`]: crate::objs::impls::MMap
 * [`TaskId`]: crate::tasks::TaskId
 * [`pid_t`]: https://www.gnu.org/software/libc/manual/html_node/Process-Identification.html
 * [`pthread_t`]: https://www.man7.org/linux/man-pages/man3/pthread_self.3.html
 * [`TaskConfig`]: crate::tasks::TaskConfig
 */

#![no_std]
#![feature(asm, array_methods, min_specialization, core_intrinsics)]
#![allow(broken_intra_doc_links, private_intra_doc_links)]

pub mod bits;
pub mod caller;
pub mod ents;
pub mod errors;
pub mod namespace;
pub mod objs;
pub mod path;
pub mod tasks;
pub mod time;

mod config;
