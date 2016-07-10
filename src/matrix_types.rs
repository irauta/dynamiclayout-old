
use std::ops::{Index, IndexMut};
use {ArrayField, LayoutDynamicField, AccessDynamicField, FieldSpan};

macro_rules! make_matrix_type {
    ($matrix_type:ident [$column_count:expr][$row_count:expr] $($field:expr),+) => (
        #[repr(C, packed)]
        #[derive(Debug, Copy, Clone)]
        pub struct $matrix_type ([[f32; $row_count]; $column_count]);

        impl $matrix_type {
            pub fn new(data: [[f32; $row_count]; $column_count]) -> $matrix_type {
                $matrix_type(data)
            }
        }

        impl Index<usize> for $matrix_type {
            type Output = [f32; $row_count];

            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl IndexMut<usize> for $matrix_type {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.0[index]
            }
        }

        impl LayoutDynamicField for $matrix_type {
            type Layout = ArrayField;

            fn make_layout(layout_field: &::LayoutField) -> Result<Self::Layout, ()> {
                match *layout_field {
                    ::LayoutField::ArrayField { offset, stride } => Ok(::ArrayField { offset: offset, stride: stride }),
                    _ => Err(())
                }
            }

            fn get_field_spans(layout: &Self::Layout) -> Box<Iterator<Item=FieldSpan>> {
                let offset = layout.offset;
                let stride = layout.stride;
                Box::new((0..4).map(move |i| FieldSpan {
                    offset: (offset + stride * i) as u16,
                    length: (::std::mem::size_of::<f32>() * $row_count) as u16,
                }))
            }
        }

        impl<'a> AccessDynamicField<'a> for $matrix_type {
            type Accessor = [&'a mut [f32; $row_count]; $column_count];

            unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8) -> Self::Accessor {
                [
                    $( &mut *(layout.offset_ptr(bytes, $field) as *mut [f32; $row_count]) ),+
                ]
            }
        }
    );

    (tuple_workaround $x:expr) => ($x);
}

make_matrix_type!(Matrix2 [2][2] 0, 1);
make_matrix_type!(Matrix2x3 [2][3] 0, 1);
make_matrix_type!(Matrix2x4 [2][4] 0, 1);
make_matrix_type!(Matrix3x2 [3][2] 0, 1, 2);
make_matrix_type!(Matrix3 [3][3] 0, 1, 2);
make_matrix_type!(Matrix3x4 [3][4] 0, 1, 2);
make_matrix_type!(Matrix4x2 [4][2] 0, 1, 2, 3);
make_matrix_type!(Matrix4x3 [4][3] 0, 1, 2, 3);
make_matrix_type!(Matrix4 [4][4] 0, 1, 2, 3);
