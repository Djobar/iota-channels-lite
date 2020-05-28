//!
//! Response Signed
//!
use serde;
use serde::Serialize;

///
/// Object returned by write_signed
///
#[derive(Serialize)]
pub struct ResponseSigned {
    /// Signed message tag
    ///
    pub signed_message_tag: String,
    /// Change key tag
    ///
    pub change_key_tag: Option<String>,
}
