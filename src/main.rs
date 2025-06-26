// HashMap
fn main() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Синяя"), 10);
    scores.insert(String::from("Желтая"), 50);

    let field_name = String::from("Любимый цвет");
    let field_value = String::from("Синий");
    let mut map = HashMap::new();
    map.insert(&field_name, field_value);

    println!("Третий элемент равен {}", field_name);

    let mut scores = HashMap::new();
    scores.insert(String::from("Синяя"), 10);
    scores.entry(String::from("Желтая")).or_insert(50);
    scores.entry(String::from("Синяя")).or_insert(50);
    println!("{:?}", scores);
}

// тип String
/*fn main() {
    let mut hello = String::from("Olá");

    println!("Третий элемент равен {}", hello);

    hello.push_str("bar");

    println!("Третий элемент равен {}", hello);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("{}", s);

    let s11 = String::from("tic");
    let s = format!("{}-{}-{}", s11, s2, s3);

    println!("{}", s);

    let len1 = String::from("Hola").len();
    println!("{}", len1);

    let len2 = String::from("Здравствуйте").len();
    println!("{}", len2);

    for c in "Здравствуйте".chars() {
        println!("{}", c);
    }

    // графемный кластер
    /*use unicode_segmentation::UnicodeSegmentation;

    let s = "é😊";
    let graphemes = s.graphemes(true).collect::<Vec<&str>>();
    println!("{:?}", graphemes); // ["é", "😊"]*/
}*/    

/*fn main() {
    // объявление вектора
    // let v: Vec<i32> = Vec::new();
    // или
    let mut v = vec![1, 2, 3];

    // вставка элементов
    v.push(5);
    // v.push("Hello, world!"); ERROR

    // получение значения по индексу
    let third: &i32 = &v[2];
    println!("Третий элемент равен {}", third);

    match v.get(6) {
        Some(third) => println!("Третий элемент равен {}", third),
        None => println!("Третий элемент отсутствует."),
    }

    // перебор элементов
    for i in &v {
        println!("{}", i);
    }
}*/