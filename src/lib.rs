
use std::default::Default;
use std::marker::PhantomData;

pub mod primitive_types {
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Vec2 (pub f32, pub f32);

    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Vec3 (pub f32, pub f32, pub f32);

    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone)]
    pub struct Vec4 (pub f32, pub f32, pub f32, pub f32);

    pub trait PrimitiveType: Copy {}
    impl PrimitiveType for f32 {}
    impl PrimitiveType for i32 {}
    impl PrimitiveType for u32 {}
    impl PrimitiveType for Vec2 {}
    impl PrimitiveType for Vec3 {}
    impl PrimitiveType for Vec4 {}

    impl<T: PrimitiveType> ::LayoutDynamicField for T {
        type Layout = ::DynamicField<T>;
    }
    impl<'a, T: 'a + PrimitiveType> ::AccessDynamicField<'a> for T {
        type Accessor = &'a T;

        unsafe fn accessor_from_layout(layout: &'a Self::Layout, bytes: *mut u8) -> Self::Accessor {
            //unimplemented!();
            &*(bytes.offset(layout.offset as isize) as *mut T)
        }
    }
}

pub mod complex_types {
    use ::{ArrayField,LayoutDynamicField,AccessDynamicField};
    use ::primitive_types::{Vec2,Vec3,Vec4};

    pub struct Matrix2x<T> (pub T, pub T);
    pub struct Matrix3x<T> (pub T, pub T, pub T);
    pub struct Matrix4x<T> (pub T, pub T, pub T, pub T);

    pub type Matrix2 = Matrix2x<Vec2>;
    pub type Matrix2x3 = Matrix2x<Vec3>;
    pub type Matrix2x4 = Matrix2x<Vec4>;
    pub type Matrix3 = Matrix3x<Vec3>;
    pub type Matrix3x2 = Matrix3x<Vec2>;
    pub type Matrix3x4 = Matrix3x<Vec4>;
    pub type Matrix4 = Matrix4x<Vec4>;
    pub type Matrix4x2 = Matrix4x<Vec2>;
    pub type Matrix4x3 = Matrix4x<Vec3>;

    pub struct Layout2<T> (pub ArrayField<T>, pub ArrayField<T>);
    pub struct Layout3<T> (pub ArrayField<T>, pub ArrayField<T>, pub ArrayField<T>);
    pub struct Layout4<T> (pub ArrayField<T>, pub ArrayField<T>, pub ArrayField<T>, pub ArrayField<T>);

    pub struct Accessor2<'a, T: 'a> (pub &'a T, pub &'a T);
    pub struct Accessor3<'a, T: 'a> (pub &'a T, pub &'a T, pub &'a T);
    pub struct Accessor4<'a, T: 'a> (pub &'a T, pub &'a T, pub &'a T, pub &'a T);


}

pub struct DynamicField<T> {
    offset: u16,
    phantom: PhantomData<T>
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
        four: Vec4
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
        layout
    }

    fn new_foo() -> Foo {
        use ::primitive_types::*;
        Foo {
            three: Vec3 (1.0, 2.0, 3.0),
            one: 4.0,
            four: Vec4 (5.0, 6.0, 7.0, 8.0),
            two: Vec2 (9.0, 10.0),
            compound: Bar { one: 11.0, four: Vec4 (12.0, 13.0, 14.0, 15.0) }
        }
    }

    #[test]
    fn it_works() {
        let layout = make_foo_layout();
        let foo = new_foo();

        let mut bytes: [u8; 60] = unsafe { ::std::mem::transmute(foo) };

        let acc = layout.accessor(&mut bytes);

        assert_eq!(foo.three.0, acc.three.0);
        assert_eq!(foo.three.1, acc.three.1);
        assert_eq!(foo.three.2, acc.three.2);

        assert_eq!(foo.one, *acc.one);

        assert_eq!(foo.four.0, acc.four.0);
        assert_eq!(foo.four.1, acc.four.1);
        assert_eq!(foo.four.2, acc.four.2);
        assert_eq!(foo.four.3, acc.four.3);

        assert_eq!(foo.two.0, acc.two.0);
        assert_eq!(foo.two.1, acc.two.1);

        assert_eq!(foo.compound.one, *acc.compound.one);
        assert_eq!(foo.compound.four.0, acc.compound.four.0);
        assert_eq!(foo.compound.four.1, acc.compound.four.1);
        assert_eq!(foo.compound.four.2, acc.compound.four.2);
        assert_eq!(foo.compound.four.3, acc.compound.four.3);
    }
}
