// response_post_method_mod.rs

use crate::rpc_methods_mod::*;
use cargo_crev_reviews_common::*;
use std::str::FromStr;
use unwrap::unwrap;

pub fn parse_post_data_and_match_method(body: &Vec<u8>) -> anyhow::Result<String> {
    let p: RpcRequest = unwrap!(serde_json::from_slice(body));
    match_request_method_and_call_function(p.request_method.as_str(), p.request_data)
}

fn match_request_method_and_call_function(request_method: &str, request_data: serde_json::Value) -> anyhow::Result<String> {
    let request_enum = RequestMethod::from_str(request_method);
    match request_enum {
        Ok(request_enum) => match request_enum {
            RequestMethod::RpcReviewList => rpc_reviews_list(request_data),
            RequestMethod::RpcReviewNew => rpc_review_new(request_data),
            RequestMethod::RpcReviewSave => rpc_review_save(request_data),
            RequestMethod::RpcReviewEdit => rpc_review_edit(request_data),
            RequestMethod::RpcReviewPublish => rpc_review_publish(request_data),
        },
        Err(_err) => Err(anyhow::anyhow!("unknown server method = {}",)),
    }
}

// the first parameter is the Serialize trait and not a struct
pub fn return_rpc_response<T>(response_method: ResponseMethod, data: T, response_html: &str) -> String
where
    T: serde::Serialize,
{
    let response_method: &'static str = response_method.into();
    let data = unwrap!(serde_json::to_value(data));
    let r = RpcResponse {
        response_method: response_method.to_string(),
        response_data: data,
        response_html: response_html.to_string(),
    };
    let body = unwrap!(serde_json::to_string(&r));
    body
}