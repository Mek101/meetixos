/*! Open Memory Mapping `Object` */

use core::{
    mem::size_of,
    ops::{
        Deref,
        DerefMut
    },
    slice
};

use api_data::obj::{
    modes::MMapPtrMode,
    types::ObjType
};

use crate::{
    handle::Result,
    obj::{
        ExecutableDataObject,
        ObjHandle,
        Object,
        SizeableDataObject,
        UserCreatableObject
    }
};
use api_data::sys::{
    codes::KernMMapFnId,
    fn_path::KernFnPath
};
use core::{
    mem::forget,
    ptr::slice_from_raw_parts_mut
};

#[repr(transparent)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct MMap {
    m_obj_handle: ObjHandle
}

impl MMap {
    pub fn ptr<T>(&self) -> Result<MMapBox<T>> {
        self.obtain_area(MMapPtrMode::ForRead)
            .map(|(raw_area_ptr, area_size_in_bytes)| {
                MMapBox::new(self, raw_area_ptr as *const T, area_size_in_bytes)
            })
    }

    pub fn ptr_mut<T>(&self) -> Result<MMapBoxMut<T>> {
        self.obtain_area(MMapPtrMode::ForWrite)
            .map(|(raw_area_ptr, area_size_in_bytes)| {
                MMapBoxMut::new(self, raw_area_ptr as *mut T, area_size_in_bytes)
            })
    }

    pub fn leak<T>(self) -> &'static mut [T] {
        let mmap_slice_ref =
            self.obtain_area(MMapPtrMode::ForWrite)
                .map(|(raw_area_ptr, area_size_in_bytes)| {
                    assert_eq!(area_size_in_bytes % size_of::<T>(), 0);
                    unsafe {
                        slice::from_raw_parts_mut(raw_area_ptr as *mut T,
                                                  area_size_in_bytes / size_of::<T>())
                    }
                })
                .unwrap_or_else(|os_err| {
                    panic!("MMap::leak() failed: cause: {}", os_err)
                });
        forget(self);
        mmap_slice_ref
    }

    pub fn is_file_backed(&self) -> bool {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_0(KernFnPath::MMap(KernMMapFnId::IsFile))
            .map(|res| res != 0)
            .unwrap_or(false)
    }

    pub fn is_device_backed(&self) -> bool {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_0(KernFnPath::MMap(KernMMapFnId::IsDevice))
            .map(|res| res != 0)
            .unwrap_or(false)
    }

    fn obtain_area(&self, ptr_mode: MMapPtrMode) -> Result<(usize, usize)> {
        let mut area_size = 0;
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_2(KernFnPath::MMap(KernMMapFnId::GetPtr),
                              ptr_mode.into(),
                              &mut area_size as *mut _ as usize)
            .map(|raw_ptr| (raw_ptr, area_size))
    }

    fn drop_ptr(&self) {
        self.obj_handle()
            .kern_handle()
            .inst_kern_call_0(KernFnPath::MMap(KernMMapFnId::DropPtr))
            .unwrap();
    }
}

impl From<ObjHandle> for MMap {
    fn from(obj_handle: ObjHandle) -> Self {
        Self { m_obj_handle: obj_handle }
    }
}

impl Object for MMap {
    const TYPE: ObjType = ObjType::MMap;

    fn obj_handle(&self) -> &ObjHandle {
        &self.m_obj_handle
    }

    fn obj_handle_mut(&mut self) -> &mut ObjHandle {
        &mut self.m_obj_handle
    }
}

impl UserCreatableObject for MMap {
    /* No methods to implement */
}

impl ExecutableDataObject for MMap {
    /* No methods to implement */
}

impl SizeableDataObject for MMap {
    /* No methods to implement */
}

pub struct MMapBox<'a, T> {
    m_mmap: &'a MMap,
    m_mem_ref: &'a [T]
}

impl<'a, T> MMapBox<'a, T> {
    /**
     * Constructs a new `MMapBox` with the given parameters
     */
    fn new(mmap: &'a MMap, raw_mem_ptr: *const T, size_in_bytes: usize) -> Self {
        assert_eq!(size_in_bytes % size_of::<T>(), 0);
        Self { m_mmap: mmap,
               m_mem_ref: unsafe {
                   slice::from_raw_parts(raw_mem_ptr, size_in_bytes / size_of::<T>())
               } }
    }
}

impl<'a, T> Deref for MMapBox<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.m_mem_ref
    }
}

impl<'a, T> Drop for MMapBox<'a, T> {
    fn drop(&mut self) {
        self.m_mmap.drop_ptr();
    }
}

pub struct MMapBoxMut<'a, T> {
    m_mmap: &'a MMap,
    m_mem_ref: &'a mut [T]
}

impl<'a, T> MMapBoxMut<'a, T> {
    /**
     * Constructs a new `MMapBoxMut` with the given parameters
     */
    fn new(mmap: &'a MMap, raw_mem_ptr: *mut T, size_in_bytes: usize) -> Self {
        assert_eq!(size_in_bytes % size_of::<T>(), 0);
        Self { m_mmap: mmap,
               m_mem_ref: unsafe {
                   slice::from_raw_parts_mut(raw_mem_ptr, size_in_bytes / size_of::<T>())
               } }
    }
}

impl<'a, T> Deref for MMapBoxMut<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.m_mem_ref
    }
}

impl<'a, T> DerefMut for MMapBoxMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.m_mem_ref
    }
}

impl<'a, T> Drop for MMapBoxMut<'a, T> {
    fn drop(&mut self) {
        self.m_mmap.drop_ptr();
    }
}
