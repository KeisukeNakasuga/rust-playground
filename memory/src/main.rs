static STATIC_VAR_A: u32 = 0xDEADBEEF;
static STATIC_VAR_B: u32 = 0xDEADBEEF;
static STATIC_VAR_C: u32 = 0xDEADBEEF;

fn main() {
    // 16進数
    let hex1: u32 = 0xDEADBEEF;
    println!("n1: {:p}", &hex1);
    println!("n1 size: {:?} bytes", std::mem::size_of_val(&hex1));

    // static変数
    println!("STATIC_VAR: {:p}", &STATIC_VAR_A);
    println!("STATIC_VAR size: {:?} bytes", std::mem::size_of_val(&STATIC_VAR_A));
    println!("STATIC_VAR: {:p}", &STATIC_VAR_B);
    println!("STATIC_VAR size: {:?} bytes", std::mem::size_of_val(&STATIC_VAR_B));
    println!("STATIC_VAR: {:p}", &STATIC_VAR_C);
    println!("STATIC_VAR size: {:?} bytes", std::mem::size_of_val(&STATIC_VAR_C));

    // bool
    // → stack
    let bool_a: bool = true;
    let bool_b: bool = false;
    println!("bool_a: {:p}", &bool_a);
    println!("bool_a size: {:?} bytes", std::mem::size_of_val(&bool_a));
    println!("bool_b: {:p}", &bool_b);
    println!("bool_b size: {:?} bytes", std::mem::size_of_val(&bool_b));

    // int
    // → stack
    let int_a: i8 = 100;
    println!("int_a: {:p}", &int_a);
    println!("int_a size: {:?} bytes", std::mem::size_of_val(&int_a));

    // array
    // → stack
    let array_a: [i8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("array_a: {:p}", &array_a);
    println!("array_a size: {:?} bytes", std::mem::size_of_val(&array_a));

    let array_b: [i16; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("array_b: {:p}", &array_b);
    println!("array_b size: {:?} bytes", std::mem::size_of_val(&array_b));

    let array_c: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("array_c: {:p}", &array_c);
    println!("array_c size: {:?} bytes", std::mem::size_of_val(&array_c));

    // string
    let str_a: &str = "ABCDE";
    println!("str_a: {:p}", &str_a);
    println!("str_a size: {:?} bytes", std::mem::size_of_val(&str_a));
    // println!("str_a data_ptr: {:p}", str_a.data_ptr);
    let str_b: String = str_a.to_string();
    println!("str_b: {:p}", &str_b);
    println!("str_b size: {:?} bytes", std::mem::size_of_val(&str_b));

    println!("STOP");
}
