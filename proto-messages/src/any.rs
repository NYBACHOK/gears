use serde::{Deserialize, Serialize};
use ibc_proto::google::protobuf::Any as GoogleAny;
use ibc::primitives::proto::Any as PrimitiveAny;


#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct Any {
    /// A URL/resource name that uniquely identifies the type of the serialized
    /// protocol buffer message. This string must contain at least
    /// one "/" character. The last segment of the URL's path must represent
    /// the fully qualified name of the type (as in
    /// `path/google.protobuf.Duration`). The name should be in a canonical form
    /// (e.g., leading "." is not accepted).
    ///
    /// In practice, teams usually precompile into the binary all types that they
    /// expect it to use in the context of Any. However, for URLs which use the
    /// scheme `http`, `https`, or no scheme, one can optionally set up a type
    /// server that maps type URLs to message definitions as follows:
    ///
    /// * If no scheme is provided, `https` is assumed.
    /// * An HTTP GET on the URL must yield a [google.protobuf.Type][]
    ///    value in binary format, or produce an error.
    /// * Applications are allowed to cache lookup results based on the
    ///    URL, or have them precompiled into a binary to avoid any
    ///    lookup. Therefore, binary compatibility needs to be preserved
    ///    on changes to types. (Use versioned type names to manage
    ///    breaking changes.)
    ///
    /// Note: this functionality is not currently available in the official
    /// protobuf release, and it is not used for type URLs beginning with
    /// type.googleapis.com. As of May 2023, there are no widely used type server
    /// implementations and no plans to implement one.
    ///
    /// Schemes other than `http`, `https` (or the empty scheme) might be
    /// used with implementation specific semantics.
    ///
    #[prost(string, tag = "1")]
    pub type_url: ::prost::alloc::string::String,
    /// Must be a valid serialized protocol buffer of the above specified type.
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}

impl From<GoogleAny> for Any
{
    fn from(value: GoogleAny) -> Self {
        let GoogleAny { type_url, value }   = value;

        Self { type_url, value }
    }
}

impl From<PrimitiveAny> for Any
{
    fn from(value: PrimitiveAny) -> Self {
        let PrimitiveAny { type_url, value }   = value;

        Self { type_url, value }
    }
}

impl From<Any> for GoogleAny
{
    fn from(value: Any) -> Self {
        let Any { type_url, value } = value;

        Self { type_url, value }
    }
}

impl From<Any> for PrimitiveAny
{
    fn from(value: Any) -> Self {
        let Any { type_url, value } = value;

        Self { type_url, value }
    }
}