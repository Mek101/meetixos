/*! # `Task` Types
 *
 * Implements the variants that identifies the various [`TaskId`]
 * implementations
 *
 * [`TaskId`]: /api/tasks/struct.TaskId.html
 */

c_handy_enum! {
    /** # `Task` Types
     *
     * Lists the available object types represented by an [`TaskId`]
     *
     * [`TaskId`]: /api/tasks/struct.TaskId.html
     */
    pub enum TaskType: u8 {
        /** No real uses, used as default value
         */
        Unknown = 0,

        /** Identifies a [`Thread`] task
         *
         * [`Thread`]: /api/tasks/impls/struct.Thread.html
         */
        Thread  = 1,

        /** Identifies a [`Proc`] task
         *
         * [`Proc`]: /api/tasks/impls/struct.Proc.html
         */
        Proc    = 2,
    }
}
