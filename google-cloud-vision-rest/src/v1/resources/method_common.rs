use core::fmt;

use http_api_client_endpoint::{
    http::{Error as HttpError, StatusCode},
    Body, Response,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;

//
//
//
#[derive(Debug, Clone)]
pub enum MethodEndpointRet<T>
where
    T: fmt::Debug + Clone,
{
    Ok(T),
    Other((StatusCode, Result<MethodResponseErrorBody, Body>)),
}

//
//
//
#[derive(thiserror::Error, Debug)]
pub enum MethodEndpointError {
    #[error("SerRequestBodyFailed {0}")]
    SerRequestBodyFailed(SerdeJsonError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
}

//
//
//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MethodResponseErrorBody {
    pub error: MethodResponseErrorBodyError,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MethodResponseErrorBodyError {
    pub code: usize,
    pub message: String,
    pub status: String,
}

//
//
//
pub fn method_endpoint_parse_response<T>(
    response: Response<Body>,
) -> Result<MethodEndpointRet<T>, MethodEndpointError>
where
    T: fmt::Debug + Clone + DeserializeOwned,
{
    let status = response.status();
    match status {
        StatusCode::OK => {
            let ok_json = serde_json::from_slice::<T>(response.body())
                .map_err(MethodEndpointError::DeResponseBodyFailed)?;

            Ok(MethodEndpointRet::Ok(ok_json))
        }
        status => match serde_json::from_slice::<MethodResponseErrorBody>(response.body()) {
            Ok(err_json) => Ok(MethodEndpointRet::Other((status, Ok(err_json)))),
            Err(_) => Ok(MethodEndpointRet::Other((
                status,
                Err(response.body().to_owned()),
            ))),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de_response_error_body() {
        match serde_json::from_str::<MethodResponseErrorBody>(include_str!(
            "../../../tests/response_body_files/err__401.json"
        )) {
            Ok(body) => {
                assert_eq!(body.error.code, 401);
            }
            Err(err) => panic!("{}", err),
        }
    }
}
