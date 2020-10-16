use std::{
    collections::HashMap,
    io::{Read, Seek}
};
use crate::enumerations::{
    ResourceKind
};
use std::ops::Range;

pub(crate) struct ResourceHeader {
    pub(crate) offset: u32,
    pub(crate) size: u32,
    pub(crate) name: String,
    pub(crate) kind: ResourceKind
}

pub struct ResourceFile<T: Read+Seek> {
    pub(crate) stream: T,
    pub(crate) file_lookup: HashMap<String, (Range<usize>, Option<Vec<u8>>)>
}