use super::{
    ArrayAttribute,
    ScalarAttribute,
    Schema,
    SchemaAttribute,
    SIZEOF_BYTE,
    SIZEOF_INT,
};
use super::schema_common_definition::schema_common_definition;

pub fn register_namespace_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut register_namespace_definition: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ScalarAttribute::new("namespace_type", SIZEOF_BYTE)),
        Box::new(ArrayAttribute::new("duration_parent_id", SIZEOF_INT)),
        Box::new(ArrayAttribute::new("namespace_id", SIZEOF_INT)),
        Box::new(ScalarAttribute::new("namespace_name_size", SIZEOF_BYTE)),
        Box::new(ArrayAttribute::new("name", SIZEOF_BYTE)),
    ];

    schema_definition.append(&mut register_namespace_definition);

    Schema::new(schema_definition)
}