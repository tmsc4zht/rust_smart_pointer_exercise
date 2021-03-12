use std::ops::Deref;

// Cons, Nilはenum Listの列挙子としてここで「定義」している
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

// Boxは1要素のタプルとして実装できる。
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// スマートポインタとして振る舞うため参照外し演算子用のメソッドが必要
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let b = Box::new(5);
    // 値5はヒープ領域に確保されている
    // bかスコープから抜けるとき、メモリの開放が起きる。
    println!("b = {}", b);

    // Boxの参照外し演算子について
    let x = 5;
    let y = Box::new(x);
    assert_eq!(x, 5);
    assert_eq!(*y, 5); // yはポインタのように振る舞うので、参照外し演算子が必要。

    // スマートポインタの自前実装について
    let y = MyBox::new(x);
    assert_eq!(*y, 5);

    // Consの実行テスト
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}
