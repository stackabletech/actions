fn main() {
    let schema = schemars::schema_for!(interu::config::Config);

    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
