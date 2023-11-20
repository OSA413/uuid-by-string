use uuid_by_string::generate_uuid;

fn main() {
    println!("{}", generate_uuid::generate_uuid("hello world", 3));
    println!("{}", generate_uuid::generate_uuid("hello world", 5));
}