use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

// 木構造データ
#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

// Cons, Nilはenum Listの列挙子としてここで「定義」している
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
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
    let list = Cons(
        Rc::new(RefCell::new(1)),
        Rc::new(Cons(
            Rc::new(RefCell::new(2)),
            Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::new(Nil))),
        )),
    );
    println!("{:?}", list);

    // Consの実行テスト（データ共有）
    // 右辺の所有権はaにある。
    let a = Rc::new(Cons(
        Rc::new(RefCell::new(5)),
        Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil))),
    ));
    // a生成後のカウント = {}
    println!("count after creating a = {}", Rc::strong_count(&a));

    // aの指す値の所有権はaにあるので、直接aをConsの後ろに入れると
    // 所有権がbに移り、aを使って参照できなくなる
    let _b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    // b生成後のカウント = {}
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
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

    // 可変化できるList
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // 木構造データのテスト
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // leafの親 = {:?}
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
