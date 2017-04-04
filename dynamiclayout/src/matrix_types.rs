
use std::ops::{Index, IndexMut};
use {LayoutInfo, ArrayFieldLayout, MatrixArrayFieldLayout, LayoutDynamicField, AccessDynamicField, FieldSpan,
     OffsetType, LengthType, LayoutArrayDynamicField, AccessArrayDynamicField};

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
            type Layout = ArrayFieldLayout;

            fn make_layout(layout_field: &::LayoutInfo) -> Result<Self::Layout, ()> {
                if let ::LayoutInfo::ArrayField (offset, stride) = *layout_field {
                    Ok(ArrayFieldLayout { offset: offset, stride: stride })
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

        impl LayoutArrayDynamicField for $matrix_type {
            type Layout = MatrixArrayFieldLayout;

            fn make_layout(layout_field: &LayoutInfo, _: usize) -> Result<Self::Layout, ()> {
                if let LayoutInfo::MatrixArrayField(offset, array_stride, matrix_stride) = *layout_field {
                    Ok(MatrixArrayFieldLayout { offset: offset, array_stride: array_stride, matrix_stride: matrix_stride })
                } else {
                    Err(())
                }
            }

            fn get_field_spans(layout: &Self::Layout, len: usize) -> Box<Iterator<Item=FieldSpan>> {
                let offset = layout.offset;
                let array_stride = layout.array_stride;
                let matrix_stride = layout.matrix_stride;
                Box::new((0..len as u16).flat_map(move |i| (0..$row_count).map(move |r| FieldSpan {
                    offset: (offset + array_stride * i + matrix_stride * r) as OffsetType,
                    length: ::std::mem::size_of::<f32>() as LengthType * $column_count as LengthType,
                })))
            }
        }

        impl<'a> AccessArrayDynamicField<'a> for $matrix_type {
            type Accessor = Vec<<$matrix_type as AccessDynamicField<'a>>::Accessor>;

            unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8, len: usize) -> Self::Accessor {
                let mut accessor = Vec::with_capacity(len);
                for i in 0..len {
                    let offset = (i as OffsetType) * layout.array_stride + layout.offset;
                    // The pointer given to accessor_from_layout already has the offset calculated, therefore use 0 here
                    let matrix_layout = ArrayFieldLayout { offset: 0, stride: layout.matrix_stride };
                    accessor.push($matrix_type::accessor_from_layout(&matrix_layout, bytes.offset(offset as isize)));
                }
                accessor
            }
        }


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
