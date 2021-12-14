use std::result;

use serde::{de, Deserialize, Deserializer};
use serde_json::{Map, Value};

#[derive(Debug)]
pub enum ResponseBody<Resource> {
    Success(Resource),
    Error(ErrorResponseBody),
}
impl<'de, Resource> Deserialize<'de> for ResponseBody<Resource>
where
    Resource: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = Map::deserialize(deserializer)?;

        if map.get("error").is_none() {
            let value = Value::Object(map);

            Resource::deserialize(value)
                .map(ResponseBody::Success)
                .map_err(de::Error::custom)
        } else {
            let value = Value::Object(map);
            ErrorResponseBody::deserialize(value)
                .map(ResponseBody::Error)
                .map_err(de::Error::custom)
        }
    }
}

//
//
//
#[derive(Debug, Deserialize)]
pub struct ErrorResponseBody {
    pub error: ErrorResponseBodyError,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponseBodyError {
    pub errors: Vec<ErrorResponseBodyErrorError>,
    pub code: usize,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponseBodyErrorError {
    pub domain: String,
    pub reason: String,
    pub message: String,
    #[serde(rename(deserialize = "locationType"))]
    pub location_type: Option<String>,
    pub location: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize)]
    struct Foo {
        pub name: String,
    }

    #[test]
    fn success() {
        let json = r#"
        {
            "name": "bar"
        }
        "#;

        let b: ResponseBody<Foo> = serde_json::from_str(json).unwrap();
        match b {
            ResponseBody::Success(foo) => {
                assert_eq!(foo.name, "bar");
            }
            ResponseBody::Error(_) => panic!(),
        }
    }

    #[test]
    fn error_code_is_400() {
        let json = r#"
        {
            "error": {
                "errors": [
                    {
                        "domain": "global",
                        "reason": "invalid",
                        "message": "Invalid Value"
                    }
                ],
                "code": 400,
                "message": "Invalid Value"
            }
        }
        "#;

        let b: ResponseBody<()> = serde_json::from_str(json).unwrap();
        match b {
            ResponseBody::Success(_) => panic!(),
            ResponseBody::Error(b) => {
                assert_eq!(b.error.errors.len(), 1);
                let err = b.error.errors.first().unwrap();
                assert_eq!(err.domain, "global");
                assert_eq!(err.reason, "invalid");
                assert_eq!(err.message, "Invalid Value");
                assert_eq!(err.location_type, None);
                assert_eq!(err.location, None);
                assert_eq!(b.error.code, 400);
                assert_eq!(b.error.message, "Invalid Value");
            }
        }
    }

    #[test]
    fn error_code_is_404() {
        let json = r#"
        {
            "error": {
                "errors": [
                    {
                        "domain": "global",
                        "reason": "applicationNotFound",
                        "message": "No application was found for the given package name.",
                        "locationType": "parameter",
                        "location": "packageName"
                    }
                ],
                "code": 404,
                "message": "No application was found for the given package name."
            }
        }
        "#;

        let b: ResponseBody<()> = serde_json::from_str(json).unwrap();
        match b {
            ResponseBody::Success(_) => panic!(),
            ResponseBody::Error(b) => {
                assert_eq!(b.error.errors.len(), 1);
                let err = b.error.errors.first().unwrap();
                assert_eq!(err.location_type, Some("parameter".to_owned()));
                assert_eq!(err.location, Some("packageName".to_owned()));
            }
        }
    }
}
