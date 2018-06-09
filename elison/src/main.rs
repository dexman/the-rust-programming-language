struct Foo {
    x: f32,
}

impl Foo {
    fn doit<'a>(&'a self, y: & f32) -> &'a f32 {
        y
    }
}

fn main() {
    println!("Hello, world!");
    let foo = Foo { x: 4.2 };
    let y = 5.3;
    let d = foo.doit(&y);
    println!("doit={}", d);
}
