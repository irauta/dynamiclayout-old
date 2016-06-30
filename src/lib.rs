
use std::default::Default;
use std::marker::PhantomData;

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
        type Layout = ::DynamicField<T>;
    }
    impl<'a, T: 'a + PrimitiveType> ::AccessDynamicField<'a> for T {
        type Accessor = &'a T;

        unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8) -> Self::Accessor {
            &*(layout.offset_ptr(bytes) as *mut T)
        }
    }
}

pub mod complex_types {
    use ::{ArrayField,LayoutDynamicField,AccessDynamicField};

    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Matrix2 (pub (f32, f32), pub (f32, f32));
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Matrix2x3 (pub (f32, f32, f32), pub (f32, f32, f32));
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Matrix2x4 (pub (f32, f32, f32, f32), pub (f32, f32, f32, f32));

    impl LayoutDynamicField for Matrix2 { type Layout = Layout2<(f32, f32)>; }
    impl LayoutDynamicField for Matrix2x3 { type Layout = Layout2<(f32, f32, f32)>; }
    impl LayoutDynamicField for Matrix2x4 { type Layout = Layout2<(f32, f32, f32, f32)>; }

    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Matrix3 (pub (f32, f32, f32), pub (f32, f32, f32), pub (f32, f32, f32));
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Matrix3x2 (pub (f32, f32), pub (f32, f32), pub (f32, f32));
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Matrix3x4 (pub (f32, f32, f32, f32), pub (f32, f32, f32, f32), pub (f32, f32, f32, f32));

    impl LayoutDynamicField for Matrix3 { type Layout = Layout3<(f32, f32, f32)>; }
    impl LayoutDynamicField for Matrix3x2 { type Layout = Layout3<(f32, f32)>; }
    impl LayoutDynamicField for Matrix3x4 { type Layout = Layout3<(f32, f32, f32, f32)>; }

    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Matrix4 (pub (f32, f32, f32, f32), pub (f32, f32, f32, f32), pub (f32, f32, f32, f32), pub (f32, f32, f32, f32));
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Matrix4x2 (pub (f32, f32), pub (f32, f32), pub (f32, f32), pub (f32, f32));
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Matrix4x3 (pub (f32, f32, f32), pub (f32, f32, f32), pub (f32, f32, f32), pub (f32, f32, f32));

    impl LayoutDynamicField for Matrix4 { type Layout = Layout4<(f32, f32, f32, f32)>; }
    impl LayoutDynamicField for Matrix4x2 { type Layout = Layout4<(f32, f32)>; }
    impl LayoutDynamicField for Matrix4x3 { type Layout = Layout4<(f32, f32, f32)>; }


    #[derive(Default)]
    pub struct Layout2<T> (pub ArrayField<T>, pub ArrayField<T>);
    #[derive(Default)]
    pub struct Layout3<T> (pub ArrayField<T>, pub ArrayField<T>, pub ArrayField<T>);
    #[derive(Default)]
    pub struct Layout4<T> (pub ArrayField<T>, pub ArrayField<T>, pub ArrayField<T>, pub ArrayField<T>);

    pub struct Accessor2<'a, T: 'a> (pub &'a T, pub &'a T);
    pub struct Accessor3<'a, T: 'a> (pub &'a T, pub &'a T, pub &'a T);
    pub struct Accessor4<'a, T: 'a> (pub &'a T, pub &'a T, pub &'a T, pub &'a T);

    // https://www.reddit.com/r/rust/comments/339yj3/tuple_indexing_in_a_macro/
    macro_rules! expr { ($x:expr) => ($x) }
    macro_rules! matrix_access_dynamic_field {
        ($matrix_type:ty : $accessor_type:ident : $field_type:ty : $($field:tt),+) => (
            impl<'a> AccessDynamicField<'a> for $matrix_type {
                type Accessor = $accessor_type<'a, $field_type>;

                unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8) -> Self::Accessor {
                    $accessor_type (
                        $( &mut *(expr!(layout.$field).offset_ptr(bytes, expr!($field)) as *mut $field_type) ),+
                    )
                }
            }
        );

        (tuple_workaround $x:expr) => ($x);
    }

    matrix_access_dynamic_field!(Matrix2 : Accessor2 : (f32, f32) : 0, 1);
    matrix_access_dynamic_field!(Matrix2x3 : Accessor2 : (f32, f32, f32) : 0, 1);
    matrix_access_dynamic_field!(Matrix2x4 : Accessor2 : (f32, f32, f32, f32) : 0, 1);

    matrix_access_dynamic_field!(Matrix3x2 : Accessor3 : (f32, f32) : 0, 1, 2);
    matrix_access_dynamic_field!(Matrix3 : Accessor3 : (f32, f32, f32) : 0, 1, 2);
    matrix_access_dynamic_field!(Matrix3x4 : Accessor3 : (f32, f32, f32, f32) : 0, 1, 2);

    matrix_access_dynamic_field!(Matrix4x2 : Accessor4 : (f32, f32) : 0, 1, 2, 3);
    matrix_access_dynamic_field!(Matrix4x3 : Accessor4 : (f32, f32, f32) : 0, 1, 2, 3);
    matrix_access_dynamic_field!(Matrix4 : Accessor4 : (f32, f32, f32, f32) : 0, 1, 2, 3);
}

pub struct DynamicField<T> {
    offset: u16,
    phantom: PhantomData<T>
}

impl<T> DynamicField<T> {
    pub unsafe fn offset_ptr(&self, ptr: *mut u8) -> *mut u8 {
        ptr.offset(self.offset as isize)
    }
}

impl<T> Default for DynamicField<T> {
    fn default() -> Self {
        DynamicField {
            offset: 0,
            phantom: PhantomData
        }
    }
}

pub struct ArrayField<T> {
    offset: u16,
    stride: u16,
    phantom: PhantomData<T>
}

impl<T> ArrayField<T> {
    pub unsafe fn offset_ptr(&self, ptr: *mut u8, index: u16) -> *mut u8 {
        let total_offset: isize = self.offset as isize + self.stride as isize * index as isize;
        ptr.offset(total_offset)
    }
}

impl<T> Default for ArrayField<T> {
    fn default() -> Self {
        ArrayField {
            offset: 0,
            stride: 0,
            phantom: PhantomData
        }
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
        layout.matrix.0.offset = 60;
        layout.matrix.0.stride = 16;
        layout.matrix.1.offset = 60;
        layout.matrix.1.stride = 16;
        layout.matrix.2.offset = 60;
        layout.matrix.2.stride = 16;
        layout.matrix.3.offset = 60;
        layout.matrix.3.stride = 16;
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
                matrix: Matrix4 (
                    (101.0, 102.0, 103.0, 104.0),
                    (105.0, 106.0, 107.0, 108.0),
                    (109.0, 110.0, 111.0, 112.0),
                    (113.0, 114.0, 115.0, 116.0)
                )
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

        assert_eq!( (foo.compound.matrix.0).0, (acc.compound.matrix.0).0 );
        assert_eq!( (foo.compound.matrix.0).1, (acc.compound.matrix.0).1 );
        assert_eq!( (foo.compound.matrix.0).2, (acc.compound.matrix.0).2 );
        assert_eq!( (foo.compound.matrix.0).3, (acc.compound.matrix.0).3 );
        assert_eq!( (foo.compound.matrix.1).0, (acc.compound.matrix.1).0 );
        assert_eq!( (foo.compound.matrix.1).1, (acc.compound.matrix.1).1 );
        assert_eq!( (foo.compound.matrix.1).2, (acc.compound.matrix.1).2 );
        assert_eq!( (foo.compound.matrix.1).3, (acc.compound.matrix.1).3 );
        assert_eq!( (foo.compound.matrix.2).0, (acc.compound.matrix.2).0 );
        assert_eq!( (foo.compound.matrix.2).1, (acc.compound.matrix.2).1 );
        assert_eq!( (foo.compound.matrix.2).2, (acc.compound.matrix.2).2 );
        assert_eq!( (foo.compound.matrix.2).3, (acc.compound.matrix.2).3 );
        assert_eq!( (foo.compound.matrix.3).0, (acc.compound.matrix.3).0 );
        assert_eq!( (foo.compound.matrix.3).1, (acc.compound.matrix.3).1 );
        assert_eq!( (foo.compound.matrix.3).2, (acc.compound.matrix.3).2 );
        assert_eq!( (foo.compound.matrix.3).3, (acc.compound.matrix.3).3 );
    }
}
