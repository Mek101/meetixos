/*! # Object Modes Bits
 *
 * Implements various enumerations that are used for certain [`Object`]
 * related calls
 *
 * [`Object`]: /api/objs/trait.Object.html
 */

c_handy_enum! {
    /** # `Object::recv()` Waiting Modes
     *
     * Lists the available modes for [`Object::recv()`]
     *
     * [`Object::recv()`]: /api/objs/trait.Object.html#method.recv
     */
    pub enum RecvMode: u8 {
        /** This mode simply asks to the kernel whether an object of the requested
         * type is already available into the object receiving queue, if not it
         * returns an [`Err(RecvErr::NoAvailObj)`] error
         *
         * [`Err(RecvErr::NoAvailObj)`]: /api/errors/obj/enum.RecvErr.html#variant.NoAvailObj
         */
        Poll = 0,

        /** This mode puts the task in a waiting state until an object of the
         * requested type is available into the task's receiving queue.
         *
         * If already available the system call immediately returns and the task
         * will not fall into waiting state.
         *
         * If the kernel's wait queue is full the system call returns an
         * [`Err(RecvErr::KernWaitQueueFull)`] error
         *
         * [`Err(RecvErr::KernWaitQueueFull)`]: /api/errors/obj/enum.RecvErr.html#variant.KernWaitQueueFull
         */
        Sync = 1,
    }
}

c_handy_enum! {
    /** # `KrnIterator::find_next()` Modes
     *
     * Lists the internally used modes to identify the direction of the
     * [`KrnIterator`] in use
     *
     * [`KrnIterator`]: /api/objs/impls/struct.KrnIterator.html
     */
    pub enum KrnIterDirection: u8 {
        /** Internally used when called [`KrnIterator::find_next()`]
         *
         * [`KrnIterator::find_next()`]: /api/objs/impls/struct.KrnIterator.html#method.find_next
         */
        BeginToEnd = 0,

        /** Internally used when called [`KrnIterator::find_next_back()`]
         *
         * [`KrnIterator::find_next_back()`]: /api/objs/impls/struct.KrnIterator.html#method.find_next_back
         */
        EndToBegin = 1,
    }
}

c_handy_enum! {
    /** # `MMap::get_ptr` Modes
     *
     * Lists the internally used modes that are given to the kernel to manage
     * synchronization over the memory of a [`MMap`]
     *
     * [`MMap`]: /api/objs/impls/struct.MMap.html
     */
    pub enum MMapPtrMode: u8 {
        /** Internally used when called [`MMap::get_ptr()`]
         *
         * [`MMap::get_ptr()`]: /api/objs/impls/struct.MMap.html#method.get_ptr
         */
        Readable = 0,

        /** Internally used when called [`MMap::get_ptr_mut()`]
         *
         * [`MMap::get_ptr_mut()`]: /api/objs/impls/struct.MMap.html#method.get_ptr_mut
         */
        Writeable = 1,

        /** Internally used when called [`MMap::leak_ptr()`]
         *
         * [`MMap::leak_ptr()`]: /api/objs/impls/struct.MMap.html#method.leak_ptr
         */
        Leak = 2,
    }
}

rust_handy_enum! {
    /** # `File::set_pos` & `Dir::set_pos` Modes
     *
     * Lists the available modes for [`File::set_pos()`] and [`Dir::set_index()`]
     *
     * [`File::set_pos()`]: /api/objs/impls/struct.File.html#method.set_pos
     * [`Dir::set_pos()`]: /api/objs/impls/struct.Dir.html#method.set_index
     */
    pub enum SeekMode: u8 {
        /** The given offset will be interpreted as an absolute offset
         */
        Absolute(offset: u64) = 0,

        /** The given offset will be added to the current offset
         * (position relative)
         */
        Relative(offset: i64) = 1,

        /** The cursor of the object will be moved to the end
         */
        End = 2,
    }
}

impl SeekMode {
    /** Returns [`Some(offset)`] if the variants have any
     *
     * [`Some(offset)`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
     */
    pub fn off(&self) -> Option<usize> {
        match *self {
            SeekMode::Absolute(off) => Some(off as usize),
            SeekMode::Relative(off) => Some(off as usize),
            SeekMode::End => None
        }
    }
}
