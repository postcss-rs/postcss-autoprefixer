use postcss_autoprefixer::vendor::{get_prefix, remove_prefix};

fn main() {
    let prefix = get_prefix("-test-te");
    println!("{:?}", remove_prefix("-test-test-test-test"));
}
