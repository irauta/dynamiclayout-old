
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use {FieldSpan, LayoutField, DynamicField, ArrayField, LayoutDynamicField, AccessDynamicField,
     LengthType, OffsetType, StrideType};
use vector_types::*;

pub struct PrimitiveArrayAccessor<'a, T: 'a> {
    pub bytes: *mut u8,
    pub stride: StrideType,
    pub length: usize,
    pub phantom: PhantomData<&'a T>,
}

impl<'a, T: 'a> PrimitiveArrayAccessor<'a, T> {
    fn index(&self, index: usize) -> *mut T {
        if index >= self.length {
            panic!("PrimitiveArrayAccessor index out of bounds: the len is {} but the index is {}",
                   self.length,
                   index);
        }
        unsafe { self.bytes.offset(index as isize * self.stride as isize) as *mut T }
    }
}

impl<'a, T: 'a> Index<usize> for PrimitiveArrayAccessor<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.index(index) }
    }
}

impl<'a, T: 'a> IndexMut<usize> for PrimitiveArrayAccessor<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *self.index(index) }
    }
}

macro_rules! impl_primitive_type_array {
    ($array_size:expr, $primitive_type:ty) => (
        impl LayoutDynamicField for [$primitive_type; $array_size] {
            type Layout = ArrayField;

            fn make_layout(layout_field: &LayoutField) -> Result<Self::Layout, ()> {
                if let LayoutField::ArrayField(offset, stride) = *layout_field {
                    Ok(ArrayField { offset: offset, stride: stride })
                } else {
                    Err(())
                }
            }

            fn get_field_spans(layout: &Self::Layout) -> Box<Iterator<Item=FieldSpan>> {
                let offset = layout.offset;
                let stride = layout.stride;
                Box::new((0..$array_size).map(move |i| FieldSpan {
                    offset: (offset + stride * i) as OffsetType,
                    length: ::std::mem::size_of::<$primitive_type>() as LengthType,
                }))
            }
        }

        impl<'a> AccessDynamicField<'a> for [$primitive_type; $array_size] {
            type Accessor = PrimitiveArrayAccessor<'a, $primitive_type>;

            unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8) -> Self::Accessor {
                PrimitiveArrayAccessor {
                    bytes: bytes.offset(layout.offset as isize),
                    stride: layout.stride,
                    length: $array_size,
                    phantom: PhantomData,
                }
            }
        }
    );
}

macro_rules! impl_primitive_type {

    ($primitive_type:ty) => (
        impl LayoutDynamicField for $primitive_type {
            type Layout = DynamicField;

            fn make_layout(layout_field: &LayoutField) -> Result<Self::Layout, ()> {
                if let LayoutField::PrimitiveField(offset) = *layout_field {
                    Ok(DynamicField { offset: offset })
                } else {
                    Err(())
                }
            }

            fn get_field_spans(layout: &Self::Layout) -> Box<Iterator<Item=FieldSpan>> {
                let span = FieldSpan {
                    offset: layout.offset,
                    length: ::std::mem::size_of::<$primitive_type>() as LengthType,
                };
                Box::new(Some(span).into_iter())
            }
        }

        impl<'a> AccessDynamicField<'a> for $primitive_type {
            type Accessor = &'a mut $primitive_type;

            unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8) -> Self::Accessor {
                &mut *(layout.offset_ptr(bytes) as *mut $primitive_type)
            }
        }

        repeat_macro!(impl_primitive_type_array, $primitive_type);
    );

    //($repetitions:expr, array_impl $primitive_type:ty) => ();
}

impl_primitive_type!(f32);
impl_primitive_type!(i32);
impl_primitive_type!(u32);

impl_primitive_type!(Vec2);
impl_primitive_type!(IVec2);
impl_primitive_type!(UVec2);

impl_primitive_type!(Vec3);
impl_primitive_type!(IVec3);
impl_primitive_type!(UVec3);

impl_primitive_type!(Vec4);
impl_primitive_type!(IVec4);
impl_primitive_type!(UVec4);
