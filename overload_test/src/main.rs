use std::fmt::Display;

struct Bar {
    x : i64,
}

impl Bar {
    fn test(self) {
        println!("In Bar self impl: {}", self.x);
    }
}
/*
impl &Bar {
    fn test(self) {
        println!("In &Bar self impl: {}", self.x);
    }
}

impl &mut Bar{
    fn test(self) {
        println!("In &mut Bar self impl: {}", self.x);
    }
}
*/
// impl &Bar {
//  ^^^^ impl requires a base type

struct Foo<T> {
    x : T,
}


impl<T> Foo<T>
    where T : Display {
    fn test(self) {
        println!("In Bar self impl: {}", self.x);
    }
}
/*
impl<T> &Foo<T>
    where T: Display {
    fn test(self) {
        println!("In &Bar self impl: {}", self.x);
    }
}

impl<T> &mut Foo<T>
    where T: Display {
    fn test(self) {
        println!("In &mut Bar self impl: {}", self.x);
    }
}
*/

/*40 | impl<T> &Foo<T>
   |         ^^^^^^^ impl requires a base type
 */

trait Animal {
    fn print(self);
}

impl Animal for Bar {

    fn print(self) {
        println!("In Bar self impl: {}", self.x);
    }
    /*
    // duplicate definitions with name `print`:
    fn print(&self) {
        println!("In bar self impl: {}", self.x);
    }

    // duplicate definitions with name `print`:
    fn print(&mut self) {
        println!("In bar self impl: {}", self.x);
    }
    */

}

/*
impl Animal for Bar {

    // duplicate definitions with name `print`:
    fn print(&self) {
        println!("In bar self impl: {}", self.x);
    }

}
// 82 | impl Animal for Bar {
//    | ^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Bar`
*/

impl Animal for &Bar {
    fn print(self) {
        println!("In &Bar self impl: {}", self.x);
    }
}

impl Animal for &mut Bar {
    fn print(self) {
        println!("In &mut Bar self impl: {}", self.x);
    }
}

impl Bar {
    fn print(self) {
        println!("Not in trait: {}", self.x);
    }
}

fn main() {
    let bar = Bar { x : 7 };
    bar.test(); // print: In Bar self impl: 7

    //let bar2 = Bar { x : 8 };
    //let bar2_ref = &bar2;
    //bar2_ref.test();
    /*
    64 |     bar2_ref.test();
       |     ^^^^^^^^ cannot move out of borrowed content
    */
    //let mut bar3 = Bar { x : 9 };
    //let bar3_mut_ref = &mut bar3;
    //bar3_mut_ref.test();
    /*
    71 |     bar3_mut_ref.test();
       |     ^^^^^^^^^^^^ cannot move out of borrowed content
   */
    let bar4 = Bar { x : 10 };
    bar4.print(); // print: In Bar self impl: 10 (without the print implemented in Bar), and print: Not in trait: 10 (with the print(self)  implemented in Bar)
    //bar4.print(); //error[E0382]: use of moved value: `bar4`

    let mut bar5 = Bar { x : 11 };
    {
        let bar5_ref = &bar5;

        bar5_ref.print(); // print: In Bar self impl: 11 (without the print implemented in Bar), and print: Not in trait: 11(& self)  (with the print implemented in Bar)
        bar5_ref.print(); // print: In Bar self impl: 11 (without the print implemented in Bar), and print: Not in trait: 11(& self)  (with the print implemented in Bar)
    }
    {
        let bar5_mut_ref = &mut bar5;
        bar5_mut_ref.print(); // print: In Bar self impl: 11 (without the print implemented in Bar), and print: Not in trait: 11 (with the print(&mut self) implemented in Bar)
        bar5_mut_ref.print(); // print: In Bar self impl: 11 (without the print implemented in Bar), and print: Not in trait: 11 (with the print(&mut self) implemented in Bar)
    }

}