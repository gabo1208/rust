const TWO: u32 = 1 + 1;
fn main() {
    // ### VARIABLES ###
    // other cases
    // let x = 5;
    // let x;
    let mut x = 5; // without mut it gives an error
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    let y = 3;
    println!("The value of y is: {y}");
    let y = y * y;
    println!("The value of y*y is: {y}");

    // let x = 10; <- you cannot change variable types in the same context
    // let x = "test"; <- error unless it's in another context
    let z = 1;
    {
        let z = "hello";
        println!("The value of internal z is: {z}");
    }
    println!("The value of z is: {z}");

    let number = if z == 1 { 5 } else { 3 };
    println!("The value of number is: {number}");

    // ### CONSTANTS ###
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of three hours in seconds is: {THREE_HOURS_IN_SECONDS}");
    println!("The value of TWO is: {TWO}");

    // ### TUPLES ###
    let tup = (500, 6.4, 1);
    let first = tup.0;
    println!("The value of tup.0 is: {first}");
    let (_x, y, _z) = tup;
    println!("The value of y and z is: {y}");

    // ### ARRAYS ###
    // arrays must contain same type elements
    // let a = [1,"test"] <- error
    let t = ([1; 2], [3; 4]);
    let (a, _b) = t;
    println!("tuple with array prints: {}", a[0] + t.1[0]);

    one_function();
    value_changed();

    // ### LOOPS ###
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
    // ### POINTERS ###
    // Box construct allocate data in the heap and return the pointer
    let a = Box::new([0; 1000000]);
    let b = a; // a loose the ownership of the box her that's why we print b
    println!("{:?} {}", b.as_ptr(), b[1000000 - 1]);
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    println!("{} {}", m1, m2);

    // if we dont pass the pointers then the ownership is lost, i.e
    // greet(m1, m2)
    // format!("{} {}", m1, m2) <- error borrowed variable
    fn greet(g1: &String, g2: &String) {
        // note the ampersands
        println!("{} {}!", g1, g2);
    }
    // errors also depends on order of execution, i.e
    //
    // let mut v: Vec<i32> = vec![1, 2, 3];
    // let num: &i32 = &v[2];
    // v.push(4);
    // println!("Third element is {}", *num);
    //
    // this is invalid, but this is valid:
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    println!("Third element is {}", *num); // swapped here from being the last line
    v.push(4);

    // ### MUTABLE REFERENCES ###
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);
}

// ### FUNCTIONS ###
fn one_function() {
    println!("Hello, world!");

    another_function(10);
}

fn another_function(x: i32) {
    println!("Another function. {x}");
}

fn value_changed() {
    let x = 5;
    let y = value_changed_internal(x);
    println!("The value of external x is: {x}");
    println!("The value of y is: {y}");
}

fn value_changed_internal(mut x: i32) -> i32 {
    x += 5;
    println!("The value of internal x is: {x}");
    x
}
