// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tx {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitForTxRequest {
    #[prost(uint32, tag="1")]
    pub max_depth: u32,
    #[prost(string, tag="2")]
    pub pattern: ::prost::alloc::string::String,
}
/// Encoded file descriptor set for the `utxorpc.watch.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xc5, 0x06, 0x0a, 0x1c, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2f, 0x77, 0x61, 0x74,
    0x63, 0x68, 0x2f, 0x76, 0x31, 0x2f, 0x77, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x10, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x77, 0x61, 0x74, 0x63, 0x68,
    0x2e, 0x76, 0x31, 0x22, 0x04, 0x0a, 0x02, 0x54, 0x78, 0x22, 0x49, 0x0a, 0x10, 0x57, 0x61, 0x69,
    0x74, 0x46, 0x6f, 0x72, 0x54, 0x78, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1b, 0x0a,
    0x09, 0x6d, 0x61, 0x78, 0x5f, 0x64, 0x65, 0x70, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x08, 0x6d, 0x61, 0x78, 0x44, 0x65, 0x70, 0x74, 0x68, 0x12, 0x18, 0x0a, 0x07, 0x70, 0x61,
    0x74, 0x74, 0x65, 0x72, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x70, 0x61, 0x74,
    0x74, 0x65, 0x72, 0x6e, 0x32, 0x98, 0x01, 0x0a, 0x05, 0x57, 0x61, 0x74, 0x63, 0x68, 0x12, 0x47,
    0x0a, 0x09, 0x57, 0x61, 0x69, 0x74, 0x46, 0x6f, 0x72, 0x54, 0x78, 0x12, 0x22, 0x2e, 0x75, 0x74,
    0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x77, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x76, 0x31, 0x2e, 0x57,
    0x61, 0x69, 0x74, 0x46, 0x6f, 0x72, 0x54, 0x78, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a,
    0x14, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x77, 0x61, 0x74, 0x63, 0x68, 0x2e,
    0x76, 0x31, 0x2e, 0x54, 0x78, 0x30, 0x01, 0x12, 0x46, 0x0a, 0x08, 0x46, 0x6f, 0x6c, 0x6c, 0x6f,
    0x77, 0x54, 0x78, 0x12, 0x22, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e, 0x77, 0x61,
    0x74, 0x63, 0x68, 0x2e, 0x76, 0x31, 0x2e, 0x57, 0x61, 0x69, 0x74, 0x46, 0x6f, 0x72, 0x54, 0x78,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x14, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70,
    0x63, 0x2e, 0x77, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x76, 0x31, 0x2e, 0x54, 0x78, 0x30, 0x01, 0x42,
    0xbf, 0x01, 0x0a, 0x14, 0x63, 0x6f, 0x6d, 0x2e, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x2e,
    0x77, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x76, 0x31, 0x42, 0x0a, 0x57, 0x61, 0x74, 0x63, 0x68, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x39, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63,
    0x6f, 0x6d, 0x2f, 0x62, 0x75, 0x66, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2f, 0x62, 0x75, 0x66, 0x2d,
    0x74, 0x6f, 0x75, 0x72, 0x2f, 0x67, 0x65, 0x6e, 0x2f, 0x75, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63,
    0x2f, 0x77, 0x61, 0x74, 0x63, 0x68, 0x2f, 0x76, 0x31, 0x3b, 0x77, 0x61, 0x74, 0x63, 0x68, 0x76,
    0x31, 0xa2, 0x02, 0x03, 0x55, 0x57, 0x58, 0xaa, 0x02, 0x10, 0x55, 0x74, 0x78, 0x6f, 0x72, 0x70,
    0x63, 0x2e, 0x57, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x10, 0x55, 0x74, 0x78,
    0x6f, 0x72, 0x70, 0x63, 0x5c, 0x57, 0x61, 0x74, 0x63, 0x68, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x1c,
    0x55, 0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x5c, 0x57, 0x61, 0x74, 0x63, 0x68, 0x5c, 0x56, 0x31,
    0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x12, 0x55,
    0x74, 0x78, 0x6f, 0x72, 0x70, 0x63, 0x3a, 0x3a, 0x57, 0x61, 0x74, 0x63, 0x68, 0x3a, 0x3a, 0x56,
    0x31, 0x4a, 0xdc, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x10, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x19,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x0a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x08, 0x00, 0x0b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x08, 0x08, 0x18,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x09, 0x04, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x09, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12,
    0x03, 0x0a, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0a,
    0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x0b, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x15, 0x16, 0x0a, 0x0a,
    0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x0d, 0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00,
    0x01, 0x12, 0x03, 0x0d, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x0e, 0x04, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x08,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0e, 0x12, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0e, 0x2d, 0x33, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x34, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x06,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x04, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x0f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x0f, 0x11, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0f,
    0x2c, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0f, 0x33, 0x35,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("utxorpc.watch.v1.tonic.rs");
// @@protoc_insertion_point(module)