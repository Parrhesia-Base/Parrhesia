use poem::{handler, web::Path};

#[handler]
pub fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}