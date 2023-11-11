#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerRequest {
    #[prost(oneof = "server_request::Kind", tags = "1, 2, 3, 4, 5, 6")]
    pub kind: ::core::option::Option<server_request::Kind>,
}
/// Nested message and enum types in `ServerRequest`.
pub mod server_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        Execute(super::ExecuteRequest),
        #[prost(message, tag = "2")]
        Prove(super::ProveRequest),
        #[prost(message, tag = "3")]
        ProveSegment(super::ProveSegmentRequest),
        #[prost(message, tag = "4")]
        Lift(super::LiftRequest),
        #[prost(message, tag = "5")]
        Join(super::JoinRequest),
        #[prost(message, tag = "6")]
        IdentiyP254(super::IdentityP254Request),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<super::base::SemanticVersion>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloReply {
    #[prost(oneof = "hello_reply::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<hello_reply::Kind>,
}
/// Nested message and enum types in `HelloReply`.
pub mod hello_reply {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        Ok(super::HelloResult),
        #[prost(message, tag = "2")]
        Error(super::GenericError),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloResult {
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<super::base::SemanticVersion>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteRequest {
    #[prost(message, optional, tag = "1")]
    pub env: ::core::option::Option<ExecutorEnv>,
    #[prost(message, optional, tag = "2")]
    pub segments_out: ::core::option::Option<AssetRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProveRequest {
    #[prost(message, optional, tag = "1")]
    pub env: ::core::option::Option<ExecutorEnv>,
    #[prost(message, optional, tag = "2")]
    pub opts: ::core::option::Option<ProverOpts>,
    #[prost(message, optional, tag = "3")]
    pub receipt_out: ::core::option::Option<AssetRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProveSegmentRequest {
    #[prost(message, optional, tag = "1")]
    pub opts: ::core::option::Option<ProverOpts>,
    #[prost(message, optional, tag = "2")]
    pub segment: ::core::option::Option<Asset>,
    #[prost(message, optional, tag = "3")]
    pub receipt_out: ::core::option::Option<AssetRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProveSegmentReply {
    #[prost(oneof = "prove_segment_reply::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<prove_segment_reply::Kind>,
}
/// Nested message and enum types in `ProveSegmentReply`.
pub mod prove_segment_reply {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        Ok(super::ProveSegmentResult),
        #[prost(message, tag = "2")]
        Error(super::GenericError),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiftRequest {
    #[prost(message, optional, tag = "1")]
    pub opts: ::core::option::Option<ProverOpts>,
    #[prost(message, optional, tag = "2")]
    pub receipt: ::core::option::Option<Asset>,
    #[prost(message, optional, tag = "3")]
    pub receipt_out: ::core::option::Option<AssetRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiftReply {
    #[prost(oneof = "lift_reply::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<lift_reply::Kind>,
}
/// Nested message and enum types in `LiftReply`.
pub mod lift_reply {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        Ok(super::LiftResult),
        #[prost(message, tag = "2")]
        Error(super::GenericError),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiftResult {
    #[prost(message, optional, tag = "1")]
    pub receipt: ::core::option::Option<Asset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinRequest {
    #[prost(message, optional, tag = "1")]
    pub opts: ::core::option::Option<ProverOpts>,
    #[prost(message, optional, tag = "2")]
    pub left_receipt: ::core::option::Option<Asset>,
    #[prost(message, optional, tag = "3")]
    pub right_receipt: ::core::option::Option<Asset>,
    #[prost(message, optional, tag = "4")]
    pub receipt_out: ::core::option::Option<AssetRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinReply {
    #[prost(oneof = "join_reply::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<join_reply::Kind>,
}
/// Nested message and enum types in `JoinReply`.
pub mod join_reply {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        Ok(super::JoinResult),
        #[prost(message, tag = "2")]
        Error(super::GenericError),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinResult {
    #[prost(message, optional, tag = "1")]
    pub receipt: ::core::option::Option<Asset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityP254Request {
    #[prost(message, optional, tag = "1")]
    pub opts: ::core::option::Option<ProverOpts>,
    #[prost(message, optional, tag = "2")]
    pub receipt: ::core::option::Option<Asset>,
    #[prost(message, optional, tag = "3")]
    pub receipt_out: ::core::option::Option<AssetRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityP254Reply {
    #[prost(oneof = "identity_p254_reply::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<identity_p254_reply::Kind>,
}
/// Nested message and enum types in `IdentityP254Reply`.
pub mod identity_p254_reply {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        Ok(super::IdentityP254Result),
        #[prost(message, tag = "2")]
        Error(super::GenericError),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityP254Result {
    #[prost(message, optional, tag = "1")]
    pub receipt: ::core::option::Option<Asset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutorEnv {
    #[prost(message, optional, tag = "1")]
    pub binary: ::core::option::Option<Binary>,
    #[prost(map = "string, string", tag = "2")]
    pub env_vars: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(string, repeated, tag = "3")]
    pub slice_ios: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, repeated, tag = "4")]
    pub read_fds: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, tag = "5")]
    pub write_fds: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub segment_limit_po2: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "7")]
    pub session_limit: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Binary {
    #[prost(enumeration = "binary::Kind", tag = "1")]
    pub kind: i32,
    #[prost(message, optional, tag = "2")]
    pub asset: ::core::option::Option<Asset>,
}
/// Nested message and enum types in `Binary`.
pub mod binary {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Kind {
        Unspecified = 0,
        Image = 1,
        Elf = 2,
    }
    impl Kind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Kind::Unspecified => "UNSPECIFIED",
                Kind::Image => "IMAGE",
                Kind::Elf => "ELF",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "IMAGE" => Some(Self::Image),
                "ELF" => Some(Self::Elf),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProverOpts {
    #[prost(string, tag = "1")]
    pub hashfn: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionInfo {
    #[prost(uint32, tag = "1")]
    pub segments: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub journal: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub exit_code: ::core::option::Option<super::base::ExitCode>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentInfo {
    #[prost(uint32, tag = "1")]
    pub index: u32,
    #[prost(uint32, tag = "2")]
    pub po2: u32,
    #[prost(uint32, tag = "3")]
    pub cycles: u32,
    #[prost(message, optional, tag = "4")]
    pub segment: ::core::option::Option<Asset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProveSegmentResult {
    #[prost(message, optional, tag = "1")]
    pub receipt: ::core::option::Option<Asset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    #[prost(oneof = "asset::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<asset::Kind>,
}
/// Nested message and enum types in `Asset`.
pub mod asset {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(bytes, tag = "1")]
        Inline(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag = "2")]
        Path(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetRequest {
    #[prost(oneof = "asset_request::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<asset_request::Kind>,
}
/// Nested message and enum types in `AssetRequest`.
pub mod asset_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        Inline(()),
        #[prost(string, tag = "2")]
        Path(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerReply {
    #[prost(oneof = "server_reply::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<server_reply::Kind>,
}
/// Nested message and enum types in `ServerReply`.
pub mod server_reply {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        Ok(super::ClientCallback),
        #[prost(message, tag = "2")]
        Error(super::GenericError),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientCallback {
    #[prost(oneof = "client_callback::Kind", tags = "1, 2, 3, 4")]
    pub kind: ::core::option::Option<client_callback::Kind>,
}
/// Nested message and enum types in `ClientCallback`.
pub mod client_callback {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        Io(super::OnIoRequest),
        #[prost(message, tag = "2")]
        SegmentDone(super::OnSegmentDone),
        #[prost(message, tag = "3")]
        SessionDone(super::OnSessionDone),
        #[prost(message, tag = "4")]
        ProveDone(super::OnProveDone),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnIoRequest {
    #[prost(oneof = "on_io_request::Kind", tags = "1, 2, 3")]
    pub kind: ::core::option::Option<on_io_request::Kind>,
}
/// Nested message and enum types in `OnIoRequest`.
pub mod on_io_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        Posix(super::PosixIo),
        #[prost(message, tag = "2")]
        Slice(super::SliceIo),
        #[prost(message, tag = "3")]
        Trace(super::TraceEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SliceIo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub from_guest: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PosixIo {
    #[prost(uint32, tag = "1")]
    pub fd: u32,
    #[prost(message, optional, tag = "2")]
    pub cmd: ::core::option::Option<PosixCmd>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PosixCmd {
    #[prost(oneof = "posix_cmd::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<posix_cmd::Kind>,
}
/// Nested message and enum types in `PosixCmd`.
pub mod posix_cmd {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(uint32, tag = "1")]
        Read(u32),
        #[prost(bytes, tag = "2")]
        Write(::prost::alloc::vec::Vec<u8>),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraceEvent {
    #[prost(oneof = "trace_event::Kind", tags = "1, 2, 3")]
    pub kind: ::core::option::Option<trace_event::Kind>,
}
/// Nested message and enum types in `TraceEvent`.
pub mod trace_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InstructionStart {
        #[prost(uint32, tag = "1")]
        pub cycle: u32,
        #[prost(uint32, tag = "2")]
        pub pc: u32,
        #[prost(uint32, tag = "3")]
        pub insn: u32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RegisterSet {
        #[prost(uint32, tag = "1")]
        pub idx: u32,
        #[prost(uint32, tag = "2")]
        pub value: u32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MemorySet {
        #[prost(uint32, tag = "1")]
        pub addr: u32,
        #[prost(uint32, tag = "2")]
        pub value: u32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        InsnStart(InstructionStart),
        #[prost(message, tag = "2")]
        RegisterSet(RegisterSet),
        #[prost(message, tag = "3")]
        MemorySet(MemorySet),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnIoReply {
    #[prost(oneof = "on_io_reply::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<on_io_reply::Kind>,
}
/// Nested message and enum types in `OnIoReply`.
pub mod on_io_reply {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(bytes, tag = "1")]
        Ok(::prost::alloc::vec::Vec<u8>),
        #[prost(message, tag = "2")]
        Error(super::GenericError),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnSegmentDone {
    #[prost(message, optional, tag = "1")]
    pub segment: ::core::option::Option<SegmentInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnSessionDone {
    #[prost(message, optional, tag = "1")]
    pub session: ::core::option::Option<SessionInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnProveDone {
    #[prost(message, optional, tag = "1")]
    pub receipt: ::core::option::Option<Asset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericReply {
    #[prost(oneof = "generic_reply::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<generic_reply::Kind>,
}
/// Nested message and enum types in `GenericReply`.
pub mod generic_reply {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        Ok(()),
        #[prost(message, tag = "2")]
        Error(super::GenericError),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericError {
    #[prost(string, tag = "1")]
    pub reason: ::prost::alloc::string::String,
}
