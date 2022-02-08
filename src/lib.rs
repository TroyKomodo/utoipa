//! Want to have your API documented with OpenAPI? But you dont want to see the
//! trouble with manual yaml or json tweaking? Would like it to be so easy that it would almost
//! be like utopic? Don't worry utoipa is just there to fill this gap. It aims to do if not all then
//! the most of heavy lifting for you enabling you to focus writing the actual API logic instead of
//! documentation. It aims to be *minimal*, *simple* and *fast*. It uses simple proc macros which
//! you can use to annotate your code to have items documented.
//!
//! Utoipa crate provides autogenerated OpenAPI documentation for Rust REST APIs. It treats
//! code first appoach as a first class citizen and simplifies API documentation by providing
//! simple macros for generating the documentation from your code.
//!
//! It also contains Rust types of OpenAPI spec allowing you to write the OpenAPI spec only using
//! Rust if autogeneration is not your flavor or does not fit your purpose.
//!
//! Long term goal of the library is to be the place to go when OpenAPI documentation is needed in Rust
//! codebase.
//!
//! # What's up with the word play?
//!
//! The name comes from words `utopic` and `api` where `uto` is the first three letters of utopic
//! and the `ipa` is api reversed.
//!
//! # Features
//!
//! * **default** Default enabled features are **json**.
//! * **json** Enables some advanced features for openapi which otherwise are not available. Thus is
//!   enabled by default.
//! * **swagger_ui** Enables the embedded Swagger UI to view openapi api documentation.
//! * **actix-web** Enables actix-web integration with pre-configured SwaggerUI service factory allowing
//!   users to use the Swagger UI without a hazzle.
//! * **actix_extras** Enhances actix-web intgration with being able to parse some documentation
//!   from actix web macro attributes and types.
//! * **debug** Add extra traits such as debug traits to openapi definitions and elsewhere.
//!
//! # Install
//!
//! Add minimal dependency declaration to Cargo.toml.
//! ```text
//! [dependencies]
//! utoipa = "0.1.beta.0"  
//! ```
//!
//! To enable more features such as use of swagger together with actix-web framework you could define the
//! dependency as follows.
//! ```text
//! [dependencies]
//! utoipa = { version = "0.1.beta.0", features = ["swagger_ui", "actix-web", "actix_extras"] }
//! ```
//!
//! # Examples
//!
//! Create a struct or it could be an enum also. Add `Component` derive macro to it so it can be registered
//! as a component in openapi schema.
//! ```rust
//! use utoipa::Component;
//! #[derive(Component)]
//! struct Pet {
//!    id: u64,
//!    name: String,
//!    age: Option<i32>,
//! }
//! ```
//!
//! Create an handler that would handle your business logic and add `path` proc attribute macro over it.
//! ```compile_fail
//! mod pet_api {
//!     /// Get pet by id
//!     ///
//!     /// Get pet from database by pet id  
//!     #[utoipa::path(
//!         get,
//!         path = "/pets/{id}"
//!         responses = [
//!             (status = 200, description = "Pet found succesfully", body = Pet),
//!             (status = 404, description = "Pet was not found", body = NotFoundError)
//!         ],
//!         params = [
//!             ("id" = u64, path, description = "Pet database id to get Pet for"),
//!         ]
//!     )]
//!     async fn get_pet_by_id(pet_id: u64) -> Pet {
//!         Pet {
//!             id: pet_id,
//!             age: None,
//!             name: "lightning".to_string(),
//!         }
//!     }
//! }
//! ```
//!
//! Tie the component and the above api to the openapi schema with following `OpenApi` derive proc macro.
//! ```compile_fail
//! use utoipa::OpenApi;
//! use crate::Pet;
//!
//! #[derive(OpenApi)]
//! #[openapi(handlers = [pet_api::get_pet_by_id], components = [Pet])]
//! struct ApiDoc;
//!
//! println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
//! ```
//!
//! If you have *swagger_ui* and the *actix-web* features enabled you can display the openapi documentation
//! as easily as follows:
//! ```compile_fail
//! HttpServer::new(move || {
//!         App::new()
//!             .service(
//!                 SwaggerUi::new("/swagger-ui/{_:.*}")
//!                     .with_url("/api-doc/openapi.json", ApiDoc::openapi()),
//!             )
//!     })
//!     .bind(format!("{}:{}", Ipv4Addr::UNSPECIFIED, 8989))?
//!     .run()
//!     .await
//! ```
//!
//! See more details in [`swagger_ui`] module. You can also browse to
//! [examples](https://github.com/juhaku/utoipa/tree/master/examples) for more comprehensinve examples.

/// Openapi module contains Rust implementation of Openapi Spec V3
pub mod openapi;
#[cfg(feature = "swagger_ui")]
/// Swagger UI module contains embedded Swagger UI and extensions for actix-web
pub mod swagger_ui;

pub use utoipa_gen::*;

/// OpenApi trait for implementing Openapi specification in Rust.
///
/// This trait is derivable and can be used with `#[derive]` attribute. The derived implementation
/// will use Cargo provided environment variables to implement the default information. For a details of
/// `#[derive(Component)]` refer to [derive documentation][derive].
///
/// # Examples
///
/// Below is derived example of `OpenApi`.
/// ```rust
/// use utoipa::OpenApi;
/// #[derive(OpenApi)]
/// #[openapi(handlers = [])]
/// struct OpenApiDoc;
/// ```
///
/// This manual `OpenApi` trait implementation is approximately equal to the above derived one except the derive
/// implementation will by default use the Cargo environment variables to set defaults for *application name,
/// version, application description, license, author name & email*.
///
///```rust
/// struct OpenApiDoc;
///
/// impl utoipa::OpenApi for OpenApiDoc {
///     fn openapi() -> utoipa::openapi::OpenApi {
///         use utoipa::{Component, Path};
///         utoipa::openapi::OpenApi::new(
///             utoipa::openapi::Info::new("application name", "version")
///                 .with_description("application description")
///                 .with_license(utoipa::openapi::License::new("MIT"))
///                 .with_contact(
///                     utoipa::openapi::Contact::new()
///                         .with_name("author name")
///                         .with_email("author email"),
///                 ),
///             utoipa::openapi::path::Paths::new(),
///         )
///         .with_components(utoipa::openapi::Schema::new())
///     }
/// }
/// ```
/// [derive]: derive.OpenApi.html
pub trait OpenApi {
    fn openapi() -> openapi::OpenApi;
}

/// Component trait for implementing swagger specification schema object for a type.
///
/// This trait is deriveable and can be used with `[#derive]` attribute. For a details of
/// `#[derive(Component)]` refer to [derive documentation][derive].
///
/// [derive]: derive.Component.html
///
/// # Examples
///
/// Use `#[derive]` to implement `Component` trait.
/// ```rust
/// # use utoipa::Component;
/// #[derive(Component)]
/// #[component(example = json!({"name": "bob the cat", "id": 1}))]
/// struct Pet {
///     id: u64,
///     name: String,
///     age: Option<i32>,
/// }
/// ```
///
/// Following manual implementation is equal to above derive one.
/// ```rust
/// # struct Pet {
/// #     id: u64,
/// #     name: String,
/// #     age: Option<i32>,
/// # }
/// #
/// impl utoipa::Component for Pet {
///     fn component() -> utoipa::openapi::schema::Component {
///         use utoipa::openapi::ToArray;
///         utoipa::openapi::Object::new()
///             .with_property(
///                 "id",
///                 utoipa::openapi::Property::new(utoipa::openapi::ComponentType::Integer)
///                     .with_format(utoipa::openapi::ComponentFormat::Int64),
///             )
///             .with_required("id")
///             .with_property(
///                 "name",
///                 utoipa::openapi::Property::new(utoipa::openapi::ComponentType::String),
///             )
///             .with_required("name")
///             .with_property(
///                 "age",
///                 utoipa::openapi::Property::new(utoipa::openapi::ComponentType::Integer)
///                     .with_format(utoipa::openapi::ComponentFormat::Int32),
///             )
///             .with_example(serde_json::json!({
///               "name":"bob the cat","id":1
///             }))
///             .into()
///     }
/// }
/// ```
pub trait Component {
    fn component() -> openapi::schema::Component;
}

pub trait Path {
    fn path() -> &'static str;

    fn path_item() -> openapi::path::PathItem;
}

pub trait DefaultTag {
    fn tag() -> &'static str;
}

pub trait Tag {
    fn tag() -> &'static str;
}
