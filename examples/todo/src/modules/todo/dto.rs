// use crate::modules::todo::ErrorResponse;
// use actix_web::web::{Json, Path};
// use apistos::actix::CreatedJson;

// #[allow(non_camel_case_types)]
// #[doc(hidden)]
// struct __openapi_a;
// #[automatically_derived]
// impl apistos::PathItemDefinition for __openapi_a {
//   fn is_visible() -> bool {
//     true
//   }
//   fn operation() -> apistos::paths::Operation {
//     use apistos::ApiComponent;
//     let mut operation_builder = apistos::paths::Operation::default();
//     let mut body_requests: Vec<std::option::Option<apistos::paths::RequestBody>> = Vec::new();
//     let mut request_body = <Path<(String,)>>::request_body();
//     let consumes: Option<String> = None;
//     if let Some(consumes) = consumes {
//       request_body.as_mut().map(|t| {
//         t.content = t
//           .content
//           .values()
//           .map(|v| (consumes.clone(), v.clone()))
//           .collect::<std::collections::BTreeMap<String, apistos::paths::MediaType>>()
//       });
//     }
//     body_requests.push(request_body);
//     let mut request_body = <Json<()>>::request_body();
//     let consumes: Option<String> = None;
//     if let Some(consumes) = consumes {
//       request_body.as_mut().map(|t| {
//         t.content = t
//           .content
//           .values()
//           .map(|v| (consumes.clone(), v.clone()))
//           .collect::<std::collections::BTreeMap<String, apistos::paths::MediaType>>()
//       });
//     }
//     body_requests.push(request_body);
//     let body_requests: Vec<apistos::paths::RequestBody> = body_requests
//       .into_iter()
//       .flatten()
//       .collect::<Vec<apistos::paths::RequestBody>>();
//     for body_request in body_requests {
//       operation_builder.request_body =
//         Some(apistos::reference_or::ReferenceOr::Object(body_request));
//     }
//     let mut parameters = Vec::new();
//     parameters.append(&mut <Path<(String,)>>::parameters());
//     parameters.append(&mut <Json<()>>::parameters());
//     if !parameters.is_empty() {
//       operation_builder.parameters = parameters
//         .into_iter()
//         .map(apistos::reference_or::ReferenceOr::Object)
//         .collect();
//     }
//     if let Some(responses) = <apistos::actix::ResponseWrapper<
//       Box<
//         dyn std::future::Future<Output = Result<CreatedJson<()>, ErrorResponse>>
//           + std::marker::Unpin,
//       >,
//       __openapi_a,
//     >>::responses(None)
//     {
//       operation_builder.responses = responses;
//     }
//     let securities = {
//       let mut needs_empty_security = false;
//       let mut securities = Vec::new();
//       let needed_scopes: std::collections::BTreeMap<String, Vec<String>> = Default::default();
//       if !<Path<(String,)>>::required() {
//         needs_empty_security = true;
//       }
//       let mut security_requirements = Vec::new();
//       if let Some(security_requirement_name) = <Path<(String,)>>::security_requirement_name() {
//         let scopes: Vec<String> = needed_scopes
//           .get(&security_requirement_name)
//           .cloned()
//           .unwrap_or_default();
//         security_requirements.push(apistos::security::SecurityRequirement {
//           requirements: std::collections::BTreeMap::from_iter(<[_]>::into_vec(Box::new([(
//             security_requirement_name,
//             scopes,
//           )]))),
//         });
//       }
//       securities.append(&mut security_requirements);
//       if !<Json<()>>::required() {
//         needs_empty_security = true;
//       }
//       let mut security_requirements = Vec::new();
//       if let Some(security_requirement_name) = <Json<()>>::security_requirement_name() {
//         let scopes: Vec<String> = needed_scopes
//           .get(&security_requirement_name)
//           .cloned()
//           .unwrap_or_default();
//         security_requirements.push(apistos::security::SecurityRequirement {
//           requirements: std::collections::BTreeMap::from_iter(
//             (<[_]>::into_vec(Box::new([(security_requirement_name, scopes)]))),
//           ),
//         });
//       }
//       securities.append(&mut security_requirements);
//       if needs_empty_security {
//         securities.push(apistos::security::SecurityRequirement::default());
//       }
//       securities
//     };
//     if !securities.is_empty() {
//       operation_builder.security = securities;
//     }
//     operation_builder.operation_id = None;
//     operation_builder.deprecated = Some(false);
//     operation_builder
//   }
//   fn components() -> Vec<apistos::components::Components> {
//     use apistos::ApiComponent;
//     let mut component_builder = apistos::components::Components::default();
//     for (name, security) in <Path<(String,)>>::securities() {
//       component_builder
//         .security_schemes
//         .insert(name, apistos::reference_or::ReferenceOr::Object(security));
//     }
//     for (name, security) in <Json<()>>::securities() {
//       component_builder
//         .security_schemes
//         .insert(name, apistos::reference_or::ReferenceOr::Object(security));
//     }
//     let mut schemas = (Vec::new());
//     schemas.push(<Path<(String,)>>::schema());
//     schemas.push(<Json<()>>::schema());
//     schemas.push(<apistos::actix::ResponseWrapper<
//       Box<
//         dyn std::future::Future<Output = Result<CreatedJson<()>, ErrorResponse>>
//           + std::marker::Unpin,
//       >,
//       __openapi_a,
//     >>::schema());
//     let mut schemas = schemas
//       .into_iter()
//       .flatten()
//       .collect::<Vec<(String, apistos::reference_or::ReferenceOr<apistos::Schema>)>>();
//     schemas.append(&mut <Path<(String,)>>::child_schemas());
//     schemas.append(&mut <Json<()>>::child_schemas());
//     schemas.append(&mut <apistos::actix::ResponseWrapper<
//       Box<
//         dyn std::future::Future<Output = Result<CreatedJson<()>, ErrorResponse>>
//           + std::marker::Unpin,
//       >,
//       __openapi_a,
//     >>::child_schemas());
//     let error_schemas = <apistos::actix::ResponseWrapper<
//       Box<
//         dyn std::future::Future<Output = Result<CreatedJson<()>, ErrorResponse>>
//           + std::marker::Unpin,
//       >,
//       __openapi_a,
//     >>::error_schemas();
//     component_builder.schemas = std::collections::BTreeMap::from_iter(schemas);
//     <[_]>::into_vec(Box::new([component_builder]))
//   }
// }
// fn a(
//   a: Path<(String,)>,
//   json: Json<()>,
// ) -> impl std::future::Future<Output = Result<CreatedJson<()>, ErrorResponse>>
//      + apistos::PathItemDefinition {
//   let inner = (move || async move { Ok(CreatedJson(())) })();
//   apistos::actix::ResponseWrapper {
//     inner,
//     path_item: __openapi_a,
//   }
// }
