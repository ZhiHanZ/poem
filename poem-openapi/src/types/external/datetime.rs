use std::borrow::Cow;

use chrono::{DateTime, FixedOffset};
use serde_json::Value;

use crate::{
    poem::web::Field,
    registry::{MetaSchema, MetaSchemaRef},
    types::{
        ParseError, ParseFromJSON, ParseFromMultipartField, ParseFromParameter, ParseResult,
        ToJSON, Type,
    },
};

impl Type for DateTime<FixedOffset> {
    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(Box::new(MetaSchema::new_with_format("string", "date-time")))
    }

    impl_value_type!();

    fn name() -> Cow<'static, str> {
        "string(date-time)".into()
    }
}

impl ParseFromJSON for DateTime<FixedOffset> {
    fn parse_from_json(value: Value) -> ParseResult<Self> {
        if let Value::String(value) = value {
            Ok(value.parse()?)
        } else {
            Err(ParseError::expected_type(value))
        }
    }
}

impl ParseFromParameter for DateTime<FixedOffset> {
    fn parse_from_parameter(value: Option<&str>) -> ParseResult<Self> {
        match value {
            Some(value) => Ok(value.parse()?),
            None => Err(ParseError::expected_input()),
        }
    }
}

#[poem::async_trait]
impl ParseFromMultipartField for DateTime<FixedOffset> {
    async fn parse_from_multipart(field: Option<Field>) -> ParseResult<Self> {
        match field {
            Some(field) => Ok(field.text().await?.parse()?),
            None => Err(ParseError::expected_input()),
        }
    }
}

impl ToJSON for DateTime<FixedOffset> {
    fn to_json(&self) -> Value {
        Value::String(self.to_rfc3339())
    }
}
