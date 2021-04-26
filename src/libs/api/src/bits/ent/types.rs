/*! # `OSEntity` Types
 *
 * Implements the variants that identifies the various [`OSEntityId`]
 * based implementations
 *
 * [`OSEntityId`]: /api/ents/struct.OSEntityId.html
 */

c_handy_enum! {
    /** # `OSEntity` Types
     *
     * Lists the available object types represented by an [`OSEntityId`]
     *
     * [`OSEntityId`]: /api/ents/struct.OSEntityId.html
     */
    pub enum OSEntityType: u8 {
        /** No real uses, used as default or error value
         */
        Unknown = 0,

        /** Identifies an [`OSUser`] entity
         *
         * [`OSUser`]: /api/ents/impls/struct.OSUser.html
         */
        User = 1,

        /** Identifies an [`OSGroup`] entity
         *
         * [`OSGroup`]: /api/ents/impls/struct.OSGroup.html
        */
        Group = 2,
    }
}
