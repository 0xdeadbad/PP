use crate::impl_UnsafeInitialization;
use crate::util::UnsafeInitialization;
use super::bindings;

pub type ImageDosHeader        = bindings::__IMAGE_DOS_HEADER;
pub type ImageDataDirectory    = bindings::__IMAGE_DATA_DIRECTORY;
pub type ImageOptionalHeader32 = bindings::__IMAGE_OPTIONAL_HEADER;
pub type ImageOptionalHeader64 = bindings::__IMAGE_OPTIONAL_HEADER64;
pub type ImageFileHeader       = bindings::__IMAGE_FILE_HEADER;
pub type ImageNtHeadesr32      = bindings::__IMAGE_NT_HEADERS;
pub type ImageNtHeadesr64      = bindings::__IMAGE_NT_HEADERS64;
pub type ImageImportDescriptor = bindings::__IMAGE_IMPORT_DESCRIPTOR;
pub type ImageBaseRelocation   = bindings::__IMAGE_BASE_RELOCATION;
pub type ImageSectionHeader    = bindings::__IMAGE_SECTION_HEADER;
pub type BaseRelocationOffset  = u16;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BaseRelocation {
    pub image_base_relocation: ImageBaseRelocation,
    pub offsets: Vec<BaseRelocationOffset>
}


impl_UnsafeInitialization!(for 
    ImageDosHeader,
    ImageDataDirectory,
    ImageOptionalHeader32,
    ImageOptionalHeader64,
    ImageFileHeader,
    ImageNtHeadesr32,
    ImageNtHeadesr64,
    ImageImportDescriptor,
    ImageBaseRelocation,
    ImageSectionHeader
);