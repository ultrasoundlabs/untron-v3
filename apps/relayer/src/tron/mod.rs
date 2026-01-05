pub mod address;
pub mod grpc;
pub mod proof;
pub mod wallet;

pub mod protocol {
    #![allow(clippy::all, clippy::pedantic, clippy::nursery)]
    #![allow(
        dead_code,
        unused_imports,
        unused_variables,
        non_snake_case,
        non_camel_case_types,
        non_upper_case_globals
    )]

    tonic::include_proto!("protocol");
}
