
use std::ops::{Index, IndexMut};
use {LayoutField, ArrayField, MatrixArrayField, LayoutDynamicField, AccessDynamicField, FieldSpan, OffsetType, LengthType};

macro_rules! impl_matrix_type_array {
    ($array_size:expr, $matrix_type:ident [$column_count:expr][$row_count:expr]) => (
        impl LayoutDynamicField for [$matrix_type; $array_size] {
            type Layout = MatrixArrayField;

            fn make_layout(layout_field: &LayoutField) -> Result<Self::Layout, ()> {
                if let LayoutField::MatrixArrayField(offset, array_stride, matrix_stride) = *layout_field {
                    Ok(MatrixArrayField { offset: offset, array_stride: array_stride, matrix_stride: matrix_stride })
                } else {
                    Err(())
                }
            }

            fn get_field_spans(layout: &Self::Layout) -> Box<Iterator<Item=FieldSpan>> {
                let offset = layout.offset;
                let array_stride = layout.array_stride;
                let matrix_stride = layout.matrix_stride;
                Box::new((0..$array_size).flat_map(move |i| (0..$row_count).map(move |r| FieldSpan {
                    offset: (offset + array_stride * i + matrix_stride * r) as OffsetType,
                    length: ::std::mem::size_of::<f32>() as LengthType * $column_count as LengthType,
                })))
            }
        }

        impl<'a> AccessDynamicField<'a> for [$matrix_type; $array_size] {
            type Accessor = [<$matrix_type as AccessDynamicField<'a>>::Accessor; $array_size];

            unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8) -> Self::Accessor {
                let mut array: Self::Accessor = ::std::mem::zeroed();
                for i in 0..array.len() {
                    let offset = (i as OffsetType) * layout.array_stride + layout.offset;
                    // The pointer given to accessor_from_layout already has the offset calculated, therefore use 0 here
                    let matrix_layout = ArrayField { offset: 0, stride: layout.matrix_stride };
                    array[i] = $matrix_type::accessor_from_layout(&matrix_layout, bytes.offset(offset as isize));
                }
                array
            }
        }
    )
}

macro_rules! make_matrix_type {
    ($matrix_type:ident [$column_count:expr][$row_count:expr] $($field:expr),+) => (
        #[repr(C, packed)]
        #[derive(Debug, Copy, Clone)]
        pub struct $matrix_type ([[f32; $row_count]; $column_count]);

        impl $matrix_type {
            pub fn new(data: [[f32; $row_count]; $column_count]) -> $matrix_type {
                $matrix_type(data)
            }

            // TODO: Make sure this actually does what it should
            unsafe fn accessor_from_layout<'a, 'b>(layout: &'a <Self as LayoutDynamicField>::Layout, bytes: *mut u8) -> <Self as AccessDynamicField<'b>>::Accessor {
                [
                    $( &mut *(layout.offset_ptr(bytes, $field) as *mut [f32; $row_count]) ),+
                ]
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
                if let ::LayoutField::ArrayField (offset, stride) = *layout_field {
                    Ok(ArrayField { offset: offset, stride: stride })
                } else {
                    Err(())
                }
            }

            fn get_field_spans(layout: &Self::Layout) -> Box<Iterator<Item=FieldSpan>> {
                let offset = layout.offset;
                let stride = layout.stride;
                // TODO: 0..4 vs. 0..$column_count
                Box::new((0..$column_count).map(move |i| FieldSpan {
                    offset: (offset + stride * i) as OffsetType,
                    length: (::std::mem::size_of::<f32>() * $row_count) as LengthType,
                }))
            }
        }

        impl<'a> AccessDynamicField<'a> for $matrix_type {
            type Accessor = [&'a mut [f32; $row_count]; $column_count];

            unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8) -> Self::Accessor {
                $matrix_type::accessor_from_layout(layout, bytes)
            }
        }

        repeat_macro!(impl_matrix_type_array, $matrix_type [$column_count][$row_count]);
    );
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
