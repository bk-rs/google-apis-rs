use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ResponseBody<Resource> {
    Success(Resource),
    Error(ErrorResponseBody),
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

    use std::io;

    #[derive(Deserialize)]
    struct Foo {
        pub name: String,
    }

    #[test]
    fn success() -> io::Result<()> {
        let json = r#"
        {
            "name": "bar"
        }
        "#;

        let b: ResponseBody<Foo> = serde_json::from_str(json)?;
        match b {
            ResponseBody::Success(foo) => {
                assert_eq!(foo.name, "bar");
            }
            ResponseBody::Error(_) => assert!(false),
        }

        Ok(())
    }

    #[test]
    fn error_code_is_400() -> io::Result<()> {
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

        let b: ResponseBody<()> = serde_json::from_str(json)?;
        match b {
            ResponseBody::Success(_) => assert!(false),
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

        Ok(())
    }

    #[test]
    fn error_code_is_404() -> io::Result<()> {
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

        let b: ResponseBody<()> = serde_json::from_str(json)?;
        match b {
            ResponseBody::Success(_) => assert!(false),
            ResponseBody::Error(b) => {
                assert_eq!(b.error.errors.len(), 1);
                let err = b.error.errors.first().unwrap();
                assert_eq!(err.location_type, Some("parameter".to_owned()));
                assert_eq!(err.location, Some("packageName".to_owned()));
            }
        }

        Ok(())
    }
}
