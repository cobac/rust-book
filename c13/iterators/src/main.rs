#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn main() {
    {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        println!("{:?}", v1_iter);
        println!("{:?}", v1_iter.next());
        println!("{:?}", v1_iter);
        println!("{:?}", v1_iter.next());
        println!("{:?}", v1_iter);
        println!("{:?}", v1_iter.next());
        println!("{:?}", v1_iter.next());
        println!("{:?}", v1_iter.next());
        println!("{:?}", v1_iter);
    }
    {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();
        println!("sum: {total}");
    }
    {
        let v1: Vec<i32> = vec![1, 2, 3];
        // Compiler can infer the type of the element, but no the type
        // of the collection
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        println!("v1: {v1:?}");
        println!("v2: {v2:?}");
    }

    {
        let mut v1: Vec<i32> = vec![1, 2, 3];
        // Compiler can infer the type of the element, but no the type
        // of the collection
        v1.iter_mut().for_each(|x| *x += 1);
        println!("v1: {v1:?}");
        for x in v1.iter_mut() {
            *x += 1
        }
        println!("v1: {v1:?}");
    }
    {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let shoe_size: u32 = 10;
        let in_my_size: Vec<Shoe> = shoes.into_iter().filter(|s| s.size == shoe_size).collect();
        println!("filtered: {:?}", in_my_size);
        // Fails, it's value vas moded into the iterator
        // println!("all shoes: {:?}", shoes);
    }
    {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let shoe_size: u32 = 10;
        let in_my_size: Vec<&Shoe> = shoes.iter().filter(|s| s.size == shoe_size).collect();
        println!("filtered: {:?}", in_my_size);
        // Works!
        println!("all shoes: {:?}", shoes);
    }
}
