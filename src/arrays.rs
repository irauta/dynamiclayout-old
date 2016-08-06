
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use {PrimitiveType, StrideType, OffsetType, LengthType, FieldSpan};

pub struct ArrayAccessor<'a, T: 'a> {
    bytes: *mut u8,
    stride: StrideType,
    length: usize,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: 'a> ArrayAccessor<'a, T> {
    fn index(&self, index: usize) -> *mut T {
        if index >= self.length {
            panic!("ArrayAccessor index out of bounds: the len is {} but the index is {}",
                   self.length,
                   index);
        }
        unsafe { self.bytes.offset(index as isize * self.stride as isize) as *mut T }
    }
}

impl<'a, T: 'a> Index<usize> for ArrayAccessor<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.index(index) }
    }
}

impl<'a, T: 'a> IndexMut<usize> for ArrayAccessor<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *self.index(index) }
    }
}

macro_rules! impl_arrays {
    ($array_length:expr) => (
        impl<T> ::LayoutDynamicField for [T; $array_length] where T: PrimitiveType {
            type Layout = ::ArrayField;

            fn make_layout(layout_field: &::LayoutField) -> Result<Self::Layout, ()> {
                if let ::LayoutField::ArrayField(offset, stride) = *layout_field {
                    Ok(::ArrayField { offset: offset, stride: stride })
                } else {
                    Err(())
                }
            }

            fn get_field_spans(layout: &Self::Layout) -> Box<Iterator<Item=::FieldSpan>> {
                let offset = layout.offset;
                let stride = layout.stride;
                Box::new((0..$array_length).map(move |i| FieldSpan {
                    offset: (offset + stride * i) as OffsetType,
                    length: (::std::mem::size_of::<T>()) as LengthType,
                }))
            }
        }

        impl<'a, T> ::AccessDynamicField<'a> for [T; $array_length] where T: 'a + PrimitiveType  {
            type Accessor = ArrayAccessor<'a, T>;

            unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8) -> Self::Accessor {
                ArrayAccessor {
                    bytes: bytes.offset(layout.offset as isize),
                    stride: layout.stride,
                    length: $array_length,
                    phantom: PhantomData,
                }
            }
        }
    )
}

impl_arrays!(1);
impl_arrays!(2);
impl_arrays!(3);
impl_arrays!(4);
impl_arrays!(5);
impl_arrays!(6);
impl_arrays!(7);
impl_arrays!(8);
impl_arrays!(9);
impl_arrays!(10);
impl_arrays!(11);
impl_arrays!(12);
impl_arrays!(13);
impl_arrays!(14);
impl_arrays!(15);
impl_arrays!(16);
impl_arrays!(17);
impl_arrays!(18);
impl_arrays!(19);
impl_arrays!(20);
impl_arrays!(21);
impl_arrays!(22);
impl_arrays!(23);
impl_arrays!(24);
impl_arrays!(25);
impl_arrays!(26);
impl_arrays!(27);
impl_arrays!(28);
impl_arrays!(29);
impl_arrays!(30);
impl_arrays!(31);
impl_arrays!(32);
impl_arrays!(64);
impl_arrays!(128);
