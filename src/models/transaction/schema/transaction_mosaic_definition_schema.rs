use super::{
    ArrayAttribute,
    ScalarAttribute,
    Schema,
    SchemaAttribute,
    SIZEOF_BYTE,
    SIZEOF_INT,
    TableArrayAttribute,
};
use super::schema_common_definition::schema_common_definition;

pub fn mosaic_definition_transaction_schema() -> Schema {
    let mut schema_definition = schema_common_definition();

    let mut transfer_schema_definition: Vec<Box<dyn SchemaAttribute>> = vec![
        Box::new(ScalarAttribute::new("mosaic_nonce", SIZEOF_INT)),
        Box::new(ArrayAttribute::new("mosaic_id", SIZEOF_INT)),
        Box::new(ScalarAttribute::new("num_optional_properties", SIZEOF_BYTE)),
        Box::new(ScalarAttribute::new("flags", SIZEOF_BYTE)),
        Box::new(ScalarAttribute::new("divisibility", SIZEOF_BYTE)),
        Box::new(TableArrayAttribute::new(
            "modifications",
            vec![
                Box::new(ScalarAttribute::new("mosaic_property_id", SIZEOF_BYTE)),
                Box::new(ArrayAttribute::new("value", SIZEOF_INT)),
            ],
        )
        )
    ];

    schema_definition.append(&mut transfer_schema_definition);

    Schema::new(schema_definition)
}