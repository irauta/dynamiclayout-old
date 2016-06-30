
pub mod primitive_types {
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Vec2 { pub x: f32, pub y: f32 }

    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct IVec2 { pub x: i32, pub y: i32 }

    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct UVec2 { pub x: u32, pub y: u32 }

    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }

    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct IVec3 { pub x: i32, pub y: i32, pub z: i32 }

    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct UVec3 { pub x: u32, pub y: u32, pub z: u32 }

    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Vec4 { pub x: f32, pub y: f32, pub z: f32, pub w: f32 }

    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct IVec4 { pub x: i32, pub y: i32, pub z: i32, pub w: i32 }

    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct UVec4 { pub x: u32, pub y: u32, pub z: u32, pub w: u32 }

    pub trait PrimitiveType: Copy {}
    impl PrimitiveType for f32 {}
    impl PrimitiveType for i32 {}
    impl PrimitiveType for u32 {}
    impl PrimitiveType for Vec2 {}
    impl PrimitiveType for Vec3 {}
    impl PrimitiveType for Vec4 {}
    impl PrimitiveType for IVec2 {}
    impl PrimitiveType for IVec3 {}
    impl PrimitiveType for IVec4 {}
    impl PrimitiveType for UVec2 {}
    impl PrimitiveType for UVec3 {}
    impl PrimitiveType for UVec4 {}

    impl<T: PrimitiveType> ::LayoutDynamicField for T {
        type Layout = ::DynamicField;
    }
    impl<'a, T: 'a + PrimitiveType> ::AccessDynamicField<'a> for T {
        type Accessor = &'a T;

        unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8) -> Self::Accessor {
            &*(layout.offset_ptr(bytes) as *mut T)
        }
    }
}

pub mod complex_types {
    use std::ops::{Index,IndexMut};
    use ::{ArrayField,LayoutDynamicField,AccessDynamicField};

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
            }

            impl<'a> AccessDynamicField<'a> for $matrix_type {
                type Accessor = [&'a [f32; $row_count]; $column_count];

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
}

#[derive(Default)]
pub struct DynamicField {
    offset: u16
}

impl DynamicField {
    pub unsafe fn offset_ptr(&self, ptr: *mut u8) -> *mut u8 {
        ptr.offset(self.offset as isize)
    }
}

#[derive(Default)]
pub struct ArrayField {
    offset: u16,
    stride: u16
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
mod tests {
    use ::primitive_types::{Vec2,Vec3,Vec4};
    use ::complex_types::Matrix4;

    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Foo {
        pub three: Vec3,
        pub one: f32,
        pub four: Vec4,
        pub two: Vec2,
        pub compound: Bar
    }

    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Bar {
        pub one: f32,
        pub four: Vec4,
        pub matrix: Matrix4
    }

    dynamiclayout!(FooLayout + FooAccessor {
        three: Vec3,
        one: f32,
        four: Vec4,
        two: Vec2,
        compound: BarLayout
    });

    dynamiclayout!(BarLayout + BarAccessor {
        one: f32,
        four: Vec4,
        matrix: Matrix4
    });

    fn make_foo_layout() -> FooLayout {
        let mut layout: FooLayout = Default::default();
        layout.three.offset = 0;
        layout.one.offset = 12;
        layout.four.offset = 16;
        layout.two.offset = 32;
        layout.compound = make_bar_layout();
        layout
    }

    fn make_bar_layout() -> BarLayout {
        let mut layout: BarLayout = Default::default();
        layout.one.offset = 40;
        layout.four.offset = 44;
        layout.matrix.offset = 60;
        layout.matrix.stride = 16;
        layout
    }

    fn new_foo() -> Foo {
        use ::primitive_types::*;
        Foo {
            three: Vec3 { x: 1.0, y: 2.0, z: 3.0 },
            one: 4.0,
            four: Vec4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 },
            two: Vec2 { x: 9.0, y: 10.0 },
            compound: Bar {
                one: 11.0,
                four: Vec4 { x: 12.0, y: 13.0, z: 14.0, w: 15.0 },
                matrix: Matrix4::new([
                    [101.0, 102.0, 103.0, 104.0],
                    [105.0, 106.0, 107.0, 108.0],
                    [109.0, 110.0, 111.0, 112.0],
                    [113.0, 114.0, 115.0, 116.0]
                ])
            }
        }
    }

    #[test]
    fn it_works() {
        let layout = make_foo_layout();
        let foo = new_foo();

        let mut bytes: [u8; 124] = unsafe { ::std::mem::transmute(foo) };

        let acc = layout.accessor(&mut bytes);

        assert_eq!(foo.three.x, acc.three.x);
        assert_eq!(foo.three.y, acc.three.y);
        assert_eq!(foo.three.z, acc.three.z);

        assert_eq!(foo.one, *acc.one);

        assert_eq!(foo.four.x, acc.four.x);
        assert_eq!(foo.four.y, acc.four.y);
        assert_eq!(foo.four.z, acc.four.z);
        assert_eq!(foo.four.w, acc.four.w);

        assert_eq!(foo.two.x, acc.two.x);
        assert_eq!(foo.two.y, acc.two.y);

        assert_eq!(foo.compound.one, *acc.compound.one);
        assert_eq!(foo.compound.four.x, acc.compound.four.x);
        assert_eq!(foo.compound.four.y, acc.compound.four.y);
        assert_eq!(foo.compound.four.z, acc.compound.four.z);
        assert_eq!(foo.compound.four.w, acc.compound.four.w);

        assert_eq!( foo.compound.matrix[0][0], acc.compound.matrix[0][0] );
        assert_eq!( foo.compound.matrix[0][1], acc.compound.matrix[0][1] );
        assert_eq!( foo.compound.matrix[0][2], acc.compound.matrix[0][2] );
        assert_eq!( foo.compound.matrix[0][3], acc.compound.matrix[0][3] );
        assert_eq!( foo.compound.matrix[1][0], acc.compound.matrix[1][0] );
        assert_eq!( foo.compound.matrix[1][1], acc.compound.matrix[1][1] );
        assert_eq!( foo.compound.matrix[1][2], acc.compound.matrix[1][2] );
        assert_eq!( foo.compound.matrix[1][3], acc.compound.matrix[1][3] );
        assert_eq!( foo.compound.matrix[2][0], acc.compound.matrix[2][0] );
        assert_eq!( foo.compound.matrix[2][1], acc.compound.matrix[2][1] );
        assert_eq!( foo.compound.matrix[2][2], acc.compound.matrix[2][2] );
        assert_eq!( foo.compound.matrix[2][3], acc.compound.matrix[2][3] );
        assert_eq!( foo.compound.matrix[3][0], acc.compound.matrix[3][0] );
        assert_eq!( foo.compound.matrix[3][1], acc.compound.matrix[3][1] );
        assert_eq!( foo.compound.matrix[3][2], acc.compound.matrix[3][2] );
        assert_eq!( foo.compound.matrix[3][3], acc.compound.matrix[3][3] );
    }
}
