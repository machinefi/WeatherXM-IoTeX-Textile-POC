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
    #[prost(oneof = "inner_receipt::Kind", tags = "1, 2, 3, 4")]
    pub kind: ::core::option::Option<inner_receipt::Kind>,
}
/// Nested message and enum types in `InnerReceipt`.
pub mod inner_receipt {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        Composite(super::CompositeReceipt),
        #[prost(message, tag = "2")]
        Succinct(super::SuccinctReceipt),
        #[prost(message, tag = "3")]
        Fake(super::FakeReceipt),
        #[prost(message, tag = "4")]
        Groth16(super::Groth16Receipt),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompositeReceipt {
    #[prost(message, repeated, tag = "1")]
    pub segments: ::prost::alloc::vec::Vec<SegmentReceipt>,
    #[prost(message, repeated, tag = "2")]
    pub assumptions: ::prost::alloc::vec::Vec<InnerReceipt>,
    #[prost(message, optional, tag = "3")]
    pub journal_digest: ::core::option::Option<Digest>,
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
    #[prost(message, optional, tag = "5")]
    pub claim: ::core::option::Option<ReceiptClaim>,
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
    pub claim: ::core::option::Option<ReceiptClaim>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Groth16Receipt {
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<super::base::CompatVersion>,
    #[prost(bytes = "vec", tag = "2")]
    pub seal: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub claim: ::core::option::Option<ReceiptClaim>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiptClaim {
    /// MaybePruned<SystemState>
    #[prost(message, optional, tag = "1")]
    pub pre: ::core::option::Option<MaybePruned>,
    /// MaybePruned<SystemState>
    #[prost(message, optional, tag = "2")]
    pub post: ::core::option::Option<MaybePruned>,
    #[prost(message, optional, tag = "3")]
    pub exit_code: ::core::option::Option<super::base::ExitCode>,
    #[prost(message, optional, tag = "4")]
    pub input: ::core::option::Option<Digest>,
    /// Option<MaybePruned<Output>>
    #[prost(message, optional, tag = "5")]
    pub output: ::core::option::Option<MaybePruned>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaybePruned {
    #[prost(oneof = "maybe_pruned::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<maybe_pruned::Kind>,
}
/// Nested message and enum types in `MaybePruned`.
pub mod maybe_pruned {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// Protobuf encoded bytes of the inner value.
        #[prost(bytes, tag = "1")]
        Value(::prost::alloc::vec::Vec<u8>),
        #[prost(message, tag = "2")]
        Pruned(super::Digest),
    }
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
pub struct Output {
    /// MaybePruned<bytes>
    #[prost(message, optional, tag = "1")]
    pub journal: ::core::option::Option<MaybePruned>,
    /// MaybePruned<Assumptions>
    #[prost(message, optional, tag = "2")]
    pub assumptions: ::core::option::Option<MaybePruned>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assumptions {
    /// MaybePruned<ReceiptClaim>
    #[prost(message, repeated, tag = "1")]
    pub inner: ::prost::alloc::vec::Vec<MaybePruned>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FakeReceipt {
    #[prost(message, optional, tag = "1")]
    pub claim: ::core::option::Option<ReceiptClaim>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Digest {
    #[prost(uint32, repeated, tag = "1")]
    pub words: ::prost::alloc::vec::Vec<u32>,
}
