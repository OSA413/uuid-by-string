use uuid_by_string::generate_uuid_with_namespace::generate_uuid_with_namespace;

fn main() {
    println!("{:?}", generate_uuid_with_namespace("hello world", "D3486AE9-136e-5856-bc42-212385ea7970", 3));
    println!("{:?}", generate_uuid_with_namespace("hello world", "d3486ae9-136e-5856-bc42-212385ea7970", 3));
}