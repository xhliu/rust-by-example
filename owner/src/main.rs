// https://depth-first.com/articles/2020/01/27/rust-ownership-by-example/

#[derive(Debug)]
// #[derive(Debug, Clone, Copy)]
struct Person {
    age: i8,
}

fn main() {
    {
        let a = vec![1, 2, 3]; // a growable array literal
        let b = a; // move: `a` can no longer be used

        // println!("b: {:?}", a); // error: borrow of moved value: `a`
        println!("b: {:?}", b);
    }

    {
        fn sum(left: i32, right: i32) -> i32 {
            left + right
        }

        let a = 42;
        let b = 1;
        let s = sum(a, b);
        println!("this sum of {} and {} is {}", a, b, s); // no error! i32 is copied, vector is not
    }

    {
        fn sum(vector: Vec<i32>) -> i32 {
            let mut sum = 0;
            for item in vector {
                sum = sum + item
            }
            sum
        }

        let v = vec![1, 2, 3];
        let s = sum(v);

        println!("sum: {}", s);
        // println!("sum of {:?}: {}", v, s); // error: borrow of moved value: `v`
    }

    {
        fn sum(vector: &Vec<i32>) -> i32 {
            // borrow signature
            let mut sum = 0;
            for item in vector {
                sum = sum + item
            }
            sum
        }

        let v = vec![1, 2, 3];
        let v_ref = &v; // v_ref borrows v
        let s = sum(v_ref);
        println!("sum of {:?}: {}", v_ref, s); // no error
    }

    {
        fn create_series(x: i32) -> Vec<i32> {
            let result = vec![x, x + 1, x + 2];
            result
        }

        let series = create_series(42); // function return moves
        println!("series: {:?}", series);
    }

    {
        let alice = Person { age: 42 };
        let bob = alice;
        // println!("alice: {:?}\nbob: {:?}", alice, bob); // ERROR: alice moved, if no Copy trait
    }

    {
        let alice = Person { age: 42 };
        let bob = &alice; // bob borrows alice
        println!("alice: {:?}\nbob: {:?}", alice, bob);
    }

    {
        fn pass_number_by_reference(number: &i8) -> bool {
            number.is_negative()
        }
        fn pass_number_by_value(number: i8) -> bool {
            number.is_negative()
        }
        fn pass_vec_by_reference(vec: &Vec<i8>) -> bool {
            vec.is_empty()
        }

        // numbers implement Copy, and so can be passed by value or reference
        let number = 42;
        // does not move number because of borrow
        let is_negative_by_ref = pass_number_by_reference(&number);
        // moves number, which can never be used again
        let is_negative_by_value = pass_number_by_value(number);
        // copy not implemented - must be passed by reference
        let vec = vec![];
        // does not move vec
        let is_empty = pass_vec_by_reference(&vec);
        println!("is_negative_by_value: {}", is_negative_by_value);
        println!("is_negative_by_ref: {}", is_negative_by_ref);
        println!("vec {:?} is_empty: {}", vec, is_empty);
    }

    {
        fn byte_length(string: &str) -> usize {
            string.bytes().len()
        }

        let string = "hello world";
        let length = byte_length(string);
        println!("Bytes in \"{}\": {}", string, length);
    }

    {
        // Errors!
        // fn longest(x: &str, y: &str) -> &str {
        //     // missing lifetime specifier
        //     if x.bytes().len() > y.bytes().len() {
        //         x
        //     } else {
        //         y
        //     }
        // }

        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.bytes().len() > y.bytes().len() {
                x
            } else {
                y
            }
        }

        let alice = "Alice";
        let bob = "Bob";
        println!("{}", longest(alice, bob));
    }

    {
        #[derive(Debug)]
        // struct Person {
        //     name: &str, // error: expected lifetime parameter
        // }
        struct Person<'a> {
            name: &'a str, // error: expected lifetime parameter
        }

        let alice = Person { name: "Alice" };
        println!("alice: {:?}", alice);
    }

    {
        let mut numbers = vec![1, 2, 3];
        numbers.push(4); // mutable Vec supports push
        println!("numbers: {:?}", numbers); // numbers: [1, 2, 3, 4]
    }

    // multiple immutable borrows or one mutable borrow
    {
        let mut writer = vec![1, 2, 3];
        // let reader = &writer;

        writer.push(4); // cannot borrow `writer` as mutable because it is also borrowed as immutable

        let reader = &writer;
        println!("len: {}", reader.len()); // no error, reader is not active because it was borrowed
    }

    {
        // https://www.openmymind.net/Rust-Ownership-Move-and-Borrow-part-1/
        let mut a1 = 1;
        let a2 = &a1;
        let a3 = &a1; // No Problem. Can have multiple borrows
        println!("{:?} {:?} {:?}", a1, a2, a3);

        // let mut b1 = 1;
        // let b2 = &mut b1;
        // let b3 = &mut b1; // Fail. Cannot mutably borrow when already mutably borrowed
        // println!("{:?} {:?} {:?}", b1, b2, b3);

        // let mut c1 = 1;
        // let c2 = &c1;
        // let c3 = &mut c1; // Fail. Cannot mutably borrow when already borrowed
        // println!("{:?} {:?} {:?}", c1, c2, c3);

        // let mut d1 = 1;
        // let d2 = &mut d1;
        // let d3 = &d1; // Fail. Cannot borrow when already mutably borrowed
        // println!("{:?} {:?} {:?}", d1, d2, d3);
    }
}
