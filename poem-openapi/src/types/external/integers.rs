use std::borrow::Cow;

use poem::web::Field;
use serde_json::Value;

use crate::{
    registry::{MetaSchema, MetaSchemaRef},
    types::{
        ParseError, ParseFromJSON, ParseFromMultipartField, ParseFromParameter, ParseResult,
        ToJSON, Type,
    },
};

macro_rules! impl_type_for_integers {
    ($(($ty:ty, $format:literal)),*) => {
        $(
        impl Type for $ty {
            fn name() -> Cow<'static, str> {
                format!("integer({})", $format).into()
            }

            fn schema_ref() -> MetaSchemaRef {
                MetaSchemaRef::Inline(Box::new(MetaSchema::new_with_format("integer", $format)))
            }

            impl_value_type!();
        }

        impl ParseFromJSON for $ty {
             fn parse_from_json(value: Value) -> ParseResult<Self> {
                if let Value::Number(n) = value {
                    let n = n
                        .as_i64()
                        .ok_or_else(|| ParseError::from("invalid integer"))?;

                    if n < Self::MIN as i64 || n > Self::MAX as i64 {
                        return Err(ParseError::from(format!(
                            "Only integers from {} to {} are accepted.",
                            Self::MIN,
                            Self::MAX
                        )));
                    }

                    Ok(n as Self)
                } else {
                    Err(ParseError::expected_type(value))
                }
            }
        }

        impl ParseFromParameter for $ty {
            fn parse_from_parameter(value: Option<&str>) -> ParseResult<Self> {
                match value {
                    Some(value) => value.parse().map_err(ParseError::custom),
                    None => Err(ParseError::expected_input()),
                }
            }
        }

        #[poem::async_trait]
        impl ParseFromMultipartField for $ty {
            async fn parse_from_multipart(field: Option<Field>) -> ParseResult<Self> {
                match field {
                    Some(field) => Ok(field.text().await?.parse()?),
                    None => Err(ParseError::expected_input()),
                }
            }
        }

        impl ToJSON for $ty {
            fn to_json(&self) -> Value {
                Value::Number((*self).into())
            }
        }

        )*
    };
}

macro_rules! impl_type_for_unsigneds {
    ($(($ty:ty, $format:literal)),*) => {
        $(
        impl Type for $ty {
            fn name() -> Cow<'static, str> {
                format!("integer({})", $format).into()
            }

            fn schema_ref() -> MetaSchemaRef {
                MetaSchemaRef::Inline(Box::new(MetaSchema::new_with_format("integer", $format)))
            }

            impl_value_type!();
        }

        impl ParseFromJSON for $ty {
             fn parse_from_json(value: Value) -> ParseResult<Self> {
                if let Value::Number(n) = value {
                    let n = n
                        .as_u64()
                        .ok_or_else(|| ParseError::from("invalid integer"))?;

                    if n < Self::MIN as u64 || n > Self::MAX as u64 {
                        return Err(ParseError::from(format!(
                            "Only integers from {} to {} are accepted.",
                            Self::MIN,
                            Self::MAX
                        )));
                    }

                    Ok(n as Self)
                } else {
                    Err(ParseError::expected_type(value))
                }
            }
        }

        impl ParseFromParameter for $ty {
            fn parse_from_parameter(value: Option<&str>) -> ParseResult<Self> {
                match value {
                    Some(value) => value.parse().map_err(ParseError::custom),
                    None => Err(ParseError::expected_input()),
                }
            }
        }

        #[poem::async_trait]
        impl ParseFromMultipartField for $ty {
            async fn parse_from_multipart(field: Option<Field>) -> ParseResult<Self> {
                match field {
                    Some(field) => Ok(field.text().await?.parse()?),
                    None => Err(ParseError::expected_input()),
                }
            }
        }

        impl ToJSON for $ty {
            fn to_json(&self) -> Value {
                Value::Number((*self).into())
            }
        }

        )*
    };
}

impl_type_for_integers!((i8, "int8"), (i16, "int16"), (i32, "int32"), (i64, "int64"));

impl_type_for_unsigneds!(
    (u8, "uint8"),
    (u16, "uint16"),
    (u32, "uint32"),
    (u64, "uint64")
);
