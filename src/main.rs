// Cons, Nilはenum Listの列挙子としてここで「定義」している
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    // 値5はヒープ領域に確保されている
    // bかスコープから抜けるとき、メモリの開放が起きる。
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:?}", list);
}
