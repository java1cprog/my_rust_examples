fn main() {
    println!("u8 min = {} ", <u8>::min_value());
    println!("u8 min = {} ", <u8>::max_value());
    println!("u16 min = {} ", <u16>::min_value());
    println!("u16 min = {} ", <u16>::max_value());
    println!("u32 min = {} ", <u32>::min_value());
    println!("u32 min = {} ", <u32>::max_value());
    println!("u64 min = {} ", <u64>::min_value());
    println!("u64 min = {} ", <u64>::max_value());
    println!("usize min = {} ", <usize>::min_value());
    println!("usize min = {} ", <usize>::max_value());
    println!("-------------------------------------");

    println!("i8 min = {} ", <i8>::min_value());
    println!("i8 min = {} ", <i8>::max_value());
    println!("i16 min = {} ", <i16>::min_value());
    println!("i16 min = {} ", <i16>::max_value());
    println!("i32 min = {} ", <i32>::min_value());
    println!("i32 min = {} ", <i32>::max_value());
    println!("i64 min = {} ", <i64>::min_value());
    println!("i64 min = {} ", <i64>::max_value());
    println!("isize min = {} ", <isize>::min_value());
    println!("isize min = {} ", <isize>::max_value());
    println!("-------------------------------------");

    println!("f32 min = {} ", std::f32::MIN);
    println!("f32 min = {} ", std::f32::MAX);
    println!("f64 min = {} ", std::f64::MIN);
    println!("f64 min = {} ", std::f64::MAX);
    println!(
        "floating point value (default) = {} ",
        -3.40282350000000000000000000000000000001f32
    );
    println!(
        "f32 value = {} ",
        -3.40282350000000000000000000000000000001f32
    );
    println!(
        "f64 value = {} ",
        -3.40282350000000000000000000000000000001f64
    );

    println!("-------------------------------------");

    let t = true;

    println!("t = {}", t);
    println!("!t = {}", !t);
    println!("t && t = {}", t && t);
    println!("t || t = {}", t || t);

    let f: bool = false; // with explicit type annotation

    println!("f = {}", f);
    println!("!f = {}", !f);
    println!("f && f = {}", f && f);
    println!("f || f = {}", f || f);

    println!("f && t = {}", f && t);
    println!("f || t = {}", f || t);

    println!("-------------------------------------");

    let c: char = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c = {}", c);
    println!("z = {}", z);
    println!("heart_eyed_cat = {}", heart_eyed_cat);
    println!("-------------------------------------");
}
