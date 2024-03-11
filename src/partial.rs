/*
    let vf = vec![Foo::F1, Foo::F2];
    let vb = vec![Bar::B1, Bar::B2];

    if vf == vb {
        println!("Equal");
    } else {
        println!("Not equal");
    }
*/

pub enum Foo {
    F1,
    F2,
}

impl PartialEq<Bar> for Foo {
    fn eq(&self, other: &Bar) -> bool {
        match (self, other) {
            (Foo::F1, Bar::B1) => true,
            (Foo::F2, Bar::B2) => true,
            _ => false,
        }
    }
}

pub enum Bar {
    B1,
    B2,
}