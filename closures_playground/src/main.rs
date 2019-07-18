fn winner<T, F> (a : T, b : T, bigger_fn : F) -> bool
    where F : FnOnce(T, T) -> bool {
    bigger_fn(a,b)
}

enum TypedIf {
    Then,
    Else
}

fn do_a() -> bool {
    // Complex operation which might fail
    true
}

fn do_very_expensive_a() -> bool {
    // Complex operation which might fail
    true
}

impl TypedIf {
    fn do_if(test: bool) -> TypedIf {
        if test {
            TypedIf::Then
        } else {
            TypedIf::Else
        }
    }

    fn compose_do_if<F, U>(test:bool, compose : F) -> U
        where F: FnOnce(TypedIf) -> U {
        if test {
            compose(TypedIf::Then)
        } else {
            compose(TypedIf::Else)
        }
    }

    fn and_then(self, test : bool) -> TypedIf {
        match self {
            TypedIf::Then => TypedIf::do_if(test),
            TypedIf::Else => TypedIf::Else
        }
    }

    fn better_and_then<F>(self, test : F) -> TypedIf
        where F: FnOnce() -> bool {
        match self {
            TypedIf::Then => TypedIf::do_if(test()),
            TypedIf::Else => TypedIf::Else
        }
    }

    fn compose_better_and_then<P, F, U>(self, test : P, compose : F) -> U
        where P: FnOnce() -> bool,
              F: FnOnce(TypedIf) -> U {
        match self {
            TypedIf::Then => TypedIf::compose_do_if(test(), compose),
            TypedIf::Else => compose(TypedIf::Else)
        }
    }
}

fn main() {
    let bigger_fn = |a : i32, b: i32| -> bool {
        a > b
    };

    let string = String::from("test");

    if winner(String::from("test"), String::from("test2"), move |_a,_b| {
        println!("{}", string); // Doesn't require ownership on the string
        true
    }) {
        //println!("{}", string);
        // The line above will only compile if we removes the move keyword
    }

    if winner(1, 7,  bigger_fn) {
        println!("One is the winner");
    } else {
        println!("Seven is the winner");
    }

    if winner(&1, &7, |a, b| a < b) {
        println!("Now one is the winner");
    } else {
        println!("Now seven is the winner");
    }

    let helper : i32 = 7;
    if winner(1, 7, |a, b| a + helper > b) {
        println!("Helper made one a winner");
    } else {
        println!("Helper wasn't enough, seven is still the winner");
    }

    let _type_id = TypedIf::do_if(do_a()).and_then(do_very_expensive_a());
    let _type_id = TypedIf::do_if(do_a()).better_and_then(|| { do_very_expensive_a() });

    let _bool_result = TypedIf::do_if(do_a())
        .compose_better_and_then(|| {do_very_expensive_a()}, |result| {
            match result {
                TypedIf::Then => true,
                TypedIf::Else => false
            }
        });
}
