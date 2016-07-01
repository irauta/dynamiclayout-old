
use vector_types::{Vec2, Vec3, Vec4};
use matrix_types::Matrix4;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Foo {
    pub three: Vec3,
    pub one: f32,
    pub four: Vec4,
    pub two: Vec2,
    pub compound: Bar,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Bar {
    pub one: f32,
    pub four: Vec4,
    pub matrix: Matrix4,
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
    Foo {
        three: Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        one: 4.0,
        four: Vec4 {
            x: 5.0,
            y: 6.0,
            z: 7.0,
            w: 8.0,
        },
        two: Vec2 { x: 9.0, y: 10.0 },
        compound: Bar {
            one: 11.0,
            four: Vec4 {
                x: 12.0,
                y: 13.0,
                z: 14.0,
                w: 15.0,
            },
            matrix: Matrix4::new([[101.0, 102.0, 103.0, 104.0],
                                  [105.0, 106.0, 107.0, 108.0],
                                  [109.0, 110.0, 111.0, 112.0],
                                  [113.0, 114.0, 115.0, 116.0]]),
        },
    }
}

#[test]
fn one_to_one_mapping() {
    let layout = make_foo_layout();
    let mut foo = new_foo();

    const FOO_SIZE: usize = 124;
    assert_eq!(FOO_SIZE, ::std::mem::size_of_val(&foo));
    let mut bytes: &mut [u8] = unsafe { &mut *(&mut foo as *mut Foo as *mut [u8; FOO_SIZE]) };

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

    assert_eq!(foo.compound.matrix[0][0], acc.compound.matrix[0][0]);
    assert_eq!(foo.compound.matrix[0][1], acc.compound.matrix[0][1]);
    assert_eq!(foo.compound.matrix[0][2], acc.compound.matrix[0][2]);
    assert_eq!(foo.compound.matrix[0][3], acc.compound.matrix[0][3]);
    assert_eq!(foo.compound.matrix[1][0], acc.compound.matrix[1][0]);
    assert_eq!(foo.compound.matrix[1][1], acc.compound.matrix[1][1]);
    assert_eq!(foo.compound.matrix[1][2], acc.compound.matrix[1][2]);
    assert_eq!(foo.compound.matrix[1][3], acc.compound.matrix[1][3]);
    assert_eq!(foo.compound.matrix[2][0], acc.compound.matrix[2][0]);
    assert_eq!(foo.compound.matrix[2][1], acc.compound.matrix[2][1]);
    assert_eq!(foo.compound.matrix[2][2], acc.compound.matrix[2][2]);
    assert_eq!(foo.compound.matrix[2][3], acc.compound.matrix[2][3]);
    assert_eq!(foo.compound.matrix[3][0], acc.compound.matrix[3][0]);
    assert_eq!(foo.compound.matrix[3][1], acc.compound.matrix[3][1]);
    assert_eq!(foo.compound.matrix[3][2], acc.compound.matrix[3][2]);
    assert_eq!(foo.compound.matrix[3][3], acc.compound.matrix[3][3]);

    acc.three.y = 999.0;
    assert_eq!(foo.three.y, 999.0);
    acc.two[0] = 888.0;
    assert_eq!(foo.two.x, 888.0);
    *acc.one = 777.0;
    assert_eq!(foo.one, 777.0);
}

#[test]
fn vector_indexing() {
    let vec = Vec4::new(1.0, 2.0, 3.0, 4.0);

    assert_eq!(vec[0], 1.0);
    assert_eq!(vec[1], 2.0);
    assert_eq!(vec[2], 3.0);
    assert_eq!(vec[3], 4.0);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn vector_out_of_bounds() {
    let mut vec = Vec3::new(1.0, 2.0, 3.0);
    vec[4] = 4.0;
}

dynamiclayout!(MatrixLayout + MatrixAccessor {
    matrix: Matrix4
});

const MATRIX: [[f32; 4]; 4] = [[0.0f32; 4]; 4];

fn matrix_bytes() -> [u8; 64] {
    unsafe { ::std::mem::transmute(MATRIX) }
}

fn matrix_layout() -> MatrixLayout {
    let mut layout: MatrixLayout = Default::default();
    layout.matrix.offset = 0;
    layout.matrix.stride = 16;
    layout
}

#[test]
fn dynamic_matrix_indexing() {
    let layout = matrix_layout();
    let mut bytes = matrix_bytes();
    let mut acc = layout.accessor(&mut bytes);
    assert_eq!(acc.matrix[0][0], 0.0);
    assert_eq!(acc.matrix[3][3], 0.0);
    acc.matrix[2][2] = 5.0;
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn dynamic_matrix_out_of_bounds_1() {
    let layout = matrix_layout();
    let mut bytes = matrix_bytes();
    let mut acc = layout.accessor(&mut bytes);
    // Cause panic when accessing the outer array
    acc.matrix[4][0] = 1.0;
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn dynamic_matrix_out_of_bounds_2() {
    let layout = matrix_layout();
    let mut bytes = matrix_bytes();
    let mut acc = layout.accessor(&mut bytes);
    // Cause panic when accessing the nested array
    acc.matrix[0][4] = 1.0;
}
