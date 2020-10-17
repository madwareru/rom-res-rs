mod enumerations;
mod repr;

use std::{
    collections::{VecDeque, HashMap},
    io::{Read, Seek, SeekFrom},
    rc::Rc
};
use crate::enumerations::*;
use crate::repr::*;

pub use crate::repr::ResourceFile;
pub use crate::enumerations::RomResourceError;

impl<T: Read+Seek> ResourceFile<T> {
    fn seek_stream_from_current(stream: &mut T, offset: i64) -> Result<(), RomResourceError> {
        if stream.seek(SeekFrom::Current(offset)).is_err() {
            Err(RomResourceError::UnableToSeekFromCurrent(offset))
        } else {
            Ok(())
        }
    }

    fn seek_stream_from_start(stream: &mut T, offset: usize) -> Result<(), RomResourceError> {
        if stream.seek(SeekFrom::Start(offset as u64)).is_err() {
            Err(RomResourceError::UnableToSeekFromStart(offset))
        } else {
            Ok(())
        }
    }

    fn read_u32(stream: &mut T) -> Result<u32, RomResourceError> {
        let buf = &mut [0u8; 4];
        if stream.read(buf).is_err() {
            return Err(RomResourceError::UnableToRead)
        };
        Ok( buf[0] as u32 +
            buf[1] as u32 * 0x100 +
            buf[2] as u32 * 0x10000 +
            buf[3] as u32 * 0x1000000
        )
    }

    fn read_resource_header(stream: &mut T) -> Result<ResourceHeader, RomResourceError> {
        Self::seek_stream_from_current(stream, 4)?;
        let offset = Self::read_u32(stream)?;
        let size = Self::read_u32(stream)?;
        let kind = match Self::read_u32(stream)? {
            0 => ResourceKind::File,
            1 => ResourceKind::Directory,
            _ => return Err(RomResourceError::UnknownResourceKind)
        };
        let mut name_bytes = [0u8; 0x10];
        if stream.read(&mut name_bytes).is_err() {
            return Err(RomResourceError::UnableToRead)
        };
        let name = cp866_rs::decode_bytes(&name_bytes);
        return Ok(
            ResourceHeader {
                offset,
                size,
                name,
                kind
            }
        )
    }

    pub fn new(stream: T) -> Result<Self, RomResourceError> {
        let mut stream = stream;
        Self::seek_stream_from_start(&mut stream, 0)?;
        let signature = Self::read_u32(&mut stream)?;
        if signature != 0x31_41_59_26 {
            return Err(RomResourceError::IncorrectSignature)
        };

        let root_offset = Self::read_u32(&mut stream)? as usize;
        let root_size = Self::read_u32(&mut stream)? as usize;
        let _resource_flags = Self::read_u32(&mut stream)?;
        let fat_offset = Self::read_u32(&mut stream)? as usize;
        let fat_size = Self::read_u32(&mut stream)? as usize;

        Self::seek_stream_from_start(&mut stream, fat_offset)?;
        let mut file_allocation_table =
            Vec::with_capacity(fat_size as usize);
        for _ in 0..fat_size {
            file_allocation_table.push(Self::read_resource_header(&mut stream)?);
        }

        let mut queue = VecDeque::new();
        let root_path = Rc::new("".to_string());
        for i in 0..root_size {
            queue.push_back((root_path.clone(), root_offset + i));
        }

        let mut file_lookup = HashMap::new();

        while !queue.is_empty() {
            let (parent_path, child_header_id) = queue.pop_front().unwrap();
            let child_header = &file_allocation_table[child_header_id];
            let mut name = (*parent_path).clone();
            name.push_str(&child_header.name);
            match child_header.kind {
                ResourceKind::Directory => {
                    name.push('/');
                    let parent_path = Rc::new(name);
                    for i in 0..child_header.size {
                        queue.push_back((
                            parent_path.clone(),
                            (child_header.offset + i) as usize
                        ));
                    }
                },
                ResourceKind::File => {
                    let offset = child_header.offset as usize;
                    let size = child_header.size as usize;
                    file_lookup.insert(
                        name,
                        ( offset..offset+size, None )
                    );
                }
            };
        }

        Ok(ResourceFile{
            stream,
            file_lookup
        })
    }

    fn ensure_resource_bytes(&mut self, path: &str) -> Result<(), RomResourceError> {
        match self.file_lookup.get_mut(path) {
            None => Err(RomResourceError::NonExistentResource),
            Some(data_entry) => {
                if let None = data_entry.1 {
                    let offset = data_entry.0.start;
                    let size = data_entry.0.end - offset;
                    Self::seek_stream_from_start(&mut self.stream, offset)?;
                    let mut vec = vec![0u8; size];
                    if self.stream.read(&mut vec).is_err() {
                        return Err(RomResourceError::UnableToRead);
                    }
                    data_entry.1 = Some(vec);
                }
                Ok(())
            }
        }
    }

    pub fn get_resource_bytes(&mut self, path: &str) -> Result<&[u8], RomResourceError> {
        self.ensure_resource_bytes(path)?;
        match self.file_lookup.get(path) {
            Some((_, Some(bytes))) => Ok(bytes),
            _ => unreachable!()
        }
    }

    pub fn get_resource_list(&self) -> Vec<String> {
        let mut vec = Vec::with_capacity(self.file_lookup.len());
        for (key, _) in self.file_lookup.iter() {
            vec.push(key.clone());
        };
        vec.sort();
        vec
    }

    pub fn flush_cache(&mut self) {
        for (_, (_, data)) in self.file_lookup.iter_mut() {
            if data.is_some() {
                *data = None;
            }
        };
    }
}