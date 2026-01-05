///Module containing a contract's types and functions.
/**

```solidity
library ITronTxReader {
    struct TriggerSmartContract { bytes32 txId; uint256 tronBlockNumber; uint32 tronBlockTimestamp; bytes21 senderTron; bytes21 toTron; bytes data; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ITronTxReader {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct TriggerSmartContract { bytes32 txId; uint256 tronBlockNumber; uint32 tronBlockTimestamp; bytes21 senderTron; bytes21 toTron; bytes data; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TriggerSmartContract {
        #[allow(missing_docs)]
        pub txId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub tronBlockNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub tronBlockTimestamp: u32,
        #[allow(missing_docs)]
        pub senderTron: alloy::sol_types::private::FixedBytes<21>,
        #[allow(missing_docs)]
        pub toTron: alloy::sol_types::private::FixedBytes<21>,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::FixedBytes<21>,
            alloy::sol_types::sol_data::FixedBytes<21>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U256,
            u32,
            alloy::sol_types::private::FixedBytes<21>,
            alloy::sol_types::private::FixedBytes<21>,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TriggerSmartContract> for UnderlyingRustTuple<'_> {
            fn from(value: TriggerSmartContract) -> Self {
                (
                    value.txId,
                    value.tronBlockNumber,
                    value.tronBlockTimestamp,
                    value.senderTron,
                    value.toTron,
                    value.data,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TriggerSmartContract {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    txId: tuple.0,
                    tronBlockNumber: tuple.1,
                    tronBlockTimestamp: tuple.2,
                    senderTron: tuple.3,
                    toTron: tuple.4,
                    data: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for TriggerSmartContract {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for TriggerSmartContract {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.txId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.tronBlockNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.tronBlockTimestamp),
                    <alloy::sol_types::sol_data::FixedBytes<
                        21,
                    > as alloy_sol_types::SolType>::tokenize(&self.senderTron),
                    <alloy::sol_types::sol_data::FixedBytes<
                        21,
                    > as alloy_sol_types::SolType>::tokenize(&self.toTron),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for TriggerSmartContract {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for TriggerSmartContract {
            const NAME: &'static str = "TriggerSmartContract";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TriggerSmartContract(bytes32 txId,uint256 tronBlockNumber,uint32 tronBlockTimestamp,bytes21 senderTron,bytes21 toTron,bytes data)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.txId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.tronBlockNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.tronBlockTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        21,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.senderTron)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        21,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.toTron)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.data,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TriggerSmartContract {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.txId)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tronBlockNumber,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tronBlockTimestamp,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        21,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.senderTron,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        21,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.toTron,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.data,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.txId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tronBlockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tronBlockTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    21,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.senderTron,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    21,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.toTron,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.data,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ITronTxReader`](self) contract instance.

See the [wrapper's documentation](`ITronTxReaderInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> ITronTxReaderInstance<P, N> {
        ITronTxReaderInstance::<P, N>::new(address, __provider)
    }
    /**A [`ITronTxReader`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ITronTxReader`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ITronTxReaderInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ITronTxReaderInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ITronTxReaderInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ITronTxReaderInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`ITronTxReader`](self) contract instance.

See the [wrapper's documentation](`ITronTxReaderInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> ITronTxReaderInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ITronTxReaderInstance<P, N> {
            ITronTxReaderInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ITronTxReaderInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ITronTxReaderInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library ITronTxReader {
    struct TriggerSmartContract {
        bytes32 txId;
        uint256 tronBlockNumber;
        uint32 tronBlockTimestamp;
        bytes21 senderTron;
        bytes21 toTron;
        bytes data;
    }
}

interface StatefulTronTxReader {
    error DuplicateSr(bytes20 sr);
    error InvalidBlockSequence();
    error InvalidEncodedBlockLength(uint256 got);
    error InvalidHeaderPrefix();
    error InvalidTxMerkleProof();
    error InvalidWitnessAddressPrefix(uint8 got);
    error InvalidWitnessSignature();
    error NotTriggerSmartContract();
    error ProtoInvalidWireType();
    error ProtoTruncated();
    error SrSetNotSorted(uint256 index, bytes20 prev, bytes20 next);
    error TimestampOverflow();
    error TronInvalidContractLength();
    error TronInvalidContractPrefix();
    error TronInvalidOwnerLength();
    error TronInvalidOwnerPrefix();
    error TronTxNotSuccessful();
    error UnknownSr(bytes20 sr);

    constructor(bytes20[27] _srs, bytes20[27] _witnessDelegatees);

    function readTriggerSmartContract(bytes[20] memory blocks, bytes memory encodedTx, bytes32[] memory proof, uint256 index) external view returns (ITronTxReader.TriggerSmartContract memory callData);
    function srIndexPlus1(bytes20) external view returns (uint8);
    function srs(uint256) external view returns (bytes20);
    function witnessDelegatees(uint256) external view returns (bytes20);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_srs",
        "type": "bytes20[27]",
        "internalType": "bytes20[27]"
      },
      {
        "name": "_witnessDelegatees",
        "type": "bytes20[27]",
        "internalType": "bytes20[27]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "readTriggerSmartContract",
    "inputs": [
      {
        "name": "blocks",
        "type": "bytes[20]",
        "internalType": "bytes[20]"
      },
      {
        "name": "encodedTx",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "proof",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "callData",
        "type": "tuple",
        "internalType": "struct ITronTxReader.TriggerSmartContract",
        "components": [
          {
            "name": "txId",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "tronBlockNumber",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "tronBlockTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "senderTron",
            "type": "bytes21",
            "internalType": "bytes21"
          },
          {
            "name": "toTron",
            "type": "bytes21",
            "internalType": "bytes21"
          },
          {
            "name": "data",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "srIndexPlus1",
    "inputs": [
      {
        "name": "",
        "type": "bytes20",
        "internalType": "bytes20"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "srs",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes20",
        "internalType": "bytes20"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "witnessDelegatees",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes20",
        "internalType": "bytes20"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "error",
    "name": "DuplicateSr",
    "inputs": [
      {
        "name": "sr",
        "type": "bytes20",
        "internalType": "bytes20"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidBlockSequence",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidEncodedBlockLength",
    "inputs": [
      {
        "name": "got",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidHeaderPrefix",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidTxMerkleProof",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidWitnessAddressPrefix",
    "inputs": [
      {
        "name": "got",
        "type": "uint8",
        "internalType": "uint8"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidWitnessSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotTriggerSmartContract",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ProtoInvalidWireType",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ProtoTruncated",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SrSetNotSorted",
    "inputs": [
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "prev",
        "type": "bytes20",
        "internalType": "bytes20"
      },
      {
        "name": "next",
        "type": "bytes20",
        "internalType": "bytes20"
      }
    ]
  },
  {
    "type": "error",
    "name": "TimestampOverflow",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TronInvalidContractLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TronInvalidContractPrefix",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TronInvalidOwnerLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TronInvalidOwnerPrefix",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TronTxNotSuccessful",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UnknownSr",
    "inputs": [
      {
        "name": "sr",
        "type": "bytes20",
        "internalType": "bytes20"
      }
    ]
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod StatefulTronTxReader {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50604051611bd5380380611bd583398101604081905261002e91610264565b81516001600160601b0319165f908152602081905260409020805460ff191660019081179091555b601b81101561013e575f8361006c6001846102c1565b601b811061007c5761007c610299565b602002015190505f8483601b811061009657610096610299565b60200201519050606081811c9083901c106100e257604051624c919360e81b8152600481018490526001600160601b031980841660248301528216604482015260640160405180910390fd5b6100ed8360016102da565b5f5f8786601b811061010157610101610299565b602090810291909101516001600160601b03191682528101919091526040015f20805460ff191660ff929092169190911790555050600101610056565b5061014c600183601b610162565b5061015a601c82601b610162565b5050506102ed565b82601b81019282156101a5579160200282015b828111156101a557825182546001600160a01b03191660609190911c178255602090920191600190910190610175565b506101b19291506101b5565b5090565b5b808211156101b1575f81556001016101b6565b80516001600160601b0319811681146101e0575f5ffd5b919050565b5f82601f8301126101f4575f5ffd5b60405161036081016001600160401b038111828210171561022357634e487b7160e01b5f52604160045260245ffd5b60405280610360840185811115610238575f5ffd5b845b818110156102595761024b816101c9565b83526020928301920161023a565b509195945050505050565b5f5f6106c08385031215610276575f5ffd5b61028084846101e5565b91506102908461036085016101e5565b90509250929050565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b818103818111156102d4576102d46102ad565b92915050565b808201808211156102d4576102d46102ad565b6118db806102fa5f395ff3fe608060405234801561000f575f5ffd5b506004361061004a575f3560e01c806312d713c21461004e57806339adfeff1461007f57806349cd9f98146100925780637e670eb3146100b2575b5f5ffd5b61006161005c366004611560565b6100e6565b6040516001600160601b031990911681526020015b60405180910390f35b61006161008d366004611560565b6100ff565b6100a56100a03660046115be565b61010e565b6040516100769190611690565b6100d46100c036600461171d565b5f6020819052908152604090205460ff1681565b60405160ff9091168152602001610076565b601c81601b81106100f5575f80fd5b015460601b905081565b600181601b81106100f5575f80fd5b610116611515565b5f5f5f6101228a6101fc565b9250925092506101b68160028b8b60405161013e929190611744565b602060405180830381855afa158015610159573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061017c9190611753565b8989808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152508b9250610278915050565b6101d3576040516301d7cdd360e21b815260040160405180910390fd5b6101dd89896103ae565b60208101939093525063ffffffff166040820152979650505050505050565b5f80808080805b601481101561026e575f5f5f5f5f61023d8c87601481106102265761022661176a565b602002810190610236919061177e565b8a8a610490565b94509450945094509450849750839650855f0361025e57829a508199508098505b5050505050806001019050610203565b5050509193909250565b5f83815b84518110156103a0575f8582815181106102985761029861176a565b60200260200101519050816001901b85165f0361032557604080516020810185905290810182905260029060600160408051601f19818403018152908290526102e0916117c0565b602060405180830381855afa1580156102fb573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061031e9190611753565b9250610397565b604080516020810183905290810184905260029060600160408051601f1981840301815290829052610356916117c0565b602060405180830381855afa158015610371573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906103949190611753565b92505b5060010161027c565b50851490505b949350505050565b6103b6611515565b5f5f5f5f5f5f5f6103c78a8a610600565b9850965090508581118015906103dd5750888611155b6103e9576103e96117d6565b6103f58a8a83896106dc565b92975090955093509150508115801561040c575080155b1561042a5760405163306e189b60e21b815260040160405180910390fd5b610436898987816107a4565b6104535760405163c2c062d160e01b815260040160405180910390fd5b8587526affffffffffffffffffffff1980851660608901528316608088015261047e8989848461089e565b60a08801525094979650505050505050565b5f5f5f5f5f5f5f6104a18b8b610986565b9098509296509094509250905088158015906104bd5750818914155b156104db5760405163e14a793160e01b815260040160405180910390fd5b5f6104e68c8c610b71565b90505f6104f4828e8e610bd4565b6001600160601b031984165f9081526020819052604081205491925060ff909116908190036105475760405163cd42738b60e01b81526001600160601b0319851660048201526024015b60405180910390fd5b5f6105536001836117fe565b9050600160ff82161b8c811663ffffffff161561058f5760405163583a88ff60e11b81526001600160601b03198716600482015260240161053e565b8c81179a506001600160601b03198416601c60ff8416601b81106105b5576105b561176a565b015460601b6001600160601b031916146105e2576040516313d6dc7360e01b815260040160405180910390fd5b6105ec8a86610c46565b9b5050505050505050945094509450945094565b5f80808380158061062c575085855f81811061061e5761061e61176a565b9091013560f81c600a141590505b1561064a5760405163306e189b60e21b815260040160405180910390fd5b60015f61065988888486610c69565b96508692509050610674826001600160401b03831685610d26565b9450600261068486888a8c611817565b604051610692929190611744565b602060405180830381855afa1580156106ad573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906106d09190611753565b93505050509250925092565b5f5f5f5f5f5f5f6106ef8b8b8b8b610d5b565b9194509250905081831080156107055750878211155b610711576107116117d6565b6001600160401b038116601f1461073b5760405163306e189b60e21b815260040160405180910390fd5b5f5f6107498d8d8787610e66565b90925090508115801561075a575080155b156107785760405163306e189b60e21b815260040160405180910390fd5b6107848d8d8484610f20565b809950819a50829b50839c50505050505050505050945094509450949050565b5f825b82841080156107d057508585858181106107c3576107c361176a565b9091013560f81c60121490505b1561081e5750826107e08161183e565b93505f6107ef87878787610c69565b95509050818511610802576108026117d6565b61081685826001600160401b031686610d26565b9450506107a7565b5f5b8385108015610849575086868681811061083c5761083c61176a565b9091013560f81c602a1490505b156108945750600161085a8561183e565b94505f5f61086a898989896110eb565b9850909250905061087d89898484611151565b61088d575f9450505050506103a6565b5050610820565b9695505050505050565b6060828210806108ad57508382115b156108cb57604051633ffd665960e01b815260040160405180910390fd5b5f6108d68484611856565b9050806001600160401b038111156108f0576108f0611869565b6040519080825280601f01601f19166020018201604052801561091a576020820181803683370190505b5091505f5b8181101561097c578686610933838861187d565b8181106109425761094261176a565b9050013560f81c60f81b83828151811061095e5761095e61176a565b60200101906001600160f81b03191690815f1a90535060010161091f565b5050949350505050565b5f8080808060ae86146109af5760405163600d155160e01b81526004810187905260240161053e565b86865f8181106109c1576109c161176a565b9050013560f81c60f81b6001600160f81b031916600a60f81b141580610a115750868660018181106109f5576109f561176a565b9050013560f81c60f81b6001600160f81b031916606960f81b14155b80610a4657508686606b818110610a2a57610a2a61176a565b9050013560f81c60f81b6001600160f81b031916601260f81b14155b80610a7b57508686606c818110610a5f57610a5f61176a565b9050013560f81c60f81b6001600160f81b031916604160f81b14155b15610a995760405163ef02c9bb60e01b815260040160405180910390fd5b5f610aa8888860036009610c69565b506001600160401b031690505f610ac16103e883611890565b905063ffffffff811115610ae85760405163549a019760e01b815260040160405180910390fd5b9550600b8801359450602d8801359350855f610b088a8a604e6052610c69565b6001600160401b03909116955090505f8a8a6054818110610b2b57610b2b61176a565b919091013560f81c91505060418114610b5c57604051634fa88d5f60e11b815260ff8216600482015260240161053e565b50969995989497509295505050506055013590565b5f6002610b81606b828587611817565b604051610b8f929190611744565b602060405180830381855afa158015610baa573d5f5f3e3d5ffd5b5050506040513d601f19601f82011682018060405250810190610bcd9190611753565b9392505050565b5f60ad830135811a606d840135608d850135601b831015610bf657601b830192505b601c8314601b841417610c0a575f5f525f5ffd5b60405187815283602082015282604082015281606082015260208160808360015afa610c37575f5f525f5ffd5b5160601b979650505050505050565b5f80610c576001600160c01b611856565b831660c085901b179150505b92915050565b5f808080805b600a811015610d0357858710610c9857604051633ffd665960e01b815260040160405180910390fd5b5f898989610ca58161183e565b9a50818110610cb657610cb661176a565b607f92013560f81c9182166001600160401b0386161b9590951794509050608081165f03610ced5783889550955050505050610d1d565b610cf86007846118af565b925050600101610c6f565b50604051633ffd665960e01b815260040160405180910390fd5b94509492505050565b5f610d318483611856565b831115610d5157604051633ffd665960e01b815260040160405180910390fd5b6103a6838561187d565b5f80808481815b86831015610e3b5750815f80610d7a8c8c858c611231565b96509092509050828511610d9057610d906117d6565b816001600160401b0316600b148015610db257506001600160401b0381166002145b15610e25578315610dd65760405163306e189b60e21b815260040160405180910390fd5b60019350610de68c8c878c6110eb565b919950975094505f610dfa8d8d8b8b611265565b909750905080610e1d5760405163306e189b60e21b815260040160405180910390fd5b505050610e3b565b610e328c8c878c856112fb565b94505050610d62565b81610e595760405163306e189b60e21b815260040160405180910390fd5b5050509450945094915050565b5f80838180825b86841015610f135750825f80610e858c8c858c611231565b97509092509050828611610e9b57610e9b6117d6565b816001600160401b03166002148015610ebd57506001600160401b0381166002145b15610efc57610ece8c8c888c6110eb565b975090955093505f80610ee38e8e89896113a4565b91509150815f14610ef5578199508098505b5050610f0c565b610f098c8c888c856112fb565b95505b5050610e6d565b5050505094509492505050565b5f80808085805b868210156110de5750805f80610f3f8c8c858c611231565b95509092509050828411610f5557610f556117d6565b816001600160401b03166001148015610f7757506001600160401b0381166002145b15610ff2575f5f610f8a8e8e888e6110eb565b97509092509050610f9b8282611856565b601514610fbb5760405163902757b160e01b815260040160405180910390fd5b610fc68e8e8461146c565b9950895f1a604114610feb5760405163a4645d6560e01b815260040160405180910390fd5b50506110d7565b816001600160401b0316600214801561101457506001600160401b0381166002145b15611088575f5f6110278e8e888e6110eb565b975090925090506110388282611856565b60151461105857604051636c8ee0d960e11b815260040160405180910390fd5b6110638e8e8461146c565b9850885f1a604114610feb5760405163547793ab60e11b815260040160405180910390fd5b816001600160401b031660041480156110aa57506001600160401b0381166002145b156110c7576110bb8c8c868c6110eb565b919750955093506110d7565b6110d48c8c868c856112fb565b93505b5050610f27565b5050945094509450949050565b5f5f5f5f6110fb88888888610c69565b965086945090506001600160401b0381166111168587611856565b81111561113657604051633ffd665960e01b815260040160405180910390fd5b611140818861187d565b935083925050509450945094915050565b5f808084805b8582101561120e5750805f8061116f8b8b858b611231565b95509092509050828411611185576111856117d6565b6001600160401b0381166111f7575f6111a08c8c878c610c69565b955090506001600160401b0383166002036111d8576001600160401b038116156111d3575f9750505050505050506103a6565b6111f1565b826001600160401b03166003036111f157600195508096505b50611207565b6112048b8b868b856112fb565b93505b5050611157565b8280156112245750836001600160401b03166001145b9998505050505050505050565b5f5f5f5f5f61124289898989610c69565b600382901c671fffffffffffffff169b60079092169a5098509650505050505050565b5f8083805b848210156112f05750805f806112828a8a858a611231565b95509092509050828411611298576112986117d6565b816001600160401b031660011480156112b857506001600160401b038116155b156112da576112c98a8a868a610c69565b50955060019450610d1d9350505050565b6112e78a8a868a856112fb565b9350505061126a565b505094509492505050565b5f6001600160401b03821661131d57611316868686866114a3565b905061139b565b6001196001600160401b03831601611347575f61133c878787876110eb565b50925061139b915050565b6004196001600160401b038316016113655761131684600485610d26565b5f196001600160401b038316016113825761131684600885610d26565b60405163a5a5fc4360e01b815260040160405180910390fd5b95945050505050565b5f8083805b848210156112f05750805f806113c18a8a858a611231565b955090925090508284116113d7576113d76117d6565b816001600160401b031660011480156113f957506001600160401b0381166002145b156114145761140a8a8a868a6110eb565b5094506114659050565b816001600160401b0316600214801561143657506001600160401b0381166002145b15611455576114478a8a868a6110eb565b509096509450849350611465565b6114628a8a868a856112fb565b93505b50506113a9565b5f8261147983601561187d565b111561149857604051633ffd665960e01b815260040160405180910390fd5b509190910135919050565b5f805b600a811015610d03578284106114cf57604051633ffd665960e01b815260040160405180910390fd5b5f8686866114dc8161183e565b97508181106114ed576114ed61176a565b919091013560f81c915050608081165f0361150c5784925050506103a6565b506001016114a6565b6040518060c001604052805f81526020015f81526020015f63ffffffff1681526020015f6001600160581b03191681526020015f6001600160581b0319168152602001606081525090565b5f60208284031215611570575f5ffd5b5035919050565b5f5f83601f840112611587575f5ffd5b5081356001600160401b0381111561159d575f5ffd5b6020830191508360208260051b85010111156115b7575f5ffd5b9250929050565b5f5f5f5f5f5f608087890312156115d3575f5ffd5b86356001600160401b038111156115e8575f5ffd5b870161028081018910156115fa575f5ffd5b955060208701356001600160401b03811115611614575f5ffd5b8701601f81018913611624575f5ffd5b80356001600160401b03811115611639575f5ffd5b89602082840101111561164a575f5ffd5b6020919091019550935060408701356001600160401b0381111561166c575f5ffd5b61167889828a01611577565b979a9699509497949695606090950135949350505050565b60208152815160208201526020820151604082015263ffffffff60408301511660608201526001600160581b031960608301511660808201526001600160581b031960808301511660a08201525f60a083015160c08084015280518060e0850152806020830161010086015e5f6101008286010152610100601f19601f8301168501019250505092915050565b5f6020828403121561172d575f5ffd5b81356001600160601b031981168114610bcd575f5ffd5b818382375f9101908152919050565b5f60208284031215611763575f5ffd5b5051919050565b634e487b7160e01b5f52603260045260245ffd5b5f5f8335601e19843603018112611793575f5ffd5b8301803591506001600160401b038211156117ac575f5ffd5b6020019150368190038213156115b7575f5ffd5b5f82518060208501845e5f920191825250919050565b634e487b7160e01b5f52600160045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b60ff8281168282160390811115610c6357610c636117ea565b5f5f85851115611825575f5ffd5b83861115611831575f5ffd5b5050820193919092039150565b5f6001820161184f5761184f6117ea565b5060010190565b81810381811115610c6357610c636117ea565b634e487b7160e01b5f52604160045260245ffd5b80820180821115610c6357610c636117ea565b5f826118aa57634e487b7160e01b5f52601260045260245ffd5b500490565b6001600160401b038181168382160190811115610c6357610c636117ea56fea164736f6c634300081b000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x1B\xD58\x03\x80a\x1B\xD5\x839\x81\x01`@\x81\x90Ra\0.\x91a\x02dV[\x81Q`\x01`\x01``\x1B\x03\x19\x16_\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U[`\x1B\x81\x10\x15a\x01>W_\x83a\0l`\x01\x84a\x02\xC1V[`\x1B\x81\x10a\0|Wa\0|a\x02\x99V[` \x02\x01Q\x90P_\x84\x83`\x1B\x81\x10a\0\x96Wa\0\x96a\x02\x99V[` \x02\x01Q\x90P``\x81\x81\x1C\x90\x83\x90\x1C\x10a\0\xE2W`@QbL\x91\x93`\xE8\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01``\x1B\x03\x19\x80\x84\x16`$\x83\x01R\x82\x16`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[a\0\xED\x83`\x01a\x02\xDAV[__\x87\x86`\x1B\x81\x10a\x01\x01Wa\x01\x01a\x02\x99V[` \x90\x81\x02\x91\x90\x91\x01Q`\x01`\x01``\x1B\x03\x19\x16\x82R\x81\x01\x91\x90\x91R`@\x01_ \x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UPP`\x01\x01a\0VV[Pa\x01L`\x01\x83`\x1Ba\x01bV[Pa\x01Z`\x1C\x82`\x1Ba\x01bV[PPPa\x02\xEDV[\x82`\x1B\x81\x01\x92\x82\x15a\x01\xA5W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x01\xA5W\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x91\x90\x91\x1C\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\x01uV[Pa\x01\xB1\x92\x91Pa\x01\xB5V[P\x90V[[\x80\x82\x11\x15a\x01\xB1W_\x81U`\x01\x01a\x01\xB6V[\x80Q`\x01`\x01``\x1B\x03\x19\x81\x16\x81\x14a\x01\xE0W__\xFD[\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x01\xF4W__\xFD[`@Qa\x03`\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x02#WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@R\x80a\x03`\x84\x01\x85\x81\x11\x15a\x028W__\xFD[\x84[\x81\x81\x10\x15a\x02YWa\x02K\x81a\x01\xC9V[\x83R` \x92\x83\x01\x92\x01a\x02:V[P\x91\x95\x94PPPPPV[__a\x06\xC0\x83\x85\x03\x12\x15a\x02vW__\xFD[a\x02\x80\x84\x84a\x01\xE5V[\x91Pa\x02\x90\x84a\x03`\x85\x01a\x01\xE5V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\xD4Wa\x02\xD4a\x02\xADV[\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x02\xD4Wa\x02\xD4a\x02\xADV[a\x18\xDB\x80a\x02\xFA_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0JW_5`\xE0\x1C\x80c\x12\xD7\x13\xC2\x14a\0NW\x80c9\xAD\xFE\xFF\x14a\0\x7FW\x80cI\xCD\x9F\x98\x14a\0\x92W\x80c~g\x0E\xB3\x14a\0\xB2W[__\xFD[a\0aa\0\\6`\x04a\x15`V[a\0\xE6V[`@Q`\x01`\x01``\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0aa\0\x8D6`\x04a\x15`V[a\0\xFFV[a\0\xA5a\0\xA06`\x04a\x15\xBEV[a\x01\x0EV[`@Qa\0v\x91\x90a\x16\x90V[a\0\xD4a\0\xC06`\x04a\x17\x1DV[_` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\0vV[`\x1C\x81`\x1B\x81\x10a\0\xF5W_\x80\xFD[\x01T``\x1B\x90P\x81V[`\x01\x81`\x1B\x81\x10a\0\xF5W_\x80\xFD[a\x01\x16a\x15\x15V[___a\x01\"\x8Aa\x01\xFCV[\x92P\x92P\x92Pa\x01\xB6\x81`\x02\x8B\x8B`@Qa\x01>\x92\x91\x90a\x17DV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x01YW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01|\x91\x90a\x17SV[\x89\x89\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8B\x92Pa\x02x\x91PPV[a\x01\xD3W`@Qc\x01\xD7\xCD\xD3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01\xDD\x89\x89a\x03\xAEV[` \x81\x01\x93\x90\x93RPc\xFF\xFF\xFF\xFF\x16`@\x82\x01R\x97\x96PPPPPPPV[_\x80\x80\x80\x80\x80[`\x14\x81\x10\x15a\x02nW_____a\x02=\x8C\x87`\x14\x81\x10a\x02&Wa\x02&a\x17jV[` \x02\x81\x01\x90a\x026\x91\x90a\x17~V[\x8A\x8Aa\x04\x90V[\x94P\x94P\x94P\x94P\x94P\x84\x97P\x83\x96P\x85_\x03a\x02^W\x82\x9AP\x81\x99P\x80\x98P[PPPPP\x80`\x01\x01\x90Pa\x02\x03V[PPP\x91\x93\x90\x92PV[_\x83\x81[\x84Q\x81\x10\x15a\x03\xA0W_\x85\x82\x81Q\x81\x10a\x02\x98Wa\x02\x98a\x17jV[` \x02` \x01\x01Q\x90P\x81`\x01\x90\x1B\x85\x16_\x03a\x03%W`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x82\x90R`\x02\x90``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x02\xE0\x91a\x17\xC0V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x02\xFBW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x1E\x91\x90a\x17SV[\x92Pa\x03\x97V[`@\x80Q` \x81\x01\x83\x90R\x90\x81\x01\x84\x90R`\x02\x90``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x03V\x91a\x17\xC0V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x03qW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x94\x91\x90a\x17SV[\x92P[P`\x01\x01a\x02|V[P\x85\x14\x90P[\x94\x93PPPPV[a\x03\xB6a\x15\x15V[_______a\x03\xC7\x8A\x8Aa\x06\0V[\x98P\x96P\x90P\x85\x81\x11\x80\x15\x90a\x03\xDDWP\x88\x86\x11\x15[a\x03\xE9Wa\x03\xE9a\x17\xD6V[a\x03\xF5\x8A\x8A\x83\x89a\x06\xDCV[\x92\x97P\x90\x95P\x93P\x91PP\x81\x15\x80\x15a\x04\x0CWP\x80\x15[\x15a\x04*W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x046\x89\x89\x87\x81a\x07\xA4V[a\x04SW`@Qc\xC2\xC0b\xD1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x87Rj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x85\x16``\x89\x01R\x83\x16`\x80\x88\x01Ra\x04~\x89\x89\x84\x84a\x08\x9EV[`\xA0\x88\x01RP\x94\x97\x96PPPPPPPV[_______a\x04\xA1\x8B\x8Ba\t\x86V[\x90\x98P\x92\x96P\x90\x94P\x92P\x90P\x88\x15\x80\x15\x90a\x04\xBDWP\x81\x89\x14\x15[\x15a\x04\xDBW`@Qc\xE1Jy1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x04\xE6\x8C\x8Ca\x0BqV[\x90P_a\x04\xF4\x82\x8E\x8Ea\x0B\xD4V[`\x01`\x01``\x1B\x03\x19\x84\x16_\x90\x81R` \x81\x90R`@\x81 T\x91\x92P`\xFF\x90\x91\x16\x90\x81\x90\x03a\x05GW`@Qc\xCDBs\x8B`\xE0\x1B\x81R`\x01`\x01``\x1B\x03\x19\x85\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[_a\x05S`\x01\x83a\x17\xFEV[\x90P`\x01`\xFF\x82\x16\x1B\x8C\x81\x16c\xFF\xFF\xFF\xFF\x16\x15a\x05\x8FW`@QcX:\x88\xFF`\xE1\x1B\x81R`\x01`\x01``\x1B\x03\x19\x87\x16`\x04\x82\x01R`$\x01a\x05>V[\x8C\x81\x17\x9AP`\x01`\x01``\x1B\x03\x19\x84\x16`\x1C`\xFF\x84\x16`\x1B\x81\x10a\x05\xB5Wa\x05\xB5a\x17jV[\x01T``\x1B`\x01`\x01``\x1B\x03\x19\x16\x14a\x05\xE2W`@Qc\x13\xD6\xDCs`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xEC\x8A\x86a\x0CFV[\x9BPPPPPPPP\x94P\x94P\x94P\x94P\x94V[_\x80\x80\x83\x80\x15\x80a\x06,WP\x85\x85_\x81\x81\x10a\x06\x1EWa\x06\x1Ea\x17jV[\x90\x91\x015`\xF8\x1C`\n\x14\x15\x90P[\x15a\x06JW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01_a\x06Y\x88\x88\x84\x86a\x0CiV[\x96P\x86\x92P\x90Pa\x06t\x82`\x01`\x01`@\x1B\x03\x83\x16\x85a\r&V[\x94P`\x02a\x06\x84\x86\x88\x8A\x8Ca\x18\x17V[`@Qa\x06\x92\x92\x91\x90a\x17DV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x06\xADW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD0\x91\x90a\x17SV[\x93PPPP\x92P\x92P\x92V[_______a\x06\xEF\x8B\x8B\x8B\x8Ba\r[V[\x91\x94P\x92P\x90P\x81\x83\x10\x80\x15a\x07\x05WP\x87\x82\x11\x15[a\x07\x11Wa\x07\x11a\x17\xD6V[`\x01`\x01`@\x1B\x03\x81\x16`\x1F\x14a\x07;W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\x07I\x8D\x8D\x87\x87a\x0EfV[\x90\x92P\x90P\x81\x15\x80\x15a\x07ZWP\x80\x15[\x15a\x07xW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\x84\x8D\x8D\x84\x84a\x0F V[\x80\x99P\x81\x9AP\x82\x9BP\x83\x9CPPPPPPPPPP\x94P\x94P\x94P\x94\x90PV[_\x82[\x82\x84\x10\x80\x15a\x07\xD0WP\x85\x85\x85\x81\x81\x10a\x07\xC3Wa\x07\xC3a\x17jV[\x90\x91\x015`\xF8\x1C`\x12\x14\x90P[\x15a\x08\x1EWP\x82a\x07\xE0\x81a\x18>V[\x93P_a\x07\xEF\x87\x87\x87\x87a\x0CiV[\x95P\x90P\x81\x85\x11a\x08\x02Wa\x08\x02a\x17\xD6V[a\x08\x16\x85\x82`\x01`\x01`@\x1B\x03\x16\x86a\r&V[\x94PPa\x07\xA7V[_[\x83\x85\x10\x80\x15a\x08IWP\x86\x86\x86\x81\x81\x10a\x08<Wa\x08<a\x17jV[\x90\x91\x015`\xF8\x1C`*\x14\x90P[\x15a\x08\x94WP`\x01a\x08Z\x85a\x18>V[\x94P__a\x08j\x89\x89\x89\x89a\x10\xEBV[\x98P\x90\x92P\x90Pa\x08}\x89\x89\x84\x84a\x11QV[a\x08\x8DW_\x94PPPPPa\x03\xA6V[PPa\x08 V[\x96\x95PPPPPPV[``\x82\x82\x10\x80a\x08\xADWP\x83\x82\x11[\x15a\x08\xCBW`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x08\xD6\x84\x84a\x18VV[\x90P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xF0Wa\x08\xF0a\x18iV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\t\x1AW` \x82\x01\x81\x806\x837\x01\x90P[P\x91P_[\x81\x81\x10\x15a\t|W\x86\x86a\t3\x83\x88a\x18}V[\x81\x81\x10a\tBWa\tBa\x17jV[\x90P\x015`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\t^Wa\t^a\x17jV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\t\x1FV[PP\x94\x93PPPPV[_\x80\x80\x80\x80`\xAE\x86\x14a\t\xAFW`@Qc`\r\x15Q`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x05>V[\x86\x86_\x81\x81\x10a\t\xC1Wa\t\xC1a\x17jV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`\n`\xF8\x1B\x14\x15\x80a\n\x11WP\x86\x86`\x01\x81\x81\x10a\t\xF5Wa\t\xF5a\x17jV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`i`\xF8\x1B\x14\x15[\x80a\nFWP\x86\x86`k\x81\x81\x10a\n*Wa\n*a\x17jV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`\x12`\xF8\x1B\x14\x15[\x80a\n{WP\x86\x86`l\x81\x81\x10a\n_Wa\n_a\x17jV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`A`\xF8\x1B\x14\x15[\x15a\n\x99W`@Qc\xEF\x02\xC9\xBB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\n\xA8\x88\x88`\x03`\ta\x0CiV[P`\x01`\x01`@\x1B\x03\x16\x90P_a\n\xC1a\x03\xE8\x83a\x18\x90V[\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15a\n\xE8W`@QcT\x9A\x01\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x95P`\x0B\x88\x015\x94P`-\x88\x015\x93P\x85_a\x0B\x08\x8A\x8A`N`Ra\x0CiV[`\x01`\x01`@\x1B\x03\x90\x91\x16\x95P\x90P_\x8A\x8A`T\x81\x81\x10a\x0B+Wa\x0B+a\x17jV[\x91\x90\x91\x015`\xF8\x1C\x91PP`A\x81\x14a\x0B\\W`@QcO\xA8\x8D_`\xE1\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`$\x01a\x05>V[P\x96\x99\x95\x98\x94\x97P\x92\x95PPPP`U\x015\x90V[_`\x02a\x0B\x81`k\x82\x85\x87a\x18\x17V[`@Qa\x0B\x8F\x92\x91\x90a\x17DV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x0B\xAAW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xCD\x91\x90a\x17SV[\x93\x92PPPV[_`\xAD\x83\x015\x81\x1A`m\x84\x015`\x8D\x85\x015`\x1B\x83\x10\x15a\x0B\xF6W`\x1B\x83\x01\x92P[`\x1C\x83\x14`\x1B\x84\x14\x17a\x0C\nW__R__\xFD[`@Q\x87\x81R\x83` \x82\x01R\x82`@\x82\x01R\x81``\x82\x01R` \x81`\x80\x83`\x01Z\xFAa\x0C7W__R__\xFD[Q``\x1B\x97\x96PPPPPPPV[_\x80a\x0CW`\x01`\x01`\xC0\x1Ba\x18VV[\x83\x16`\xC0\x85\x90\x1B\x17\x91PP[\x92\x91PPV[_\x80\x80\x80\x80[`\n\x81\x10\x15a\r\x03W\x85\x87\x10a\x0C\x98W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x89\x89\x89a\x0C\xA5\x81a\x18>V[\x9AP\x81\x81\x10a\x0C\xB6Wa\x0C\xB6a\x17jV[`\x7F\x92\x015`\xF8\x1C\x91\x82\x16`\x01`\x01`@\x1B\x03\x86\x16\x1B\x95\x90\x95\x17\x94P\x90P`\x80\x81\x16_\x03a\x0C\xEDW\x83\x88\x95P\x95PPPPPa\r\x1DV[a\x0C\xF8`\x07\x84a\x18\xAFV[\x92PP`\x01\x01a\x0CoV[P`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x94P\x94\x92PPPV[_a\r1\x84\x83a\x18VV[\x83\x11\x15a\rQW`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xA6\x83\x85a\x18}V[_\x80\x80\x84\x81\x81[\x86\x83\x10\x15a\x0E;WP\x81_\x80a\rz\x8C\x8C\x85\x8Ca\x121V[\x96P\x90\x92P\x90P\x82\x85\x11a\r\x90Wa\r\x90a\x17\xD6V[\x81`\x01`\x01`@\x1B\x03\x16`\x0B\x14\x80\x15a\r\xB2WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x0E%W\x83\x15a\r\xD6W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x93Pa\r\xE6\x8C\x8C\x87\x8Ca\x10\xEBV[\x91\x99P\x97P\x94P_a\r\xFA\x8D\x8D\x8B\x8Ba\x12eV[\x90\x97P\x90P\x80a\x0E\x1DW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPa\x0E;V[a\x0E2\x8C\x8C\x87\x8C\x85a\x12\xFBV[\x94PPPa\rbV[\x81a\x0EYW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPP\x94P\x94P\x94\x91PPV[_\x80\x83\x81\x80\x82[\x86\x84\x10\x15a\x0F\x13WP\x82_\x80a\x0E\x85\x8C\x8C\x85\x8Ca\x121V[\x97P\x90\x92P\x90P\x82\x86\x11a\x0E\x9BWa\x0E\x9Ba\x17\xD6V[\x81`\x01`\x01`@\x1B\x03\x16`\x02\x14\x80\x15a\x0E\xBDWP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x0E\xFCWa\x0E\xCE\x8C\x8C\x88\x8Ca\x10\xEBV[\x97P\x90\x95P\x93P_\x80a\x0E\xE3\x8E\x8E\x89\x89a\x13\xA4V[\x91P\x91P\x81_\x14a\x0E\xF5W\x81\x99P\x80\x98P[PPa\x0F\x0CV[a\x0F\t\x8C\x8C\x88\x8C\x85a\x12\xFBV[\x95P[PPa\x0EmV[PPPP\x94P\x94\x92PPPV[_\x80\x80\x80\x85\x80[\x86\x82\x10\x15a\x10\xDEWP\x80_\x80a\x0F?\x8C\x8C\x85\x8Ca\x121V[\x95P\x90\x92P\x90P\x82\x84\x11a\x0FUWa\x0FUa\x17\xD6V[\x81`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x0FwWP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x0F\xF2W__a\x0F\x8A\x8E\x8E\x88\x8Ea\x10\xEBV[\x97P\x90\x92P\x90Pa\x0F\x9B\x82\x82a\x18VV[`\x15\x14a\x0F\xBBW`@Qc\x90'W\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xC6\x8E\x8E\x84a\x14lV[\x99P\x89_\x1A`A\x14a\x0F\xEBW`@Qc\xA4d]e`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPa\x10\xD7V[\x81`\x01`\x01`@\x1B\x03\x16`\x02\x14\x80\x15a\x10\x14WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x10\x88W__a\x10'\x8E\x8E\x88\x8Ea\x10\xEBV[\x97P\x90\x92P\x90Pa\x108\x82\x82a\x18VV[`\x15\x14a\x10XW`@Qcl\x8E\xE0\xD9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10c\x8E\x8E\x84a\x14lV[\x98P\x88_\x1A`A\x14a\x0F\xEBW`@QcTw\x93\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`@\x1B\x03\x16`\x04\x14\x80\x15a\x10\xAAWP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x10\xC7Wa\x10\xBB\x8C\x8C\x86\x8Ca\x10\xEBV[\x91\x97P\x95P\x93Pa\x10\xD7V[a\x10\xD4\x8C\x8C\x86\x8C\x85a\x12\xFBV[\x93P[PPa\x0F'V[PP\x94P\x94P\x94P\x94\x90PV[____a\x10\xFB\x88\x88\x88\x88a\x0CiV[\x96P\x86\x94P\x90P`\x01`\x01`@\x1B\x03\x81\x16a\x11\x16\x85\x87a\x18VV[\x81\x11\x15a\x116W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11@\x81\x88a\x18}V[\x93P\x83\x92PPP\x94P\x94P\x94\x91PPV[_\x80\x80\x84\x80[\x85\x82\x10\x15a\x12\x0EWP\x80_\x80a\x11o\x8B\x8B\x85\x8Ba\x121V[\x95P\x90\x92P\x90P\x82\x84\x11a\x11\x85Wa\x11\x85a\x17\xD6V[`\x01`\x01`@\x1B\x03\x81\x16a\x11\xF7W_a\x11\xA0\x8C\x8C\x87\x8Ca\x0CiV[\x95P\x90P`\x01`\x01`@\x1B\x03\x83\x16`\x02\x03a\x11\xD8W`\x01`\x01`@\x1B\x03\x81\x16\x15a\x11\xD3W_\x97PPPPPPPPa\x03\xA6V[a\x11\xF1V[\x82`\x01`\x01`@\x1B\x03\x16`\x03\x03a\x11\xF1W`\x01\x95P\x80\x96P[Pa\x12\x07V[a\x12\x04\x8B\x8B\x86\x8B\x85a\x12\xFBV[\x93P[PPa\x11WV[\x82\x80\x15a\x12$WP\x83`\x01`\x01`@\x1B\x03\x16`\x01\x14[\x99\x98PPPPPPPPPV[_____a\x12B\x89\x89\x89\x89a\x0CiV[`\x03\x82\x90\x1Cg\x1F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x9B`\x07\x90\x92\x16\x9AP\x98P\x96PPPPPPPV[_\x80\x83\x80[\x84\x82\x10\x15a\x12\xF0WP\x80_\x80a\x12\x82\x8A\x8A\x85\x8Aa\x121V[\x95P\x90\x92P\x90P\x82\x84\x11a\x12\x98Wa\x12\x98a\x17\xD6V[\x81`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x12\xB8WP`\x01`\x01`@\x1B\x03\x81\x16\x15[\x15a\x12\xDAWa\x12\xC9\x8A\x8A\x86\x8Aa\x0CiV[P\x95P`\x01\x94Pa\r\x1D\x93PPPPV[a\x12\xE7\x8A\x8A\x86\x8A\x85a\x12\xFBV[\x93PPPa\x12jV[PP\x94P\x94\x92PPPV[_`\x01`\x01`@\x1B\x03\x82\x16a\x13\x1DWa\x13\x16\x86\x86\x86\x86a\x14\xA3V[\x90Pa\x13\x9BV[`\x01\x19`\x01`\x01`@\x1B\x03\x83\x16\x01a\x13GW_a\x13<\x87\x87\x87\x87a\x10\xEBV[P\x92Pa\x13\x9B\x91PPV[`\x04\x19`\x01`\x01`@\x1B\x03\x83\x16\x01a\x13eWa\x13\x16\x84`\x04\x85a\r&V[_\x19`\x01`\x01`@\x1B\x03\x83\x16\x01a\x13\x82Wa\x13\x16\x84`\x08\x85a\r&V[`@Qc\xA5\xA5\xFCC`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x95\x94PPPPPV[_\x80\x83\x80[\x84\x82\x10\x15a\x12\xF0WP\x80_\x80a\x13\xC1\x8A\x8A\x85\x8Aa\x121V[\x95P\x90\x92P\x90P\x82\x84\x11a\x13\xD7Wa\x13\xD7a\x17\xD6V[\x81`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x13\xF9WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x14\x14Wa\x14\n\x8A\x8A\x86\x8Aa\x10\xEBV[P\x94Pa\x14e\x90PV[\x81`\x01`\x01`@\x1B\x03\x16`\x02\x14\x80\x15a\x146WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x14UWa\x14G\x8A\x8A\x86\x8Aa\x10\xEBV[P\x90\x96P\x94P\x84\x93Pa\x14eV[a\x14b\x8A\x8A\x86\x8A\x85a\x12\xFBV[\x93P[PPa\x13\xA9V[_\x82a\x14y\x83`\x15a\x18}V[\x11\x15a\x14\x98W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x91\x90\x91\x015\x91\x90PV[_\x80[`\n\x81\x10\x15a\r\x03W\x82\x84\x10a\x14\xCFW`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x86\x86\x86a\x14\xDC\x81a\x18>V[\x97P\x81\x81\x10a\x14\xEDWa\x14\xEDa\x17jV[\x91\x90\x91\x015`\xF8\x1C\x91PP`\x80\x81\x16_\x03a\x15\x0CW\x84\x92PPPa\x03\xA6V[P`\x01\x01a\x14\xA6V[`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\x01`\x01`X\x1B\x03\x19\x16\x81R` \x01_`\x01`\x01`X\x1B\x03\x19\x16\x81R` \x01``\x81RP\x90V[_` \x82\x84\x03\x12\x15a\x15pW__\xFD[P5\x91\x90PV[__\x83`\x1F\x84\x01\x12a\x15\x87W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\x9DW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x15\xB7W__\xFD[\x92P\x92\x90PV[______`\x80\x87\x89\x03\x12\x15a\x15\xD3W__\xFD[\x865`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\xE8W__\xFD[\x87\x01a\x02\x80\x81\x01\x89\x10\x15a\x15\xFAW__\xFD[\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\x14W__\xFD[\x87\x01`\x1F\x81\x01\x89\x13a\x16$W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x169W__\xFD[\x89` \x82\x84\x01\x01\x11\x15a\x16JW__\xFD[` \x91\x90\x91\x01\x95P\x93P`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16lW__\xFD[a\x16x\x89\x82\x8A\x01a\x15wV[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[` \x81R\x81Q` \x82\x01R` \x82\x01Q`@\x82\x01Rc\xFF\xFF\xFF\xFF`@\x83\x01Q\x16``\x82\x01R`\x01`\x01`X\x1B\x03\x19``\x83\x01Q\x16`\x80\x82\x01R`\x01`\x01`X\x1B\x03\x19`\x80\x83\x01Q\x16`\xA0\x82\x01R_`\xA0\x83\x01Q`\xC0\x80\x84\x01R\x80Q\x80`\xE0\x85\x01R\x80` \x83\x01a\x01\0\x86\x01^_a\x01\0\x82\x86\x01\x01Ra\x01\0`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x17-W__\xFD[\x815`\x01`\x01``\x1B\x03\x19\x81\x16\x81\x14a\x0B\xCDW__\xFD[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_` \x82\x84\x03\x12\x15a\x17cW__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x17\x93W__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x17\xACW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x15\xB7W__\xFD[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0CcWa\x0Cca\x17\xEAV[__\x85\x85\x11\x15a\x18%W__\xFD[\x83\x86\x11\x15a\x181W__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[_`\x01\x82\x01a\x18OWa\x18Oa\x17\xEAV[P`\x01\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x0CcWa\x0Cca\x17\xEAV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0CcWa\x0Cca\x17\xEAV[_\x82a\x18\xAAWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0CcWa\x0Cca\x17\xEAV\xFE\xA1dsolcC\0\x08\x1B\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061004a575f3560e01c806312d713c21461004e57806339adfeff1461007f57806349cd9f98146100925780637e670eb3146100b2575b5f5ffd5b61006161005c366004611560565b6100e6565b6040516001600160601b031990911681526020015b60405180910390f35b61006161008d366004611560565b6100ff565b6100a56100a03660046115be565b61010e565b6040516100769190611690565b6100d46100c036600461171d565b5f6020819052908152604090205460ff1681565b60405160ff9091168152602001610076565b601c81601b81106100f5575f80fd5b015460601b905081565b600181601b81106100f5575f80fd5b610116611515565b5f5f5f6101228a6101fc565b9250925092506101b68160028b8b60405161013e929190611744565b602060405180830381855afa158015610159573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061017c9190611753565b8989808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152508b9250610278915050565b6101d3576040516301d7cdd360e21b815260040160405180910390fd5b6101dd89896103ae565b60208101939093525063ffffffff166040820152979650505050505050565b5f80808080805b601481101561026e575f5f5f5f5f61023d8c87601481106102265761022661176a565b602002810190610236919061177e565b8a8a610490565b94509450945094509450849750839650855f0361025e57829a508199508098505b5050505050806001019050610203565b5050509193909250565b5f83815b84518110156103a0575f8582815181106102985761029861176a565b60200260200101519050816001901b85165f0361032557604080516020810185905290810182905260029060600160408051601f19818403018152908290526102e0916117c0565b602060405180830381855afa1580156102fb573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061031e9190611753565b9250610397565b604080516020810183905290810184905260029060600160408051601f1981840301815290829052610356916117c0565b602060405180830381855afa158015610371573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906103949190611753565b92505b5060010161027c565b50851490505b949350505050565b6103b6611515565b5f5f5f5f5f5f5f6103c78a8a610600565b9850965090508581118015906103dd5750888611155b6103e9576103e96117d6565b6103f58a8a83896106dc565b92975090955093509150508115801561040c575080155b1561042a5760405163306e189b60e21b815260040160405180910390fd5b610436898987816107a4565b6104535760405163c2c062d160e01b815260040160405180910390fd5b8587526affffffffffffffffffffff1980851660608901528316608088015261047e8989848461089e565b60a08801525094979650505050505050565b5f5f5f5f5f5f5f6104a18b8b610986565b9098509296509094509250905088158015906104bd5750818914155b156104db5760405163e14a793160e01b815260040160405180910390fd5b5f6104e68c8c610b71565b90505f6104f4828e8e610bd4565b6001600160601b031984165f9081526020819052604081205491925060ff909116908190036105475760405163cd42738b60e01b81526001600160601b0319851660048201526024015b60405180910390fd5b5f6105536001836117fe565b9050600160ff82161b8c811663ffffffff161561058f5760405163583a88ff60e11b81526001600160601b03198716600482015260240161053e565b8c81179a506001600160601b03198416601c60ff8416601b81106105b5576105b561176a565b015460601b6001600160601b031916146105e2576040516313d6dc7360e01b815260040160405180910390fd5b6105ec8a86610c46565b9b5050505050505050945094509450945094565b5f80808380158061062c575085855f81811061061e5761061e61176a565b9091013560f81c600a141590505b1561064a5760405163306e189b60e21b815260040160405180910390fd5b60015f61065988888486610c69565b96508692509050610674826001600160401b03831685610d26565b9450600261068486888a8c611817565b604051610692929190611744565b602060405180830381855afa1580156106ad573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906106d09190611753565b93505050509250925092565b5f5f5f5f5f5f5f6106ef8b8b8b8b610d5b565b9194509250905081831080156107055750878211155b610711576107116117d6565b6001600160401b038116601f1461073b5760405163306e189b60e21b815260040160405180910390fd5b5f5f6107498d8d8787610e66565b90925090508115801561075a575080155b156107785760405163306e189b60e21b815260040160405180910390fd5b6107848d8d8484610f20565b809950819a50829b50839c50505050505050505050945094509450949050565b5f825b82841080156107d057508585858181106107c3576107c361176a565b9091013560f81c60121490505b1561081e5750826107e08161183e565b93505f6107ef87878787610c69565b95509050818511610802576108026117d6565b61081685826001600160401b031686610d26565b9450506107a7565b5f5b8385108015610849575086868681811061083c5761083c61176a565b9091013560f81c602a1490505b156108945750600161085a8561183e565b94505f5f61086a898989896110eb565b9850909250905061087d89898484611151565b61088d575f9450505050506103a6565b5050610820565b9695505050505050565b6060828210806108ad57508382115b156108cb57604051633ffd665960e01b815260040160405180910390fd5b5f6108d68484611856565b9050806001600160401b038111156108f0576108f0611869565b6040519080825280601f01601f19166020018201604052801561091a576020820181803683370190505b5091505f5b8181101561097c578686610933838861187d565b8181106109425761094261176a565b9050013560f81c60f81b83828151811061095e5761095e61176a565b60200101906001600160f81b03191690815f1a90535060010161091f565b5050949350505050565b5f8080808060ae86146109af5760405163600d155160e01b81526004810187905260240161053e565b86865f8181106109c1576109c161176a565b9050013560f81c60f81b6001600160f81b031916600a60f81b141580610a115750868660018181106109f5576109f561176a565b9050013560f81c60f81b6001600160f81b031916606960f81b14155b80610a4657508686606b818110610a2a57610a2a61176a565b9050013560f81c60f81b6001600160f81b031916601260f81b14155b80610a7b57508686606c818110610a5f57610a5f61176a565b9050013560f81c60f81b6001600160f81b031916604160f81b14155b15610a995760405163ef02c9bb60e01b815260040160405180910390fd5b5f610aa8888860036009610c69565b506001600160401b031690505f610ac16103e883611890565b905063ffffffff811115610ae85760405163549a019760e01b815260040160405180910390fd5b9550600b8801359450602d8801359350855f610b088a8a604e6052610c69565b6001600160401b03909116955090505f8a8a6054818110610b2b57610b2b61176a565b919091013560f81c91505060418114610b5c57604051634fa88d5f60e11b815260ff8216600482015260240161053e565b50969995989497509295505050506055013590565b5f6002610b81606b828587611817565b604051610b8f929190611744565b602060405180830381855afa158015610baa573d5f5f3e3d5ffd5b5050506040513d601f19601f82011682018060405250810190610bcd9190611753565b9392505050565b5f60ad830135811a606d840135608d850135601b831015610bf657601b830192505b601c8314601b841417610c0a575f5f525f5ffd5b60405187815283602082015282604082015281606082015260208160808360015afa610c37575f5f525f5ffd5b5160601b979650505050505050565b5f80610c576001600160c01b611856565b831660c085901b179150505b92915050565b5f808080805b600a811015610d0357858710610c9857604051633ffd665960e01b815260040160405180910390fd5b5f898989610ca58161183e565b9a50818110610cb657610cb661176a565b607f92013560f81c9182166001600160401b0386161b9590951794509050608081165f03610ced5783889550955050505050610d1d565b610cf86007846118af565b925050600101610c6f565b50604051633ffd665960e01b815260040160405180910390fd5b94509492505050565b5f610d318483611856565b831115610d5157604051633ffd665960e01b815260040160405180910390fd5b6103a6838561187d565b5f80808481815b86831015610e3b5750815f80610d7a8c8c858c611231565b96509092509050828511610d9057610d906117d6565b816001600160401b0316600b148015610db257506001600160401b0381166002145b15610e25578315610dd65760405163306e189b60e21b815260040160405180910390fd5b60019350610de68c8c878c6110eb565b919950975094505f610dfa8d8d8b8b611265565b909750905080610e1d5760405163306e189b60e21b815260040160405180910390fd5b505050610e3b565b610e328c8c878c856112fb565b94505050610d62565b81610e595760405163306e189b60e21b815260040160405180910390fd5b5050509450945094915050565b5f80838180825b86841015610f135750825f80610e858c8c858c611231565b97509092509050828611610e9b57610e9b6117d6565b816001600160401b03166002148015610ebd57506001600160401b0381166002145b15610efc57610ece8c8c888c6110eb565b975090955093505f80610ee38e8e89896113a4565b91509150815f14610ef5578199508098505b5050610f0c565b610f098c8c888c856112fb565b95505b5050610e6d565b5050505094509492505050565b5f80808085805b868210156110de5750805f80610f3f8c8c858c611231565b95509092509050828411610f5557610f556117d6565b816001600160401b03166001148015610f7757506001600160401b0381166002145b15610ff2575f5f610f8a8e8e888e6110eb565b97509092509050610f9b8282611856565b601514610fbb5760405163902757b160e01b815260040160405180910390fd5b610fc68e8e8461146c565b9950895f1a604114610feb5760405163a4645d6560e01b815260040160405180910390fd5b50506110d7565b816001600160401b0316600214801561101457506001600160401b0381166002145b15611088575f5f6110278e8e888e6110eb565b975090925090506110388282611856565b60151461105857604051636c8ee0d960e11b815260040160405180910390fd5b6110638e8e8461146c565b9850885f1a604114610feb5760405163547793ab60e11b815260040160405180910390fd5b816001600160401b031660041480156110aa57506001600160401b0381166002145b156110c7576110bb8c8c868c6110eb565b919750955093506110d7565b6110d48c8c868c856112fb565b93505b5050610f27565b5050945094509450949050565b5f5f5f5f6110fb88888888610c69565b965086945090506001600160401b0381166111168587611856565b81111561113657604051633ffd665960e01b815260040160405180910390fd5b611140818861187d565b935083925050509450945094915050565b5f808084805b8582101561120e5750805f8061116f8b8b858b611231565b95509092509050828411611185576111856117d6565b6001600160401b0381166111f7575f6111a08c8c878c610c69565b955090506001600160401b0383166002036111d8576001600160401b038116156111d3575f9750505050505050506103a6565b6111f1565b826001600160401b03166003036111f157600195508096505b50611207565b6112048b8b868b856112fb565b93505b5050611157565b8280156112245750836001600160401b03166001145b9998505050505050505050565b5f5f5f5f5f61124289898989610c69565b600382901c671fffffffffffffff169b60079092169a5098509650505050505050565b5f8083805b848210156112f05750805f806112828a8a858a611231565b95509092509050828411611298576112986117d6565b816001600160401b031660011480156112b857506001600160401b038116155b156112da576112c98a8a868a610c69565b50955060019450610d1d9350505050565b6112e78a8a868a856112fb565b9350505061126a565b505094509492505050565b5f6001600160401b03821661131d57611316868686866114a3565b905061139b565b6001196001600160401b03831601611347575f61133c878787876110eb565b50925061139b915050565b6004196001600160401b038316016113655761131684600485610d26565b5f196001600160401b038316016113825761131684600885610d26565b60405163a5a5fc4360e01b815260040160405180910390fd5b95945050505050565b5f8083805b848210156112f05750805f806113c18a8a858a611231565b955090925090508284116113d7576113d76117d6565b816001600160401b031660011480156113f957506001600160401b0381166002145b156114145761140a8a8a868a6110eb565b5094506114659050565b816001600160401b0316600214801561143657506001600160401b0381166002145b15611455576114478a8a868a6110eb565b509096509450849350611465565b6114628a8a868a856112fb565b93505b50506113a9565b5f8261147983601561187d565b111561149857604051633ffd665960e01b815260040160405180910390fd5b509190910135919050565b5f805b600a811015610d03578284106114cf57604051633ffd665960e01b815260040160405180910390fd5b5f8686866114dc8161183e565b97508181106114ed576114ed61176a565b919091013560f81c915050608081165f0361150c5784925050506103a6565b506001016114a6565b6040518060c001604052805f81526020015f81526020015f63ffffffff1681526020015f6001600160581b03191681526020015f6001600160581b0319168152602001606081525090565b5f60208284031215611570575f5ffd5b5035919050565b5f5f83601f840112611587575f5ffd5b5081356001600160401b0381111561159d575f5ffd5b6020830191508360208260051b85010111156115b7575f5ffd5b9250929050565b5f5f5f5f5f5f608087890312156115d3575f5ffd5b86356001600160401b038111156115e8575f5ffd5b870161028081018910156115fa575f5ffd5b955060208701356001600160401b03811115611614575f5ffd5b8701601f81018913611624575f5ffd5b80356001600160401b03811115611639575f5ffd5b89602082840101111561164a575f5ffd5b6020919091019550935060408701356001600160401b0381111561166c575f5ffd5b61167889828a01611577565b979a9699509497949695606090950135949350505050565b60208152815160208201526020820151604082015263ffffffff60408301511660608201526001600160581b031960608301511660808201526001600160581b031960808301511660a08201525f60a083015160c08084015280518060e0850152806020830161010086015e5f6101008286010152610100601f19601f8301168501019250505092915050565b5f6020828403121561172d575f5ffd5b81356001600160601b031981168114610bcd575f5ffd5b818382375f9101908152919050565b5f60208284031215611763575f5ffd5b5051919050565b634e487b7160e01b5f52603260045260245ffd5b5f5f8335601e19843603018112611793575f5ffd5b8301803591506001600160401b038211156117ac575f5ffd5b6020019150368190038213156115b7575f5ffd5b5f82518060208501845e5f920191825250919050565b634e487b7160e01b5f52600160045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b60ff8281168282160390811115610c6357610c636117ea565b5f5f85851115611825575f5ffd5b83861115611831575f5ffd5b5050820193919092039150565b5f6001820161184f5761184f6117ea565b5060010190565b81810381811115610c6357610c636117ea565b634e487b7160e01b5f52604160045260245ffd5b80820180821115610c6357610c636117ea565b5f826118aa57634e487b7160e01b5f52601260045260245ffd5b500490565b6001600160401b038181168382160190811115610c6357610c636117ea56fea164736f6c634300081b000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0JW_5`\xE0\x1C\x80c\x12\xD7\x13\xC2\x14a\0NW\x80c9\xAD\xFE\xFF\x14a\0\x7FW\x80cI\xCD\x9F\x98\x14a\0\x92W\x80c~g\x0E\xB3\x14a\0\xB2W[__\xFD[a\0aa\0\\6`\x04a\x15`V[a\0\xE6V[`@Q`\x01`\x01``\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0aa\0\x8D6`\x04a\x15`V[a\0\xFFV[a\0\xA5a\0\xA06`\x04a\x15\xBEV[a\x01\x0EV[`@Qa\0v\x91\x90a\x16\x90V[a\0\xD4a\0\xC06`\x04a\x17\x1DV[_` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\0vV[`\x1C\x81`\x1B\x81\x10a\0\xF5W_\x80\xFD[\x01T``\x1B\x90P\x81V[`\x01\x81`\x1B\x81\x10a\0\xF5W_\x80\xFD[a\x01\x16a\x15\x15V[___a\x01\"\x8Aa\x01\xFCV[\x92P\x92P\x92Pa\x01\xB6\x81`\x02\x8B\x8B`@Qa\x01>\x92\x91\x90a\x17DV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x01YW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01|\x91\x90a\x17SV[\x89\x89\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8B\x92Pa\x02x\x91PPV[a\x01\xD3W`@Qc\x01\xD7\xCD\xD3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01\xDD\x89\x89a\x03\xAEV[` \x81\x01\x93\x90\x93RPc\xFF\xFF\xFF\xFF\x16`@\x82\x01R\x97\x96PPPPPPPV[_\x80\x80\x80\x80\x80[`\x14\x81\x10\x15a\x02nW_____a\x02=\x8C\x87`\x14\x81\x10a\x02&Wa\x02&a\x17jV[` \x02\x81\x01\x90a\x026\x91\x90a\x17~V[\x8A\x8Aa\x04\x90V[\x94P\x94P\x94P\x94P\x94P\x84\x97P\x83\x96P\x85_\x03a\x02^W\x82\x9AP\x81\x99P\x80\x98P[PPPPP\x80`\x01\x01\x90Pa\x02\x03V[PPP\x91\x93\x90\x92PV[_\x83\x81[\x84Q\x81\x10\x15a\x03\xA0W_\x85\x82\x81Q\x81\x10a\x02\x98Wa\x02\x98a\x17jV[` \x02` \x01\x01Q\x90P\x81`\x01\x90\x1B\x85\x16_\x03a\x03%W`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x82\x90R`\x02\x90``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x02\xE0\x91a\x17\xC0V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x02\xFBW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x1E\x91\x90a\x17SV[\x92Pa\x03\x97V[`@\x80Q` \x81\x01\x83\x90R\x90\x81\x01\x84\x90R`\x02\x90``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x03V\x91a\x17\xC0V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x03qW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x94\x91\x90a\x17SV[\x92P[P`\x01\x01a\x02|V[P\x85\x14\x90P[\x94\x93PPPPV[a\x03\xB6a\x15\x15V[_______a\x03\xC7\x8A\x8Aa\x06\0V[\x98P\x96P\x90P\x85\x81\x11\x80\x15\x90a\x03\xDDWP\x88\x86\x11\x15[a\x03\xE9Wa\x03\xE9a\x17\xD6V[a\x03\xF5\x8A\x8A\x83\x89a\x06\xDCV[\x92\x97P\x90\x95P\x93P\x91PP\x81\x15\x80\x15a\x04\x0CWP\x80\x15[\x15a\x04*W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x046\x89\x89\x87\x81a\x07\xA4V[a\x04SW`@Qc\xC2\xC0b\xD1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x87Rj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x85\x16``\x89\x01R\x83\x16`\x80\x88\x01Ra\x04~\x89\x89\x84\x84a\x08\x9EV[`\xA0\x88\x01RP\x94\x97\x96PPPPPPPV[_______a\x04\xA1\x8B\x8Ba\t\x86V[\x90\x98P\x92\x96P\x90\x94P\x92P\x90P\x88\x15\x80\x15\x90a\x04\xBDWP\x81\x89\x14\x15[\x15a\x04\xDBW`@Qc\xE1Jy1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x04\xE6\x8C\x8Ca\x0BqV[\x90P_a\x04\xF4\x82\x8E\x8Ea\x0B\xD4V[`\x01`\x01``\x1B\x03\x19\x84\x16_\x90\x81R` \x81\x90R`@\x81 T\x91\x92P`\xFF\x90\x91\x16\x90\x81\x90\x03a\x05GW`@Qc\xCDBs\x8B`\xE0\x1B\x81R`\x01`\x01``\x1B\x03\x19\x85\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[_a\x05S`\x01\x83a\x17\xFEV[\x90P`\x01`\xFF\x82\x16\x1B\x8C\x81\x16c\xFF\xFF\xFF\xFF\x16\x15a\x05\x8FW`@QcX:\x88\xFF`\xE1\x1B\x81R`\x01`\x01``\x1B\x03\x19\x87\x16`\x04\x82\x01R`$\x01a\x05>V[\x8C\x81\x17\x9AP`\x01`\x01``\x1B\x03\x19\x84\x16`\x1C`\xFF\x84\x16`\x1B\x81\x10a\x05\xB5Wa\x05\xB5a\x17jV[\x01T``\x1B`\x01`\x01``\x1B\x03\x19\x16\x14a\x05\xE2W`@Qc\x13\xD6\xDCs`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xEC\x8A\x86a\x0CFV[\x9BPPPPPPPP\x94P\x94P\x94P\x94P\x94V[_\x80\x80\x83\x80\x15\x80a\x06,WP\x85\x85_\x81\x81\x10a\x06\x1EWa\x06\x1Ea\x17jV[\x90\x91\x015`\xF8\x1C`\n\x14\x15\x90P[\x15a\x06JW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01_a\x06Y\x88\x88\x84\x86a\x0CiV[\x96P\x86\x92P\x90Pa\x06t\x82`\x01`\x01`@\x1B\x03\x83\x16\x85a\r&V[\x94P`\x02a\x06\x84\x86\x88\x8A\x8Ca\x18\x17V[`@Qa\x06\x92\x92\x91\x90a\x17DV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x06\xADW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD0\x91\x90a\x17SV[\x93PPPP\x92P\x92P\x92V[_______a\x06\xEF\x8B\x8B\x8B\x8Ba\r[V[\x91\x94P\x92P\x90P\x81\x83\x10\x80\x15a\x07\x05WP\x87\x82\x11\x15[a\x07\x11Wa\x07\x11a\x17\xD6V[`\x01`\x01`@\x1B\x03\x81\x16`\x1F\x14a\x07;W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\x07I\x8D\x8D\x87\x87a\x0EfV[\x90\x92P\x90P\x81\x15\x80\x15a\x07ZWP\x80\x15[\x15a\x07xW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\x84\x8D\x8D\x84\x84a\x0F V[\x80\x99P\x81\x9AP\x82\x9BP\x83\x9CPPPPPPPPPP\x94P\x94P\x94P\x94\x90PV[_\x82[\x82\x84\x10\x80\x15a\x07\xD0WP\x85\x85\x85\x81\x81\x10a\x07\xC3Wa\x07\xC3a\x17jV[\x90\x91\x015`\xF8\x1C`\x12\x14\x90P[\x15a\x08\x1EWP\x82a\x07\xE0\x81a\x18>V[\x93P_a\x07\xEF\x87\x87\x87\x87a\x0CiV[\x95P\x90P\x81\x85\x11a\x08\x02Wa\x08\x02a\x17\xD6V[a\x08\x16\x85\x82`\x01`\x01`@\x1B\x03\x16\x86a\r&V[\x94PPa\x07\xA7V[_[\x83\x85\x10\x80\x15a\x08IWP\x86\x86\x86\x81\x81\x10a\x08<Wa\x08<a\x17jV[\x90\x91\x015`\xF8\x1C`*\x14\x90P[\x15a\x08\x94WP`\x01a\x08Z\x85a\x18>V[\x94P__a\x08j\x89\x89\x89\x89a\x10\xEBV[\x98P\x90\x92P\x90Pa\x08}\x89\x89\x84\x84a\x11QV[a\x08\x8DW_\x94PPPPPa\x03\xA6V[PPa\x08 V[\x96\x95PPPPPPV[``\x82\x82\x10\x80a\x08\xADWP\x83\x82\x11[\x15a\x08\xCBW`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x08\xD6\x84\x84a\x18VV[\x90P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xF0Wa\x08\xF0a\x18iV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\t\x1AW` \x82\x01\x81\x806\x837\x01\x90P[P\x91P_[\x81\x81\x10\x15a\t|W\x86\x86a\t3\x83\x88a\x18}V[\x81\x81\x10a\tBWa\tBa\x17jV[\x90P\x015`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\t^Wa\t^a\x17jV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\t\x1FV[PP\x94\x93PPPPV[_\x80\x80\x80\x80`\xAE\x86\x14a\t\xAFW`@Qc`\r\x15Q`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x05>V[\x86\x86_\x81\x81\x10a\t\xC1Wa\t\xC1a\x17jV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`\n`\xF8\x1B\x14\x15\x80a\n\x11WP\x86\x86`\x01\x81\x81\x10a\t\xF5Wa\t\xF5a\x17jV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`i`\xF8\x1B\x14\x15[\x80a\nFWP\x86\x86`k\x81\x81\x10a\n*Wa\n*a\x17jV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`\x12`\xF8\x1B\x14\x15[\x80a\n{WP\x86\x86`l\x81\x81\x10a\n_Wa\n_a\x17jV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`A`\xF8\x1B\x14\x15[\x15a\n\x99W`@Qc\xEF\x02\xC9\xBB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\n\xA8\x88\x88`\x03`\ta\x0CiV[P`\x01`\x01`@\x1B\x03\x16\x90P_a\n\xC1a\x03\xE8\x83a\x18\x90V[\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15a\n\xE8W`@QcT\x9A\x01\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x95P`\x0B\x88\x015\x94P`-\x88\x015\x93P\x85_a\x0B\x08\x8A\x8A`N`Ra\x0CiV[`\x01`\x01`@\x1B\x03\x90\x91\x16\x95P\x90P_\x8A\x8A`T\x81\x81\x10a\x0B+Wa\x0B+a\x17jV[\x91\x90\x91\x015`\xF8\x1C\x91PP`A\x81\x14a\x0B\\W`@QcO\xA8\x8D_`\xE1\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`$\x01a\x05>V[P\x96\x99\x95\x98\x94\x97P\x92\x95PPPP`U\x015\x90V[_`\x02a\x0B\x81`k\x82\x85\x87a\x18\x17V[`@Qa\x0B\x8F\x92\x91\x90a\x17DV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x0B\xAAW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xCD\x91\x90a\x17SV[\x93\x92PPPV[_`\xAD\x83\x015\x81\x1A`m\x84\x015`\x8D\x85\x015`\x1B\x83\x10\x15a\x0B\xF6W`\x1B\x83\x01\x92P[`\x1C\x83\x14`\x1B\x84\x14\x17a\x0C\nW__R__\xFD[`@Q\x87\x81R\x83` \x82\x01R\x82`@\x82\x01R\x81``\x82\x01R` \x81`\x80\x83`\x01Z\xFAa\x0C7W__R__\xFD[Q``\x1B\x97\x96PPPPPPPV[_\x80a\x0CW`\x01`\x01`\xC0\x1Ba\x18VV[\x83\x16`\xC0\x85\x90\x1B\x17\x91PP[\x92\x91PPV[_\x80\x80\x80\x80[`\n\x81\x10\x15a\r\x03W\x85\x87\x10a\x0C\x98W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x89\x89\x89a\x0C\xA5\x81a\x18>V[\x9AP\x81\x81\x10a\x0C\xB6Wa\x0C\xB6a\x17jV[`\x7F\x92\x015`\xF8\x1C\x91\x82\x16`\x01`\x01`@\x1B\x03\x86\x16\x1B\x95\x90\x95\x17\x94P\x90P`\x80\x81\x16_\x03a\x0C\xEDW\x83\x88\x95P\x95PPPPPa\r\x1DV[a\x0C\xF8`\x07\x84a\x18\xAFV[\x92PP`\x01\x01a\x0CoV[P`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x94P\x94\x92PPPV[_a\r1\x84\x83a\x18VV[\x83\x11\x15a\rQW`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xA6\x83\x85a\x18}V[_\x80\x80\x84\x81\x81[\x86\x83\x10\x15a\x0E;WP\x81_\x80a\rz\x8C\x8C\x85\x8Ca\x121V[\x96P\x90\x92P\x90P\x82\x85\x11a\r\x90Wa\r\x90a\x17\xD6V[\x81`\x01`\x01`@\x1B\x03\x16`\x0B\x14\x80\x15a\r\xB2WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x0E%W\x83\x15a\r\xD6W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x93Pa\r\xE6\x8C\x8C\x87\x8Ca\x10\xEBV[\x91\x99P\x97P\x94P_a\r\xFA\x8D\x8D\x8B\x8Ba\x12eV[\x90\x97P\x90P\x80a\x0E\x1DW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPa\x0E;V[a\x0E2\x8C\x8C\x87\x8C\x85a\x12\xFBV[\x94PPPa\rbV[\x81a\x0EYW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPP\x94P\x94P\x94\x91PPV[_\x80\x83\x81\x80\x82[\x86\x84\x10\x15a\x0F\x13WP\x82_\x80a\x0E\x85\x8C\x8C\x85\x8Ca\x121V[\x97P\x90\x92P\x90P\x82\x86\x11a\x0E\x9BWa\x0E\x9Ba\x17\xD6V[\x81`\x01`\x01`@\x1B\x03\x16`\x02\x14\x80\x15a\x0E\xBDWP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x0E\xFCWa\x0E\xCE\x8C\x8C\x88\x8Ca\x10\xEBV[\x97P\x90\x95P\x93P_\x80a\x0E\xE3\x8E\x8E\x89\x89a\x13\xA4V[\x91P\x91P\x81_\x14a\x0E\xF5W\x81\x99P\x80\x98P[PPa\x0F\x0CV[a\x0F\t\x8C\x8C\x88\x8C\x85a\x12\xFBV[\x95P[PPa\x0EmV[PPPP\x94P\x94\x92PPPV[_\x80\x80\x80\x85\x80[\x86\x82\x10\x15a\x10\xDEWP\x80_\x80a\x0F?\x8C\x8C\x85\x8Ca\x121V[\x95P\x90\x92P\x90P\x82\x84\x11a\x0FUWa\x0FUa\x17\xD6V[\x81`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x0FwWP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x0F\xF2W__a\x0F\x8A\x8E\x8E\x88\x8Ea\x10\xEBV[\x97P\x90\x92P\x90Pa\x0F\x9B\x82\x82a\x18VV[`\x15\x14a\x0F\xBBW`@Qc\x90'W\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xC6\x8E\x8E\x84a\x14lV[\x99P\x89_\x1A`A\x14a\x0F\xEBW`@Qc\xA4d]e`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPa\x10\xD7V[\x81`\x01`\x01`@\x1B\x03\x16`\x02\x14\x80\x15a\x10\x14WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x10\x88W__a\x10'\x8E\x8E\x88\x8Ea\x10\xEBV[\x97P\x90\x92P\x90Pa\x108\x82\x82a\x18VV[`\x15\x14a\x10XW`@Qcl\x8E\xE0\xD9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10c\x8E\x8E\x84a\x14lV[\x98P\x88_\x1A`A\x14a\x0F\xEBW`@QcTw\x93\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`@\x1B\x03\x16`\x04\x14\x80\x15a\x10\xAAWP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x10\xC7Wa\x10\xBB\x8C\x8C\x86\x8Ca\x10\xEBV[\x91\x97P\x95P\x93Pa\x10\xD7V[a\x10\xD4\x8C\x8C\x86\x8C\x85a\x12\xFBV[\x93P[PPa\x0F'V[PP\x94P\x94P\x94P\x94\x90PV[____a\x10\xFB\x88\x88\x88\x88a\x0CiV[\x96P\x86\x94P\x90P`\x01`\x01`@\x1B\x03\x81\x16a\x11\x16\x85\x87a\x18VV[\x81\x11\x15a\x116W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11@\x81\x88a\x18}V[\x93P\x83\x92PPP\x94P\x94P\x94\x91PPV[_\x80\x80\x84\x80[\x85\x82\x10\x15a\x12\x0EWP\x80_\x80a\x11o\x8B\x8B\x85\x8Ba\x121V[\x95P\x90\x92P\x90P\x82\x84\x11a\x11\x85Wa\x11\x85a\x17\xD6V[`\x01`\x01`@\x1B\x03\x81\x16a\x11\xF7W_a\x11\xA0\x8C\x8C\x87\x8Ca\x0CiV[\x95P\x90P`\x01`\x01`@\x1B\x03\x83\x16`\x02\x03a\x11\xD8W`\x01`\x01`@\x1B\x03\x81\x16\x15a\x11\xD3W_\x97PPPPPPPPa\x03\xA6V[a\x11\xF1V[\x82`\x01`\x01`@\x1B\x03\x16`\x03\x03a\x11\xF1W`\x01\x95P\x80\x96P[Pa\x12\x07V[a\x12\x04\x8B\x8B\x86\x8B\x85a\x12\xFBV[\x93P[PPa\x11WV[\x82\x80\x15a\x12$WP\x83`\x01`\x01`@\x1B\x03\x16`\x01\x14[\x99\x98PPPPPPPPPV[_____a\x12B\x89\x89\x89\x89a\x0CiV[`\x03\x82\x90\x1Cg\x1F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x9B`\x07\x90\x92\x16\x9AP\x98P\x96PPPPPPPV[_\x80\x83\x80[\x84\x82\x10\x15a\x12\xF0WP\x80_\x80a\x12\x82\x8A\x8A\x85\x8Aa\x121V[\x95P\x90\x92P\x90P\x82\x84\x11a\x12\x98Wa\x12\x98a\x17\xD6V[\x81`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x12\xB8WP`\x01`\x01`@\x1B\x03\x81\x16\x15[\x15a\x12\xDAWa\x12\xC9\x8A\x8A\x86\x8Aa\x0CiV[P\x95P`\x01\x94Pa\r\x1D\x93PPPPV[a\x12\xE7\x8A\x8A\x86\x8A\x85a\x12\xFBV[\x93PPPa\x12jV[PP\x94P\x94\x92PPPV[_`\x01`\x01`@\x1B\x03\x82\x16a\x13\x1DWa\x13\x16\x86\x86\x86\x86a\x14\xA3V[\x90Pa\x13\x9BV[`\x01\x19`\x01`\x01`@\x1B\x03\x83\x16\x01a\x13GW_a\x13<\x87\x87\x87\x87a\x10\xEBV[P\x92Pa\x13\x9B\x91PPV[`\x04\x19`\x01`\x01`@\x1B\x03\x83\x16\x01a\x13eWa\x13\x16\x84`\x04\x85a\r&V[_\x19`\x01`\x01`@\x1B\x03\x83\x16\x01a\x13\x82Wa\x13\x16\x84`\x08\x85a\r&V[`@Qc\xA5\xA5\xFCC`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x95\x94PPPPPV[_\x80\x83\x80[\x84\x82\x10\x15a\x12\xF0WP\x80_\x80a\x13\xC1\x8A\x8A\x85\x8Aa\x121V[\x95P\x90\x92P\x90P\x82\x84\x11a\x13\xD7Wa\x13\xD7a\x17\xD6V[\x81`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x13\xF9WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x14\x14Wa\x14\n\x8A\x8A\x86\x8Aa\x10\xEBV[P\x94Pa\x14e\x90PV[\x81`\x01`\x01`@\x1B\x03\x16`\x02\x14\x80\x15a\x146WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x14UWa\x14G\x8A\x8A\x86\x8Aa\x10\xEBV[P\x90\x96P\x94P\x84\x93Pa\x14eV[a\x14b\x8A\x8A\x86\x8A\x85a\x12\xFBV[\x93P[PPa\x13\xA9V[_\x82a\x14y\x83`\x15a\x18}V[\x11\x15a\x14\x98W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x91\x90\x91\x015\x91\x90PV[_\x80[`\n\x81\x10\x15a\r\x03W\x82\x84\x10a\x14\xCFW`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x86\x86\x86a\x14\xDC\x81a\x18>V[\x97P\x81\x81\x10a\x14\xEDWa\x14\xEDa\x17jV[\x91\x90\x91\x015`\xF8\x1C\x91PP`\x80\x81\x16_\x03a\x15\x0CW\x84\x92PPPa\x03\xA6V[P`\x01\x01a\x14\xA6V[`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\x01`\x01`X\x1B\x03\x19\x16\x81R` \x01_`\x01`\x01`X\x1B\x03\x19\x16\x81R` \x01``\x81RP\x90V[_` \x82\x84\x03\x12\x15a\x15pW__\xFD[P5\x91\x90PV[__\x83`\x1F\x84\x01\x12a\x15\x87W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\x9DW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x15\xB7W__\xFD[\x92P\x92\x90PV[______`\x80\x87\x89\x03\x12\x15a\x15\xD3W__\xFD[\x865`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\xE8W__\xFD[\x87\x01a\x02\x80\x81\x01\x89\x10\x15a\x15\xFAW__\xFD[\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\x14W__\xFD[\x87\x01`\x1F\x81\x01\x89\x13a\x16$W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x169W__\xFD[\x89` \x82\x84\x01\x01\x11\x15a\x16JW__\xFD[` \x91\x90\x91\x01\x95P\x93P`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16lW__\xFD[a\x16x\x89\x82\x8A\x01a\x15wV[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[` \x81R\x81Q` \x82\x01R` \x82\x01Q`@\x82\x01Rc\xFF\xFF\xFF\xFF`@\x83\x01Q\x16``\x82\x01R`\x01`\x01`X\x1B\x03\x19``\x83\x01Q\x16`\x80\x82\x01R`\x01`\x01`X\x1B\x03\x19`\x80\x83\x01Q\x16`\xA0\x82\x01R_`\xA0\x83\x01Q`\xC0\x80\x84\x01R\x80Q\x80`\xE0\x85\x01R\x80` \x83\x01a\x01\0\x86\x01^_a\x01\0\x82\x86\x01\x01Ra\x01\0`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x17-W__\xFD[\x815`\x01`\x01``\x1B\x03\x19\x81\x16\x81\x14a\x0B\xCDW__\xFD[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_` \x82\x84\x03\x12\x15a\x17cW__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x17\x93W__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x17\xACW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x15\xB7W__\xFD[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0CcWa\x0Cca\x17\xEAV[__\x85\x85\x11\x15a\x18%W__\xFD[\x83\x86\x11\x15a\x181W__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[_`\x01\x82\x01a\x18OWa\x18Oa\x17\xEAV[P`\x01\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x0CcWa\x0Cca\x17\xEAV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0CcWa\x0Cca\x17\xEAV[_\x82a\x18\xAAWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0CcWa\x0Cca\x17\xEAV\xFE\xA1dsolcC\0\x08\x1B\0\n",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `DuplicateSr(bytes20)` and selector `0xb07511fe`.
```solidity
error DuplicateSr(bytes20 sr);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DuplicateSr {
        #[allow(missing_docs)]
        pub sr: alloy::sol_types::private::FixedBytes<20>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<20>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<20>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<DuplicateSr> for UnderlyingRustTuple<'_> {
            fn from(value: DuplicateSr) -> Self {
                (value.sr,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DuplicateSr {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { sr: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DuplicateSr {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DuplicateSr(bytes20)";
            const SELECTOR: [u8; 4] = [176u8, 117u8, 17u8, 254u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        20,
                    > as alloy_sol_types::SolType>::tokenize(&self.sr),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidBlockSequence()` and selector `0xe14a7931`.
```solidity
error InvalidBlockSequence();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBlockSequence;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidBlockSequence> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBlockSequence) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBlockSequence {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBlockSequence {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBlockSequence()";
            const SELECTOR: [u8; 4] = [225u8, 74u8, 121u8, 49u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidEncodedBlockLength(uint256)` and selector `0x600d1551`.
```solidity
error InvalidEncodedBlockLength(uint256 got);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidEncodedBlockLength {
        #[allow(missing_docs)]
        pub got: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidEncodedBlockLength>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidEncodedBlockLength) -> Self {
                (value.got,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidEncodedBlockLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { got: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidEncodedBlockLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidEncodedBlockLength(uint256)";
            const SELECTOR: [u8; 4] = [96u8, 13u8, 21u8, 81u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.got),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidHeaderPrefix()` and selector `0xef02c9bb`.
```solidity
error InvalidHeaderPrefix();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidHeaderPrefix;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidHeaderPrefix> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidHeaderPrefix) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidHeaderPrefix {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidHeaderPrefix {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidHeaderPrefix()";
            const SELECTOR: [u8; 4] = [239u8, 2u8, 201u8, 187u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidTxMerkleProof()` and selector `0x075f374c`.
```solidity
error InvalidTxMerkleProof();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidTxMerkleProof;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidTxMerkleProof> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidTxMerkleProof) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidTxMerkleProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidTxMerkleProof {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidTxMerkleProof()";
            const SELECTOR: [u8; 4] = [7u8, 95u8, 55u8, 76u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidWitnessAddressPrefix(uint8)` and selector `0x9f511abe`.
```solidity
error InvalidWitnessAddressPrefix(uint8 got);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidWitnessAddressPrefix {
        #[allow(missing_docs)]
        pub got: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u8,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidWitnessAddressPrefix>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidWitnessAddressPrefix) -> Self {
                (value.got,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidWitnessAddressPrefix {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { got: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidWitnessAddressPrefix {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidWitnessAddressPrefix(uint8)";
            const SELECTOR: [u8; 4] = [159u8, 81u8, 26u8, 190u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.got),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidWitnessSignature()` and selector `0x13d6dc73`.
```solidity
error InvalidWitnessSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidWitnessSignature;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidWitnessSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidWitnessSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidWitnessSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidWitnessSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidWitnessSignature()";
            const SELECTOR: [u8; 4] = [19u8, 214u8, 220u8, 115u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotTriggerSmartContract()` and selector `0xc1b8626c`.
```solidity
error NotTriggerSmartContract();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotTriggerSmartContract;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotTriggerSmartContract> for UnderlyingRustTuple<'_> {
            fn from(value: NotTriggerSmartContract) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotTriggerSmartContract {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotTriggerSmartContract {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotTriggerSmartContract()";
            const SELECTOR: [u8; 4] = [193u8, 184u8, 98u8, 108u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ProtoInvalidWireType()` and selector `0xa5a5fc43`.
```solidity
error ProtoInvalidWireType();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ProtoInvalidWireType;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ProtoInvalidWireType> for UnderlyingRustTuple<'_> {
            fn from(value: ProtoInvalidWireType) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ProtoInvalidWireType {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ProtoInvalidWireType {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ProtoInvalidWireType()";
            const SELECTOR: [u8; 4] = [165u8, 165u8, 252u8, 67u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ProtoTruncated()` and selector `0x3ffd6659`.
```solidity
error ProtoTruncated();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ProtoTruncated;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ProtoTruncated> for UnderlyingRustTuple<'_> {
            fn from(value: ProtoTruncated) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ProtoTruncated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ProtoTruncated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ProtoTruncated()";
            const SELECTOR: [u8; 4] = [63u8, 253u8, 102u8, 89u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `SrSetNotSorted(uint256,bytes20,bytes20)` and selector `0x4c919300`.
```solidity
error SrSetNotSorted(uint256 index, bytes20 prev, bytes20 next);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SrSetNotSorted {
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub prev: alloy::sol_types::private::FixedBytes<20>,
        #[allow(missing_docs)]
        pub next: alloy::sol_types::private::FixedBytes<20>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::FixedBytes<20>,
            alloy::sol_types::sol_data::FixedBytes<20>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::FixedBytes<20>,
            alloy::sol_types::private::FixedBytes<20>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SrSetNotSorted> for UnderlyingRustTuple<'_> {
            fn from(value: SrSetNotSorted) -> Self {
                (value.index, value.prev, value.next)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SrSetNotSorted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    index: tuple.0,
                    prev: tuple.1,
                    next: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SrSetNotSorted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SrSetNotSorted(uint256,bytes20,bytes20)";
            const SELECTOR: [u8; 4] = [76u8, 145u8, 147u8, 0u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                    <alloy::sol_types::sol_data::FixedBytes<
                        20,
                    > as alloy_sol_types::SolType>::tokenize(&self.prev),
                    <alloy::sol_types::sol_data::FixedBytes<
                        20,
                    > as alloy_sol_types::SolType>::tokenize(&self.next),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `TimestampOverflow()` and selector `0x549a0197`.
```solidity
error TimestampOverflow();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimestampOverflow;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TimestampOverflow> for UnderlyingRustTuple<'_> {
            fn from(value: TimestampOverflow) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TimestampOverflow {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimestampOverflow {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimestampOverflow()";
            const SELECTOR: [u8; 4] = [84u8, 154u8, 1u8, 151u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `TronInvalidContractLength()` and selector `0xd91dc1b2`.
```solidity
error TronInvalidContractLength();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TronInvalidContractLength;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TronInvalidContractLength>
        for UnderlyingRustTuple<'_> {
            fn from(value: TronInvalidContractLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TronInvalidContractLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TronInvalidContractLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TronInvalidContractLength()";
            const SELECTOR: [u8; 4] = [217u8, 29u8, 193u8, 178u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `TronInvalidContractPrefix()` and selector `0xa8ef2756`.
```solidity
error TronInvalidContractPrefix();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TronInvalidContractPrefix;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TronInvalidContractPrefix>
        for UnderlyingRustTuple<'_> {
            fn from(value: TronInvalidContractPrefix) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TronInvalidContractPrefix {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TronInvalidContractPrefix {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TronInvalidContractPrefix()";
            const SELECTOR: [u8; 4] = [168u8, 239u8, 39u8, 86u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `TronInvalidOwnerLength()` and selector `0x902757b1`.
```solidity
error TronInvalidOwnerLength();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TronInvalidOwnerLength;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TronInvalidOwnerLength> for UnderlyingRustTuple<'_> {
            fn from(value: TronInvalidOwnerLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TronInvalidOwnerLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TronInvalidOwnerLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TronInvalidOwnerLength()";
            const SELECTOR: [u8; 4] = [144u8, 39u8, 87u8, 177u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `TronInvalidOwnerPrefix()` and selector `0xa4645d65`.
```solidity
error TronInvalidOwnerPrefix();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TronInvalidOwnerPrefix;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TronInvalidOwnerPrefix> for UnderlyingRustTuple<'_> {
            fn from(value: TronInvalidOwnerPrefix) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TronInvalidOwnerPrefix {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TronInvalidOwnerPrefix {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TronInvalidOwnerPrefix()";
            const SELECTOR: [u8; 4] = [164u8, 100u8, 93u8, 101u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `TronTxNotSuccessful()` and selector `0xc2c062d1`.
```solidity
error TronTxNotSuccessful();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TronTxNotSuccessful;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TronTxNotSuccessful> for UnderlyingRustTuple<'_> {
            fn from(value: TronTxNotSuccessful) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TronTxNotSuccessful {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TronTxNotSuccessful {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TronTxNotSuccessful()";
            const SELECTOR: [u8; 4] = [194u8, 192u8, 98u8, 209u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UnknownSr(bytes20)` and selector `0xcd42738b`.
```solidity
error UnknownSr(bytes20 sr);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnknownSr {
        #[allow(missing_docs)]
        pub sr: alloy::sol_types::private::FixedBytes<20>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<20>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<20>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnknownSr> for UnderlyingRustTuple<'_> {
            fn from(value: UnknownSr) -> Self {
                (value.sr,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnknownSr {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { sr: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnknownSr {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnknownSr(bytes20)";
            const SELECTOR: [u8; 4] = [205u8, 66u8, 115u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        20,
                    > as alloy_sol_types::SolType>::tokenize(&self.sr),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Self::new)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(bytes20[27] _srs, bytes20[27] _witnessDelegatees);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _srs: [alloy::sol_types::private::FixedBytes<20>; 27usize],
        #[allow(missing_docs)]
        pub _witnessDelegatees: [alloy::sol_types::private::FixedBytes<20>; 27usize],
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<20>,
                    27usize,
                >,
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<20>,
                    27usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<20>; 27usize],
                [alloy::sol_types::private::FixedBytes<20>; 27usize],
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._srs, value._witnessDelegatees)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _srs: tuple.0,
                        _witnessDelegatees: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<20>,
                    27usize,
                >,
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<20>,
                    27usize,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::FixedBytes<20>,
                        27usize,
                    > as alloy_sol_types::SolType>::tokenize(&self._srs),
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::FixedBytes<20>,
                        27usize,
                    > as alloy_sol_types::SolType>::tokenize(&self._witnessDelegatees),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `readTriggerSmartContract(bytes[20],bytes,bytes32[],uint256)` and selector `0x49cd9f98`.
```solidity
function readTriggerSmartContract(bytes[20] memory blocks, bytes memory encodedTx, bytes32[] memory proof, uint256 index) external view returns (ITronTxReader.TriggerSmartContract memory callData);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct readTriggerSmartContractCall {
        #[allow(missing_docs)]
        pub blocks: [alloy::sol_types::private::Bytes; 20usize],
        #[allow(missing_docs)]
        pub encodedTx: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub proof: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`readTriggerSmartContract(bytes[20],bytes,bytes32[],uint256)`](readTriggerSmartContractCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct readTriggerSmartContractReturn {
        #[allow(missing_docs)]
        pub callData: <ITronTxReader::TriggerSmartContract as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Bytes,
                    20usize,
                >,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::Bytes; 20usize],
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<readTriggerSmartContractCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: readTriggerSmartContractCall) -> Self {
                    (value.blocks, value.encodedTx, value.proof, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for readTriggerSmartContractCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blocks: tuple.0,
                        encodedTx: tuple.1,
                        proof: tuple.2,
                        index: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (ITronTxReader::TriggerSmartContract,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ITronTxReader::TriggerSmartContract as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<readTriggerSmartContractReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: readTriggerSmartContractReturn) -> Self {
                    (value.callData,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for readTriggerSmartContractReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { callData: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for readTriggerSmartContractCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Bytes,
                    20usize,
                >,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <ITronTxReader::TriggerSmartContract as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (ITronTxReader::TriggerSmartContract,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "readTriggerSmartContract(bytes[20],bytes,bytes32[],uint256)";
            const SELECTOR: [u8; 4] = [73u8, 205u8, 159u8, 152u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Bytes,
                        20usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.blocks),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.encodedTx,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.proof),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <ITronTxReader::TriggerSmartContract as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: readTriggerSmartContractReturn = r.into();
                        r.callData
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: readTriggerSmartContractReturn = r.into();
                        r.callData
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `srIndexPlus1(bytes20)` and selector `0x7e670eb3`.
```solidity
function srIndexPlus1(bytes20) external view returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct srIndexPlus1Call(pub alloy::sol_types::private::FixedBytes<20>);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`srIndexPlus1(bytes20)`](srIndexPlus1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct srIndexPlus1Return {
        #[allow(missing_docs)]
        pub _0: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<20>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<20>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<srIndexPlus1Call> for UnderlyingRustTuple<'_> {
                fn from(value: srIndexPlus1Call) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for srIndexPlus1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<srIndexPlus1Return> for UnderlyingRustTuple<'_> {
                fn from(value: srIndexPlus1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for srIndexPlus1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for srIndexPlus1Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<20>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u8;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "srIndexPlus1(bytes20)";
            const SELECTOR: [u8; 4] = [126u8, 103u8, 14u8, 179u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        20,
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: srIndexPlus1Return = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: srIndexPlus1Return = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `srs(uint256)` and selector `0x39adfeff`.
```solidity
function srs(uint256) external view returns (bytes20);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct srsCall(pub alloy::sol_types::private::primitives::aliases::U256);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`srs(uint256)`](srsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct srsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<20>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<srsCall> for UnderlyingRustTuple<'_> {
                fn from(value: srsCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for srsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<20>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<20>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<srsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: srsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for srsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for srsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<20>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<20>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "srs(uint256)";
            const SELECTOR: [u8; 4] = [57u8, 173u8, 254u8, 255u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        20,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: srsReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: srsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `witnessDelegatees(uint256)` and selector `0x12d713c2`.
```solidity
function witnessDelegatees(uint256) external view returns (bytes20);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct witnessDelegateesCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`witnessDelegatees(uint256)`](witnessDelegateesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct witnessDelegateesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<20>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<witnessDelegateesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: witnessDelegateesCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for witnessDelegateesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<20>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<20>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<witnessDelegateesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: witnessDelegateesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for witnessDelegateesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for witnessDelegateesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<20>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<20>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "witnessDelegatees(uint256)";
            const SELECTOR: [u8; 4] = [18u8, 215u8, 19u8, 194u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        20,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: witnessDelegateesReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: witnessDelegateesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    ///Container for all the [`StatefulTronTxReader`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum StatefulTronTxReaderCalls {
        #[allow(missing_docs)]
        readTriggerSmartContract(readTriggerSmartContractCall),
        #[allow(missing_docs)]
        srIndexPlus1(srIndexPlus1Call),
        #[allow(missing_docs)]
        srs(srsCall),
        #[allow(missing_docs)]
        witnessDelegatees(witnessDelegateesCall),
    }
    impl StatefulTronTxReaderCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [18u8, 215u8, 19u8, 194u8],
            [57u8, 173u8, 254u8, 255u8],
            [73u8, 205u8, 159u8, 152u8],
            [126u8, 103u8, 14u8, 179u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(witnessDelegatees),
            ::core::stringify!(srs),
            ::core::stringify!(readTriggerSmartContract),
            ::core::stringify!(srIndexPlus1),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <witnessDelegateesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <srsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <readTriggerSmartContractCall as alloy_sol_types::SolCall>::SIGNATURE,
            <srIndexPlus1Call as alloy_sol_types::SolCall>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for StatefulTronTxReaderCalls {
        const NAME: &'static str = "StatefulTronTxReaderCalls";
        const MIN_DATA_LENGTH: usize = 32usize;
        const COUNT: usize = 4usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::readTriggerSmartContract(_) => {
                    <readTriggerSmartContractCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::srIndexPlus1(_) => {
                    <srIndexPlus1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::srs(_) => <srsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::witnessDelegatees(_) => {
                    <witnessDelegateesCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<StatefulTronTxReaderCalls>] = &[
                {
                    fn witnessDelegatees(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderCalls> {
                        <witnessDelegateesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderCalls::witnessDelegatees)
                    }
                    witnessDelegatees
                },
                {
                    fn srs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderCalls> {
                        <srsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(StatefulTronTxReaderCalls::srs)
                    }
                    srs
                },
                {
                    fn readTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderCalls> {
                        <readTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderCalls::readTriggerSmartContract)
                    }
                    readTriggerSmartContract
                },
                {
                    fn srIndexPlus1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderCalls> {
                        <srIndexPlus1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderCalls::srIndexPlus1)
                    }
                    srIndexPlus1
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<StatefulTronTxReaderCalls>] = &[
                {
                    fn witnessDelegatees(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderCalls> {
                        <witnessDelegateesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderCalls::witnessDelegatees)
                    }
                    witnessDelegatees
                },
                {
                    fn srs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderCalls> {
                        <srsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderCalls::srs)
                    }
                    srs
                },
                {
                    fn readTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderCalls> {
                        <readTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderCalls::readTriggerSmartContract)
                    }
                    readTriggerSmartContract
                },
                {
                    fn srIndexPlus1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderCalls> {
                        <srIndexPlus1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderCalls::srIndexPlus1)
                    }
                    srIndexPlus1
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::readTriggerSmartContract(inner) => {
                    <readTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::srIndexPlus1(inner) => {
                    <srIndexPlus1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::srs(inner) => {
                    <srsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::witnessDelegatees(inner) => {
                    <witnessDelegateesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::readTriggerSmartContract(inner) => {
                    <readTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::srIndexPlus1(inner) => {
                    <srIndexPlus1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::srs(inner) => {
                    <srsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::witnessDelegatees(inner) => {
                    <witnessDelegateesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`StatefulTronTxReader`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum StatefulTronTxReaderErrors {
        #[allow(missing_docs)]
        DuplicateSr(DuplicateSr),
        #[allow(missing_docs)]
        InvalidBlockSequence(InvalidBlockSequence),
        #[allow(missing_docs)]
        InvalidEncodedBlockLength(InvalidEncodedBlockLength),
        #[allow(missing_docs)]
        InvalidHeaderPrefix(InvalidHeaderPrefix),
        #[allow(missing_docs)]
        InvalidTxMerkleProof(InvalidTxMerkleProof),
        #[allow(missing_docs)]
        InvalidWitnessAddressPrefix(InvalidWitnessAddressPrefix),
        #[allow(missing_docs)]
        InvalidWitnessSignature(InvalidWitnessSignature),
        #[allow(missing_docs)]
        NotTriggerSmartContract(NotTriggerSmartContract),
        #[allow(missing_docs)]
        ProtoInvalidWireType(ProtoInvalidWireType),
        #[allow(missing_docs)]
        ProtoTruncated(ProtoTruncated),
        #[allow(missing_docs)]
        SrSetNotSorted(SrSetNotSorted),
        #[allow(missing_docs)]
        TimestampOverflow(TimestampOverflow),
        #[allow(missing_docs)]
        TronInvalidContractLength(TronInvalidContractLength),
        #[allow(missing_docs)]
        TronInvalidContractPrefix(TronInvalidContractPrefix),
        #[allow(missing_docs)]
        TronInvalidOwnerLength(TronInvalidOwnerLength),
        #[allow(missing_docs)]
        TronInvalidOwnerPrefix(TronInvalidOwnerPrefix),
        #[allow(missing_docs)]
        TronTxNotSuccessful(TronTxNotSuccessful),
        #[allow(missing_docs)]
        UnknownSr(UnknownSr),
    }
    impl StatefulTronTxReaderErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [7u8, 95u8, 55u8, 76u8],
            [19u8, 214u8, 220u8, 115u8],
            [63u8, 253u8, 102u8, 89u8],
            [76u8, 145u8, 147u8, 0u8],
            [84u8, 154u8, 1u8, 151u8],
            [96u8, 13u8, 21u8, 81u8],
            [144u8, 39u8, 87u8, 177u8],
            [159u8, 81u8, 26u8, 190u8],
            [164u8, 100u8, 93u8, 101u8],
            [165u8, 165u8, 252u8, 67u8],
            [168u8, 239u8, 39u8, 86u8],
            [176u8, 117u8, 17u8, 254u8],
            [193u8, 184u8, 98u8, 108u8],
            [194u8, 192u8, 98u8, 209u8],
            [205u8, 66u8, 115u8, 139u8],
            [217u8, 29u8, 193u8, 178u8],
            [225u8, 74u8, 121u8, 49u8],
            [239u8, 2u8, 201u8, 187u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(InvalidTxMerkleProof),
            ::core::stringify!(InvalidWitnessSignature),
            ::core::stringify!(ProtoTruncated),
            ::core::stringify!(SrSetNotSorted),
            ::core::stringify!(TimestampOverflow),
            ::core::stringify!(InvalidEncodedBlockLength),
            ::core::stringify!(TronInvalidOwnerLength),
            ::core::stringify!(InvalidWitnessAddressPrefix),
            ::core::stringify!(TronInvalidOwnerPrefix),
            ::core::stringify!(ProtoInvalidWireType),
            ::core::stringify!(TronInvalidContractPrefix),
            ::core::stringify!(DuplicateSr),
            ::core::stringify!(NotTriggerSmartContract),
            ::core::stringify!(TronTxNotSuccessful),
            ::core::stringify!(UnknownSr),
            ::core::stringify!(TronInvalidContractLength),
            ::core::stringify!(InvalidBlockSequence),
            ::core::stringify!(InvalidHeaderPrefix),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <InvalidTxMerkleProof as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidWitnessSignature as alloy_sol_types::SolError>::SIGNATURE,
            <ProtoTruncated as alloy_sol_types::SolError>::SIGNATURE,
            <SrSetNotSorted as alloy_sol_types::SolError>::SIGNATURE,
            <TimestampOverflow as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidEncodedBlockLength as alloy_sol_types::SolError>::SIGNATURE,
            <TronInvalidOwnerLength as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidWitnessAddressPrefix as alloy_sol_types::SolError>::SIGNATURE,
            <TronInvalidOwnerPrefix as alloy_sol_types::SolError>::SIGNATURE,
            <ProtoInvalidWireType as alloy_sol_types::SolError>::SIGNATURE,
            <TronInvalidContractPrefix as alloy_sol_types::SolError>::SIGNATURE,
            <DuplicateSr as alloy_sol_types::SolError>::SIGNATURE,
            <NotTriggerSmartContract as alloy_sol_types::SolError>::SIGNATURE,
            <TronTxNotSuccessful as alloy_sol_types::SolError>::SIGNATURE,
            <UnknownSr as alloy_sol_types::SolError>::SIGNATURE,
            <TronInvalidContractLength as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidBlockSequence as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidHeaderPrefix as alloy_sol_types::SolError>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for StatefulTronTxReaderErrors {
        const NAME: &'static str = "StatefulTronTxReaderErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 18usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::DuplicateSr(_) => {
                    <DuplicateSr as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBlockSequence(_) => {
                    <InvalidBlockSequence as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidEncodedBlockLength(_) => {
                    <InvalidEncodedBlockLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidHeaderPrefix(_) => {
                    <InvalidHeaderPrefix as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidTxMerkleProof(_) => {
                    <InvalidTxMerkleProof as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidWitnessAddressPrefix(_) => {
                    <InvalidWitnessAddressPrefix as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidWitnessSignature(_) => {
                    <InvalidWitnessSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotTriggerSmartContract(_) => {
                    <NotTriggerSmartContract as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ProtoInvalidWireType(_) => {
                    <ProtoInvalidWireType as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ProtoTruncated(_) => {
                    <ProtoTruncated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SrSetNotSorted(_) => {
                    <SrSetNotSorted as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TimestampOverflow(_) => {
                    <TimestampOverflow as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TronInvalidContractLength(_) => {
                    <TronInvalidContractLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TronInvalidContractPrefix(_) => {
                    <TronInvalidContractPrefix as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TronInvalidOwnerLength(_) => {
                    <TronInvalidOwnerLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TronInvalidOwnerPrefix(_) => {
                    <TronInvalidOwnerPrefix as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TronTxNotSuccessful(_) => {
                    <TronTxNotSuccessful as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnknownSr(_) => <UnknownSr as alloy_sol_types::SolError>::SELECTOR,
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors>] = &[
                {
                    fn InvalidTxMerkleProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <InvalidTxMerkleProof as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::InvalidTxMerkleProof)
                    }
                    InvalidTxMerkleProof
                },
                {
                    fn InvalidWitnessSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <InvalidWitnessSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::InvalidWitnessSignature)
                    }
                    InvalidWitnessSignature
                },
                {
                    fn ProtoTruncated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <ProtoTruncated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::ProtoTruncated)
                    }
                    ProtoTruncated
                },
                {
                    fn SrSetNotSorted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <SrSetNotSorted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::SrSetNotSorted)
                    }
                    SrSetNotSorted
                },
                {
                    fn TimestampOverflow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <TimestampOverflow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::TimestampOverflow)
                    }
                    TimestampOverflow
                },
                {
                    fn InvalidEncodedBlockLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <InvalidEncodedBlockLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::InvalidEncodedBlockLength)
                    }
                    InvalidEncodedBlockLength
                },
                {
                    fn TronInvalidOwnerLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <TronInvalidOwnerLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::TronInvalidOwnerLength)
                    }
                    TronInvalidOwnerLength
                },
                {
                    fn InvalidWitnessAddressPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <InvalidWitnessAddressPrefix as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::InvalidWitnessAddressPrefix)
                    }
                    InvalidWitnessAddressPrefix
                },
                {
                    fn TronInvalidOwnerPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <TronInvalidOwnerPrefix as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::TronInvalidOwnerPrefix)
                    }
                    TronInvalidOwnerPrefix
                },
                {
                    fn ProtoInvalidWireType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <ProtoInvalidWireType as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::ProtoInvalidWireType)
                    }
                    ProtoInvalidWireType
                },
                {
                    fn TronInvalidContractPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <TronInvalidContractPrefix as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::TronInvalidContractPrefix)
                    }
                    TronInvalidContractPrefix
                },
                {
                    fn DuplicateSr(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <DuplicateSr as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(StatefulTronTxReaderErrors::DuplicateSr)
                    }
                    DuplicateSr
                },
                {
                    fn NotTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <NotTriggerSmartContract as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::NotTriggerSmartContract)
                    }
                    NotTriggerSmartContract
                },
                {
                    fn TronTxNotSuccessful(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <TronTxNotSuccessful as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::TronTxNotSuccessful)
                    }
                    TronTxNotSuccessful
                },
                {
                    fn UnknownSr(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <UnknownSr as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(StatefulTronTxReaderErrors::UnknownSr)
                    }
                    UnknownSr
                },
                {
                    fn TronInvalidContractLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <TronInvalidContractLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::TronInvalidContractLength)
                    }
                    TronInvalidContractLength
                },
                {
                    fn InvalidBlockSequence(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <InvalidBlockSequence as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::InvalidBlockSequence)
                    }
                    InvalidBlockSequence
                },
                {
                    fn InvalidHeaderPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <InvalidHeaderPrefix as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::InvalidHeaderPrefix)
                    }
                    InvalidHeaderPrefix
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors>] = &[
                {
                    fn InvalidTxMerkleProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <InvalidTxMerkleProof as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::InvalidTxMerkleProof)
                    }
                    InvalidTxMerkleProof
                },
                {
                    fn InvalidWitnessSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <InvalidWitnessSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::InvalidWitnessSignature)
                    }
                    InvalidWitnessSignature
                },
                {
                    fn ProtoTruncated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <ProtoTruncated as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::ProtoTruncated)
                    }
                    ProtoTruncated
                },
                {
                    fn SrSetNotSorted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <SrSetNotSorted as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::SrSetNotSorted)
                    }
                    SrSetNotSorted
                },
                {
                    fn TimestampOverflow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <TimestampOverflow as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::TimestampOverflow)
                    }
                    TimestampOverflow
                },
                {
                    fn InvalidEncodedBlockLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <InvalidEncodedBlockLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::InvalidEncodedBlockLength)
                    }
                    InvalidEncodedBlockLength
                },
                {
                    fn TronInvalidOwnerLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <TronInvalidOwnerLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::TronInvalidOwnerLength)
                    }
                    TronInvalidOwnerLength
                },
                {
                    fn InvalidWitnessAddressPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <InvalidWitnessAddressPrefix as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::InvalidWitnessAddressPrefix)
                    }
                    InvalidWitnessAddressPrefix
                },
                {
                    fn TronInvalidOwnerPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <TronInvalidOwnerPrefix as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::TronInvalidOwnerPrefix)
                    }
                    TronInvalidOwnerPrefix
                },
                {
                    fn ProtoInvalidWireType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <ProtoInvalidWireType as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::ProtoInvalidWireType)
                    }
                    ProtoInvalidWireType
                },
                {
                    fn TronInvalidContractPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <TronInvalidContractPrefix as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::TronInvalidContractPrefix)
                    }
                    TronInvalidContractPrefix
                },
                {
                    fn DuplicateSr(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <DuplicateSr as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::DuplicateSr)
                    }
                    DuplicateSr
                },
                {
                    fn NotTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <NotTriggerSmartContract as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::NotTriggerSmartContract)
                    }
                    NotTriggerSmartContract
                },
                {
                    fn TronTxNotSuccessful(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <TronTxNotSuccessful as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::TronTxNotSuccessful)
                    }
                    TronTxNotSuccessful
                },
                {
                    fn UnknownSr(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <UnknownSr as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::UnknownSr)
                    }
                    UnknownSr
                },
                {
                    fn TronInvalidContractLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <TronInvalidContractLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::TronInvalidContractLength)
                    }
                    TronInvalidContractLength
                },
                {
                    fn InvalidBlockSequence(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <InvalidBlockSequence as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::InvalidBlockSequence)
                    }
                    InvalidBlockSequence
                },
                {
                    fn InvalidHeaderPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderErrors> {
                        <InvalidHeaderPrefix as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderErrors::InvalidHeaderPrefix)
                    }
                    InvalidHeaderPrefix
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::DuplicateSr(inner) => {
                    <DuplicateSr as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidBlockSequence(inner) => {
                    <InvalidBlockSequence as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidEncodedBlockLength(inner) => {
                    <InvalidEncodedBlockLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidHeaderPrefix(inner) => {
                    <InvalidHeaderPrefix as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidTxMerkleProof(inner) => {
                    <InvalidTxMerkleProof as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidWitnessAddressPrefix(inner) => {
                    <InvalidWitnessAddressPrefix as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidWitnessSignature(inner) => {
                    <InvalidWitnessSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotTriggerSmartContract(inner) => {
                    <NotTriggerSmartContract as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ProtoInvalidWireType(inner) => {
                    <ProtoInvalidWireType as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ProtoTruncated(inner) => {
                    <ProtoTruncated as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SrSetNotSorted(inner) => {
                    <SrSetNotSorted as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TimestampOverflow(inner) => {
                    <TimestampOverflow as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TronInvalidContractLength(inner) => {
                    <TronInvalidContractLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TronInvalidContractPrefix(inner) => {
                    <TronInvalidContractPrefix as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TronInvalidOwnerLength(inner) => {
                    <TronInvalidOwnerLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TronInvalidOwnerPrefix(inner) => {
                    <TronInvalidOwnerPrefix as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TronTxNotSuccessful(inner) => {
                    <TronTxNotSuccessful as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnknownSr(inner) => {
                    <UnknownSr as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::DuplicateSr(inner) => {
                    <DuplicateSr as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidBlockSequence(inner) => {
                    <InvalidBlockSequence as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidEncodedBlockLength(inner) => {
                    <InvalidEncodedBlockLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidHeaderPrefix(inner) => {
                    <InvalidHeaderPrefix as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidTxMerkleProof(inner) => {
                    <InvalidTxMerkleProof as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidWitnessAddressPrefix(inner) => {
                    <InvalidWitnessAddressPrefix as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidWitnessSignature(inner) => {
                    <InvalidWitnessSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotTriggerSmartContract(inner) => {
                    <NotTriggerSmartContract as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ProtoInvalidWireType(inner) => {
                    <ProtoInvalidWireType as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ProtoTruncated(inner) => {
                    <ProtoTruncated as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SrSetNotSorted(inner) => {
                    <SrSetNotSorted as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TimestampOverflow(inner) => {
                    <TimestampOverflow as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TronInvalidContractLength(inner) => {
                    <TronInvalidContractLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TronInvalidContractPrefix(inner) => {
                    <TronInvalidContractPrefix as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TronInvalidOwnerLength(inner) => {
                    <TronInvalidOwnerLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TronInvalidOwnerPrefix(inner) => {
                    <TronInvalidOwnerPrefix as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TronTxNotSuccessful(inner) => {
                    <TronTxNotSuccessful as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnknownSr(inner) => {
                    <UnknownSr as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`StatefulTronTxReader`](self) contract instance.

See the [wrapper's documentation](`StatefulTronTxReaderInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> StatefulTronTxReaderInstance<P, N> {
        StatefulTronTxReaderInstance::<P, N>::new(address, __provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        _srs: [alloy::sol_types::private::FixedBytes<20>; 27usize],
        _witnessDelegatees: [alloy::sol_types::private::FixedBytes<20>; 27usize],
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<StatefulTronTxReaderInstance<P, N>>,
    > {
        StatefulTronTxReaderInstance::<
            P,
            N,
        >::deploy(__provider, _srs, _witnessDelegatees)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        _srs: [alloy::sol_types::private::FixedBytes<20>; 27usize],
        _witnessDelegatees: [alloy::sol_types::private::FixedBytes<20>; 27usize],
    ) -> alloy_contract::RawCallBuilder<P, N> {
        StatefulTronTxReaderInstance::<
            P,
            N,
        >::deploy_builder(__provider, _srs, _witnessDelegatees)
    }
    /**A [`StatefulTronTxReader`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`StatefulTronTxReader`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct StatefulTronTxReaderInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for StatefulTronTxReaderInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("StatefulTronTxReaderInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StatefulTronTxReaderInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`StatefulTronTxReader`](self) contract instance.

See the [wrapper's documentation](`StatefulTronTxReaderInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            __provider: P,
            _srs: [alloy::sol_types::private::FixedBytes<20>; 27usize],
            _witnessDelegatees: [alloy::sol_types::private::FixedBytes<20>; 27usize],
        ) -> alloy_contract::Result<StatefulTronTxReaderInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                __provider,
                _srs,
                _witnessDelegatees,
            );
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            __provider: P,
            _srs: [alloy::sol_types::private::FixedBytes<20>; 27usize],
            _witnessDelegatees: [alloy::sol_types::private::FixedBytes<20>; 27usize],
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _srs,
                            _witnessDelegatees,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> StatefulTronTxReaderInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> StatefulTronTxReaderInstance<P, N> {
            StatefulTronTxReaderInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StatefulTronTxReaderInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`readTriggerSmartContract`] function.
        pub fn readTriggerSmartContract(
            &self,
            blocks: [alloy::sol_types::private::Bytes; 20usize],
            encodedTx: alloy::sol_types::private::Bytes,
            proof: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, readTriggerSmartContractCall, N> {
            self.call_builder(
                &readTriggerSmartContractCall {
                    blocks,
                    encodedTx,
                    proof,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`srIndexPlus1`] function.
        pub fn srIndexPlus1(
            &self,
            _0: alloy::sol_types::private::FixedBytes<20>,
        ) -> alloy_contract::SolCallBuilder<&P, srIndexPlus1Call, N> {
            self.call_builder(&srIndexPlus1Call(_0))
        }
        ///Creates a new call builder for the [`srs`] function.
        pub fn srs(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, srsCall, N> {
            self.call_builder(&srsCall(_0))
        }
        ///Creates a new call builder for the [`witnessDelegatees`] function.
        pub fn witnessDelegatees(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, witnessDelegateesCall, N> {
            self.call_builder(&witnessDelegateesCall(_0))
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StatefulTronTxReaderInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
