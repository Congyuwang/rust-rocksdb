use crate::ffi;
use std::{
    marker::PhantomData,
    ptr::{null, slice_from_raw_parts},
};

pub struct WideColumn<'a> {
    pub name: &'a [u8],
    pub value: &'a [u8],
}

pub struct WideColumns<'a> {
    inner: *const ffi::rocksdb_widecolumns_t,
    columns_size: usize,
    iter: PhantomData<&'a ()>,
}

impl<'a> WideColumns<'a> {
    pub unsafe fn from_c(inner: *const ffi::rocksdb_widecolumns_t) -> Self {
        Self {
            inner,
            columns_size: ffi::rocksdb_widecolumns_len(inner),
            iter: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.columns_size
    }

    pub unsafe fn get_column_name_unchecked(&self, idx: usize) -> &[u8] {
        let mut name_len: usize = 0;
        let name_len_ptr: *mut usize = &mut name_len;
        let name = null::<*const i8>() as *mut *const i8;
        ffi::rocksdb_widecolumns_name(self.inner, idx, name, name_len_ptr);
        &*slice_from_raw_parts(name as *const u8, name_len)
    }

    pub unsafe fn get_column_value_unchecked(&self, idx: usize) -> &[u8] {
        let mut value_len: usize = 0;
        let value_len_ptr: *mut usize = &mut value_len;
        let value = null::<*const i8>() as *mut *const i8;
        ffi::rocksdb_widecolumns_value(self.inner, idx, value, value_len_ptr);
        &*slice_from_raw_parts(value as *const u8, value_len)
    }

    pub unsafe fn get_column_unchecked(&self, idx: usize) -> WideColumn {
        WideColumn {
            name: self.get_column_name_unchecked(idx),
            value: self.get_column_value_unchecked(idx),
        }
    }
}

pub struct PinnableWideColumns<'a> {
    inner: *const ffi::rocksdb_pinnablewidecolumns_t,
    columns_size: usize,
    iter: PhantomData<&'a ()>,
}

impl<'a> PinnableWideColumns<'a> {
    pub unsafe fn from_c(inner: *const ffi::rocksdb_pinnablewidecolumns_t) -> Self {
        Self {
            inner,
            columns_size: ffi::rocksdb_pinnablewidecolumns_len(inner),
            iter: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.columns_size
    }

    pub unsafe fn get_column_name_unchecked(&self, idx: usize) -> &[u8] {
        let mut name_len: usize = 0;
        let name_len_ptr: *mut usize = &mut name_len;
        let name = null::<*const i8>() as *mut *const i8;
        ffi::rocksdb_pinnablewidecolumns_name(self.inner, idx, name, name_len_ptr);
        &*slice_from_raw_parts(name as *const u8, name_len)
    }

    pub unsafe fn get_column_value_unchecked(&self, idx: usize) -> &[u8] {
        let mut value_len: usize = 0;
        let value_len_ptr: *mut usize = &mut value_len;
        let value = null::<*const i8>() as *mut *const i8;
        ffi::rocksdb_pinnablewidecolumns_value(self.inner, idx, value, value_len_ptr);
        &*slice_from_raw_parts(value as *const u8, value_len)
    }

    pub unsafe fn get_column_unchecked(&self, idx: usize) -> WideColumn {
        WideColumn {
            name: self.get_column_name_unchecked(idx),
            value: self.get_column_value_unchecked(idx),
        }
    }
}

impl Drop for WideColumns<'_> {
    fn drop(&mut self) {
        unsafe { ffi::rocksdb_widecolumns_destroy(self.inner as *mut _) }
    }
}

impl Drop for PinnableWideColumns<'_> {
    fn drop(&mut self) {
        unsafe { ffi::rocksdb_pinnablewidecolumns_destroy(self.inner as *mut _) }
    }
}
