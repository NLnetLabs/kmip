use serde_derive::Serialize;

use super::common::{
    Attribute, CommonTemplateAttribute, ObjectType, Operation, PrivateKeyTemplateAttribute, PublicKeyTemplateAttribute,
    TemplateAttribute, UniqueIdentifier,
};

// KMIP spec 1.0 section 2.1.2 Credential
// See: https://docs.oasis-open.org/kmip/spec/v1.0/os/kmip-spec-1.0-os.html#_Toc262581156
#[derive(Serialize)]
#[serde(rename = "0x420023")]
pub struct Credential(pub CredentialType, pub CredentialValue);

#[derive(Serialize)]
#[serde(rename = "0x420024")]
#[non_exhaustive]
pub enum CredentialType {
    #[serde(rename = "0x00000001")]
    UsernameAndPassword,
}

#[derive(Serialize)]
#[serde(rename = "0x420025")]
#[non_exhaustive]
pub enum CredentialValue {
    UsernameAndPassword(UsernameAndPasswordCredential),
}

#[derive(Serialize)]
pub struct UsernameAndPasswordCredential(pub Username, pub Option<Password>);

#[derive(Serialize)]
#[serde(rename = "0x420099")]
pub struct Username(pub String);

#[derive(Serialize)]
#[serde(rename = "0x4200A1")]
pub struct Password(pub String);

// KMIP spec 1.0 section 6.1 Protocol Version
// See: https://docs.oasis-open.org/kmip/spec/v1.0/os/kmip-spec-1.0-os.html#_Toc262581239
#[derive(Serialize)]
#[serde(rename = "0x420069")]
pub struct ProtocolVersion(pub ProtocolVersionMajor, pub ProtocolVersionMinor);

#[derive(Serialize)]
#[serde(rename = "0x42006A")]
pub struct ProtocolVersionMajor(pub i32);

#[derive(Serialize)]
#[serde(rename = "0x42006B")]
pub struct ProtocolVersionMinor(pub i32);

// KMIP spec 1.0 section 6.3 Maximum Response Size
// See: https://docs.oasis-open.org/kmip/spec/v1.0/os/kmip-spec-1.0-os.html#_Toc262581241
#[derive(Serialize)]
#[serde(rename = "0x420050")]
pub struct MaximumResponseSize(pub i32);

// KMIP spec 1.0 section 6.6 Authentication
// See: https://docs.oasis-open.org/kmip/spec/v1.0/os/kmip-spec-1.0-os.html#_Toc262581244
#[derive(Serialize)]
#[serde(rename = "0x42000C")]
pub struct Authentication(pub Credential);

// KMIP spec 1.0 section 6.14 Batch Count
// See: https://docs.oasis-open.org/kmip/spec/v1.0/os/kmip-spec-1.0-os.html#_Toc262581252
#[derive(Serialize)]
#[serde(rename = "0x42000D")]
pub struct BatchCount(pub i32);

// KMIP spec 1.0 section 6.15 Batch Item
// See: https://docs.oasis-open.org/kmip/spec/v1.0/os/kmip-spec-1.0-os.html#_Toc262581253
#[derive(Serialize)]
#[serde(rename = "0x42000F")]
pub struct BatchItem(pub Operation, pub RequestPayload);

// KMIP spec 1.0 section 7.1 Message Format
// See: https://docs.oasis-open.org/kmip/spec/v1.0/os/kmip-spec-1.0-os.html#_Toc262581256
#[derive(Serialize)]
#[serde(rename = "0x420078")]
pub struct RequestMessage(pub RequestHeader, pub Vec<BatchItem>);

// KMIP spec 1.0 section 7.2 Operations
// See: https://docs.oasis-open.org/kmip/spec/v1.0/os/kmip-spec-1.0-os.html#_Toc262581257
#[derive(Serialize)]
#[serde(rename = "0x420077")]
pub struct RequestHeader(
    pub ProtocolVersion,
    #[serde(skip_serializing_if = "Option::is_none")] pub Option<MaximumResponseSize>,
    #[serde(skip_serializing_if = "Option::is_none")] pub Option<Authentication>,
    pub BatchCount,
);

#[derive(Serialize)]
#[serde(rename = "0x420079")]
#[non_exhaustive]
pub enum RequestPayload {
    // KMIP spec 1.0 section 4.1 Create
    // See: https://docs.oasis-open.org/kmip/spec/v1.0/os/kmip-spec-1.0-os.html#_Toc262581209
    Create(ObjectType, TemplateAttribute),

    // KMIP spec 1.0 section 4.2 Create Key Pair
    // See: https://docs.oasis-open.org/kmip/spec/v1.0/os/kmip-spec-1.0-os.html#_Toc262581210
    CreateKeyPair(
        #[serde(skip_serializing_if = "Option::is_none")] Option<CommonTemplateAttribute>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<PrivateKeyTemplateAttribute>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<PublicKeyTemplateAttribute>,
    ),

    // KMIP spec 1.0 section 4.8 Locate
    // See: https://docs.oasis-open.org/kmip/spec/v1.0/os/kmip-spec-1.0-os.html#_Toc262581216
    Locate(Vec<Attribute>), // TODO: Add MaximumItems and StorageStatusMask optional request payload fields

    // KMIP spec 1.0 section 4.20 Destroy
    // See: https://docs.oasis-open.org/kmip/spec/v1.0/os/kmip-spec-1.0-os.html#_Toc262581228
    Destroy(#[serde(skip_serializing_if = "Option::is_none")] Option<UniqueIdentifier>),

    // KMIP spec 1.0 section 4.24 Query
    // See: https://docs.oasis-open.org/kmip/spec/v1.0/os/kmip-spec-1.0-os.html#_Toc262581232
    Query(Vec<QueryFunction>),

    // KMIP spec 1.1 section 4.26 Discover Versions
    // See: https://docs.oasis-open.org/kmip/spec/v1.1/cs01/kmip-spec-v1.1-cs01.html#_Toc332787652
    DiscoverVersions(Vec<ProtocolVersion>),

    // KMIP spec 1.2 section 4.31 Sign
    // See: https://docs.oasis-open.org/kmip/spec/v1.2/os/kmip-spec-v1.2-os.html#_Toc409613558
    Sign,
}

// KMIP spec 1.0 section 9.1.3.2.23 Query Function Enumeration
// See: https://docs.oasis-open.org/kmip/spec/v1.0/os/kmip-spec-1.0-os.html#_Ref242030554
#[derive(Serialize)]
#[serde(rename = "0x420074")]
#[non_exhaustive]
pub enum QueryFunction {
    #[serde(rename = "0x00000001")]
    QueryOperations,

    #[serde(rename = "0x00000002")]
    QueryObjects,

    #[serde(rename = "0x00000003")]
    QueryServerInformation,
    // Note: This set of enum variants is deliberately limited to those that we currently support.
}
