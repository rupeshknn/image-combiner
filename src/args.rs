// logic

fn get_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}

#[derive(Debug)]
pub struct Args {
    image_1: String,
    image_2: String,
    output: String,
}

impl Args{
    pub fn new() -> Self {
        Args {
            image_1: get_nth_arg(1),
            image_2: get_nth_arg(2),
            output: get_nth_arg(3),
        }
    }
}