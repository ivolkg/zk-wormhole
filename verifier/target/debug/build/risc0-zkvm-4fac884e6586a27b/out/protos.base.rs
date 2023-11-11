#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SemanticVersion {
    #[prost(uint64, tag = "1")]
    pub major: u64,
    #[prost(uint64, tag = "2")]
    pub minor: u64,
    #[prost(uint64, tag = "3")]
    pub patch: u64,
    #[prost(string, tag = "4")]
    pub pre: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub build: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompatVersion {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExitCode {
    #[prost(oneof = "exit_code::Kind", tags = "1, 2, 3, 4, 5")]
    pub kind: ::core::option::Option<exit_code::Kind>,
}
/// Nested message and enum types in `ExitCode`.
pub mod exit_code {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(uint32, tag = "1")]
        Halted(u32),
        #[prost(uint32, tag = "2")]
        Paused(u32),
        #[prost(message, tag = "3")]
        SystemSplit(()),
        #[prost(message, tag = "4")]
        SessionLimit(()),
        #[prost(message, tag = "5")]
        Fault(()),
    }
}
