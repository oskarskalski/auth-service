use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Foo {
    fn new () -> Self;
    fn get_value(&self) -> i32;
}
struct FooObj;

impl Foo for FooObj {
    fn new() -> Self {
        FooObj {}
    }
    fn get_value(&self) -> i32 {
        12
    }
}


pub fn dosmth<F>(foo: F) -> i32
where
    F: Foo
{
    let k = foo.get_value();
    10 * k
}

#[test]
fn test() {
    let mut k = MockFoo::default();
    k.expect_get_value().once().return_const(10);
    assert_eq!(100, dosmth(k))
}
