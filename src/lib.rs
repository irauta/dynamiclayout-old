
macro_rules! impl_primitive_accessor {
    ($primitive_type:ty) => (
        impl ::LayoutDynamicField for $primitive_type {
            type Layout = ::DynamicField;
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


#[derive(Default)]
pub struct DynamicField {
    offset: u16,
}

impl DynamicField {
    pub unsafe fn offset_ptr(&self, ptr: *mut u8) -> *mut u8 {
        ptr.offset(self.offset as isize)
    }
}

#[derive(Default)]
pub struct ArrayField {
    offset: u16,
    stride: u16,
}

impl ArrayField {
    pub unsafe fn offset_ptr(&self, ptr: *mut u8, index: u16) -> *mut u8 {
        let total_offset: isize = self.offset as isize + self.stride as isize * index as isize;
        ptr.offset(total_offset)
    }
}


pub trait LayoutDynamicField {
    type Layout;
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
            pub fn accessor<'a>(&'a self, bytes: &'a mut[u8]) -> $accessor_struct_name<'a> {
                unsafe {
                    <$layout_struct_name as $crate::AccessDynamicField>::accessor_from_layout(self, bytes.as_mut_ptr())
                }
            }
        }

        impl $crate::LayoutDynamicField for $layout_struct_name {
            type Layout = $layout_struct_name;
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
