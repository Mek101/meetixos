/*! # `Object` Types
 *
 * Implements the variants that identifies the various [`ObjId`]
 * implementations
 *
 * [`ObjId`]: /api/objs/struct.ObjId.html
 */

c_handy_enum! {
    /** # `Object` Types
     *
     * Lists the available object types represented by an [`ObjId`]
     *
     * [`ObjId`]: /api/objs/struct.ObjId.html
     */
    pub enum ObjType : u8 {
        /** No real uses, used as default value
         */
        Unknown = 0,

        /** Identifies a [`File`] object
         *
         * [`File`]: /api/objs/impls/struct.File.html
         */
        File = 1,

        /** Identifies a [`Dir`] object
         *
         * [`Dir`]: /api/objs/impls/struct.Dir.html
         */
        Dir = 2,

        /** Identifies a [`Link`] object
         *
         * [`Link`]: /api/objs/impls/struct.Link.html
         */
        Link = 3,

        /** Identifies a [`MMap`] object
         *
         * [`MMap`]: /api/objs/impls/struct.MMap.html
         */
        MMap = 4,

        /** Identifies an [`IpcChan`] object
         *
         * [`IpcChan`]: /api/objs/impls/struct.MMap.html
         */
        IpcChan = 5,

        /** Identifies an [`OsRawMutex`] object
         *
         * [`OsRawMutex`]: /api/objs/impls/struct.OsRawMutex.html
         */
        OsRawMutex = 6,

        /** Identifies an [`Iterator`] object
         *
         * [`Iterator`]: /api/objs/impls/struct.Iterator.html
         */
        Iterator = 7,
    }
}
