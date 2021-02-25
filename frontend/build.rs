use std::fs;

use juniper::IntrospectionFormat;

use gateway::graphql::schema::{create_schema, Context};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = Context::new(None, None, None, None, None, None, None, None);

    let (res, _errors) =
        juniper::introspect(&create_schema(), &ctx, IntrospectionFormat::default()).unwrap();

    let json_result = serde_json::to_string_pretty(&res).expect("Failed to make JSON");
    fs::write("./src/gen/schema.json", json_result).expect("Unable to write file");

    Ok(())
}
