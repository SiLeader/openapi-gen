use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, multispace0};
use nom::character::streaming::multispace1;
use nom::combinator::opt;
use nom::error::ErrorKind;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::delimited;
use nom::IResult;

use crate::data::{HttpMethod, Operation, OperationContent, Parameters, PathContent};
use crate::parser::identifier::identifier;
use crate::parser::parameter_definition::parameters_definition;
use crate::parser::request_body_definition::shorthand_request_body_definition;
use crate::parser::response_definition::shorthand_response_definition;
use crate::{ParameterType, ReferenceOr, RequestBodyContent, ResponseContent};

pub(super) fn path_content(s: &str) -> IResult<&str, PathContent> {
    let (s, parameters) = opt(path_operation_parameter)(s)?;
    let (s, operations) = separated_list1(multispace1, operation)(s)?;

    let mut contents = PathContent {
        parameters: parameters.unwrap_or_default(),
        ..Default::default()
    };

    for op in operations {
        match op.method {
            HttpMethod::Default => {}
            HttpMethod::Get => {
                contents.get = Some(op);
            }
            HttpMethod::Post => {
                contents.post = Some(op);
            }
            HttpMethod::Put => {
                contents.put = Some(op);
            }
            HttpMethod::Delete => {
                contents.delete = Some(op);
            }
            HttpMethod::Options => {
                contents.options = Some(op);
            }
            HttpMethod::Head => {
                contents.head = Some(op);
            }
            HttpMethod::Trace => {
                contents.trace = Some(op);
            }
            HttpMethod::Patch => {
                contents.patch = Some(op);
            }
        }
    }

    Ok((s, contents))
}

fn path_operation_parameter(s: &str) -> IResult<&str, Parameters> {
    let (s, ps) = separated_list1(multispace1, operation_parameters)(s)?;
    let (s, _) = multispace0(s)?;

    let mut param = Parameters::default();

    for p in ps {
        match p {
            ParameterWithType::Queries(p) => param.parameters.extend(p.parameters),
            ParameterWithType::Headers(p) => param.parameters.extend(p.parameters),
            ParameterWithType::PathParameters(p) => param.parameters.extend(p.parameters),
            ParameterWithType::Cookies(p) => param.parameters.extend(p.parameters),
            ParameterWithType::RequestBody(_) => {
                return Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::Alt)));
            }
        }
    }

    Ok((s, param))
}

fn operation(s: &str) -> IResult<&str, Operation> {
    let (s, method) = method(s)?;
    let (s, _) = multispace1(s)?;
    let (s, name) = identifier(s)?;
    let (s, (parameters, request_body)) = arguments(s)?;
    let (s, _) = multispace0(s)?;
    let (s, content) = delimited(
        char('{'),
        delimited(multispace0, content, multispace0),
        char('}'),
    )(s)?;

    Ok((
        s,
        Operation {
            name,
            method,
            parameters,
            content,
            request_body,
        },
    ))
}

fn method(s: &str) -> IResult<&str, HttpMethod> {
    let (s, ms) = alt((
        tag("get"),
        tag("post"),
        tag("put"),
        tag("delete"),
        tag("options"),
        tag("head"),
        tag("trace"),
        tag("default"),
    ))(s)?;
    let method = match ms {
        "get" => HttpMethod::Get,
        "post" => HttpMethod::Post,
        "put" => HttpMethod::Put,
        "delete" => HttpMethod::Delete,
        "options" => HttpMethod::Options,
        "head" => HttpMethod::Head,
        "trace" => HttpMethod::Trace,
        _ => HttpMethod::Default,
    };

    Ok((s, method))
}

fn arguments(s: &str) -> IResult<&str, (Parameters, Option<ReferenceOr<RequestBodyContent>>)> {
    delimited(
        multispace0,
        delimited(
            char('('),
            delimited(multispace0, arguments_impl, multispace0),
            char(')'),
        ),
        multispace0,
    )(s)
}

fn arguments_impl(s: &str) -> IResult<&str, (Parameters, Option<ReferenceOr<RequestBodyContent>>)> {
    let (s, params) = separated_list0(char(','), operation_parameters)(s)?;
    let (s, _) = opt(char(','))(s)?;

    let mut parameters = Parameters::default();
    let mut request_body = None;
    for p in params {
        match p {
            ParameterWithType::Queries(q) => parameters.parameters.extend(q.parameters),
            ParameterWithType::Headers(h) => parameters.parameters.extend(h.parameters),
            ParameterWithType::PathParameters(p) => parameters.parameters.extend(p.parameters),
            ParameterWithType::Cookies(c) => parameters.parameters.extend(c.parameters),
            ParameterWithType::RequestBody(r) => {
                request_body = Some(r);
            }
        }
    }

    Ok((s, (parameters, request_body)))
}

enum ParameterWithType {
    Queries(Parameters),
    Headers(Parameters),
    PathParameters(Parameters),
    Cookies(Parameters),
    RequestBody(ReferenceOr<RequestBodyContent>),
}

fn operation_parameters(s: &str) -> IResult<&str, ParameterWithType> {
    let (s, ty) = alt((
        tag("queries"),
        tag("headers"),
        tag("pathParameters"),
        tag("cookie"),
        tag("body"),
    ))(s)?;
    match ty {
        "queries" | "headers" | "pathParameters" | "cookie" => {
            let (s, _) = delimited(multispace0, char('='), multispace0)(s)?;
            let ty = match ty {
                "queries" => ParameterType::Query,
                "headers" => ParameterType::Header,
                "pathParameters" => ParameterType::Path,
                "cookie" => ParameterType::Cookie,
                _ => return Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::Tag))),
            };
            let (s, parameters) = parameters_definition(ty.clone())(s)?;

            let params = match ty {
                ParameterType::Query => ParameterWithType::Queries(parameters),
                ParameterType::Header => ParameterWithType::Headers(parameters),
                ParameterType::Path => ParameterWithType::PathParameters(parameters),
                ParameterType::Cookie => ParameterWithType::Cookies(parameters),
            };
            Ok((s, params))
        }
        "body" => {
            let (s, _) = delimited(multispace0, char('='), multispace0)(s)?;
            let (s, content) = operation_request_body(s)?;

            Ok((s, ParameterWithType::RequestBody(content)))
        }
        _ => Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::Tag))),
    }
}

fn operation_request_body(s: &str) -> IResult<&str, ReferenceOr<RequestBodyContent>> {
    alt((operation_request_body_sh, operation_request_body_ref))(s)
}

fn operation_request_body_ref(s: &str) -> IResult<&str, ReferenceOr<RequestBodyContent>> {
    let (s, ident) = identifier(s)?;
    Ok((s, ReferenceOr::Ref(ident.to_string())))
}

fn operation_request_body_sh(s: &str) -> IResult<&str, ReferenceOr<RequestBodyContent>> {
    let (s, content) = shorthand_request_body_definition(s)?;
    Ok((s, ReferenceOr::Value(content)))
}

fn content(s: &str) -> IResult<&str, OperationContent> {
    let (s, contents) = separated_list1(multispace1, return_statement)(s)?;
    let mut content = OperationContent::default();
    for (code, res) in contents {
        match code {
            None => {
                content.default = Some(res);
            }
            Some(code) => {
                content.response.insert(code, res);
            }
        }
    }

    Ok((s, content))
}

fn return_statement(s: &str) -> IResult<&str, (Option<String>, ReferenceOr<ResponseContent>)> {
    let (s, _) = tag("return")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, code) = alt((tag("default"), digit1))(s)?;
    if code != "default" && code.len() != 3 {
        return Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::Digit)));
    }
    let (s, _) = multispace1(s)?;
    let (s, res) = operation_response(s)?;

    let code = if code == "default" {
        None
    } else {
        Some(code.to_string())
    };

    Ok((s, (code, res)))
}

fn operation_response(s: &str) -> IResult<&str, ReferenceOr<ResponseContent>> {
    alt((operation_response_sh, operation_response_ref))(s)
}

fn operation_response_ref(s: &str) -> IResult<&str, ReferenceOr<ResponseContent>> {
    let (s, r) = identifier(s)?;
    Ok((s, ReferenceOr::Ref(r)))
}

fn operation_response_sh(s: &str) -> IResult<&str, ReferenceOr<ResponseContent>> {
    let (s, r) = shorthand_response_definition(s)?;
    Ok((s, ReferenceOr::Value(r)))
}
