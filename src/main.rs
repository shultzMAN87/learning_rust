enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_nom(nom: u8) -> &'static str {
    match nom {
        1 => "one",
        2 => "two",
        _ => "other",
    }
}

fn main() {
    let coin = Coin::Penny;
    println!("Монета: {}", value_in_cents(coin));

    let nom = 5;
    println!("Номинал: {}", value_in_nom(nom));

    let maybe_value = Some(42);
    if let Some(x) = maybe_value {
        println!("Есть значение: {}", x);
    }
}   

/*//#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    
    println!("Площадь1 равна {}", rect1.area());

    let origin = Rectangle::square(20);
    println!("Origin: ({}, {})", origin.width, origin.height);

    println!("Может ли rect1 содержать в себе rect2? {}", rect1.can_hold(&rect2));
    println!("Может ли rect1 содержать в себе rect3? {}", rect1.can_hold(&rect3));
}*/

/*// Код с ошибкой
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 равен {}", rect1);
}*/

/*struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
    };

    println!("Почта: {}", user1.email);

    //user1.email = String::from("anotheremail@example.com");
}*/

/*fn main() {
    let numbers = vec![1, 2, 3, 4, 5]; // Вектор чисел

    // Создаем неизменяемый срез на часть вектора
    let slice1: &[i32] = &numbers[1..4]; // [2, 3, 4]
    println!("Срез 1: {:?}", slice1);

    // Создаем изменяемый срез на часть вектора
    let mut mutable_numbers = vec![10, 20, 30, 40, 50];
    let slice2: &mut [i32] = &mut mutable_numbers[0..2]; // [10, 20]
    println!("Срез 2 (до изменения): {:?}", slice2);

    // Изменяем элемент через изменяемый срез
    slice2[0] = 99;
    println!("Срез 2 (после изменения): {:?}", slice2);
    println!("Исходный вектор (после изменения): {:?}", mutable_numbers);

    // Срез строки - особый случай среза
    let s = String::from("Hello, world!");
    let hello = &s[0..5]; // "Hello"
    let world = &s[7..12]; // "world"
    println!("Часть строки 'Hello': {}", hello);
    println!("Часть строки 'world': {}", world);
}*/

/*fn main() {
    let s1 = String::from("hellsso world");

    let len = first_word(&s1);

    println!("Граница первого слова: {}", len);

    let first_word = &s1[0..len];

    println!("Первое слово: {}", first_word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}*/

/*fn main() {
    /*let s1 = String::from("hello");

    println!("{}, world!", s1);

    let s2: &'static str = "hello";

    println!("{}, world!", s2);*/

    let s = 1;

    println!("{}, world!", s);

    //{
        let s = 5;

        println!("{}, world!", s);
    //}

    println!("{}, world!", s);
}*/

/*fn dangle() -> String {
    let s = String::from("hello");
    // &s -> ошибка
    s
}*/

/*fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    
    let s1 = &s;

    println!("{}, world!", s1);
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}*/

/*fn main() {
    let s1 = String::from("hello");
    //let s2 = s1;
    
    // println!("{}, world!", s1); // ошибка

    let s3 = s1.clone();

    println!("{}, world!", s1);

    let s4 = &s1;
    println!("{}, world!", s4);
    println!("{}, world!", s1);

    let x = 5;
    let y = x;

    println!("Результат равен {}", x);
}*/

/*fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Результат равен {}", result);
}*/

/*fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("Значение числа равно {}", number);
}*/

/*fn five() -> i32 {
    5
}
fn main() {
    let x = five();
    println!("Значение x равно {}", x);
}*/

/*fn main() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("Значение y равно {}", y);
}*/

/*fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("Значение x равно {}", x);
}*/