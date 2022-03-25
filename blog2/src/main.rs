use blog2::Post;

fn print_type_of<T>(msg: &str, _: &T) {
    println!("{} {}", msg, std::any::type_name::<T>())
}

fn main() {
    let mut post = Post::new();

    print_type_of("Type on creation:", &post);

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    print_type_of("Type after review:", &post);

    let post = post.approve();

    print_type_of("Type after approval:", &post);

    assert_eq!("I ate a salad for lunch today", post.content());
}
