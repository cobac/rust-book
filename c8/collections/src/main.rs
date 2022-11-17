fn main() {
    {
        // Vectors
        let _v: Vec<i32> = Vec::new();
        let _v = vec![1, 2, 3];
    }
    {
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        println!("{v:?}");
    }
    {
        let v = vec![1, 2, 3, 4, 5];

        // Panics on error
        let third = &v[2];
        println!("The third element is {}", third);

        // Handles the error
        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }
    }
    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
    }
    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
    }
    {
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        println!("{:?}", row);
    }
    {
        let s = String::new();
        println!("{s}");
        let s_to_str = "initial contents".to_string();
        println!("{s_to_str}");
        let s_from = String::from("initial contents");
        println!("{s_from}");
    }
    {
        // .push_str pushes a string, does not take ownership
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {}", s2);
    }
    {
        // .push pushes a character
        let mut s = String::from("lo");
        s.push('l');
    }
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    }
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        //let s = s1 + "-" + &s2 + "-" + &s3;
        // println!("{s}"); This would fail because the value of s1 is owned by s
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("{s}");
        println!("{s1}");
    }
    {
        println!("{:?}", "Зд".chars());
        for c in "Зд".chars() {
            println!("{}", c);
        }

        println!("{:?}", "Зд".bytes());
        for c in "Зд".bytes() {
            println!("{}", c);
        }
    }
    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        println!("{:?}", scores);

        let team_name = String::from("Blue");
        let score = scores
            .get(&team_name)
            .copied() // Option<&i32> -> Option<i32>
            .unwrap_or(0); // Option<i32> -> i32 (None => 0)
        println!("Blue: {score:?}");

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }
    {
        use std::collections::HashMap;
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), "old");
        // Overwrites
        scores.insert(String::from("Blue"), "new");

        println!("{:?}", scores);

        // Doesn't overwrite
        // Entry = Vacant | Occupied
        println!("Entry enum: {:?}", scores.entry(String::from("Yellow")));

        scores.entry(String::from("Yellow")).or_insert("yellow");
        scores.entry(String::from("Blue")).or_insert("blue_default");
        println!("{:?}", scores);
    }
    {
        use std::collections::HashMap;

        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}
