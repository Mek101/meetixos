/*! # `Error` Classes
 *
 * Implements an enumeration that is responsible to provide a restricted
 * macro groups of errors that is possible to encounter using the system
 * calls
 */

c_handy_enum! {
    /** # `Error` Classes
     *
     * List the known error classes that an [`Error`] instance is permitted to
     * fall
     *
     * [`Error`]: /api/errors/struct.Error.html
     */
    pub enum ErrorClass: u8 {
        /** Value used as placeholder for uninitialized [`Error`] value
         *
         * [`Error`]: /api/errors/struct.Error.html
         */
        Unknown = 0 => "Unknown",

        /** At least one of the given arguments of the last system call doesn't
         * match the expected range
         */
        InvalidArgument = 1 => "Invalid argument",

        /** The last instance call was referencing an invalid object
         */
        InvalidHandleReference = 2 => "Invalid handle reference",

        /** The previous system call it was supposed to create an handle with a
         * specific id or a name have failed due to an already existing handle
         * with the same id/name
         */
        IdentifierNotAvailable = 3 => "Identifier not available",

        /** The previous system call was failed because the current
         * [`OSUser`]/[`OSGroup`]s have not enough grant to perform the
         * requested operation
         *
         * [`OSUser`]: /api/ents/impls/struct.OSUser.html
         * [`OSGroup`]: /api/ents/impls/struct.OSGroup.html
         */
        NotEnoughGrants = 4 => "Not enough grants",

        /** The previous system call was failed because the kernel have exhausted
         * the virtual/physical memory available
         */
        NotEnoughMemory = 5 => "Not enough memory",

        /** The previous system call was failed because the given [`Path`]
         * references an unexisting object name
         *
         * [`Path`]: /api/path/struct.Path.html
         */
        ReferenceNotFound = 6 => "Reference not found",

        /** The two [`ObjType`] types given doesn't match
         *
         * [`ObjType`]: /api/bits/obj/enum.ObjType.html
         */
        TypesNotMatch = 7 => "Types not match",

        /** The previous system call was failed because the current [`Thread`]
         * have reached the limit of referencable resources a time
         *
         * [`Thread`]: /api/tasks/impls/struct.Thread.html
         */
        LimitReached = 8 => "Limit reached",

        /** The previous system call was failed because at least one of the
         * given parameters exceed the expected limit (i.e a [`Path`] or a name
         * to long)
         *
         * [`Path`]: /api/path/struct.Path.html
         */
        LimitOverflow = 9 => "Limit overflow",

        /** The previous system call was failed because a poll requested data was
         * not still available (i.e a [`Object::recv()`] in [`RecvMode::Poll`])
         *
         * [`Object::recv()`]: /api/objs/trait.Object.html#method.recv
         * [`RecvMode::Poll`]: /api/bits/obj/enum.RecvMode.html#variant.Poll
         */
        NoDataAvailable = 10 => "Data not available",

        /** The previous system call was failed because was requested an operation
         * not enabled by the builder (i.e a [`File::read()`] without a previous
         * [`ObjConfig::for_read()`] call)
         *
         * [`File::read()`]: /api/objs/impls/struct.File.html#method.read
         * [`ObjConfig::for_read()`]: /api/objs/struct.ObjConfig.html#method.for_read
         */
        OperationNotEnabled = 11 => "Operation not enabled",

        /** This is not properly an error, just indicates that the object have no
         * more data to read (i.e in [`File`] and [`Dir`])
         *
         * [`File`]: /api/objs/impls/struct.File.html
         * [`Dir`]: /api/objs/impls/struct.Dir.html
         */
        EndOfDataReached = 12 => "End of data reached",

        /** The previous system call was failed because the running transaction was
         * interrupted by something else
         */
        InterruptedOperation = 13 => "Interrupted operation",
    }
}
