use std::thread;

// ==========================================
// 1. CLOSURE TRAITS: Fn, FnMut, FnOnce
// ==========================================
fn demo_closure_traits() {
    println!("\n--- [Demo 1] Closure Traits (Fn, FnMut, FnOnce) ---");

    // 1.1 Fn: ยืมอ่านอย่างเดียว (Immutable Borrow)
    let greeting = String::from("Hello");
    let print_greeting = || println!("Fn trait: {}", greeting);
    print_greeting();
    println!("ค่า greeting ยังใช้ต่อได้: {}", greeting); // compile ผ่านเพราะแค่ยืมอ่าน

    // 1.2 FnMut: ยืมแบบแก้ไขค่าได้ (Mutable Borrow)
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("FnMut trait count: {}", count);
    };
    increment();
    increment();
    println!("Final count: {}", count);

    // 1.3 FnOnce: ย้าย Ownership (Move/Consume) รันได้ครั้งเดียว
    let data = vec![1, 2, 3];
    let consume_data = || {
        println!("FnOnce trait: vector length = {}", data.len());
        drop(data); // data ถูกทำลายทิ้งตรงนี้
    };
    consume_data();
    // consume_data(); // <-- หากเอาคอมเมนต์ออกจะ Compile Error ทันที! (ใช้สาธิตตอน Live Demo ได้)
}

// ==========================================
// 2. CONCURRENCY & MOVE CLOSURE
// ==========================================
fn demo_move_concurrency() {
    println!("\n--- [Demo 2] Concurrency with `move` ---");

    let thread_data = vec![10, 20, 30];

    // หากไม่ใส่ keyword `move` ตัว Compiler จะเตือนว่า data อาจมีอายุสั้นกว่า Thread (Lifetime issue)
    let handle = thread::spawn(move || {
        println!("Thread worker ได้รับ data: {:?}", thread_data);
    });

    handle.join().unwrap();
    // println!("{:?}", thread_data); // Compile Error: ownership ย้ายไปที่ Thread แล้ว
}

// ==========================================
// 3. RETURNING CLOSURES (impl Fn vs Box<dyn Fn>)
// ==========================================
// 3.1 คืนค่า Closure เดี่ยวๆ แบบ Static Dispatch (Zero-cost)
fn create_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

// 3.2 คืนค่า Closure ที่หลากหลายผ่าน Dynamic Dispatch (Heap Allocated)
fn make_operation(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
    if op == "add" {
        Box::new(|a, b| a + b)
    } else {
        Box::new(|a, b| a * b)
    }
}

fn demo_returning_closures() {
    println!("\n--- [Demo 3] Returning Closures ---");

    let add_five = create_adder(5);
    println!("create_adder(5)(10) = {}", add_five(10));

    let calc = make_operation("multiply");
    println!("make_operation('multiply')(4, 5) = {}", calc(4, 5));
}

// ==========================================
// 4. FUNCTIONAL PROGRAMMING & ITERATORS
// ==========================================
fn demo_functional_pipeline() {
    println!("\n--- [Demo 4] Functional Programming (Zero-Cost Abstractions) ---");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Pipeline: Filter -> Map -> Fold (Declarative Style)
    // Rust จะ compile เป็น Loop ภาษาเครื่องตัวเดียว ไม่มี Heap Overhead
    let sum_of_even_squares: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0) // คัดเฉพาะเลขคู่
        .map(|&x| x * x)          // ยกกำลังสอง
        .fold(0, |acc, x| acc + x); // รวมผลลัพธ์

    println!("ผลรวมของเลขคู่ยกกำลังสอง (2^2 + 4^2 + 6^2 + 8^2 + 10^2) = {}", sum_of_even_squares);
}

fn main() {
    println!("=============================================");
    println!(" Rust Closures & Functional Programming Demo ");
    println!("=============================================");

    demo_closure_traits();
    demo_move_concurrency();
    demo_returning_closures();
    demo_functional_pipeline();
}