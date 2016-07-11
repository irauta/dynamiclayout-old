
pub type OffsetType = u16;
pub type StrideType = u16;
pub type LengthType = u16;

macro_rules! impl_primitive_accessor {
    ($primitive_type:ty) => (
        impl ::LayoutDynamicField for $primitive_type {
            type Layout = ::DynamicField;

            fn make_layout(layout_field: &::LayoutField) -> Result<Self::Layout, ()> {
                match *layout_field {
                    ::LayoutField::PrimitiveField(offset) => Ok(::DynamicField { offset: offset }),
                    _ => Err(())
                }
            }

            fn get_field_spans(layout: &Self::Layout) -> Box<Iterator<Item=::FieldSpan>> {
                let span = ::FieldSpan {
                    offset: layout.offset,
                    length: ::std::mem::size_of::<$primitive_type>() as ::LengthType,
                };
                Box::new(Some(span).into_iter())
            }
        }
        impl<'a> ::AccessDynamicField<'a> for $primitive_type {
            type Accessor = &'a mut $primitive_type;

            unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8) -> Self::Accessor {
                &mut *(layout.offset_ptr(bytes) as *mut $primitive_type)
            }
        }

    )
}

impl_primitive_accessor!(f32);
impl_primitive_accessor!(i32);
impl_primitive_accessor!(u32);


pub mod vector_types;
pub mod matrix_types;


pub enum LayoutField<'a> {
    PrimitiveField (OffsetType),
    ArrayField {
        offset: OffsetType,
        stride: StrideType,
    },
    StructField(&'a LoadStructLayout),
}

pub trait LoadStructLayout {
    fn get_field_layout(&self, field_name: &str) -> Option<&LayoutField>;
}

impl<'a> LoadStructLayout for &'a [(&'a str, ::LayoutField<'a>)] {
    fn get_field_layout(&self, field_name: &str) -> Option<&LayoutField> {
        self.iter().find(|x| x.0 == field_name).map(|x| &x.1)
    }
}


#[derive(Default)]
pub struct DynamicField {
    offset: OffsetType,
}

impl DynamicField {
    pub unsafe fn offset_ptr(&self, ptr: *mut u8) -> *mut u8 {
        ptr.offset(self.offset as isize)
    }
}

#[derive(Default)]
pub struct ArrayField {
    offset: OffsetType,
    stride: StrideType,
}

impl ArrayField {
    pub unsafe fn offset_ptr(&self, ptr: *mut u8, index: usize) -> *mut u8 {
        let total_offset: isize = self.offset as isize + self.stride as isize * index as isize;
        ptr.offset(total_offset)
    }
}


#[derive(Debug)]
pub struct FieldSpan {
    pub offset: OffsetType,
    pub length: LengthType,
}

pub trait LayoutDynamicField {
    type Layout;

    fn make_layout(layout_field: &LayoutField) -> Result<Self::Layout, ()>;

    fn get_field_spans(layout: &Self::Layout) -> Box<Iterator<Item=FieldSpan>>;
}

pub trait AccessDynamicField<'a>: LayoutDynamicField {
    type Accessor: 'a;

    unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8) -> Self::Accessor;
}


macro_rules! dynamiclayout {
    (
        $layout_struct_name:ident + $accessor_struct_name:ident {
            $($field_name:ident : $field_type:ty),+
        }
    ) => (
        #[derive(Default)]
        pub struct $layout_struct_name {
            $(pub $field_name: <$field_type as $crate::LayoutDynamicField>::Layout),+
        }

        impl $layout_struct_name {
            #[allow(dead_code)]
            pub fn load_layout(layout: &$crate::LoadStructLayout) -> Result<$layout_struct_name, ()> {
                <$layout_struct_name as $crate::LayoutDynamicField>::make_layout(&$crate::LayoutField::StructField(layout))
            }

            #[allow(dead_code)]
            pub fn accessor<'a>(&'a self, bytes: &'a mut[u8]) -> $accessor_struct_name<'a> {
                unsafe {
                    <$layout_struct_name as $crate::AccessDynamicField>::accessor_from_layout(self, bytes.as_mut_ptr())
                }
            }
        }

        impl $crate::LayoutDynamicField for $layout_struct_name {
            type Layout = $layout_struct_name;

            fn make_layout(layout: &::LayoutField) -> Result<Self::Layout, ()> {
                match *layout {
                    $crate::LayoutField::StructField(ref layout) => {
                        $(let $field_name = {
                            if let Some(field) = layout.get_field_layout(stringify!($field_name)) {
                                field
                            } else {
                                return Err(());
                            }
                        };)+
                        Ok($layout_struct_name {
                            $($field_name: try!(<$field_type as $crate::LayoutDynamicField>::make_layout($field_name))),+
                        })
                    },
                    _ => Err(())
                }
            }

            fn get_field_spans(layout: &Self::Layout) -> Box<Iterator<Item=$crate::FieldSpan>> {
                Box::new(
                    ::std::iter::empty()
                    $(.chain(<$field_type as $crate::LayoutDynamicField>::get_field_spans(&layout.$field_name)))+
                )
            }
        }

        impl<'a> $crate::AccessDynamicField<'a> for $layout_struct_name {
            type Accessor = $accessor_struct_name<'a>;

            unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8) -> Self::Accessor {
                $accessor_struct_name {
                    $($field_name: <$field_type as $crate::AccessDynamicField<'a>>::accessor_from_layout(&layout.$field_name, bytes)),+
                }
            }
        }

        pub struct $accessor_struct_name<'a> {
            $(pub $field_name: <$field_type as $crate::AccessDynamicField<'a>>::Accessor),+
        }
    )
}


#[cfg(test)]
mod tests;
