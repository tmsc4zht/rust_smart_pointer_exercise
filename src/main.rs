use std::ops::Deref;
use std::rc::Rc;

// Cons, Nilはenum Listの列挙子としてここで「定義」している
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
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

// Dropトレイトの確認
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
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

    // 参照外し型強制について
    let m = MyBox::new(String::from("Rust"));

    // helloの引数の型は`&str`だが、参照外し型強制がはたらく。
    // MyBoxはderefを実装しているため、&MyBoxのderefが呼ばれ&Stringが返る
    // Stringはderefを実装しているため、&Stringのderefが呼ばれ&strが返る
    // 結局関数の引数の型に一致するようになる。
    hello(&m);

    // Consの実行テスト
    let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
    println!("{:?}", list);

    // Consの実行テスト（データ共有）
    // 右辺の所有権はaにある。
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // a生成後のカウント = {}
    println!("count after creating a = {}", Rc::strong_count(&a));

    // aの指す値の所有権はaにあるので、直接aをConsの後ろに入れると
    // 所有権がbに移り、aを使って参照できなくなる
    let _b = Cons(3, Rc::clone(&a));
    // b生成後のカウント = {}
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        // c生成後のカウント = {}
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    // cがスコープを抜けた後のカウント = {}
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // Dropの実行テスト
    let _c = CustomSmartPointer {
        data: String::from("first"),
    };
    let _d = CustomSmartPointer {
        data: String::from("second"),
    };
    drop(_c);
    {
        let _e = CustomSmartPointer {
            data: String::from("third"),
        };
    }
}
