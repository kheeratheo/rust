## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| \|x\| expr | นิยาม Closure รับพารามิเตอร์เดียว และประเมินค่านิพจน์สั้นๆ ทันที | \|x\| x + 1 |
| \|a: i32, b: i32\| -> i32 { ... } | นิยาม Closure แบบระบุชนิดข้อมูล (Type Annotations) และมีบล็อกคำสั่งหลายบรรทัด | \|a: i32, b: i32\| -> i32 { a + b } |
| move \|...\| { ... } | บังคับย้าย Ownership ของตัวแปรภายนอกที่ถูก Capture เข้ามาใน Closure อย่างเด็ดขาด | move \|\| println!("{:?}", data) |
| Fn(&self) | Trait สำหรับ Closure ที่ยืมตัวแปรภายนอกแบบอ่านอย่างเดียว (Immutable Reference) เรียกซ้ำได้หลายครั้ง | fn call_fn<F: Fn()>(f: F) |
| FnMut(&mut self) | Trait สำหรับ Closure ที่ยืมตัวแปรภายนอกมาแก้ไขค่า (Mutable Reference) เรียกซ้ำได้ | fn call_fn_mut<F: FnMut()>(mut f: F) |
| FnOnce(self) | Trait สำหรับ Closure ที่ย้าย Ownership ของตัวแปรภายนอกเข้าสู่บริบทของตน เรียกใช้งานได้เพียงครั้งเดียว | fn call_fn_once<F: FnOnce()>(f: F) |

### Important Rules

1. **Type Inference Latching:** คอมไพเลอร์ของ Rust จะอนุมานชนิดข้อมูลของพารามิเตอร์และค่าที่ส่งกลับของ Closure จากการเรียกใช้งานครั้งแรกโดยอัตโนมัติ และจะยึด Type นั้นไว้อย่างถาวร ไม่สามารถส่งอาร์กิวเมนต์ต่างชนิดกันในการเรียกครั้งถัดไปได้
2. **Least Privilege Capture Mechanism:** ตัว Borrow Checker จะเลือกวิธีการ Capture ตัวแปรจากสภาพแวดล้อมโดยใช้วิธีที่จำกัดสิทธิ์น้อยที่สุดก่อนเสมอ (&T -> &mut T -> T by-value) เว้นแต่จะระบุคีย์เวิร์ด move เพื่อบังคับย้าย Ownership
3. **Trait Hierarchy & Dispatching:** โครงสร้างลำดับขั้นของ Closure Trait เป็นไปตามกฎ Fn: FnMut: FnOnce (Closure ที่ implement Fn จะ implement FnMut และ FnOnce ด้วยเสมอ) และเนื่องจาก Closure แต่ละตัวมีชนิดข้อมูลเฉพาะตัวที่ไม่ระบุชื่อ (Unique Anonymous Type) การคืนค่า Closure ออกจากฟังก์ชันจึงต้องระบุผ่าน Static Dispatch (impl Fn(...) -> ...) หรือ Dynamic Dispatch ผ่าน Heap (Box<dyn Fn(...) -> ...>)

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร
>
> Source code: [closures_demo/src/main.rs](file:///d:/rust-tutorial-2569-main/19-closures-functional-programming/closures_demo/src/main.rs)

### Example 1 — Closure Traits: Fn, FnMut, FnOnce

**Purpose:** สาธิต 3 รูปแบบการ capture ตัวแปรของ Closure ตาม Trait ที่ Compiler เลือกให้อัตโนมัติ

```rust
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
    // consume_data(); // <-- หากเอาคอมเมนต์ออกจะ Compile Error ทันที!
}
```

**Expected Output**

```text
--- [Demo 1] Closure Traits (Fn, FnMut, FnOnce) ---
Fn trait: Hello
ค่า greeting ยังใช้ต่อได้: Hello
FnMut trait count: 1
FnMut trait count: 2
Final count: 2
FnOnce trait: vector length = 3
```

**Explanation**

- **Fn (Immutable Borrow):** print_greeting ยืมค่า greeting แบบอ่านอย่างเดียว (&String) — เรียกซ้ำกี่ครั้งก็ได้ และตัวแปรเดิมยังใช้ต่อได้หลัง closure
- **FnMut (Mutable Borrow):** increment ยืม count แบบ &mut เพื่อแก้ไขค่า — ต้องประกาศ let mut increment จึงจะเรียกได้ และเรียกซ้ำได้หลายครั้ง
- **FnOnce (Move/Consume):** consume_data ย้าย Ownership ของ data เข้ามาแล้วทำ drop() — เรียกได้ครั้งเดียวเท่านั้น หากเรียกซ้ำจะ Compile Error

---

### Example 2 — Concurrency with move Closure

**Purpose:** สาธิตการใช้ move keyword บังคับ Closure ย้าย Ownership เพื่อส่ง data ข้าม Thread อย่างปลอดภัย

```rust
use std::thread;

fn demo_move_concurrency() {
    println!("\n--- [Demo 2] Concurrency with `move` ---");

    let thread_data = vec![10, 20, 30];

    // หากไม่ใส่ keyword move ตัว Compiler จะเตือนว่า data อาจมีอายุสั้นกว่า Thread (Lifetime issue)
    let handle = thread::spawn(move || {
        println!("Thread worker ได้รับ data: {:?}", thread_data);
    });

    handle.join().unwrap();
    // println!("{:?}", thread_data); // Compile Error: ownership ย้ายไปที่ Thread แล้ว
}
```

**Expected Output**

```text
--- [Demo 2] Concurrency with `move` ---
Thread worker ได้รับ data: [10, 20, 30]
```

**Explanation**

- move บังคับให้ thread_data ย้าย Ownership เข้าไปใน Closure ที่ส่งให้ Thread — ทำให้ Thread เป็นเจ้าของ data ได้อย่างสมบูรณ์
- หากไม่ใส่ move Compiler จะ Error เพราะ thread_data อาจถูก drop ก่อนที่ Thread จะทำงานเสร็จ (Lifetime ไม่ตรง)
- หลัง move แล้ว ตัวแปร thread_data ใน scope เดิมจะใช้ไม่ได้อีกต่อไป

---

### Example 3 — Returning Closures (impl Fn vs Box<dyn Fn>)

**Purpose:** สาธิตการคืนค่า Closure จากฟังก์ชัน ทั้งแบบ Static Dispatch (impl Fn) และ Dynamic Dispatch (Box<dyn Fn>)

```rust
// Static Dispatch (Zero-cost): Compiler รู้ type ตอน compile
fn create_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

// Dynamic Dispatch (Heap Allocated): รองรับ Closure หลาย type ในค่าคืน
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
```

**Expected Output**

```text
--- [Demo 3] Returning Closures ---
create_adder(5)(10) = 15
make_operation('multiply')(4, 5) = 20
```

**Explanation**

- **impl Fn(i32) -> i32:** Compiler รู้ type ของ Closure ตอน compile → ไม่มี overhead (Static Dispatch / Zero-cost) แต่คืนได้แค่ Closure ชนิดเดียว
- **Box<dyn Fn(i32, i32) -> i32>:** ใช้ Trait Object บน Heap → รองรับการคืน Closure ต่างชนิดกันผ่าน if/else (Dynamic Dispatch) แต่มี overhead จาก heap allocation และ vtable lookup
- ทั้งสองแบบต้องใช้ move เพื่อย้าย captured variable (เช่น x) เข้า Closure ไม่ให้เกิด dangling reference

---

### Example 4 — Functional Programming Pipeline (Zero-Cost Abstractions)

**Purpose:** สาธิต Iterator Chain แบบ Functional (filter → map → fold) ที่ Rust compile เป็น loop เดียวโดยไม่มี Heap Overhead

```rust
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
```

**Expected Output**

```text
--- [Demo 4] Functional Programming (Zero-Cost Abstractions) ---
ผลรวมของเลขคู่ยกกำลังสอง (2^2 + 4^2 + 6^2 + 8^2 + 10^2) = 220
```

**Explanation**

- **.iter():** สร้าง Iterator จาก Vector (ไม่ copy ข้อมูล แค่สร้าง pointer)
- **.filter(|&&x| x % 2 == 0):** คัดเฉพาะเลขคู่ — &&x เป็นการ destructure double reference (&&i32 → i32)
- **.map(|&x| x * x):** แปลงค่าแต่ละตัวเป็นยกกำลังสอง
- **.fold(0, |acc, x| acc + x):** สะสมผลรวม เริ่มจาก 0 — เป็น consuming adaptor ที่ทำให้ทั้ง pipeline ทำงานจริง
- Rust ใช้ **Zero-Cost Abstraction** คือ code ที่เขียนแบบ Declarative/Functional จะถูก compile เป็น loop ภาษาเครื่องตัวเดียว ประสิทธิภาพเท่ากับเขียน for-loop ด้วยมือ

---