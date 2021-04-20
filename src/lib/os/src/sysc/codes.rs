/*! # System Call Codes
 *
 * Lists the system call codes enumerations.
 *
 * A system call code is an identifier of 16 bits used by the kernel in
 * conjunction with the corresponding class to access the second layer of
 * system call routines and identify the proper system call to call
 */

c_handy_enum! {
    /** # `ObjConfig` System Calls Codes
     *
     * Lists the system call codes for the [`ObjConfig`] struct
     *
     * [`ObjConfig`]: /api/objs/struct.ObjConfig.html
     */
    pub enum KernObjConfigFnId: u16 {
        ApplyConfig = 0,
    }
}

c_handy_enum! {
    /** # `TaskConfig` System Calls Codes
     *
     * Lists the system call codes for the [`TaskConfig`] struct
     *
     * [`ObjConfig`]: /api/tasks/struct.TaskConfig.html
     */
    pub enum KernTaskConfigFnId: u16 {
        CreateTask = 0,
        InitFind   = 1,
    }
}

c_handy_enum! {
    /** # `OSEntConfig` System Calls Codes
     *
     * Lists the system call codes for the [`OSEntConfig`] struct
     *
     * [`OSEntConfig`]: /api/ents/struct.OSEntConfig.html
     */
    pub enum KernOSEntConfigFnId: u16 {
        CreateEntity = 0,
        InitFind     = 1,
    }
}

c_handy_enum! {
    /** # `Object` System Calls Codes
     *
     * Lists the system call codes for the [`Object`] trait
     *
     * [`Object`]: /api/objs/trait.Object.html
     */
    pub enum KernObjectFnId: u16 {
        AddRef     = 0,
        Drop       = 1,
        DropName   = 2,
        Info       = 3,
        UpdateInfo = 4,
        Send       = 5,
        Recv       = 6,
        Watch      = 7,
        IsValid    = 8,
    }
}

c_handy_enum! {
    /** # `Task` System Calls Codes
     *
     * Lists the system call codes for the [`Task`] trait
     *
     * [`Task`]: /api/tasks/struct.Task.html
     */
    pub enum KernTaskFnId: u16 {
        This      = 0,
        Terminate = 1,
        Yield     = 2,
        IsAlive   = 3,
    }
}

c_handy_enum! {
    /** # `Dir` System Calls Codes
     *
     * Lists the system call codes for the [`Dir`] struct
     *
     * [`Dir`]: /api/objs/impls/struct.Dir.html
     */
    pub enum KernDirFnId: u16 {
        InitIter = 0,
    }
}

c_handy_enum! {
    /** # `File` System Calls Codes
     *
     * Lists the system call codes for the [`File`] struct
     *
     * [`File`]: /api/objs/impls/struct.File.html
     */
    pub enum KernFileFnId: u16 {
        ReadData  = 0,
        WriteData = 1,
        Copy      = 2,
        Move      = 3,
        SetPos    = 4,
        MapToMem  = 5,
    }
}

c_handy_enum! {
    /** # `IpcChan` System Calls Codes
     *
     * Lists the system call codes for the [`IpcChan`] struct
     *
     * [`IpcChan`]: /api/objs/impls/struct.IpcChan.html
     */
    pub enum KernIpcChanFnId: u16 {
        Send = 0,
        Recv = 1,
    }
}

c_handy_enum! {
    /** # `Iterator` System Calls Codes
     *
     * Lists the system call codes for the [`Iterator`] struct
     *
     * [`Iterator`]: /api/objs/impls/struct.Iterator.html
     */
    pub enum KrnIteratorFnId: u16 {
        NextValue        = 0,
        SetBeginToEndPos = 1,
        SetEndToBeginPos = 2,
    }
}

c_handy_enum! {
    /** # `Link` System Calls Codes
     *
     * Lists the system call codes for the [`Link`] struct
     *
     * [`Link`]: /api/objs/impls/struct.Link.html
     */
    pub enum KernLinkFnId: u16 {
        Deref   = 0,
        ReferTo = 1,
    }
}

c_handy_enum! {
    /** # `MMap` System Calls Codes
     *
     * Lists the system call codes for the [`MMap`] struct
     *
     * [`MMap`]: /api/objs/impls/struct.MMap.html
     */
    pub enum KernMMapFnId: u16 {
        GetPtr  = 0,
        DropPtr = 1,
        IsFile  = 2,
    }
}

c_handy_enum! {
    /** # `Mutex` System Calls Codes
     *
     * Lists the system call codes for the [`Mutex`] struct
     *
     * [`Mutex`]: /api/objs/impls/struct.Mutex.html
     */
    pub enum KernMutexFnId: u16 {
        Lock     = 0,
        TryLock  = 1,
        Unlock   = 2,
        IsLocked = 3,
    }
}

c_handy_enum! {
    /** # `Instant` System Calls Codes
     *
     * Lists the system call codes for the [`Instant`] struct
     *
     * [`Instant`]: /api/time/struct.Instant.html
     */
    pub enum KernTimeInstFnId: u16 {
        Now = 0,
    }
}

c_handy_enum! {
    /** # `Path` System Calls Codes
     *
     * Lists the system call codes for the [`Path`] struct
     *
     * [`Path`]: /api/path/struct.Mutex.html
     */
    pub enum KernPathFnId: u16 {
        Exists = 0,
    }
}

c_handy_enum! {
    /** # `OSEntity` System Calls Codes
     *
     * Lists the system call codes for the [`OSEntity`] trait
     *
     * [`OSEntity`]: /api/ents/trait.OSEntity.html
     */
    pub enum KernOSEntFnId: u16 {
        Name = 0,
    }
}

c_handy_enum! {
    /** # `OSUser` System Calls Codes
     *
     * Lists the system call codes for the [`OSUser`] struct
     *
     * [`OSUser`]: /api/ents/impls/struct.OSUser.html
     */
    pub enum KernOSUserFnId: u16 {
        Groups = 0,
    }
}

c_handy_enum! {
    /** # `OSGroup` System Calls Codes
     *
     * Lists the system call codes for the [`OSGroup`] struct
     *
     * [`OSGroup`]: /api/ents/impls/struct.OSGroup.html
     */
    pub enum KernOSGroupFnId: u16 {
        AddUser = 0,
    }
}

c_handy_enum! {
    /** # `Proc` System Calls Codes
     *
     * Lists the system call codes for the [`Proc`] struct
     *
     * [`Proc`]: /api/tasks/impls/struct.Proc.html
     */
    pub enum KernProcFnId: u16 {
        MainThread = 0,
    }
}

c_handy_enum! {
    /** # `Thread` System Calls Codes
     *
     * Lists the system call codes for the [`Thread`] struct
     *
     * [`Thread`]: /api/tasks/impls/struct.Thread.html
     */
    pub enum KernThreadFnId: u16 {
        WaitFor = 0,
        AddCleaner = 1,
        CallbackReturn = 2,
        GetEntryData = 3,
    }
}
