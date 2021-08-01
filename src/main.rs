mod project;
mod handler;
mod review;
mod work;
mod migrate;

fn main() {
    let mut handler = handler::Handler::new();

    handler.handle();
}
