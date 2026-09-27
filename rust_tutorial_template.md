# Rust Tutorial Project — Principles of Programming Languages

> **สำหรับนักศึกษา:** ใช้ไฟล์นี้เป็น Template สำหรับจัดทำบทเรียน Rust ของกลุ่ม  
> **Topic No.:** `XX`  
> **Topic Name:** `[ชื่อหัวข้อ]`  
> **Group No.:** `XX`

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | `[ชื่อ-นามสกุล]` | `[รหัส]` | `@[username]` | Concept + Code |
| 2 | `[ชื่อ-นามสกุล]` | `[รหัส]` | `@[username]` | Code + Demo |
| 3 | `[ชื่อ-นามสกุล]` | `[รหัส]` | `@[username]` | Rust vs Other Language + PPL |
| 4 | `[ชื่อ-นามสกุล]` | `[รหัส]` | `@[username]` | Exercises + Common Mistakes |

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. `[อธิบายแนวคิดสำคัญได้]`
2. `[เขียนโปรแกรม Rust ที่เกี่ยวข้องได้]`
3. `[วิเคราะห์พฤติกรรม/กฎของภาษาได้]`
4. `[เปรียบเทียบ Rust กับภาษาอื่นได้]`

---

## 3. Introduction

อธิบายว่า Topic นี้คืออะไร มีความสำคัญอย่างไร และใช้แก้ปัญหาอะไรในการเขียนโปรแกรม

`[เขียนเนื้อหาที่นี่]`

---

## 4. Key Concepts

### 4.1 `[Concept 1]`

**คำอธิบาย**

`[อธิบายแนวคิด]`

**ตัวอย่าง**

```rust
fn main() {
    println!("Hello, Rust!");
}
```

**Explanation**

`[อธิบายว่า code ทำงานอย่างไร]`

---

### 4.2 `[Concept 2]`

`[อธิบายแนวคิด]`

```rust
// Rust code
```

---

### 4.3 `[Concept 3]`

`[อธิบายแนวคิด]`

```rust
// Rust code
```

---

### 4.4 `[Concept 4 — ถ้ามี]`

`[อธิบายแนวคิด]`

```rust
// Rust code
```

---

### 4.5 `[Concept 5 — ถ้ามี]`

`[อธิบายแนวคิด]`

```rust
// Rust code
```

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `\|args\| expr` | ประกาศ Closure (Anonymous Function) โดยใช้ Pipe `\|...\|` ล้อม Parameters | `let add = \|a, b\| a + b;` |
| `\|\| { ... }` | Closure ที่ไม่รับ Argument (ใช้ `\|\|`) และมี Body หลายบรรทัด | `let greet = \|\| { println!("Hi"); };` |
| `move \|...\| { ... }` | บังคับให้ Closure **ย้าย Ownership** ของตัวแปรที่ capture เข้าไป แทนที่จะยืม (borrow) | `thread::spawn(move \|\| { println!("{:?}", data); });` |
| `impl Fn(T) -> R` | คืนค่า Closure จากฟังก์ชัน แบบ Static Dispatch (Zero-cost, ไม่ allocate Heap) | `fn make() -> impl Fn(i32) -> i32 { move \|x\| x + 1 }` |
| `Box<dyn Fn(T) -> R>` | คืนค่า Closure จากฟังก์ชัน แบบ Dynamic Dispatch (Heap Allocated, รองรับหลาย type) | `fn make() -> Box<dyn Fn(i32) -> i32> { Box::new(\|x\| x * 2) }` |
| `.iter().filter().map().fold()` | Iterator Pipeline แบบ Functional — Rust compile เป็น loop เดียว (Zero-Cost Abstraction) | `vec.iter().filter(\|&&x\| x > 0).map(\|&x\| x * x).fold(0, \|a, x\| a + x);` |

### Important Rules

1. **Closure จะ capture ตัวแปรจาก Environment โดยอัตโนมัติ** — Compiler จะเลือกวิธีที่เข้มงวดน้อยที่สุดตามลำดับ: `&T` (Immutable Borrow → `Fn`) → `&mut T` (Mutable Borrow → `FnMut`) → `T` (Move/Ownership → `FnOnce`)
2. **`FnOnce` closure เรียกใช้ได้เพียงครั้งเดียว** — เนื่องจาก Ownership ของตัวแปรที่ capture ถูก move เข้าไปแล้ว หาก Closure ทำ `drop()` หรือคืน Ownership ออกมา จะเรียกซ้ำอีกไม่ได้
3. **ต้องใช้ `move` keyword เมื่อส่ง Closure ข้าม Thread** — เพราะ Thread ใหม่อาจมีอายุยาวกว่า scope ของตัวแปรเดิม Compiler จะบังคับให้ย้าย Ownership เพื่อป้องกัน dangling reference
4. **`FnMut` closure ต้องผูกกับ `let mut`** — ตัวแปรที่เก็บ closure ที่แก้ไขค่า captured variable ต้องประกาศเป็น mutable ด้วย (`let mut closure = || { ... };`)
5. **Iterator adaptor เป็น Lazy** — `.filter()`, `.map()` จะไม่ทำงานจนกว่าจะเรียก consuming adaptor เช่น `.fold()`, `.collect()`, `.for_each()` เพื่อ "ดึง" ค่าออกมา

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร
>
> Source code: [`closures_demo/src/main.rs`](file:///d:/rust-tutorial-2569-main/19-closures-functional-programming/closures_demo/src/main.rs)

### Example 1 — Closure Traits: `Fn`, `FnMut`, `FnOnce`

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

- **`Fn` (Immutable Borrow):** `print_greeting` ยืมค่า `greeting` แบบอ่านอย่างเดียว (`&String`) — เรียกซ้ำกี่ครั้งก็ได้ และตัวแปรเดิมยังใช้ต่อได้หลัง closure
- **`FnMut` (Mutable Borrow):** `increment` ยืม `count` แบบ `&mut` เพื่อแก้ไขค่า — ต้องประกาศ `let mut increment` จึงจะเรียกได้ และเรียกซ้ำได้หลายครั้ง
- **`FnOnce` (Move/Consume):** `consume_data` ย้าย Ownership ของ `data` เข้ามาแล้วทำ `drop()` — เรียกได้ครั้งเดียวเท่านั้น หากเรียกซ้ำจะ Compile Error

---

### Example 2 — Concurrency with `move` Closure

**Purpose:** สาธิตการใช้ `move` keyword บังคับ Closure ย้าย Ownership เพื่อส่ง data ข้าม Thread อย่างปลอดภัย

```rust
use std::thread;

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
```

**Expected Output**

```text
--- [Demo 2] Concurrency with `move` ---
Thread worker ได้รับ data: [10, 20, 30]
```

**Explanation**

- `move` บังคับให้ `thread_data` ย้าย Ownership เข้าไปใน Closure ที่ส่งให้ Thread — ทำให้ Thread เป็นเจ้าของ data ได้อย่างสมบูรณ์
- หากไม่ใส่ `move` Compiler จะ Error เพราะ `thread_data` อาจถูก drop ก่อนที่ Thread จะทำงานเสร็จ (Lifetime ไม่ตรง)
- หลัง `move` แล้ว ตัวแปร `thread_data` ใน scope เดิมจะใช้ไม่ได้อีกต่อไป

---

### Example 3 — Returning Closures (`impl Fn` vs `Box<dyn Fn>`)

**Purpose:** สาธิตการคืนค่า Closure จากฟังก์ชัน ทั้งแบบ Static Dispatch (`impl Fn`) และ Dynamic Dispatch (`Box<dyn Fn>`)

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

- **`impl Fn(i32) -> i32`:** Compiler รู้ type ของ Closure ตอน compile → ไม่มี overhead (Static Dispatch / Zero-cost) แต่คืนได้แค่ Closure ชนิดเดียว
- **`Box<dyn Fn(i32, i32) -> i32>`:** ใช้ Trait Object บน Heap → รองรับการคืน Closure ต่างชนิดกันผ่าน `if/else` (Dynamic Dispatch) แต่มี overhead จาก heap allocation และ vtable lookup
- ทั้งสองแบบต้องใช้ `move` เพื่อย้าย captured variable (เช่น `x`) เข้า Closure ไม่ให้เกิด dangling reference

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

- **`.iter()`** สร้าง Iterator จาก Vector (ไม่ copy ข้อมูล แค่สร้าง pointer)
- **`.filter(|&&x| x % 2 == 0)`** คัดเฉพาะเลขคู่ — `&&x` เป็นการ destructure double reference (`&&i32 → i32`)
- **`.map(|&x| x * x)`** แปลงค่าแต่ละตัวเป็นยกกำลังสอง
- **`.fold(0, |acc, x| acc + x)`** สะสมผลรวม เริ่มจาก 0 — เป็น consuming adaptor ที่ทำให้ทั้ง pipeline ทำงานจริง
- Rust ใช้ **Zero-Cost Abstraction** คือ code ที่เขียนแบบ Declarative/Functional จะถูก compile เป็น loop ภาษาเครื่องตัวเดียว ประสิทธิภาพเท่ากับเขียน for-loop ด้วยมือ

---

## 7. Common Mistakes

### Mistake 1 — `[ชื่อข้อผิดพลาด]`

**Problem**

`[อธิบายปัญหา]`

**Incorrect Code**

```rust
// Incorrect example
```

**Correct Code**

```rust
// Correct example
```

**Why?**

`[อธิบายสาเหตุ]`

---

### Mistake 2 — `[ชื่อข้อผิดพลาด]`

**Problem**

`[อธิบายปัญหา]`

**Incorrect Code**

```rust
// Incorrect example
```

**Correct Code**

```rust
// Correct example
```

**Why?**

`[อธิบายสาเหตุ]`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `[ชื่อโจทย์]`

**Problem**

`[เขียนโจทย์]`

**Hint**

`[คำใบ้]`

**Solution**

```rust
// Solution code
```

**Explanation**

`[อธิบายแนวทางแก้]`

---

### Exercise 2 — `[ชื่อโจทย์]`

**Problem**

`[เขียนโจทย์]`

**Hint**

`[คำใบ้]`

**Solution**

```rust
// Solution code
```

**Explanation**

`[อธิบายแนวทางแก้]`

---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

`[Topic นี้เกี่ยวข้องกับ syntax อย่างไร]`

### 9.2 Semantics

`[คำสั่ง/construct เหล่านี้มีความหมายหรือพฤติกรรมอย่างไร]`

### 9.3 Type System

`[เกี่ยวข้องกับ type system อย่างไร ถ้ามี]`

### 9.4 Memory / Resource Management

`[เกี่ยวข้องกับ memory หรือ resource management อย่างไร ถ้ามี]`

### 9.5 Abstraction / Other PPL Concepts

`[อธิบาย abstraction, scope, binding, paradigm หรือแนวคิด PPL อื่นที่เกี่ยวข้อง]`

### 9.6 Why Rust?

`[Rust ใช้แนวคิดนี้เพื่อเพิ่ม safety, reliability หรือ performance อย่างไร]`

---

## 10. Rust vs. Other Language

**Comparison Language:** `[Python / C / C++ / Java / Kotlin / ...]`

| Aspect | Rust | Other Language |
|---|---|---|
| Syntax | `[อธิบาย]` | `[อธิบาย]` |
| Semantics / Behavior | `[อธิบาย]` | `[อธิบาย]` |
| Type System | `[อธิบาย]` | `[อธิบาย]` |
| Memory Management | `[อธิบาย]` | `[อธิบาย]` |
| Safety | `[อธิบาย]` | `[อธิบาย]` |

### Rust Example

```rust
// Rust code
```

### `[Other Language]` Example

```python
# Other language code
```

### Analysis

`[อธิบายความแตกต่างที่สำคัญ และเหตุผลด้านการออกแบบภาษา]`

---

## 11. Teach Your Topic

การนำเสนอมีสมาชิก **4 คน คนละประมาณ 5 นาที**

| Member | Responsibility | Time |
|---|---|---:|
| Member 1 | Concept + Short Code Illustration | 5 min |
| Member 2 | Detailed Code + Live Demo | 5 min |
| Member 3 | Rust vs Other Language + PPL Analysis | 5 min |
| Member 4 | Exercises + Common Mistakes + Challenge | 5 min |

### Individual Contribution

**Member 1**

`[สิ่งที่รับผิดชอบ]`

**Member 2**

`[สิ่งที่รับผิดชอบ]`

**Member 3**

`[สิ่งที่รับผิดชอบ]`

**Member 4**

`[สิ่งที่รับผิดชอบ]`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `[The Rust Programming Language — Rust Book]`
2. `[Rust by Example / Rust Reference]`
3. `[Official documentation ที่เกี่ยวข้องกับ Topic]`
4. `[แหล่งอ้างอิงเพิ่มเติม]`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `[เช่น ChatGPT]` | `[ใช้เพื่ออะไร]` | `[ตรวจสอบอย่างไร]` |
| `[AI tool]` | `[ใช้เพื่ออะไร]` | `[ตรวจสอบอย่างไร]` |

### Declaration

- [ ] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [ ] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [ ] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [ ] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`[อธิบายว่าใช้ AI ในขั้นตอนใด และสมาชิกตรวจสอบผลลัพธ์อย่างไร]`

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 2 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 3 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 4 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |

### Teamwork Reflection

**How did your team collaborate?**

`[อธิบายกระบวนการทำงานร่วมกัน]`

**Problems encountered**

`[ปัญหาที่พบ]`

**How did you solve them?**

`[วิธีแก้ปัญหา]`

---

## 15. Final Checklist

- [ ] Learning Objectives ครบ 3–4 ข้อ
- [ ] Key Concepts ครบถ้วน
- [ ] Syntax / Rules
- [ ] Runnable Code Examples
- [ ] Code Compile และ Run ได้จริง
- [ ] Common Mistakes
- [ ] Exercises 2 ข้อ พร้อม Solutions
- [ ] PPL Perspective
- [ ] Rust vs Other Language
- [ ] References อย่างน้อย 4 แหล่ง
- [ ] AI Usage Declaration
- [ ] GitHub Contribution
- [ ] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [ ] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที
- [ ] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** `[GitHub repository URL]`

**Chapter Path:** `[เช่น chapters/01-introduction/]`

**Final PR:** `#[PR number]`

**Submitted by:** `[Group XX]`

**Date:** `[YYYY-MM-DD]`
