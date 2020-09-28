pub enum RomResourceError {
    UnableToRead,
    IncorrectSignature,
    UnableToSeekFromStart(usize),
    UnableToSeekFromCurrent(i64),
    UnknownResourceKind,
    NonExistentResource
}

pub(crate) enum ResourceKind {
    Directory,
    File
}

pub(crate) enum ResourceData {
    ByteRange(usize, usize),
    Bytes(Vec<u8>)
}