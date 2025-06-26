// разделение на модули
mod front_of_house;
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

/*use std::io;
use std::io::Write;
// или
use std::io::{self, Write};

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
pub fn eat_at_restaurant() {
    // Абсолютный путь
    crate::front_of_house::hosting::add_to_waitlist();
    // Относительный путь
    front_of_house::hosting::add_to_waitlist();
}

// другой вариант с использованием USE
use crate::front_of_house::hosting;
pub fn eat_at_restaurant1() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

use self::front_of_house::hosting;
pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}*/