fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ownership() {
        {
            let point = Box::new((0.625, 0.5)); // point allocated here
            let label = format!("{:.?}", point); // label allocated here
            assert_eq!(label, "(0.625, 0.5)");
        } // both point and label are dropped here

        {
            struct Person {
                name: String,
                birth: i32,
            };
            let mut composers = Vec::new();
            composers.push(Person {
                name: "Palestina".to_string(),
                birth: 1525,
            });
            composers.push(Person {
                name: "Download".to_string(),
                birth: 1563,
            });
            composers.push(Person {
                name: "Lully".to_string(),
                birth: 1632,
            });
            for composer in &composers {
                println!("{}, born {}", composer.name, composer.birth);
            }
        }
    }

    #[test]
    fn test_moves() {
        /* Code that doesn't suppose to run
            let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
            let t = s;
            let u = s;

            output:
            error[E0382]: use of moved value: `s`
            |
            7 |     let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
            |         - move occurs because `s` has type `Vec<String>`,
            |           which does not implement the `Copy` trait
            8 |     let t = s;
            |             - value moved here
            9 |     let u = s;
            |             ^ value used here after move
        */

        // to fix the above using deep copy:
        let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
        let _t = s.clone();
        let _u = s.clone();

        let mut s = "Govinda".to_string();
        s = "Siddhartha".to_string(); // value "Govinda" dropped here

        let mut s = "Govinda".to_string();
        let t = s;
        s = "Siddhartha".to_string(); // nothing is dropped here

        {
            struct Person {
                name: String,
                birth: i32,
            }

            let mut composers = Vec::new(); // returning a vector which moves ownership
            composers.push(Person {
                // constructing new value moves the ownership of the name to the structure
                name: "Palestina".to_string(),
                birth: 1525,
                // passing the Person value to function moves the ownership of the person to the vector
            });
        }
        /* Code that doesn't suppose to run
            let x = vec![10, 20, 30];
            if c {
                f(x); // ... ok to move from x here
            } else {
                g(x); // ... and ok to also move from x here
            }
            h(x); // bad: x is uninitialized here if either path uses it


            let x = vec![10, 20, 30];
            while f() {
                g(x) // bad: x would be moved in the first iteration,
                    // uninitialized in the second
            }

            to fix the above
            let x = vec![10, 20, 30];
            while f() {
                g(x) //moves from x
                x = h() //gives x a fresh value
            }
        */

        let mut v = Vec::new();
        for i in 101..106 {
            v.push(i.to_string());
        }

        /* Code that doesn't suppose to run
           // Pull out random elements from the vector.
           let third = v[2]; // error: Cannot move out of index of Vec
           let fifth = v[4]; // here too

           output:
           error[E0507]: cannot move out of index of `Vec<String>`
           |
           14 |     let third = v[2];
           |                 ^^^^
           |                 |
           |                 move occurs because value has type `String`,
           |                 which does not implement the `Copy` trait
           |                 help: consider borrowing here: `&v[2]`
        */
        // how to fix that:

        // 1. Pop a value off the end of the vector:
        let fifth = v.pop().expect("vector empty!");
        assert_eq!(fifth, "105");

        // 2. Move a value out of a given index in the vector,
        // and move the last element into its spot:
        let second = v.swap_remove(1);
        assert_eq!(second, "102");

        // 3.Swap in another value for the one we're taking out:
        let third = std::mem::replace(&mut v[2], "substitute".to_string());
        assert_eq!(third, "103");

        // let's see what's left of our vector.
        assert_eq!(v, vec!["101", "104", "substitute"]);

        let v = vec![
            "liberté".to_string(),
            "égalité".to_string(),
            "fraternité".to_string(),
        ];

        for mut s in v {
            s.push('!'); // you can mutate as the ownership moved from v
            println!("{}", s);
        }

        struct Person {
            name: Option<String>,
            birth: i32,
        }

        let mut composers = Vec::new();
        composers.push(Person {
            name: Some("Palestine".to_string()),
            birth: 1525,
        });
        // You can't do this:
        // let first_name = compose[0].name;

        let first_name = std::mem::replace(&mut composers[0].name, None);
        assert_eq!(first_name, Some("Palestine".to_string()));
        assert_eq!(composers[0].name, None);
    }

    #[test]
    fn test_copy_types() {
        /* Code that doesn't suppose to run
        struct Label {number: u32}

        fn print(l: Label) {println!("STAMP: {}", l.number);}

        let l = Label {number: 3};
        print(l);
        println!(My label number is: {}", l.number);

        result:
        error: borrow of moved value: `l`
        |
        10 |     let l = Label { number: 3 };
        |         - move occurs because `l` has type `main::Label`,
        |           which does not implement the `Copy` trait
        11 |     print(l);
        |           - value moved here
        12 |     println!("My label number is: {}", l.number);
        |                                        ^^^^^^^^ value borrowed here after move
        */

        // this works
        {
            #[derive(Copy, Clone)]
            struct Label {
                number: u32,
            }
            fn print(l: Label) {
                println!("STAMP: {}", l.number);
            }

            let l = Label { number: 3 };
            print(l);
            println!("My label number is: {}", l.number);
        }

        /* Code that doesn't suppose to run
        #[derive(Copy, Clone)]
        struct StringLabel {name: String}

        output:
        error[E0204]: the trait `Copy` may not be implemented for this type
        --> ownership_string_label.rs:7:10
        |
        7 | #[derive(Copy, Clone)]
        |          ^^^^
        8 | struct StringLabel { name: String }
        |                      ------------ this field does not implement `Copy`
         */
    }
}
