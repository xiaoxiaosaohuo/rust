#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);
        //  Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples.0);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    let coords_arr: [f32;2] = [6.3, 15.0];

    print_array(coords_arr);
}

fn print_array(a: [f32; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}
