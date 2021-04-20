/*! # Path Related Bits
 *
 * Implements the enumeration related to certain calls of [`Path`]
 *
 * [`Path`]: /api/path/struct.Path.html
 */

use crate::bits::obj::ObjType;

rust_handy_enum! {
    /** # `Path::exists()` States
     *
     * Lists the available states for [`Path::exists()`]
     *
     * [`Path::exists()`]: /api/path/struct.Path.html
     */
    pub enum PathExistsState: u8 {
        /** The path exists from the current directory (or the root if the
         * [`Path`] is absolute) to the last component.
         *
         * It contains the [`ObjType`] of the last component referenced
         *
         * [`Path`]: /api/path/struct.Path.html
         * [`ObjType`]: /api/bits/obj/enum.ObjType.html
         */
        Exists(obj_type: ObjType) = 0,

        /** The path exists only until a certain component, the variant
         * contains the index of the last existing component
         */
        ExistsUntil(last_existing_element_index: u32) = 1,

        /** The path doesn't exists completely
         */
        NotExists = 2,

        /** An empty path was given
         */
        EmptyPath = 3,
    }
}
