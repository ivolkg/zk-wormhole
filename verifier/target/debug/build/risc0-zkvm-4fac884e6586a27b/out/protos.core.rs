#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemoryImage {
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<super::base::CompatVersion>,
    #[prost(message, optional, tag = "2")]
    pub info: ::core::option::Option<PageTableInfo>,
    #[prost(uint32, tag = "3")]
    pub pc: u32,
    #[prost(message, repeated, tag = "4")]
    pub pages: ::prost::alloc::vec::Vec<PageEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageEntry {
    #[prost(uint32, tag = "1")]
    pub addr: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageTableInfo {
    #[prost(uint32, tag = "1")]
    pub page_size: u32,
    #[prost(uint32, tag = "2")]
    pub page_table_addr: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Receipt {
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<super::base::CompatVersion>,
    #[prost(message, optional, tag = "2")]
    pub inner: ::core::option::Option<InnerReceipt>,
    #[prost(bytes = "vec", tag = "3")]
    pub journal: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InnerReceipt {
    #[prost(oneof = "inner_receipt::Kind", tags = "1, 2, 3")]
    pub kind: ::core::option::Option<inner_receipt::Kind>,
}
/// Nested message and enum types in `InnerReceipt`.
pub mod inner_receipt {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        Flat(super::SegmentReceipts),
        #[prost(message, tag = "2")]
        Succinct(super::SuccinctReceipt),
        #[prost(message, tag = "3")]
        Fake(super::FakeReceipt),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentReceipts {
    #[prost(message, repeated, tag = "1")]
    pub inner: ::prost::alloc::vec::Vec<SegmentReceipt>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentReceipt {
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<super::base::CompatVersion>,
    #[prost(bytes = "vec", tag = "2")]
    pub seal: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "3")]
    pub index: u32,
    #[prost(string, tag = "4")]
    pub hashfn: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuccinctReceipt {
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<super::base::CompatVersion>,
    #[prost(bytes = "vec", tag = "2")]
    pub seal: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub control_id: ::core::option::Option<Digest>,
    #[prost(message, optional, tag = "4")]
    pub meta: ::core::option::Option<ReceiptMetadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiptMetadata {
    #[prost(message, optional, tag = "1")]
    pub pre: ::core::option::Option<SystemState>,
    #[prost(message, optional, tag = "2")]
    pub post: ::core::option::Option<SystemState>,
    #[prost(message, optional, tag = "3")]
    pub exit_code: ::core::option::Option<super::base::ExitCode>,
    #[prost(message, optional, tag = "4")]
    pub input: ::core::option::Option<Digest>,
    #[prost(message, optional, tag = "5")]
    pub output: ::core::option::Option<Digest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemState {
    #[prost(uint32, tag = "1")]
    pub pc: u32,
    #[prost(message, optional, tag = "2")]
    pub merkle_root: ::core::option::Option<Digest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FakeReceipt {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Digest {
    #[prost(uint32, repeated, tag = "1")]
    pub words: ::prost::alloc::vec::Vec<u32>,
}
