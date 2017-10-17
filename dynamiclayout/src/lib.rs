
pub type OffsetType = u16;
pub type StrideType = u16;
pub type LengthType = u16;

pub mod primitive_types;
pub mod vector_types;
pub mod matrix_types;

pub use vector_types::*;
pub use matrix_types::*;

pub trait StructField<'a> : AccessDynamicField<'a> {}

#[derive(Copy, Clone)]
pub enum LayoutInfo<'a> {
    PrimitiveField(OffsetType),
    ArrayField(OffsetType, StrideType),
    MatrixArrayField(OffsetType, StrideType, StrideType),
    StructField(&'a LoadStructLayout),
    StructArrayField(&'a [&'a LoadStructLayout]),
}


pub trait LoadStructLayout {
    fn get_field_layout(&self, field_name: &str) -> Option<LayoutInfo>;
}

impl<'a> LoadStructLayout for LayoutInfo<'a> {
    fn get_field_layout(&self, field_name: &str) -> Option<LayoutInfo> {
        match *self {
            LayoutInfo::StructField(ref inner) => inner.get_field_layout(field_name),
            _ => None,
        }
    }
}

impl<'a> LoadStructLayout for &'a [(&'a str, ::LayoutInfo<'a>)] {
    fn get_field_layout(&self, field_name: &str) -> Option<LayoutInfo> {
        self.iter().find(|x| x.0 == field_name).map(|x| x.1)
    }
}


#[derive(Default, Debug)]
pub struct SimpleFieldLayout {
    offset: OffsetType,
}

impl SimpleFieldLayout {
    pub unsafe fn offset_ptr(&self, ptr: *mut u8) -> *mut u8 {
        ptr.offset(self.offset as isize)
    }
}

#[derive(Default, Debug)]
pub struct ArrayFieldLayout {
    offset: OffsetType,
    stride: StrideType,
}

impl ArrayFieldLayout {
    pub unsafe fn offset_ptr(&self, ptr: *mut u8, index: usize) -> *mut u8 {
        let total_offset: isize = self.offset as isize + self.stride as isize * index as isize;
        ptr.offset(total_offset)
    }
}

#[derive(Default, Debug)]
pub struct MatrixArrayFieldLayout {
    offset: OffsetType,
    array_stride: StrideType,
    matrix_stride: StrideType,
}


#[derive(Debug)]
pub struct FieldSpan {
    pub offset: OffsetType,
    pub length: LengthType,
}

pub trait DynamicLayout : LayoutDynamicField {
    fn load_layout(layout_info: &LoadStructLayout) -> Result<<Self as LayoutDynamicField>::Layout, ()>;
}

pub trait LayoutDynamicField {
    type Layout;

    fn make_layout(layout_field: LayoutInfo) -> Result<Self::Layout, ()>;

    fn get_field_spans(layout: &Self::Layout) -> Box<Iterator<Item = FieldSpan>>;
}

pub trait LayoutArrayDynamicField {
    type Layout;

    fn make_layout(layout_field: LayoutInfo, len: usize) -> Result<Self::Layout, ()>;

    fn get_field_spans(layout: &Self::Layout, len: usize) -> Box<Iterator<Item = FieldSpan>>;
}

pub trait AccessDynamicField<'a>: LayoutDynamicField {
    type Accessor: 'a;

    unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8) -> Self::Accessor;
}

pub trait AccessArrayDynamicField<'a>: LayoutArrayDynamicField {
    type Accessor: 'a;

    unsafe fn accessor_from_layout(layout: &'a Self::Layout,
                                   bytes: *mut u8,
                                   len: usize)
                                   -> Self::Accessor;
}
