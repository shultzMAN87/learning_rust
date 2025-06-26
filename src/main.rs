/*fn largest_<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}*/

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
            if item > largest {
            largest = item;
            }
    }
    largest
}

fn main() {
    let number_list = vec![250, 50, 25, 100, 65];
    /*let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }*/

    // let largest = largest_(&number_list);
    let result1 = largest(&number_list);

    println!("Наибольшее число равно {}", result1);

    let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest_char(&char_list);
    let result2 = largest(&char_list);
    println!("Наибольший символ равен {}", result2);
}

/*use std::fs::File;
fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Проблема с открытием файла: {:?}", error)
        },
    };
}*/

/*fn main() {
    //panic!("полное фиаско");

    let v = vec![1, 2, 3];
    v[99];

    // $env:RUST_BACKTRACE="1"; cargo run
}*/
