// cargo script src/collection_string.rs

fn main() {
    {
        let mut s = String::new();
    }
    {
        let data = "initial contents";
        let s = data.to_string();
        // the method also works on a literal directly:
        let s = "initial contents".to_string();
    }
    {
        let s = String::from("initial contents");
    }
    {
        let hello = String::from("السلام عليكم");
        let hello = String::from("こんにちは");
        let hello = String::from("Здравствуйте");
    }
    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {}", s2);
    }
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
        println!("s3 is {}", s3);
    }
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("s is {}", s);
    }
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("s is {}", s);
    }
}
