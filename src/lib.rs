
pub type OffsetType = u16;
pub type StrideType = u16;
pub type LengthType = u16;

macro_rules! repeat_macro {
    ($repeated_macro:ident $($extra_params:tt)*) => (
        $repeated_macro!(1 $($extra_params)*);
        $repeated_macro!(2 $($extra_params)*);
        $repeated_macro!(3 $($extra_params)*);
        $repeated_macro!(4 $($extra_params)*);
        $repeated_macro!(5 $($extra_params)*);
        $repeated_macro!(6 $($extra_params)*);
        $repeated_macro!(7 $($extra_params)*);
        $repeated_macro!(8 $($extra_params)*);
        $repeated_macro!(9 $($extra_params)*);
        $repeated_macro!(10 $($extra_params)*);
        $repeated_macro!(11 $($extra_params)*);
        $repeated_macro!(12 $($extra_params)*);
        $repeated_macro!(13 $($extra_params)*);
        $repeated_macro!(14 $($extra_params)*);
        $repeated_macro!(15 $($extra_params)*);
        $repeated_macro!(16 $($extra_params)*);
        $repeated_macro!(17 $($extra_params)*);
        $repeated_macro!(18 $($extra_params)*);
        $repeated_macro!(19 $($extra_params)*);
        $repeated_macro!(20 $($extra_params)*);
        $repeated_macro!(21 $($extra_params)*);
        $repeated_macro!(22 $($extra_params)*);
        $repeated_macro!(23 $($extra_params)*);
        $repeated_macro!(24 $($extra_params)*);
        $repeated_macro!(25 $($extra_params)*);
        $repeated_macro!(26 $($extra_params)*);
        $repeated_macro!(27 $($extra_params)*);
        $repeated_macro!(28 $($extra_params)*);
        $repeated_macro!(29 $($extra_params)*);
        $repeated_macro!(30 $($extra_params)*);
        $repeated_macro!(31 $($extra_params)*);
        $repeated_macro!(32 $($extra_params)*);
        $repeated_macro!(64 $($extra_params)*);
        $repeated_macro!(128 $($extra_params)*);
    )
}

pub mod primitive_types;
pub mod vector_types;
pub mod matrix_types;

pub use vector_types::*;
pub use matrix_types::*;

pub trait StructField<'a> : AccessDynamicField<'a> {}

pub enum LayoutField<'a> {
    PrimitiveField(OffsetType),
    ArrayField(OffsetType, StrideType),
    MatrixArrayField(OffsetType, StrideType, StrideType),
    StructField(&'a LoadStructLayout),
    StructArrayField(&'a [&'a LoadStructLayout]),
}

pub trait LoadStructLayout {
    fn get_field_layout(&self, field_name: &str) -> Option<&LayoutField>;
}

impl<'a> LoadStructLayout for LayoutField<'a> {
    fn get_field_layout(&self, field_name: &str) -> Option<&LayoutField> {
        match *self {
            LayoutField::StructField(ref inner) => inner.get_field_layout(field_name),
            _ => None
        }
    }
}

impl<'a> LoadStructLayout for &'a [(&'a str, ::LayoutField<'a>)] {
    fn get_field_layout(&self, field_name: &str) -> Option<&LayoutField> {
        self.iter().find(|x| x.0 == field_name).map(|x| &x.1)
    }
}


#[derive(Default, Debug)]
pub struct DynamicField {
    offset: OffsetType,
}

impl DynamicField {
    pub unsafe fn offset_ptr(&self, ptr: *mut u8) -> *mut u8 {
        ptr.offset(self.offset as isize)
    }
}

#[derive(Default, Debug)]
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

#[derive(Default, Debug)]
pub struct MatrixArrayField {
    offset: OffsetType,
    array_stride: StrideType,
    matrix_stride: StrideType,
}


#[derive(Debug)]
pub struct FieldSpan {
    pub offset: OffsetType,
    pub length: LengthType,
}

pub trait LayoutDynamicField {
    type Layout;

    fn make_layout(layout_field: &LayoutField) -> Result<Self::Layout, ()>;

    fn get_field_spans(layout: &Self::Layout) -> Box<Iterator<Item = FieldSpan>>;
}

pub trait AccessDynamicField<'a>: LayoutDynamicField {
    type Accessor: 'a;

    unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8) -> Self::Accessor;
}

#[macro_export]
macro_rules! dynamiclayout {
    (
        $layout_struct_name:ident + $accessor_struct_name:ident {
            $($field_name:ident : $field_type:ty),+
        }
    ) => (
        #[derive(Default, Debug)]
        pub struct $layout_struct_name {
            $($field_name: <$field_type as $crate::LayoutDynamicField>::Layout),+
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

            fn make_layout(layout: &$crate::LayoutField) -> Result<Self::Layout, ()> {
                if let $crate::LayoutField::StructField(ref layout) = *layout {
                    Ok($layout_struct_name {
                        $($field_name: try!(layout
                            .get_field_layout(stringify!($field_name))
                            .ok_or(())
                            .and_then(<$field_type as $crate::LayoutDynamicField>::make_layout))
                        ),+
                    })
                } else {
                    Err(())
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

        impl<'a> $crate::StructField<'a> for $layout_struct_name {}

        pub struct $accessor_struct_name<'a> {
            $(pub $field_name: <$field_type as $crate::AccessDynamicField<'a>>::Accessor),+
        }
    )
}

macro_rules! impl_struct_array {
    ($length:expr) => (
        impl<'a, T> LayoutDynamicField for [T; $length] where T: StructField<'a> {
            type Layout = [<T as LayoutDynamicField>::Layout; $length];

            fn make_layout(layout: &LayoutField) -> Result<Self::Layout, ()> {
                if let $crate::LayoutField::StructArrayField(ref layouts) = *layout {
                    let mut array: Self::Layout = unsafe { ::std::mem::zeroed() };
                    if array.len() != layouts.len() {
                        return Err(());
                    }
                    for i in 0..array.len() {
                        let layout_field = LayoutField::StructField(layouts[i]);
                        array[i] = try!(<T as LayoutDynamicField>::make_layout(&layout_field));
                    }
                    Ok(array)
                } else {
                    Err(())
                }
            }

            fn get_field_spans(layout: &Self::Layout) -> Box<Iterator<Item=FieldSpan>> {
                let spans: Vec<_> = layout.iter().flat_map(|l| <T as LayoutDynamicField>::get_field_spans(l)).collect();
                Box::new(spans.into_iter())
            }
        }

        impl<'a, T> AccessDynamicField<'a> for [T; $length] where T: StructField<'a> {
            type Accessor = [<T as AccessDynamicField<'a>>::Accessor; $length];

            unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8) -> Self::Accessor {
                let mut array: Self::Accessor = ::std::mem::zeroed();
                for i in 0..array.len() {
                    array[i] = <T as AccessDynamicField>::accessor_from_layout(&layout[i], bytes);
                }
                array
            }
        }
    )
}

repeat_macro!(impl_struct_array);

#[cfg(test)]
mod tests;
