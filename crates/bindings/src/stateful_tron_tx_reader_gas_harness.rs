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

interface StatefulTronTxReaderGasHarness {
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

    function parseTriggerSmartContract(bytes memory encodedTx) external pure returns (ITronTxReader.TriggerSmartContract memory callData);
    function readTriggerSmartContract(bytes[20] memory blocks, bytes memory encodedTx, bytes32[] memory proof, uint256 index) external view returns (ITronTxReader.TriggerSmartContract memory callData);
    function srIndexPlus1(bytes20) external view returns (uint8);
    function srs(uint256) external view returns (bytes20);
    function verifyFirstBlockFinality(bytes[20] memory blocks) external view returns (uint256 blockNumber, uint32 blockTimestamp, bytes32 txTrieRoot);
    function verifyInclusion(bytes32 root, bytes memory encodedTx, bytes32[] memory proof, uint256 index) external pure returns (bool ok);
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
    "name": "parseTriggerSmartContract",
    "inputs": [
      {
        "name": "encodedTx",
        "type": "bytes",
        "internalType": "bytes"
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
    "stateMutability": "pure"
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
    "name": "verifyFirstBlockFinality",
    "inputs": [
      {
        "name": "blocks",
        "type": "bytes[20]",
        "internalType": "bytes[20]"
      }
    ],
    "outputs": [
      {
        "name": "blockNumber",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "blockTimestamp",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "txTrieRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "verifyInclusion",
    "inputs": [
      {
        "name": "root",
        "type": "bytes32",
        "internalType": "bytes32"
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
        "name": "ok",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "pure"
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
pub mod StatefulTronTxReaderGasHarness {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50604051611dfd380380611dfd83398101604081905261002e9161026a565b81516001600160601b0319165f908152602081905260409020805460ff19166001908117909155829082905b601b811015610142575f836100706001846102c7565b601b81106100805761008061029f565b602002015190505f8483601b811061009a5761009a61029f565b60200201519050606081811c9083901c106100e657604051624c919360e81b8152600481018490526001600160601b031980841660248301528216604482015260640160405180910390fd5b6100f18360016102e0565b5f5f8786601b81106101055761010561029f565b602090810291909101516001600160601b03191682528101919091526040015f20805460ff191660ff92909216919091179055505060010161005a565b50610150600183601b610168565b5061015e601c82601b610168565b50505050506102f3565b82601b81019282156101ab579160200282015b828111156101ab57825182546001600160a01b03191660609190911c17825560209092019160019091019061017b565b506101b79291506101bb565b5090565b5b808211156101b7575f81556001016101bc565b80516001600160601b0319811681146101e6575f5ffd5b919050565b5f82601f8301126101fa575f5ffd5b60405161036081016001600160401b038111828210171561022957634e487b7160e01b5f52604160045260245ffd5b6040528061036084018581111561023e575f5ffd5b845b8181101561025f57610251816101cf565b835260209283019201610240565b509195945050505050565b5f5f6106c0838503121561027c575f5ffd5b61028684846101eb565b91506102968461036085016101eb565b90509250929050565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b818103818111156102da576102da6102b3565b92915050565b808201808211156102da576102da6102b3565b611afd806103005f395ff3fe608060405234801561000f575f5ffd5b506004361061007a575f3560e01c806346e6d31a1161005857806346e6d31a146100e557806349cd9f981461010557806376099a06146101185780637e670eb31461014b575f5ffd5b806307ee97351461007e57806312d713c2146100a657806339adfeff146100d2575b5f5ffd5b61009161008c366004611744565b61017f565b60405190151581526020015b60405180910390f35b6100b96100b43660046117c3565b610219565b6040516001600160601b0319909116815260200161009d565b6100b96100e03660046117c3565b610232565b6100f86100f33660046117da565b610241565b60405161009d9190611818565b6100f86101133660046118b6565b61025c565b61012b610126366004611907565b61034a565b6040805193845263ffffffff90921660208401529082015260600161009d565b61016d610159366004611938565b5f6020819052908152604090205460ff1681565b60405160ff909116815260200161009d565b5f61020e8760028888604051610196929190611966565b602060405180830381855afa1580156101b1573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906101d49190611975565b8686808060200260200160405190810160405280939291908181526020018383602002808284375f92019190915250889250610363915050565b979650505050505050565b601c81601b8110610228575f80fd5b015460601b905081565b600181601b8110610228575f80fd5b610249611675565b6102538383610499565b90505b92915050565b610264611675565b5f5f5f6102708a61057b565b9250925092506103048160028b8b60405161028c929190611966565b602060405180830381855afa1580156102a7573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906102ca9190611975565b8989808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152508b9250610363915050565b610321576040516301d7cdd360e21b815260040160405180910390fd5b61032b8989610499565b60208101939093525063ffffffff166040820152979650505050505050565b5f5f5f6103568461057b565b9250925092509193909250565b5f83815b845181101561048b575f8582815181106103835761038361198c565b60200260200101519050816001901b85165f0361041057604080516020810185905290810182905260029060600160408051601f19818403018152908290526103cb916119a0565b602060405180830381855afa1580156103e6573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906104099190611975565b9250610482565b604080516020810183905290810184905260029060600160408051601f1981840301815290829052610441916119a0565b602060405180830381855afa15801561045c573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061047f9190611975565b92505b50600101610367565b50851490505b949350505050565b6104a1611675565b5f5f5f5f5f5f5f6104b28a8a6105f7565b9850965090508581118015906104c85750888611155b6104d4576104d46119b6565b6104e08a8a83896106d3565b9297509095509350915050811580156104f7575080155b156105155760405163306e189b60e21b815260040160405180910390fd5b6105218989878161079b565b61053e5760405163c2c062d160e01b815260040160405180910390fd5b8587526affffffffffffffffffffff1980851660608901528316608088015261056989898484610895565b60a08801525094979650505050505050565b5f80808080805b60148110156105ed575f5f5f5f5f6105bc8c87601481106105a5576105a561198c565b6020028101906105b591906119ca565b8a8a61097d565b94509450945094509450849750839650855f036105dd57829a508199508098505b5050505050806001019050610582565b5050509193909250565b5f808083801580610623575085855f8181106106155761061561198c565b9091013560f81c600a141590505b156106415760405163306e189b60e21b815260040160405180910390fd5b60015f61065088888486610aed565b9650869250905061066b826001600160401b03831685610baa565b9450600261067b86888a8c611a0c565b604051610689929190611966565b602060405180830381855afa1580156106a4573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906106c79190611975565b93505050509250925092565b5f5f5f5f5f5f5f6106e68b8b8b8b610bdf565b9194509250905081831080156106fc5750878211155b610708576107086119b6565b6001600160401b038116601f146107325760405163306e189b60e21b815260040160405180910390fd5b5f5f6107408d8d8787610cea565b909250905081158015610751575080155b1561076f5760405163306e189b60e21b815260040160405180910390fd5b61077b8d8d8484610da4565b809950819a50829b50839c50505050505050505050945094509450949050565b5f825b82841080156107c757508585858181106107ba576107ba61198c565b9091013560f81c60121490505b156108155750826107d781611a47565b93505f6107e687878787610aed565b955090508185116107f9576107f96119b6565b61080d85826001600160401b031686610baa565b94505061079e565b5f5b838510801561084057508686868181106108335761083361198c565b9091013560f81c602a1490505b1561088b5750600161085185611a47565b94505f5f61086189898989610f6f565b9850909250905061087489898484610fd5565b610884575f945050505050610491565b5050610817565b9695505050505050565b6060828210806108a457508382115b156108c257604051633ffd665960e01b815260040160405180910390fd5b5f6108cd8484611a5f565b9050806001600160401b038111156108e7576108e7611a72565b6040519080825280601f01601f191660200182016040528015610911576020820181803683370190505b5091505f5b8181101561097357868661092a8388611a86565b8181106109395761093961198c565b9050013560f81c60f81b8382815181106109555761095561198c565b60200101906001600160f81b03191690815f1a905350600101610916565b5050949350505050565b5f5f5f5f5f5f5f61098e8b8b6110b5565b9098509296509094509250905088158015906109aa5750818914155b156109c85760405163e14a793160e01b815260040160405180910390fd5b5f6109d38c8c6112a0565b90505f6109e1828e8e6112fc565b6001600160601b031984165f9081526020819052604081205491925060ff90911690819003610a345760405163cd42738b60e01b81526001600160601b0319851660048201526024015b60405180910390fd5b5f610a40600183611a99565b9050600160ff82161b8c811663ffffffff1615610a7c5760405163583a88ff60e11b81526001600160601b031987166004820152602401610a2b565b8c81179a506001600160601b03198416601c60ff8416601b8110610aa257610aa261198c565b015460601b6001600160601b03191614610acf576040516313d6dc7360e01b815260040160405180910390fd5b610ad98a8661136e565b9b5050505050505050945094509450945094565b5f808080805b600a811015610b8757858710610b1c57604051633ffd665960e01b815260040160405180910390fd5b5f898989610b2981611a47565b9a50818110610b3a57610b3a61198c565b607f92013560f81c9182166001600160401b0386161b9590951794509050608081165f03610b715783889550955050505050610ba1565b610b7c600784611ab2565b925050600101610af3565b50604051633ffd665960e01b815260040160405180910390fd5b94509492505050565b5f610bb58483611a5f565b831115610bd557604051633ffd665960e01b815260040160405180910390fd5b6104918385611a86565b5f80808481815b86831015610cbf5750815f80610bfe8c8c858c611391565b96509092509050828511610c1457610c146119b6565b816001600160401b0316600b148015610c3657506001600160401b0381166002145b15610ca9578315610c5a5760405163306e189b60e21b815260040160405180910390fd5b60019350610c6a8c8c878c610f6f565b919950975094505f610c7e8d8d8b8b6113c5565b909750905080610ca15760405163306e189b60e21b815260040160405180910390fd5b505050610cbf565b610cb68c8c878c8561145b565b94505050610be6565b81610cdd5760405163306e189b60e21b815260040160405180910390fd5b5050509450945094915050565b5f80838180825b86841015610d975750825f80610d098c8c858c611391565b97509092509050828611610d1f57610d1f6119b6565b816001600160401b03166002148015610d4157506001600160401b0381166002145b15610d8057610d528c8c888c610f6f565b975090955093505f80610d678e8e8989611504565b91509150815f14610d79578199508098505b5050610d90565b610d8d8c8c888c8561145b565b95505b5050610cf1565b5050505094509492505050565b5f80808085805b86821015610f625750805f80610dc38c8c858c611391565b95509092509050828411610dd957610dd96119b6565b816001600160401b03166001148015610dfb57506001600160401b0381166002145b15610e76575f5f610e0e8e8e888e610f6f565b97509092509050610e1f8282611a5f565b601514610e3f5760405163902757b160e01b815260040160405180910390fd5b610e4a8e8e846115cc565b9950895f1a604114610e6f5760405163a4645d6560e01b815260040160405180910390fd5b5050610f5b565b816001600160401b03166002148015610e9857506001600160401b0381166002145b15610f0c575f5f610eab8e8e888e610f6f565b97509092509050610ebc8282611a5f565b601514610edc57604051636c8ee0d960e11b815260040160405180910390fd5b610ee78e8e846115cc565b9850885f1a604114610e6f5760405163547793ab60e11b815260040160405180910390fd5b816001600160401b03166004148015610f2e57506001600160401b0381166002145b15610f4b57610f3f8c8c868c610f6f565b91975095509350610f5b565b610f588c8c868c8561145b565b93505b5050610dab565b5050945094509450949050565b5f5f5f5f610f7f88888888610aed565b965086945090506001600160401b038116610f9a8587611a5f565b811115610fba57604051633ffd665960e01b815260040160405180910390fd5b610fc48188611a86565b935083925050509450945094915050565b5f808084805b858210156110925750805f80610ff38b8b858b611391565b95509092509050828411611009576110096119b6565b6001600160401b03811661107b575f6110248c8c878c610aed565b955090506001600160401b03831660020361105c576001600160401b03811615611057575f975050505050505050610491565b611075565b826001600160401b031660030361107557600195508096505b5061108b565b6110888b8b868b8561145b565b93505b5050610fdb565b8280156110a85750836001600160401b03166001145b9998505050505050505050565b5f8080808060ae86146110de5760405163600d155160e01b815260048101879052602401610a2b565b86865f8181106110f0576110f061198c565b9050013560f81c60f81b6001600160f81b031916600a60f81b1415806111405750868660018181106111245761112461198c565b9050013560f81c60f81b6001600160f81b031916606960f81b14155b8061117557508686606b8181106111595761115961198c565b9050013560f81c60f81b6001600160f81b031916601260f81b14155b806111aa57508686606c81811061118e5761118e61198c565b9050013560f81c60f81b6001600160f81b031916604160f81b14155b156111c85760405163ef02c9bb60e01b815260040160405180910390fd5b5f6111d7888860036009610aed565b506001600160401b031690505f6111f06103e883611ad1565b905063ffffffff8111156112175760405163549a019760e01b815260040160405180910390fd5b9550600b8801359450602d8801359350855f6112378a8a604e6052610aed565b6001600160401b03909116955090505f8a8a605481811061125a5761125a61198c565b919091013560f81c9150506041811461128b57604051634fa88d5f60e11b815260ff82166004820152602401610a2b565b50969995989497509295505050506055013590565b5f60026112b0606b828587611a0c565b6040516112be929190611966565b602060405180830381855afa1580156112d9573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906102539190611975565b5f60ad830135811a606d840135608d850135601b83101561131e57601b830192505b601c8314601b841417611332575f5f525f5ffd5b60405187815283602082015282604082015281606082015260208160808360015afa61135f575f5f525f5ffd5b5160601b979650505050505050565b5f8061137f6001600160c01b611a5f565b60c085901b9084161791505092915050565b5f5f5f5f5f6113a289898989610aed565b600382901c671fffffffffffffff169b60079092169a5098509650505050505050565b5f8083805b848210156114505750805f806113e28a8a858a611391565b955090925090508284116113f8576113f86119b6565b816001600160401b0316600114801561141857506001600160401b038116155b1561143a576114298a8a868a610aed565b50955060019450610ba19350505050565b6114478a8a868a8561145b565b935050506113ca565b505094509492505050565b5f6001600160401b03821661147d5761147686868686611603565b90506114fb565b6001196001600160401b038316016114a7575f61149c87878787610f6f565b5092506114fb915050565b6004196001600160401b038316016114c55761147684600485610baa565b5f196001600160401b038316016114e25761147684600885610baa565b60405163a5a5fc4360e01b815260040160405180910390fd5b95945050505050565b5f8083805b848210156114505750805f806115218a8a858a611391565b95509092509050828411611537576115376119b6565b816001600160401b0316600114801561155957506001600160401b0381166002145b156115745761156a8a8a868a610f6f565b5094506115c59050565b816001600160401b0316600214801561159657506001600160401b0381166002145b156115b5576115a78a8a868a610f6f565b5090965094508493506115c5565b6115c28a8a868a8561145b565b93505b5050611509565b5f826115d9836015611a86565b11156115f857604051633ffd665960e01b815260040160405180910390fd5b509190910135919050565b5f805b600a811015610b875782841061162f57604051633ffd665960e01b815260040160405180910390fd5b5f86868661163c81611a47565b975081811061164d5761164d61198c565b919091013560f81c915050608081165f0361166c578492505050610491565b50600101611606565b6040518060c001604052805f81526020015f81526020015f63ffffffff1681526020015f6001600160581b03191681526020015f6001600160581b0319168152602001606081525090565b5f5f83601f8401126116d0575f5ffd5b5081356001600160401b038111156116e6575f5ffd5b6020830191508360208285010111156116fd575f5ffd5b9250929050565b5f5f83601f840112611714575f5ffd5b5081356001600160401b0381111561172a575f5ffd5b6020830191508360208260051b85010111156116fd575f5ffd5b5f5f5f5f5f5f60808789031215611759575f5ffd5b8635955060208701356001600160401b03811115611775575f5ffd5b61178189828a016116c0565b90965094505060408701356001600160401b0381111561179f575f5ffd5b6117ab89828a01611704565b979a9699509497949695606090950135949350505050565b5f602082840312156117d3575f5ffd5b5035919050565b5f5f602083850312156117eb575f5ffd5b82356001600160401b03811115611800575f5ffd5b61180c858286016116c0565b90969095509350505050565b60208152815160208201526020820151604082015263ffffffff60408301511660608201526001600160581b031960608301511660808201526001600160581b031960808301511660a08201525f60a083015160c08084015280518060e0850152806020830161010086015e5f6101008286010152610100601f19601f8301168501019250505092915050565b806102808101831015610256575f5ffd5b5f5f5f5f5f5f608087890312156118cb575f5ffd5b86356001600160401b038111156118e0575f5ffd5b6118ec89828a016118a5565b96505060208701356001600160401b03811115611775575f5ffd5b5f60208284031215611917575f5ffd5b81356001600160401b0381111561192c575f5ffd5b610491848285016118a5565b5f60208284031215611948575f5ffd5b81356001600160601b03198116811461195f575f5ffd5b9392505050565b818382375f9101908152919050565b5f60208284031215611985575f5ffd5b5051919050565b634e487b7160e01b5f52603260045260245ffd5b5f82518060208501845e5f920191825250919050565b634e487b7160e01b5f52600160045260245ffd5b5f5f8335601e198436030181126119df575f5ffd5b8301803591506001600160401b038211156119f8575f5ffd5b6020019150368190038213156116fd575f5ffd5b5f5f85851115611a1a575f5ffd5b83861115611a26575f5ffd5b5050820193919092039150565b634e487b7160e01b5f52601160045260245ffd5b5f60018201611a5857611a58611a33565b5060010190565b8181038181111561025657610256611a33565b634e487b7160e01b5f52604160045260245ffd5b8082018082111561025657610256611a33565b60ff828116828216039081111561025657610256611a33565b6001600160401b03818116838216019081111561025657610256611a33565b5f82611aeb57634e487b7160e01b5f52601260045260245ffd5b50049056fea164736f6c634300081b000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x1D\xFD8\x03\x80a\x1D\xFD\x839\x81\x01`@\x81\x90Ra\0.\x91a\x02jV[\x81Q`\x01`\x01``\x1B\x03\x19\x16_\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x82\x90\x82\x90[`\x1B\x81\x10\x15a\x01BW_\x83a\0p`\x01\x84a\x02\xC7V[`\x1B\x81\x10a\0\x80Wa\0\x80a\x02\x9FV[` \x02\x01Q\x90P_\x84\x83`\x1B\x81\x10a\0\x9AWa\0\x9Aa\x02\x9FV[` \x02\x01Q\x90P``\x81\x81\x1C\x90\x83\x90\x1C\x10a\0\xE6W`@QbL\x91\x93`\xE8\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01``\x1B\x03\x19\x80\x84\x16`$\x83\x01R\x82\x16`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[a\0\xF1\x83`\x01a\x02\xE0V[__\x87\x86`\x1B\x81\x10a\x01\x05Wa\x01\x05a\x02\x9FV[` \x90\x81\x02\x91\x90\x91\x01Q`\x01`\x01``\x1B\x03\x19\x16\x82R\x81\x01\x91\x90\x91R`@\x01_ \x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UPP`\x01\x01a\0ZV[Pa\x01P`\x01\x83`\x1Ba\x01hV[Pa\x01^`\x1C\x82`\x1Ba\x01hV[PPPPPa\x02\xF3V[\x82`\x1B\x81\x01\x92\x82\x15a\x01\xABW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x01\xABW\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x91\x90\x91\x1C\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\x01{V[Pa\x01\xB7\x92\x91Pa\x01\xBBV[P\x90V[[\x80\x82\x11\x15a\x01\xB7W_\x81U`\x01\x01a\x01\xBCV[\x80Q`\x01`\x01``\x1B\x03\x19\x81\x16\x81\x14a\x01\xE6W__\xFD[\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x01\xFAW__\xFD[`@Qa\x03`\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x02)WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@R\x80a\x03`\x84\x01\x85\x81\x11\x15a\x02>W__\xFD[\x84[\x81\x81\x10\x15a\x02_Wa\x02Q\x81a\x01\xCFV[\x83R` \x92\x83\x01\x92\x01a\x02@V[P\x91\x95\x94PPPPPV[__a\x06\xC0\x83\x85\x03\x12\x15a\x02|W__\xFD[a\x02\x86\x84\x84a\x01\xEBV[\x91Pa\x02\x96\x84a\x03`\x85\x01a\x01\xEBV[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\xDAWa\x02\xDAa\x02\xB3V[\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x02\xDAWa\x02\xDAa\x02\xB3V[a\x1A\xFD\x80a\x03\0_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0zW_5`\xE0\x1C\x80cF\xE6\xD3\x1A\x11a\0XW\x80cF\xE6\xD3\x1A\x14a\0\xE5W\x80cI\xCD\x9F\x98\x14a\x01\x05W\x80cv\t\x9A\x06\x14a\x01\x18W\x80c~g\x0E\xB3\x14a\x01KW__\xFD[\x80c\x07\xEE\x975\x14a\0~W\x80c\x12\xD7\x13\xC2\x14a\0\xA6W\x80c9\xAD\xFE\xFF\x14a\0\xD2W[__\xFD[a\0\x91a\0\x8C6`\x04a\x17DV[a\x01\x7FV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB9a\0\xB46`\x04a\x17\xC3V[a\x02\x19V[`@Q`\x01`\x01``\x1B\x03\x19\x90\x91\x16\x81R` \x01a\0\x9DV[a\0\xB9a\0\xE06`\x04a\x17\xC3V[a\x022V[a\0\xF8a\0\xF36`\x04a\x17\xDAV[a\x02AV[`@Qa\0\x9D\x91\x90a\x18\x18V[a\0\xF8a\x01\x136`\x04a\x18\xB6V[a\x02\\V[a\x01+a\x01&6`\x04a\x19\x07V[a\x03JV[`@\x80Q\x93\x84Rc\xFF\xFF\xFF\xFF\x90\x92\x16` \x84\x01R\x90\x82\x01R``\x01a\0\x9DV[a\x01ma\x01Y6`\x04a\x198V[_` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\0\x9DV[_a\x02\x0E\x87`\x02\x88\x88`@Qa\x01\x96\x92\x91\x90a\x19fV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x01\xB1W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xD4\x91\x90a\x19uV[\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa\x03c\x91PPV[\x97\x96PPPPPPPV[`\x1C\x81`\x1B\x81\x10a\x02(W_\x80\xFD[\x01T``\x1B\x90P\x81V[`\x01\x81`\x1B\x81\x10a\x02(W_\x80\xFD[a\x02Ia\x16uV[a\x02S\x83\x83a\x04\x99V[\x90P[\x92\x91PPV[a\x02da\x16uV[___a\x02p\x8Aa\x05{V[\x92P\x92P\x92Pa\x03\x04\x81`\x02\x8B\x8B`@Qa\x02\x8C\x92\x91\x90a\x19fV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x02\xA7W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xCA\x91\x90a\x19uV[\x89\x89\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8B\x92Pa\x03c\x91PPV[a\x03!W`@Qc\x01\xD7\xCD\xD3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03+\x89\x89a\x04\x99V[` \x81\x01\x93\x90\x93RPc\xFF\xFF\xFF\xFF\x16`@\x82\x01R\x97\x96PPPPPPPV[___a\x03V\x84a\x05{V[\x92P\x92P\x92P\x91\x93\x90\x92PV[_\x83\x81[\x84Q\x81\x10\x15a\x04\x8BW_\x85\x82\x81Q\x81\x10a\x03\x83Wa\x03\x83a\x19\x8CV[` \x02` \x01\x01Q\x90P\x81`\x01\x90\x1B\x85\x16_\x03a\x04\x10W`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x82\x90R`\x02\x90``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x03\xCB\x91a\x19\xA0V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x03\xE6W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\t\x91\x90a\x19uV[\x92Pa\x04\x82V[`@\x80Q` \x81\x01\x83\x90R\x90\x81\x01\x84\x90R`\x02\x90``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x04A\x91a\x19\xA0V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x04\\W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x7F\x91\x90a\x19uV[\x92P[P`\x01\x01a\x03gV[P\x85\x14\x90P[\x94\x93PPPPV[a\x04\xA1a\x16uV[_______a\x04\xB2\x8A\x8Aa\x05\xF7V[\x98P\x96P\x90P\x85\x81\x11\x80\x15\x90a\x04\xC8WP\x88\x86\x11\x15[a\x04\xD4Wa\x04\xD4a\x19\xB6V[a\x04\xE0\x8A\x8A\x83\x89a\x06\xD3V[\x92\x97P\x90\x95P\x93P\x91PP\x81\x15\x80\x15a\x04\xF7WP\x80\x15[\x15a\x05\x15W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05!\x89\x89\x87\x81a\x07\x9BV[a\x05>W`@Qc\xC2\xC0b\xD1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x87Rj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x85\x16``\x89\x01R\x83\x16`\x80\x88\x01Ra\x05i\x89\x89\x84\x84a\x08\x95V[`\xA0\x88\x01RP\x94\x97\x96PPPPPPPV[_\x80\x80\x80\x80\x80[`\x14\x81\x10\x15a\x05\xEDW_____a\x05\xBC\x8C\x87`\x14\x81\x10a\x05\xA5Wa\x05\xA5a\x19\x8CV[` \x02\x81\x01\x90a\x05\xB5\x91\x90a\x19\xCAV[\x8A\x8Aa\t}V[\x94P\x94P\x94P\x94P\x94P\x84\x97P\x83\x96P\x85_\x03a\x05\xDDW\x82\x9AP\x81\x99P\x80\x98P[PPPPP\x80`\x01\x01\x90Pa\x05\x82V[PPP\x91\x93\x90\x92PV[_\x80\x80\x83\x80\x15\x80a\x06#WP\x85\x85_\x81\x81\x10a\x06\x15Wa\x06\x15a\x19\x8CV[\x90\x91\x015`\xF8\x1C`\n\x14\x15\x90P[\x15a\x06AW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01_a\x06P\x88\x88\x84\x86a\n\xEDV[\x96P\x86\x92P\x90Pa\x06k\x82`\x01`\x01`@\x1B\x03\x83\x16\x85a\x0B\xAAV[\x94P`\x02a\x06{\x86\x88\x8A\x8Ca\x1A\x0CV[`@Qa\x06\x89\x92\x91\x90a\x19fV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x06\xA4W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC7\x91\x90a\x19uV[\x93PPPP\x92P\x92P\x92V[_______a\x06\xE6\x8B\x8B\x8B\x8Ba\x0B\xDFV[\x91\x94P\x92P\x90P\x81\x83\x10\x80\x15a\x06\xFCWP\x87\x82\x11\x15[a\x07\x08Wa\x07\x08a\x19\xB6V[`\x01`\x01`@\x1B\x03\x81\x16`\x1F\x14a\x072W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\x07@\x8D\x8D\x87\x87a\x0C\xEAV[\x90\x92P\x90P\x81\x15\x80\x15a\x07QWP\x80\x15[\x15a\x07oW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07{\x8D\x8D\x84\x84a\r\xA4V[\x80\x99P\x81\x9AP\x82\x9BP\x83\x9CPPPPPPPPPP\x94P\x94P\x94P\x94\x90PV[_\x82[\x82\x84\x10\x80\x15a\x07\xC7WP\x85\x85\x85\x81\x81\x10a\x07\xBAWa\x07\xBAa\x19\x8CV[\x90\x91\x015`\xF8\x1C`\x12\x14\x90P[\x15a\x08\x15WP\x82a\x07\xD7\x81a\x1AGV[\x93P_a\x07\xE6\x87\x87\x87\x87a\n\xEDV[\x95P\x90P\x81\x85\x11a\x07\xF9Wa\x07\xF9a\x19\xB6V[a\x08\r\x85\x82`\x01`\x01`@\x1B\x03\x16\x86a\x0B\xAAV[\x94PPa\x07\x9EV[_[\x83\x85\x10\x80\x15a\x08@WP\x86\x86\x86\x81\x81\x10a\x083Wa\x083a\x19\x8CV[\x90\x91\x015`\xF8\x1C`*\x14\x90P[\x15a\x08\x8BWP`\x01a\x08Q\x85a\x1AGV[\x94P__a\x08a\x89\x89\x89\x89a\x0FoV[\x98P\x90\x92P\x90Pa\x08t\x89\x89\x84\x84a\x0F\xD5V[a\x08\x84W_\x94PPPPPa\x04\x91V[PPa\x08\x17V[\x96\x95PPPPPPV[``\x82\x82\x10\x80a\x08\xA4WP\x83\x82\x11[\x15a\x08\xC2W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x08\xCD\x84\x84a\x1A_V[\x90P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xE7Wa\x08\xE7a\x1ArV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\t\x11W` \x82\x01\x81\x806\x837\x01\x90P[P\x91P_[\x81\x81\x10\x15a\tsW\x86\x86a\t*\x83\x88a\x1A\x86V[\x81\x81\x10a\t9Wa\t9a\x19\x8CV[\x90P\x015`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\tUWa\tUa\x19\x8CV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\t\x16V[PP\x94\x93PPPPV[_______a\t\x8E\x8B\x8Ba\x10\xB5V[\x90\x98P\x92\x96P\x90\x94P\x92P\x90P\x88\x15\x80\x15\x90a\t\xAAWP\x81\x89\x14\x15[\x15a\t\xC8W`@Qc\xE1Jy1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\t\xD3\x8C\x8Ca\x12\xA0V[\x90P_a\t\xE1\x82\x8E\x8Ea\x12\xFCV[`\x01`\x01``\x1B\x03\x19\x84\x16_\x90\x81R` \x81\x90R`@\x81 T\x91\x92P`\xFF\x90\x91\x16\x90\x81\x90\x03a\n4W`@Qc\xCDBs\x8B`\xE0\x1B\x81R`\x01`\x01``\x1B\x03\x19\x85\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[_a\n@`\x01\x83a\x1A\x99V[\x90P`\x01`\xFF\x82\x16\x1B\x8C\x81\x16c\xFF\xFF\xFF\xFF\x16\x15a\n|W`@QcX:\x88\xFF`\xE1\x1B\x81R`\x01`\x01``\x1B\x03\x19\x87\x16`\x04\x82\x01R`$\x01a\n+V[\x8C\x81\x17\x9AP`\x01`\x01``\x1B\x03\x19\x84\x16`\x1C`\xFF\x84\x16`\x1B\x81\x10a\n\xA2Wa\n\xA2a\x19\x8CV[\x01T``\x1B`\x01`\x01``\x1B\x03\x19\x16\x14a\n\xCFW`@Qc\x13\xD6\xDCs`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\xD9\x8A\x86a\x13nV[\x9BPPPPPPPP\x94P\x94P\x94P\x94P\x94V[_\x80\x80\x80\x80[`\n\x81\x10\x15a\x0B\x87W\x85\x87\x10a\x0B\x1CW`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x89\x89\x89a\x0B)\x81a\x1AGV[\x9AP\x81\x81\x10a\x0B:Wa\x0B:a\x19\x8CV[`\x7F\x92\x015`\xF8\x1C\x91\x82\x16`\x01`\x01`@\x1B\x03\x86\x16\x1B\x95\x90\x95\x17\x94P\x90P`\x80\x81\x16_\x03a\x0BqW\x83\x88\x95P\x95PPPPPa\x0B\xA1V[a\x0B|`\x07\x84a\x1A\xB2V[\x92PP`\x01\x01a\n\xF3V[P`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x94P\x94\x92PPPV[_a\x0B\xB5\x84\x83a\x1A_V[\x83\x11\x15a\x0B\xD5W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\x91\x83\x85a\x1A\x86V[_\x80\x80\x84\x81\x81[\x86\x83\x10\x15a\x0C\xBFWP\x81_\x80a\x0B\xFE\x8C\x8C\x85\x8Ca\x13\x91V[\x96P\x90\x92P\x90P\x82\x85\x11a\x0C\x14Wa\x0C\x14a\x19\xB6V[\x81`\x01`\x01`@\x1B\x03\x16`\x0B\x14\x80\x15a\x0C6WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x0C\xA9W\x83\x15a\x0CZW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x93Pa\x0Cj\x8C\x8C\x87\x8Ca\x0FoV[\x91\x99P\x97P\x94P_a\x0C~\x8D\x8D\x8B\x8Ba\x13\xC5V[\x90\x97P\x90P\x80a\x0C\xA1W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPa\x0C\xBFV[a\x0C\xB6\x8C\x8C\x87\x8C\x85a\x14[V[\x94PPPa\x0B\xE6V[\x81a\x0C\xDDW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPP\x94P\x94P\x94\x91PPV[_\x80\x83\x81\x80\x82[\x86\x84\x10\x15a\r\x97WP\x82_\x80a\r\t\x8C\x8C\x85\x8Ca\x13\x91V[\x97P\x90\x92P\x90P\x82\x86\x11a\r\x1FWa\r\x1Fa\x19\xB6V[\x81`\x01`\x01`@\x1B\x03\x16`\x02\x14\x80\x15a\rAWP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\r\x80Wa\rR\x8C\x8C\x88\x8Ca\x0FoV[\x97P\x90\x95P\x93P_\x80a\rg\x8E\x8E\x89\x89a\x15\x04V[\x91P\x91P\x81_\x14a\ryW\x81\x99P\x80\x98P[PPa\r\x90V[a\r\x8D\x8C\x8C\x88\x8C\x85a\x14[V[\x95P[PPa\x0C\xF1V[PPPP\x94P\x94\x92PPPV[_\x80\x80\x80\x85\x80[\x86\x82\x10\x15a\x0FbWP\x80_\x80a\r\xC3\x8C\x8C\x85\x8Ca\x13\x91V[\x95P\x90\x92P\x90P\x82\x84\x11a\r\xD9Wa\r\xD9a\x19\xB6V[\x81`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\r\xFBWP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x0EvW__a\x0E\x0E\x8E\x8E\x88\x8Ea\x0FoV[\x97P\x90\x92P\x90Pa\x0E\x1F\x82\x82a\x1A_V[`\x15\x14a\x0E?W`@Qc\x90'W\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0EJ\x8E\x8E\x84a\x15\xCCV[\x99P\x89_\x1A`A\x14a\x0EoW`@Qc\xA4d]e`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPa\x0F[V[\x81`\x01`\x01`@\x1B\x03\x16`\x02\x14\x80\x15a\x0E\x98WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x0F\x0CW__a\x0E\xAB\x8E\x8E\x88\x8Ea\x0FoV[\x97P\x90\x92P\x90Pa\x0E\xBC\x82\x82a\x1A_V[`\x15\x14a\x0E\xDCW`@Qcl\x8E\xE0\xD9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\xE7\x8E\x8E\x84a\x15\xCCV[\x98P\x88_\x1A`A\x14a\x0EoW`@QcTw\x93\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`@\x1B\x03\x16`\x04\x14\x80\x15a\x0F.WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x0FKWa\x0F?\x8C\x8C\x86\x8Ca\x0FoV[\x91\x97P\x95P\x93Pa\x0F[V[a\x0FX\x8C\x8C\x86\x8C\x85a\x14[V[\x93P[PPa\r\xABV[PP\x94P\x94P\x94P\x94\x90PV[____a\x0F\x7F\x88\x88\x88\x88a\n\xEDV[\x96P\x86\x94P\x90P`\x01`\x01`@\x1B\x03\x81\x16a\x0F\x9A\x85\x87a\x1A_V[\x81\x11\x15a\x0F\xBAW`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xC4\x81\x88a\x1A\x86V[\x93P\x83\x92PPP\x94P\x94P\x94\x91PPV[_\x80\x80\x84\x80[\x85\x82\x10\x15a\x10\x92WP\x80_\x80a\x0F\xF3\x8B\x8B\x85\x8Ba\x13\x91V[\x95P\x90\x92P\x90P\x82\x84\x11a\x10\tWa\x10\ta\x19\xB6V[`\x01`\x01`@\x1B\x03\x81\x16a\x10{W_a\x10$\x8C\x8C\x87\x8Ca\n\xEDV[\x95P\x90P`\x01`\x01`@\x1B\x03\x83\x16`\x02\x03a\x10\\W`\x01`\x01`@\x1B\x03\x81\x16\x15a\x10WW_\x97PPPPPPPPa\x04\x91V[a\x10uV[\x82`\x01`\x01`@\x1B\x03\x16`\x03\x03a\x10uW`\x01\x95P\x80\x96P[Pa\x10\x8BV[a\x10\x88\x8B\x8B\x86\x8B\x85a\x14[V[\x93P[PPa\x0F\xDBV[\x82\x80\x15a\x10\xA8WP\x83`\x01`\x01`@\x1B\x03\x16`\x01\x14[\x99\x98PPPPPPPPPV[_\x80\x80\x80\x80`\xAE\x86\x14a\x10\xDEW`@Qc`\r\x15Q`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\n+V[\x86\x86_\x81\x81\x10a\x10\xF0Wa\x10\xF0a\x19\x8CV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`\n`\xF8\x1B\x14\x15\x80a\x11@WP\x86\x86`\x01\x81\x81\x10a\x11$Wa\x11$a\x19\x8CV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`i`\xF8\x1B\x14\x15[\x80a\x11uWP\x86\x86`k\x81\x81\x10a\x11YWa\x11Ya\x19\x8CV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`\x12`\xF8\x1B\x14\x15[\x80a\x11\xAAWP\x86\x86`l\x81\x81\x10a\x11\x8EWa\x11\x8Ea\x19\x8CV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`A`\xF8\x1B\x14\x15[\x15a\x11\xC8W`@Qc\xEF\x02\xC9\xBB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x11\xD7\x88\x88`\x03`\ta\n\xEDV[P`\x01`\x01`@\x1B\x03\x16\x90P_a\x11\xF0a\x03\xE8\x83a\x1A\xD1V[\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x17W`@QcT\x9A\x01\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x95P`\x0B\x88\x015\x94P`-\x88\x015\x93P\x85_a\x127\x8A\x8A`N`Ra\n\xEDV[`\x01`\x01`@\x1B\x03\x90\x91\x16\x95P\x90P_\x8A\x8A`T\x81\x81\x10a\x12ZWa\x12Za\x19\x8CV[\x91\x90\x91\x015`\xF8\x1C\x91PP`A\x81\x14a\x12\x8BW`@QcO\xA8\x8D_`\xE1\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`$\x01a\n+V[P\x96\x99\x95\x98\x94\x97P\x92\x95PPPP`U\x015\x90V[_`\x02a\x12\xB0`k\x82\x85\x87a\x1A\x0CV[`@Qa\x12\xBE\x92\x91\x90a\x19fV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x12\xD9W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02S\x91\x90a\x19uV[_`\xAD\x83\x015\x81\x1A`m\x84\x015`\x8D\x85\x015`\x1B\x83\x10\x15a\x13\x1EW`\x1B\x83\x01\x92P[`\x1C\x83\x14`\x1B\x84\x14\x17a\x132W__R__\xFD[`@Q\x87\x81R\x83` \x82\x01R\x82`@\x82\x01R\x81``\x82\x01R` \x81`\x80\x83`\x01Z\xFAa\x13_W__R__\xFD[Q``\x1B\x97\x96PPPPPPPV[_\x80a\x13\x7F`\x01`\x01`\xC0\x1Ba\x1A_V[`\xC0\x85\x90\x1B\x90\x84\x16\x17\x91PP\x92\x91PPV[_____a\x13\xA2\x89\x89\x89\x89a\n\xEDV[`\x03\x82\x90\x1Cg\x1F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x9B`\x07\x90\x92\x16\x9AP\x98P\x96PPPPPPPV[_\x80\x83\x80[\x84\x82\x10\x15a\x14PWP\x80_\x80a\x13\xE2\x8A\x8A\x85\x8Aa\x13\x91V[\x95P\x90\x92P\x90P\x82\x84\x11a\x13\xF8Wa\x13\xF8a\x19\xB6V[\x81`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x14\x18WP`\x01`\x01`@\x1B\x03\x81\x16\x15[\x15a\x14:Wa\x14)\x8A\x8A\x86\x8Aa\n\xEDV[P\x95P`\x01\x94Pa\x0B\xA1\x93PPPPV[a\x14G\x8A\x8A\x86\x8A\x85a\x14[V[\x93PPPa\x13\xCAV[PP\x94P\x94\x92PPPV[_`\x01`\x01`@\x1B\x03\x82\x16a\x14}Wa\x14v\x86\x86\x86\x86a\x16\x03V[\x90Pa\x14\xFBV[`\x01\x19`\x01`\x01`@\x1B\x03\x83\x16\x01a\x14\xA7W_a\x14\x9C\x87\x87\x87\x87a\x0FoV[P\x92Pa\x14\xFB\x91PPV[`\x04\x19`\x01`\x01`@\x1B\x03\x83\x16\x01a\x14\xC5Wa\x14v\x84`\x04\x85a\x0B\xAAV[_\x19`\x01`\x01`@\x1B\x03\x83\x16\x01a\x14\xE2Wa\x14v\x84`\x08\x85a\x0B\xAAV[`@Qc\xA5\xA5\xFCC`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x95\x94PPPPPV[_\x80\x83\x80[\x84\x82\x10\x15a\x14PWP\x80_\x80a\x15!\x8A\x8A\x85\x8Aa\x13\x91V[\x95P\x90\x92P\x90P\x82\x84\x11a\x157Wa\x157a\x19\xB6V[\x81`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x15YWP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x15tWa\x15j\x8A\x8A\x86\x8Aa\x0FoV[P\x94Pa\x15\xC5\x90PV[\x81`\x01`\x01`@\x1B\x03\x16`\x02\x14\x80\x15a\x15\x96WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x15\xB5Wa\x15\xA7\x8A\x8A\x86\x8Aa\x0FoV[P\x90\x96P\x94P\x84\x93Pa\x15\xC5V[a\x15\xC2\x8A\x8A\x86\x8A\x85a\x14[V[\x93P[PPa\x15\tV[_\x82a\x15\xD9\x83`\x15a\x1A\x86V[\x11\x15a\x15\xF8W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x91\x90\x91\x015\x91\x90PV[_\x80[`\n\x81\x10\x15a\x0B\x87W\x82\x84\x10a\x16/W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x86\x86\x86a\x16<\x81a\x1AGV[\x97P\x81\x81\x10a\x16MWa\x16Ma\x19\x8CV[\x91\x90\x91\x015`\xF8\x1C\x91PP`\x80\x81\x16_\x03a\x16lW\x84\x92PPPa\x04\x91V[P`\x01\x01a\x16\x06V[`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\x01`\x01`X\x1B\x03\x19\x16\x81R` \x01_`\x01`\x01`X\x1B\x03\x19\x16\x81R` \x01``\x81RP\x90V[__\x83`\x1F\x84\x01\x12a\x16\xD0W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xE6W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x16\xFDW__\xFD[\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a\x17\x14W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17*W__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x16\xFDW__\xFD[______`\x80\x87\x89\x03\x12\x15a\x17YW__\xFD[\x865\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17uW__\xFD[a\x17\x81\x89\x82\x8A\x01a\x16\xC0V[\x90\x96P\x94PP`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\x9FW__\xFD[a\x17\xAB\x89\x82\x8A\x01a\x17\x04V[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\x17\xD3W__\xFD[P5\x91\x90PV[__` \x83\x85\x03\x12\x15a\x17\xEBW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\0W__\xFD[a\x18\x0C\x85\x82\x86\x01a\x16\xC0V[\x90\x96\x90\x95P\x93PPPPV[` \x81R\x81Q` \x82\x01R` \x82\x01Q`@\x82\x01Rc\xFF\xFF\xFF\xFF`@\x83\x01Q\x16``\x82\x01R`\x01`\x01`X\x1B\x03\x19``\x83\x01Q\x16`\x80\x82\x01R`\x01`\x01`X\x1B\x03\x19`\x80\x83\x01Q\x16`\xA0\x82\x01R_`\xA0\x83\x01Q`\xC0\x80\x84\x01R\x80Q\x80`\xE0\x85\x01R\x80` \x83\x01a\x01\0\x86\x01^_a\x01\0\x82\x86\x01\x01Ra\x01\0`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x80a\x02\x80\x81\x01\x83\x10\x15a\x02VW__\xFD[______`\x80\x87\x89\x03\x12\x15a\x18\xCBW__\xFD[\x865`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xE0W__\xFD[a\x18\xEC\x89\x82\x8A\x01a\x18\xA5V[\x96PP` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17uW__\xFD[_` \x82\x84\x03\x12\x15a\x19\x17W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19,W__\xFD[a\x04\x91\x84\x82\x85\x01a\x18\xA5V[_` \x82\x84\x03\x12\x15a\x19HW__\xFD[\x815`\x01`\x01``\x1B\x03\x19\x81\x16\x81\x14a\x19_W__\xFD[\x93\x92PPPV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_` \x82\x84\x03\x12\x15a\x19\x85W__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x19\xDFW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x19\xF8W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x16\xFDW__\xFD[__\x85\x85\x11\x15a\x1A\x1AW__\xFD[\x83\x86\x11\x15a\x1A&W__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01a\x1AXWa\x1AXa\x1A3V[P`\x01\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x02VWa\x02Va\x1A3V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x02VWa\x02Va\x1A3V[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x02VWa\x02Va\x1A3V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x02VWa\x02Va\x1A3V[_\x82a\x1A\xEBWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA1dsolcC\0\x08\x1B\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061007a575f3560e01c806346e6d31a1161005857806346e6d31a146100e557806349cd9f981461010557806376099a06146101185780637e670eb31461014b575f5ffd5b806307ee97351461007e57806312d713c2146100a657806339adfeff146100d2575b5f5ffd5b61009161008c366004611744565b61017f565b60405190151581526020015b60405180910390f35b6100b96100b43660046117c3565b610219565b6040516001600160601b0319909116815260200161009d565b6100b96100e03660046117c3565b610232565b6100f86100f33660046117da565b610241565b60405161009d9190611818565b6100f86101133660046118b6565b61025c565b61012b610126366004611907565b61034a565b6040805193845263ffffffff90921660208401529082015260600161009d565b61016d610159366004611938565b5f6020819052908152604090205460ff1681565b60405160ff909116815260200161009d565b5f61020e8760028888604051610196929190611966565b602060405180830381855afa1580156101b1573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906101d49190611975565b8686808060200260200160405190810160405280939291908181526020018383602002808284375f92019190915250889250610363915050565b979650505050505050565b601c81601b8110610228575f80fd5b015460601b905081565b600181601b8110610228575f80fd5b610249611675565b6102538383610499565b90505b92915050565b610264611675565b5f5f5f6102708a61057b565b9250925092506103048160028b8b60405161028c929190611966565b602060405180830381855afa1580156102a7573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906102ca9190611975565b8989808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152508b9250610363915050565b610321576040516301d7cdd360e21b815260040160405180910390fd5b61032b8989610499565b60208101939093525063ffffffff166040820152979650505050505050565b5f5f5f6103568461057b565b9250925092509193909250565b5f83815b845181101561048b575f8582815181106103835761038361198c565b60200260200101519050816001901b85165f0361041057604080516020810185905290810182905260029060600160408051601f19818403018152908290526103cb916119a0565b602060405180830381855afa1580156103e6573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906104099190611975565b9250610482565b604080516020810183905290810184905260029060600160408051601f1981840301815290829052610441916119a0565b602060405180830381855afa15801561045c573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061047f9190611975565b92505b50600101610367565b50851490505b949350505050565b6104a1611675565b5f5f5f5f5f5f5f6104b28a8a6105f7565b9850965090508581118015906104c85750888611155b6104d4576104d46119b6565b6104e08a8a83896106d3565b9297509095509350915050811580156104f7575080155b156105155760405163306e189b60e21b815260040160405180910390fd5b6105218989878161079b565b61053e5760405163c2c062d160e01b815260040160405180910390fd5b8587526affffffffffffffffffffff1980851660608901528316608088015261056989898484610895565b60a08801525094979650505050505050565b5f80808080805b60148110156105ed575f5f5f5f5f6105bc8c87601481106105a5576105a561198c565b6020028101906105b591906119ca565b8a8a61097d565b94509450945094509450849750839650855f036105dd57829a508199508098505b5050505050806001019050610582565b5050509193909250565b5f808083801580610623575085855f8181106106155761061561198c565b9091013560f81c600a141590505b156106415760405163306e189b60e21b815260040160405180910390fd5b60015f61065088888486610aed565b9650869250905061066b826001600160401b03831685610baa565b9450600261067b86888a8c611a0c565b604051610689929190611966565b602060405180830381855afa1580156106a4573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906106c79190611975565b93505050509250925092565b5f5f5f5f5f5f5f6106e68b8b8b8b610bdf565b9194509250905081831080156106fc5750878211155b610708576107086119b6565b6001600160401b038116601f146107325760405163306e189b60e21b815260040160405180910390fd5b5f5f6107408d8d8787610cea565b909250905081158015610751575080155b1561076f5760405163306e189b60e21b815260040160405180910390fd5b61077b8d8d8484610da4565b809950819a50829b50839c50505050505050505050945094509450949050565b5f825b82841080156107c757508585858181106107ba576107ba61198c565b9091013560f81c60121490505b156108155750826107d781611a47565b93505f6107e687878787610aed565b955090508185116107f9576107f96119b6565b61080d85826001600160401b031686610baa565b94505061079e565b5f5b838510801561084057508686868181106108335761083361198c565b9091013560f81c602a1490505b1561088b5750600161085185611a47565b94505f5f61086189898989610f6f565b9850909250905061087489898484610fd5565b610884575f945050505050610491565b5050610817565b9695505050505050565b6060828210806108a457508382115b156108c257604051633ffd665960e01b815260040160405180910390fd5b5f6108cd8484611a5f565b9050806001600160401b038111156108e7576108e7611a72565b6040519080825280601f01601f191660200182016040528015610911576020820181803683370190505b5091505f5b8181101561097357868661092a8388611a86565b8181106109395761093961198c565b9050013560f81c60f81b8382815181106109555761095561198c565b60200101906001600160f81b03191690815f1a905350600101610916565b5050949350505050565b5f5f5f5f5f5f5f61098e8b8b6110b5565b9098509296509094509250905088158015906109aa5750818914155b156109c85760405163e14a793160e01b815260040160405180910390fd5b5f6109d38c8c6112a0565b90505f6109e1828e8e6112fc565b6001600160601b031984165f9081526020819052604081205491925060ff90911690819003610a345760405163cd42738b60e01b81526001600160601b0319851660048201526024015b60405180910390fd5b5f610a40600183611a99565b9050600160ff82161b8c811663ffffffff1615610a7c5760405163583a88ff60e11b81526001600160601b031987166004820152602401610a2b565b8c81179a506001600160601b03198416601c60ff8416601b8110610aa257610aa261198c565b015460601b6001600160601b03191614610acf576040516313d6dc7360e01b815260040160405180910390fd5b610ad98a8661136e565b9b5050505050505050945094509450945094565b5f808080805b600a811015610b8757858710610b1c57604051633ffd665960e01b815260040160405180910390fd5b5f898989610b2981611a47565b9a50818110610b3a57610b3a61198c565b607f92013560f81c9182166001600160401b0386161b9590951794509050608081165f03610b715783889550955050505050610ba1565b610b7c600784611ab2565b925050600101610af3565b50604051633ffd665960e01b815260040160405180910390fd5b94509492505050565b5f610bb58483611a5f565b831115610bd557604051633ffd665960e01b815260040160405180910390fd5b6104918385611a86565b5f80808481815b86831015610cbf5750815f80610bfe8c8c858c611391565b96509092509050828511610c1457610c146119b6565b816001600160401b0316600b148015610c3657506001600160401b0381166002145b15610ca9578315610c5a5760405163306e189b60e21b815260040160405180910390fd5b60019350610c6a8c8c878c610f6f565b919950975094505f610c7e8d8d8b8b6113c5565b909750905080610ca15760405163306e189b60e21b815260040160405180910390fd5b505050610cbf565b610cb68c8c878c8561145b565b94505050610be6565b81610cdd5760405163306e189b60e21b815260040160405180910390fd5b5050509450945094915050565b5f80838180825b86841015610d975750825f80610d098c8c858c611391565b97509092509050828611610d1f57610d1f6119b6565b816001600160401b03166002148015610d4157506001600160401b0381166002145b15610d8057610d528c8c888c610f6f565b975090955093505f80610d678e8e8989611504565b91509150815f14610d79578199508098505b5050610d90565b610d8d8c8c888c8561145b565b95505b5050610cf1565b5050505094509492505050565b5f80808085805b86821015610f625750805f80610dc38c8c858c611391565b95509092509050828411610dd957610dd96119b6565b816001600160401b03166001148015610dfb57506001600160401b0381166002145b15610e76575f5f610e0e8e8e888e610f6f565b97509092509050610e1f8282611a5f565b601514610e3f5760405163902757b160e01b815260040160405180910390fd5b610e4a8e8e846115cc565b9950895f1a604114610e6f5760405163a4645d6560e01b815260040160405180910390fd5b5050610f5b565b816001600160401b03166002148015610e9857506001600160401b0381166002145b15610f0c575f5f610eab8e8e888e610f6f565b97509092509050610ebc8282611a5f565b601514610edc57604051636c8ee0d960e11b815260040160405180910390fd5b610ee78e8e846115cc565b9850885f1a604114610e6f5760405163547793ab60e11b815260040160405180910390fd5b816001600160401b03166004148015610f2e57506001600160401b0381166002145b15610f4b57610f3f8c8c868c610f6f565b91975095509350610f5b565b610f588c8c868c8561145b565b93505b5050610dab565b5050945094509450949050565b5f5f5f5f610f7f88888888610aed565b965086945090506001600160401b038116610f9a8587611a5f565b811115610fba57604051633ffd665960e01b815260040160405180910390fd5b610fc48188611a86565b935083925050509450945094915050565b5f808084805b858210156110925750805f80610ff38b8b858b611391565b95509092509050828411611009576110096119b6565b6001600160401b03811661107b575f6110248c8c878c610aed565b955090506001600160401b03831660020361105c576001600160401b03811615611057575f975050505050505050610491565b611075565b826001600160401b031660030361107557600195508096505b5061108b565b6110888b8b868b8561145b565b93505b5050610fdb565b8280156110a85750836001600160401b03166001145b9998505050505050505050565b5f8080808060ae86146110de5760405163600d155160e01b815260048101879052602401610a2b565b86865f8181106110f0576110f061198c565b9050013560f81c60f81b6001600160f81b031916600a60f81b1415806111405750868660018181106111245761112461198c565b9050013560f81c60f81b6001600160f81b031916606960f81b14155b8061117557508686606b8181106111595761115961198c565b9050013560f81c60f81b6001600160f81b031916601260f81b14155b806111aa57508686606c81811061118e5761118e61198c565b9050013560f81c60f81b6001600160f81b031916604160f81b14155b156111c85760405163ef02c9bb60e01b815260040160405180910390fd5b5f6111d7888860036009610aed565b506001600160401b031690505f6111f06103e883611ad1565b905063ffffffff8111156112175760405163549a019760e01b815260040160405180910390fd5b9550600b8801359450602d8801359350855f6112378a8a604e6052610aed565b6001600160401b03909116955090505f8a8a605481811061125a5761125a61198c565b919091013560f81c9150506041811461128b57604051634fa88d5f60e11b815260ff82166004820152602401610a2b565b50969995989497509295505050506055013590565b5f60026112b0606b828587611a0c565b6040516112be929190611966565b602060405180830381855afa1580156112d9573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906102539190611975565b5f60ad830135811a606d840135608d850135601b83101561131e57601b830192505b601c8314601b841417611332575f5f525f5ffd5b60405187815283602082015282604082015281606082015260208160808360015afa61135f575f5f525f5ffd5b5160601b979650505050505050565b5f8061137f6001600160c01b611a5f565b60c085901b9084161791505092915050565b5f5f5f5f5f6113a289898989610aed565b600382901c671fffffffffffffff169b60079092169a5098509650505050505050565b5f8083805b848210156114505750805f806113e28a8a858a611391565b955090925090508284116113f8576113f86119b6565b816001600160401b0316600114801561141857506001600160401b038116155b1561143a576114298a8a868a610aed565b50955060019450610ba19350505050565b6114478a8a868a8561145b565b935050506113ca565b505094509492505050565b5f6001600160401b03821661147d5761147686868686611603565b90506114fb565b6001196001600160401b038316016114a7575f61149c87878787610f6f565b5092506114fb915050565b6004196001600160401b038316016114c55761147684600485610baa565b5f196001600160401b038316016114e25761147684600885610baa565b60405163a5a5fc4360e01b815260040160405180910390fd5b95945050505050565b5f8083805b848210156114505750805f806115218a8a858a611391565b95509092509050828411611537576115376119b6565b816001600160401b0316600114801561155957506001600160401b0381166002145b156115745761156a8a8a868a610f6f565b5094506115c59050565b816001600160401b0316600214801561159657506001600160401b0381166002145b156115b5576115a78a8a868a610f6f565b5090965094508493506115c5565b6115c28a8a868a8561145b565b93505b5050611509565b5f826115d9836015611a86565b11156115f857604051633ffd665960e01b815260040160405180910390fd5b509190910135919050565b5f805b600a811015610b875782841061162f57604051633ffd665960e01b815260040160405180910390fd5b5f86868661163c81611a47565b975081811061164d5761164d61198c565b919091013560f81c915050608081165f0361166c578492505050610491565b50600101611606565b6040518060c001604052805f81526020015f81526020015f63ffffffff1681526020015f6001600160581b03191681526020015f6001600160581b0319168152602001606081525090565b5f5f83601f8401126116d0575f5ffd5b5081356001600160401b038111156116e6575f5ffd5b6020830191508360208285010111156116fd575f5ffd5b9250929050565b5f5f83601f840112611714575f5ffd5b5081356001600160401b0381111561172a575f5ffd5b6020830191508360208260051b85010111156116fd575f5ffd5b5f5f5f5f5f5f60808789031215611759575f5ffd5b8635955060208701356001600160401b03811115611775575f5ffd5b61178189828a016116c0565b90965094505060408701356001600160401b0381111561179f575f5ffd5b6117ab89828a01611704565b979a9699509497949695606090950135949350505050565b5f602082840312156117d3575f5ffd5b5035919050565b5f5f602083850312156117eb575f5ffd5b82356001600160401b03811115611800575f5ffd5b61180c858286016116c0565b90969095509350505050565b60208152815160208201526020820151604082015263ffffffff60408301511660608201526001600160581b031960608301511660808201526001600160581b031960808301511660a08201525f60a083015160c08084015280518060e0850152806020830161010086015e5f6101008286010152610100601f19601f8301168501019250505092915050565b806102808101831015610256575f5ffd5b5f5f5f5f5f5f608087890312156118cb575f5ffd5b86356001600160401b038111156118e0575f5ffd5b6118ec89828a016118a5565b96505060208701356001600160401b03811115611775575f5ffd5b5f60208284031215611917575f5ffd5b81356001600160401b0381111561192c575f5ffd5b610491848285016118a5565b5f60208284031215611948575f5ffd5b81356001600160601b03198116811461195f575f5ffd5b9392505050565b818382375f9101908152919050565b5f60208284031215611985575f5ffd5b5051919050565b634e487b7160e01b5f52603260045260245ffd5b5f82518060208501845e5f920191825250919050565b634e487b7160e01b5f52600160045260245ffd5b5f5f8335601e198436030181126119df575f5ffd5b8301803591506001600160401b038211156119f8575f5ffd5b6020019150368190038213156116fd575f5ffd5b5f5f85851115611a1a575f5ffd5b83861115611a26575f5ffd5b5050820193919092039150565b634e487b7160e01b5f52601160045260245ffd5b5f60018201611a5857611a58611a33565b5060010190565b8181038181111561025657610256611a33565b634e487b7160e01b5f52604160045260245ffd5b8082018082111561025657610256611a33565b60ff828116828216039081111561025657610256611a33565b6001600160401b03818116838216019081111561025657610256611a33565b5f82611aeb57634e487b7160e01b5f52601260045260245ffd5b50049056fea164736f6c634300081b000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0zW_5`\xE0\x1C\x80cF\xE6\xD3\x1A\x11a\0XW\x80cF\xE6\xD3\x1A\x14a\0\xE5W\x80cI\xCD\x9F\x98\x14a\x01\x05W\x80cv\t\x9A\x06\x14a\x01\x18W\x80c~g\x0E\xB3\x14a\x01KW__\xFD[\x80c\x07\xEE\x975\x14a\0~W\x80c\x12\xD7\x13\xC2\x14a\0\xA6W\x80c9\xAD\xFE\xFF\x14a\0\xD2W[__\xFD[a\0\x91a\0\x8C6`\x04a\x17DV[a\x01\x7FV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB9a\0\xB46`\x04a\x17\xC3V[a\x02\x19V[`@Q`\x01`\x01``\x1B\x03\x19\x90\x91\x16\x81R` \x01a\0\x9DV[a\0\xB9a\0\xE06`\x04a\x17\xC3V[a\x022V[a\0\xF8a\0\xF36`\x04a\x17\xDAV[a\x02AV[`@Qa\0\x9D\x91\x90a\x18\x18V[a\0\xF8a\x01\x136`\x04a\x18\xB6V[a\x02\\V[a\x01+a\x01&6`\x04a\x19\x07V[a\x03JV[`@\x80Q\x93\x84Rc\xFF\xFF\xFF\xFF\x90\x92\x16` \x84\x01R\x90\x82\x01R``\x01a\0\x9DV[a\x01ma\x01Y6`\x04a\x198V[_` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\0\x9DV[_a\x02\x0E\x87`\x02\x88\x88`@Qa\x01\x96\x92\x91\x90a\x19fV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x01\xB1W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xD4\x91\x90a\x19uV[\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa\x03c\x91PPV[\x97\x96PPPPPPPV[`\x1C\x81`\x1B\x81\x10a\x02(W_\x80\xFD[\x01T``\x1B\x90P\x81V[`\x01\x81`\x1B\x81\x10a\x02(W_\x80\xFD[a\x02Ia\x16uV[a\x02S\x83\x83a\x04\x99V[\x90P[\x92\x91PPV[a\x02da\x16uV[___a\x02p\x8Aa\x05{V[\x92P\x92P\x92Pa\x03\x04\x81`\x02\x8B\x8B`@Qa\x02\x8C\x92\x91\x90a\x19fV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x02\xA7W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xCA\x91\x90a\x19uV[\x89\x89\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8B\x92Pa\x03c\x91PPV[a\x03!W`@Qc\x01\xD7\xCD\xD3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03+\x89\x89a\x04\x99V[` \x81\x01\x93\x90\x93RPc\xFF\xFF\xFF\xFF\x16`@\x82\x01R\x97\x96PPPPPPPV[___a\x03V\x84a\x05{V[\x92P\x92P\x92P\x91\x93\x90\x92PV[_\x83\x81[\x84Q\x81\x10\x15a\x04\x8BW_\x85\x82\x81Q\x81\x10a\x03\x83Wa\x03\x83a\x19\x8CV[` \x02` \x01\x01Q\x90P\x81`\x01\x90\x1B\x85\x16_\x03a\x04\x10W`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x82\x90R`\x02\x90``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x03\xCB\x91a\x19\xA0V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x03\xE6W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\t\x91\x90a\x19uV[\x92Pa\x04\x82V[`@\x80Q` \x81\x01\x83\x90R\x90\x81\x01\x84\x90R`\x02\x90``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x04A\x91a\x19\xA0V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x04\\W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x7F\x91\x90a\x19uV[\x92P[P`\x01\x01a\x03gV[P\x85\x14\x90P[\x94\x93PPPPV[a\x04\xA1a\x16uV[_______a\x04\xB2\x8A\x8Aa\x05\xF7V[\x98P\x96P\x90P\x85\x81\x11\x80\x15\x90a\x04\xC8WP\x88\x86\x11\x15[a\x04\xD4Wa\x04\xD4a\x19\xB6V[a\x04\xE0\x8A\x8A\x83\x89a\x06\xD3V[\x92\x97P\x90\x95P\x93P\x91PP\x81\x15\x80\x15a\x04\xF7WP\x80\x15[\x15a\x05\x15W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05!\x89\x89\x87\x81a\x07\x9BV[a\x05>W`@Qc\xC2\xC0b\xD1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x87Rj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x85\x16``\x89\x01R\x83\x16`\x80\x88\x01Ra\x05i\x89\x89\x84\x84a\x08\x95V[`\xA0\x88\x01RP\x94\x97\x96PPPPPPPV[_\x80\x80\x80\x80\x80[`\x14\x81\x10\x15a\x05\xEDW_____a\x05\xBC\x8C\x87`\x14\x81\x10a\x05\xA5Wa\x05\xA5a\x19\x8CV[` \x02\x81\x01\x90a\x05\xB5\x91\x90a\x19\xCAV[\x8A\x8Aa\t}V[\x94P\x94P\x94P\x94P\x94P\x84\x97P\x83\x96P\x85_\x03a\x05\xDDW\x82\x9AP\x81\x99P\x80\x98P[PPPPP\x80`\x01\x01\x90Pa\x05\x82V[PPP\x91\x93\x90\x92PV[_\x80\x80\x83\x80\x15\x80a\x06#WP\x85\x85_\x81\x81\x10a\x06\x15Wa\x06\x15a\x19\x8CV[\x90\x91\x015`\xF8\x1C`\n\x14\x15\x90P[\x15a\x06AW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01_a\x06P\x88\x88\x84\x86a\n\xEDV[\x96P\x86\x92P\x90Pa\x06k\x82`\x01`\x01`@\x1B\x03\x83\x16\x85a\x0B\xAAV[\x94P`\x02a\x06{\x86\x88\x8A\x8Ca\x1A\x0CV[`@Qa\x06\x89\x92\x91\x90a\x19fV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x06\xA4W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC7\x91\x90a\x19uV[\x93PPPP\x92P\x92P\x92V[_______a\x06\xE6\x8B\x8B\x8B\x8Ba\x0B\xDFV[\x91\x94P\x92P\x90P\x81\x83\x10\x80\x15a\x06\xFCWP\x87\x82\x11\x15[a\x07\x08Wa\x07\x08a\x19\xB6V[`\x01`\x01`@\x1B\x03\x81\x16`\x1F\x14a\x072W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\x07@\x8D\x8D\x87\x87a\x0C\xEAV[\x90\x92P\x90P\x81\x15\x80\x15a\x07QWP\x80\x15[\x15a\x07oW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07{\x8D\x8D\x84\x84a\r\xA4V[\x80\x99P\x81\x9AP\x82\x9BP\x83\x9CPPPPPPPPPP\x94P\x94P\x94P\x94\x90PV[_\x82[\x82\x84\x10\x80\x15a\x07\xC7WP\x85\x85\x85\x81\x81\x10a\x07\xBAWa\x07\xBAa\x19\x8CV[\x90\x91\x015`\xF8\x1C`\x12\x14\x90P[\x15a\x08\x15WP\x82a\x07\xD7\x81a\x1AGV[\x93P_a\x07\xE6\x87\x87\x87\x87a\n\xEDV[\x95P\x90P\x81\x85\x11a\x07\xF9Wa\x07\xF9a\x19\xB6V[a\x08\r\x85\x82`\x01`\x01`@\x1B\x03\x16\x86a\x0B\xAAV[\x94PPa\x07\x9EV[_[\x83\x85\x10\x80\x15a\x08@WP\x86\x86\x86\x81\x81\x10a\x083Wa\x083a\x19\x8CV[\x90\x91\x015`\xF8\x1C`*\x14\x90P[\x15a\x08\x8BWP`\x01a\x08Q\x85a\x1AGV[\x94P__a\x08a\x89\x89\x89\x89a\x0FoV[\x98P\x90\x92P\x90Pa\x08t\x89\x89\x84\x84a\x0F\xD5V[a\x08\x84W_\x94PPPPPa\x04\x91V[PPa\x08\x17V[\x96\x95PPPPPPV[``\x82\x82\x10\x80a\x08\xA4WP\x83\x82\x11[\x15a\x08\xC2W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x08\xCD\x84\x84a\x1A_V[\x90P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xE7Wa\x08\xE7a\x1ArV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\t\x11W` \x82\x01\x81\x806\x837\x01\x90P[P\x91P_[\x81\x81\x10\x15a\tsW\x86\x86a\t*\x83\x88a\x1A\x86V[\x81\x81\x10a\t9Wa\t9a\x19\x8CV[\x90P\x015`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\tUWa\tUa\x19\x8CV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\t\x16V[PP\x94\x93PPPPV[_______a\t\x8E\x8B\x8Ba\x10\xB5V[\x90\x98P\x92\x96P\x90\x94P\x92P\x90P\x88\x15\x80\x15\x90a\t\xAAWP\x81\x89\x14\x15[\x15a\t\xC8W`@Qc\xE1Jy1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\t\xD3\x8C\x8Ca\x12\xA0V[\x90P_a\t\xE1\x82\x8E\x8Ea\x12\xFCV[`\x01`\x01``\x1B\x03\x19\x84\x16_\x90\x81R` \x81\x90R`@\x81 T\x91\x92P`\xFF\x90\x91\x16\x90\x81\x90\x03a\n4W`@Qc\xCDBs\x8B`\xE0\x1B\x81R`\x01`\x01``\x1B\x03\x19\x85\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[_a\n@`\x01\x83a\x1A\x99V[\x90P`\x01`\xFF\x82\x16\x1B\x8C\x81\x16c\xFF\xFF\xFF\xFF\x16\x15a\n|W`@QcX:\x88\xFF`\xE1\x1B\x81R`\x01`\x01``\x1B\x03\x19\x87\x16`\x04\x82\x01R`$\x01a\n+V[\x8C\x81\x17\x9AP`\x01`\x01``\x1B\x03\x19\x84\x16`\x1C`\xFF\x84\x16`\x1B\x81\x10a\n\xA2Wa\n\xA2a\x19\x8CV[\x01T``\x1B`\x01`\x01``\x1B\x03\x19\x16\x14a\n\xCFW`@Qc\x13\xD6\xDCs`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\xD9\x8A\x86a\x13nV[\x9BPPPPPPPP\x94P\x94P\x94P\x94P\x94V[_\x80\x80\x80\x80[`\n\x81\x10\x15a\x0B\x87W\x85\x87\x10a\x0B\x1CW`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x89\x89\x89a\x0B)\x81a\x1AGV[\x9AP\x81\x81\x10a\x0B:Wa\x0B:a\x19\x8CV[`\x7F\x92\x015`\xF8\x1C\x91\x82\x16`\x01`\x01`@\x1B\x03\x86\x16\x1B\x95\x90\x95\x17\x94P\x90P`\x80\x81\x16_\x03a\x0BqW\x83\x88\x95P\x95PPPPPa\x0B\xA1V[a\x0B|`\x07\x84a\x1A\xB2V[\x92PP`\x01\x01a\n\xF3V[P`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x94P\x94\x92PPPV[_a\x0B\xB5\x84\x83a\x1A_V[\x83\x11\x15a\x0B\xD5W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\x91\x83\x85a\x1A\x86V[_\x80\x80\x84\x81\x81[\x86\x83\x10\x15a\x0C\xBFWP\x81_\x80a\x0B\xFE\x8C\x8C\x85\x8Ca\x13\x91V[\x96P\x90\x92P\x90P\x82\x85\x11a\x0C\x14Wa\x0C\x14a\x19\xB6V[\x81`\x01`\x01`@\x1B\x03\x16`\x0B\x14\x80\x15a\x0C6WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x0C\xA9W\x83\x15a\x0CZW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x93Pa\x0Cj\x8C\x8C\x87\x8Ca\x0FoV[\x91\x99P\x97P\x94P_a\x0C~\x8D\x8D\x8B\x8Ba\x13\xC5V[\x90\x97P\x90P\x80a\x0C\xA1W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPa\x0C\xBFV[a\x0C\xB6\x8C\x8C\x87\x8C\x85a\x14[V[\x94PPPa\x0B\xE6V[\x81a\x0C\xDDW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPP\x94P\x94P\x94\x91PPV[_\x80\x83\x81\x80\x82[\x86\x84\x10\x15a\r\x97WP\x82_\x80a\r\t\x8C\x8C\x85\x8Ca\x13\x91V[\x97P\x90\x92P\x90P\x82\x86\x11a\r\x1FWa\r\x1Fa\x19\xB6V[\x81`\x01`\x01`@\x1B\x03\x16`\x02\x14\x80\x15a\rAWP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\r\x80Wa\rR\x8C\x8C\x88\x8Ca\x0FoV[\x97P\x90\x95P\x93P_\x80a\rg\x8E\x8E\x89\x89a\x15\x04V[\x91P\x91P\x81_\x14a\ryW\x81\x99P\x80\x98P[PPa\r\x90V[a\r\x8D\x8C\x8C\x88\x8C\x85a\x14[V[\x95P[PPa\x0C\xF1V[PPPP\x94P\x94\x92PPPV[_\x80\x80\x80\x85\x80[\x86\x82\x10\x15a\x0FbWP\x80_\x80a\r\xC3\x8C\x8C\x85\x8Ca\x13\x91V[\x95P\x90\x92P\x90P\x82\x84\x11a\r\xD9Wa\r\xD9a\x19\xB6V[\x81`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\r\xFBWP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x0EvW__a\x0E\x0E\x8E\x8E\x88\x8Ea\x0FoV[\x97P\x90\x92P\x90Pa\x0E\x1F\x82\x82a\x1A_V[`\x15\x14a\x0E?W`@Qc\x90'W\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0EJ\x8E\x8E\x84a\x15\xCCV[\x99P\x89_\x1A`A\x14a\x0EoW`@Qc\xA4d]e`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPa\x0F[V[\x81`\x01`\x01`@\x1B\x03\x16`\x02\x14\x80\x15a\x0E\x98WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x0F\x0CW__a\x0E\xAB\x8E\x8E\x88\x8Ea\x0FoV[\x97P\x90\x92P\x90Pa\x0E\xBC\x82\x82a\x1A_V[`\x15\x14a\x0E\xDCW`@Qcl\x8E\xE0\xD9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\xE7\x8E\x8E\x84a\x15\xCCV[\x98P\x88_\x1A`A\x14a\x0EoW`@QcTw\x93\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`@\x1B\x03\x16`\x04\x14\x80\x15a\x0F.WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x0FKWa\x0F?\x8C\x8C\x86\x8Ca\x0FoV[\x91\x97P\x95P\x93Pa\x0F[V[a\x0FX\x8C\x8C\x86\x8C\x85a\x14[V[\x93P[PPa\r\xABV[PP\x94P\x94P\x94P\x94\x90PV[____a\x0F\x7F\x88\x88\x88\x88a\n\xEDV[\x96P\x86\x94P\x90P`\x01`\x01`@\x1B\x03\x81\x16a\x0F\x9A\x85\x87a\x1A_V[\x81\x11\x15a\x0F\xBAW`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xC4\x81\x88a\x1A\x86V[\x93P\x83\x92PPP\x94P\x94P\x94\x91PPV[_\x80\x80\x84\x80[\x85\x82\x10\x15a\x10\x92WP\x80_\x80a\x0F\xF3\x8B\x8B\x85\x8Ba\x13\x91V[\x95P\x90\x92P\x90P\x82\x84\x11a\x10\tWa\x10\ta\x19\xB6V[`\x01`\x01`@\x1B\x03\x81\x16a\x10{W_a\x10$\x8C\x8C\x87\x8Ca\n\xEDV[\x95P\x90P`\x01`\x01`@\x1B\x03\x83\x16`\x02\x03a\x10\\W`\x01`\x01`@\x1B\x03\x81\x16\x15a\x10WW_\x97PPPPPPPPa\x04\x91V[a\x10uV[\x82`\x01`\x01`@\x1B\x03\x16`\x03\x03a\x10uW`\x01\x95P\x80\x96P[Pa\x10\x8BV[a\x10\x88\x8B\x8B\x86\x8B\x85a\x14[V[\x93P[PPa\x0F\xDBV[\x82\x80\x15a\x10\xA8WP\x83`\x01`\x01`@\x1B\x03\x16`\x01\x14[\x99\x98PPPPPPPPPV[_\x80\x80\x80\x80`\xAE\x86\x14a\x10\xDEW`@Qc`\r\x15Q`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\n+V[\x86\x86_\x81\x81\x10a\x10\xF0Wa\x10\xF0a\x19\x8CV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`\n`\xF8\x1B\x14\x15\x80a\x11@WP\x86\x86`\x01\x81\x81\x10a\x11$Wa\x11$a\x19\x8CV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`i`\xF8\x1B\x14\x15[\x80a\x11uWP\x86\x86`k\x81\x81\x10a\x11YWa\x11Ya\x19\x8CV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`\x12`\xF8\x1B\x14\x15[\x80a\x11\xAAWP\x86\x86`l\x81\x81\x10a\x11\x8EWa\x11\x8Ea\x19\x8CV[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`A`\xF8\x1B\x14\x15[\x15a\x11\xC8W`@Qc\xEF\x02\xC9\xBB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x11\xD7\x88\x88`\x03`\ta\n\xEDV[P`\x01`\x01`@\x1B\x03\x16\x90P_a\x11\xF0a\x03\xE8\x83a\x1A\xD1V[\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x17W`@QcT\x9A\x01\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x95P`\x0B\x88\x015\x94P`-\x88\x015\x93P\x85_a\x127\x8A\x8A`N`Ra\n\xEDV[`\x01`\x01`@\x1B\x03\x90\x91\x16\x95P\x90P_\x8A\x8A`T\x81\x81\x10a\x12ZWa\x12Za\x19\x8CV[\x91\x90\x91\x015`\xF8\x1C\x91PP`A\x81\x14a\x12\x8BW`@QcO\xA8\x8D_`\xE1\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`$\x01a\n+V[P\x96\x99\x95\x98\x94\x97P\x92\x95PPPP`U\x015\x90V[_`\x02a\x12\xB0`k\x82\x85\x87a\x1A\x0CV[`@Qa\x12\xBE\x92\x91\x90a\x19fV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x12\xD9W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02S\x91\x90a\x19uV[_`\xAD\x83\x015\x81\x1A`m\x84\x015`\x8D\x85\x015`\x1B\x83\x10\x15a\x13\x1EW`\x1B\x83\x01\x92P[`\x1C\x83\x14`\x1B\x84\x14\x17a\x132W__R__\xFD[`@Q\x87\x81R\x83` \x82\x01R\x82`@\x82\x01R\x81``\x82\x01R` \x81`\x80\x83`\x01Z\xFAa\x13_W__R__\xFD[Q``\x1B\x97\x96PPPPPPPV[_\x80a\x13\x7F`\x01`\x01`\xC0\x1Ba\x1A_V[`\xC0\x85\x90\x1B\x90\x84\x16\x17\x91PP\x92\x91PPV[_____a\x13\xA2\x89\x89\x89\x89a\n\xEDV[`\x03\x82\x90\x1Cg\x1F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x9B`\x07\x90\x92\x16\x9AP\x98P\x96PPPPPPPV[_\x80\x83\x80[\x84\x82\x10\x15a\x14PWP\x80_\x80a\x13\xE2\x8A\x8A\x85\x8Aa\x13\x91V[\x95P\x90\x92P\x90P\x82\x84\x11a\x13\xF8Wa\x13\xF8a\x19\xB6V[\x81`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x14\x18WP`\x01`\x01`@\x1B\x03\x81\x16\x15[\x15a\x14:Wa\x14)\x8A\x8A\x86\x8Aa\n\xEDV[P\x95P`\x01\x94Pa\x0B\xA1\x93PPPPV[a\x14G\x8A\x8A\x86\x8A\x85a\x14[V[\x93PPPa\x13\xCAV[PP\x94P\x94\x92PPPV[_`\x01`\x01`@\x1B\x03\x82\x16a\x14}Wa\x14v\x86\x86\x86\x86a\x16\x03V[\x90Pa\x14\xFBV[`\x01\x19`\x01`\x01`@\x1B\x03\x83\x16\x01a\x14\xA7W_a\x14\x9C\x87\x87\x87\x87a\x0FoV[P\x92Pa\x14\xFB\x91PPV[`\x04\x19`\x01`\x01`@\x1B\x03\x83\x16\x01a\x14\xC5Wa\x14v\x84`\x04\x85a\x0B\xAAV[_\x19`\x01`\x01`@\x1B\x03\x83\x16\x01a\x14\xE2Wa\x14v\x84`\x08\x85a\x0B\xAAV[`@Qc\xA5\xA5\xFCC`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x95\x94PPPPPV[_\x80\x83\x80[\x84\x82\x10\x15a\x14PWP\x80_\x80a\x15!\x8A\x8A\x85\x8Aa\x13\x91V[\x95P\x90\x92P\x90P\x82\x84\x11a\x157Wa\x157a\x19\xB6V[\x81`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x15YWP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x15tWa\x15j\x8A\x8A\x86\x8Aa\x0FoV[P\x94Pa\x15\xC5\x90PV[\x81`\x01`\x01`@\x1B\x03\x16`\x02\x14\x80\x15a\x15\x96WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x15\xB5Wa\x15\xA7\x8A\x8A\x86\x8Aa\x0FoV[P\x90\x96P\x94P\x84\x93Pa\x15\xC5V[a\x15\xC2\x8A\x8A\x86\x8A\x85a\x14[V[\x93P[PPa\x15\tV[_\x82a\x15\xD9\x83`\x15a\x1A\x86V[\x11\x15a\x15\xF8W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x91\x90\x91\x015\x91\x90PV[_\x80[`\n\x81\x10\x15a\x0B\x87W\x82\x84\x10a\x16/W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x86\x86\x86a\x16<\x81a\x1AGV[\x97P\x81\x81\x10a\x16MWa\x16Ma\x19\x8CV[\x91\x90\x91\x015`\xF8\x1C\x91PP`\x80\x81\x16_\x03a\x16lW\x84\x92PPPa\x04\x91V[P`\x01\x01a\x16\x06V[`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\x01`\x01`X\x1B\x03\x19\x16\x81R` \x01_`\x01`\x01`X\x1B\x03\x19\x16\x81R` \x01``\x81RP\x90V[__\x83`\x1F\x84\x01\x12a\x16\xD0W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xE6W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x16\xFDW__\xFD[\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a\x17\x14W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17*W__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x16\xFDW__\xFD[______`\x80\x87\x89\x03\x12\x15a\x17YW__\xFD[\x865\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17uW__\xFD[a\x17\x81\x89\x82\x8A\x01a\x16\xC0V[\x90\x96P\x94PP`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\x9FW__\xFD[a\x17\xAB\x89\x82\x8A\x01a\x17\x04V[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\x17\xD3W__\xFD[P5\x91\x90PV[__` \x83\x85\x03\x12\x15a\x17\xEBW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\0W__\xFD[a\x18\x0C\x85\x82\x86\x01a\x16\xC0V[\x90\x96\x90\x95P\x93PPPPV[` \x81R\x81Q` \x82\x01R` \x82\x01Q`@\x82\x01Rc\xFF\xFF\xFF\xFF`@\x83\x01Q\x16``\x82\x01R`\x01`\x01`X\x1B\x03\x19``\x83\x01Q\x16`\x80\x82\x01R`\x01`\x01`X\x1B\x03\x19`\x80\x83\x01Q\x16`\xA0\x82\x01R_`\xA0\x83\x01Q`\xC0\x80\x84\x01R\x80Q\x80`\xE0\x85\x01R\x80` \x83\x01a\x01\0\x86\x01^_a\x01\0\x82\x86\x01\x01Ra\x01\0`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x80a\x02\x80\x81\x01\x83\x10\x15a\x02VW__\xFD[______`\x80\x87\x89\x03\x12\x15a\x18\xCBW__\xFD[\x865`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xE0W__\xFD[a\x18\xEC\x89\x82\x8A\x01a\x18\xA5V[\x96PP` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17uW__\xFD[_` \x82\x84\x03\x12\x15a\x19\x17W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19,W__\xFD[a\x04\x91\x84\x82\x85\x01a\x18\xA5V[_` \x82\x84\x03\x12\x15a\x19HW__\xFD[\x815`\x01`\x01``\x1B\x03\x19\x81\x16\x81\x14a\x19_W__\xFD[\x93\x92PPPV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_` \x82\x84\x03\x12\x15a\x19\x85W__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x19\xDFW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x19\xF8W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x16\xFDW__\xFD[__\x85\x85\x11\x15a\x1A\x1AW__\xFD[\x83\x86\x11\x15a\x1A&W__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01a\x1AXWa\x1AXa\x1A3V[P`\x01\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x02VWa\x02Va\x1A3V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x02VWa\x02Va\x1A3V[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x02VWa\x02Va\x1A3V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x02VWa\x02Va\x1A3V[_\x82a\x1A\xEBWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA1dsolcC\0\x08\x1B\0\n",
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
    /**Function with signature `parseTriggerSmartContract(bytes)` and selector `0x46e6d31a`.
```solidity
function parseTriggerSmartContract(bytes memory encodedTx) external pure returns (ITronTxReader.TriggerSmartContract memory callData);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct parseTriggerSmartContractCall {
        #[allow(missing_docs)]
        pub encodedTx: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`parseTriggerSmartContract(bytes)`](parseTriggerSmartContractCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct parseTriggerSmartContractReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
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
            impl ::core::convert::From<parseTriggerSmartContractCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: parseTriggerSmartContractCall) -> Self {
                    (value.encodedTx,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for parseTriggerSmartContractCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { encodedTx: tuple.0 }
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
            impl ::core::convert::From<parseTriggerSmartContractReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: parseTriggerSmartContractReturn) -> Self {
                    (value.callData,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for parseTriggerSmartContractReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { callData: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for parseTriggerSmartContractCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <ITronTxReader::TriggerSmartContract as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (ITronTxReader::TriggerSmartContract,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "parseTriggerSmartContract(bytes)";
            const SELECTOR: [u8; 4] = [70u8, 230u8, 211u8, 26u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.encodedTx,
                    ),
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
                        let r: parseTriggerSmartContractReturn = r.into();
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
                        let r: parseTriggerSmartContractReturn = r.into();
                        r.callData
                    })
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
    /**Function with signature `verifyFirstBlockFinality(bytes[20])` and selector `0x76099a06`.
```solidity
function verifyFirstBlockFinality(bytes[20] memory blocks) external view returns (uint256 blockNumber, uint32 blockTimestamp, bytes32 txTrieRoot);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyFirstBlockFinalityCall {
        #[allow(missing_docs)]
        pub blocks: [alloy::sol_types::private::Bytes; 20usize],
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`verifyFirstBlockFinality(bytes[20])`](verifyFirstBlockFinalityCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyFirstBlockFinalityReturn {
        #[allow(missing_docs)]
        pub blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub blockTimestamp: u32,
        #[allow(missing_docs)]
        pub txTrieRoot: alloy::sol_types::private::FixedBytes<32>,
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::Bytes; 20usize],
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
            impl ::core::convert::From<verifyFirstBlockFinalityCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyFirstBlockFinalityCall) -> Self {
                    (value.blocks,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyFirstBlockFinalityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { blocks: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                u32,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<verifyFirstBlockFinalityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyFirstBlockFinalityReturn) -> Self {
                    (value.blockNumber, value.blockTimestamp, value.txTrieRoot)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyFirstBlockFinalityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blockNumber: tuple.0,
                        blockTimestamp: tuple.1,
                        txTrieRoot: tuple.2,
                    }
                }
            }
        }
        impl verifyFirstBlockFinalityReturn {
            fn _tokenize(
                &self,
            ) -> <verifyFirstBlockFinalityCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockTimestamp),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.txTrieRoot),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyFirstBlockFinalityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Bytes,
                    20usize,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyFirstBlockFinalityReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyFirstBlockFinality(bytes[20])";
            const SELECTOR: [u8; 4] = [118u8, 9u8, 154u8, 6u8];
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
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                verifyFirstBlockFinalityReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `verifyInclusion(bytes32,bytes,bytes32[],uint256)` and selector `0x07ee9735`.
```solidity
function verifyInclusion(bytes32 root, bytes memory encodedTx, bytes32[] memory proof, uint256 index) external pure returns (bool ok);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyInclusionCall {
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
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
    ///Container type for the return parameters of the [`verifyInclusion(bytes32,bytes,bytes32[],uint256)`](verifyInclusionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyInclusionReturn {
        #[allow(missing_docs)]
        pub ok: bool,
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<verifyInclusionCall> for UnderlyingRustTuple<'_> {
                fn from(value: verifyInclusionCall) -> Self {
                    (value.root, value.encodedTx, value.proof, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyInclusionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        root: tuple.0,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
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
            impl ::core::convert::From<verifyInclusionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyInclusionReturn) -> Self {
                    (value.ok,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyInclusionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { ok: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyInclusionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyInclusion(bytes32,bytes,bytes32[],uint256)";
            const SELECTOR: [u8; 4] = [7u8, 238u8, 151u8, 53u8];
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
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.root),
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
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
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
                        let r: verifyInclusionReturn = r.into();
                        r.ok
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
                        let r: verifyInclusionReturn = r.into();
                        r.ok
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
    ///Container for all the [`StatefulTronTxReaderGasHarness`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum StatefulTronTxReaderGasHarnessCalls {
        #[allow(missing_docs)]
        parseTriggerSmartContract(parseTriggerSmartContractCall),
        #[allow(missing_docs)]
        readTriggerSmartContract(readTriggerSmartContractCall),
        #[allow(missing_docs)]
        srIndexPlus1(srIndexPlus1Call),
        #[allow(missing_docs)]
        srs(srsCall),
        #[allow(missing_docs)]
        verifyFirstBlockFinality(verifyFirstBlockFinalityCall),
        #[allow(missing_docs)]
        verifyInclusion(verifyInclusionCall),
        #[allow(missing_docs)]
        witnessDelegatees(witnessDelegateesCall),
    }
    impl StatefulTronTxReaderGasHarnessCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [7u8, 238u8, 151u8, 53u8],
            [18u8, 215u8, 19u8, 194u8],
            [57u8, 173u8, 254u8, 255u8],
            [70u8, 230u8, 211u8, 26u8],
            [73u8, 205u8, 159u8, 152u8],
            [118u8, 9u8, 154u8, 6u8],
            [126u8, 103u8, 14u8, 179u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(verifyInclusion),
            ::core::stringify!(witnessDelegatees),
            ::core::stringify!(srs),
            ::core::stringify!(parseTriggerSmartContract),
            ::core::stringify!(readTriggerSmartContract),
            ::core::stringify!(verifyFirstBlockFinality),
            ::core::stringify!(srIndexPlus1),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <verifyInclusionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <witnessDelegateesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <srsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <parseTriggerSmartContractCall as alloy_sol_types::SolCall>::SIGNATURE,
            <readTriggerSmartContractCall as alloy_sol_types::SolCall>::SIGNATURE,
            <verifyFirstBlockFinalityCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for StatefulTronTxReaderGasHarnessCalls {
        const NAME: &'static str = "StatefulTronTxReaderGasHarnessCalls";
        const MIN_DATA_LENGTH: usize = 32usize;
        const COUNT: usize = 7usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::parseTriggerSmartContract(_) => {
                    <parseTriggerSmartContractCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::readTriggerSmartContract(_) => {
                    <readTriggerSmartContractCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::srIndexPlus1(_) => {
                    <srIndexPlus1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::srs(_) => <srsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::verifyFirstBlockFinality(_) => {
                    <verifyFirstBlockFinalityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyInclusion(_) => {
                    <verifyInclusionCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
            ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessCalls>] = &[
                {
                    fn verifyInclusion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessCalls> {
                        <verifyInclusionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderGasHarnessCalls::verifyInclusion)
                    }
                    verifyInclusion
                },
                {
                    fn witnessDelegatees(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessCalls> {
                        <witnessDelegateesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderGasHarnessCalls::witnessDelegatees)
                    }
                    witnessDelegatees
                },
                {
                    fn srs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessCalls> {
                        <srsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(StatefulTronTxReaderGasHarnessCalls::srs)
                    }
                    srs
                },
                {
                    fn parseTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessCalls> {
                        <parseTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessCalls::parseTriggerSmartContract,
                            )
                    }
                    parseTriggerSmartContract
                },
                {
                    fn readTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessCalls> {
                        <readTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessCalls::readTriggerSmartContract,
                            )
                    }
                    readTriggerSmartContract
                },
                {
                    fn verifyFirstBlockFinality(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessCalls> {
                        <verifyFirstBlockFinalityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessCalls::verifyFirstBlockFinality,
                            )
                    }
                    verifyFirstBlockFinality
                },
                {
                    fn srIndexPlus1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessCalls> {
                        <srIndexPlus1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderGasHarnessCalls::srIndexPlus1)
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
            ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessCalls>] = &[
                {
                    fn verifyInclusion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessCalls> {
                        <verifyInclusionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderGasHarnessCalls::verifyInclusion)
                    }
                    verifyInclusion
                },
                {
                    fn witnessDelegatees(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessCalls> {
                        <witnessDelegateesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderGasHarnessCalls::witnessDelegatees)
                    }
                    witnessDelegatees
                },
                {
                    fn srs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessCalls> {
                        <srsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderGasHarnessCalls::srs)
                    }
                    srs
                },
                {
                    fn parseTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessCalls> {
                        <parseTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessCalls::parseTriggerSmartContract,
                            )
                    }
                    parseTriggerSmartContract
                },
                {
                    fn readTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessCalls> {
                        <readTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessCalls::readTriggerSmartContract,
                            )
                    }
                    readTriggerSmartContract
                },
                {
                    fn verifyFirstBlockFinality(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessCalls> {
                        <verifyFirstBlockFinalityCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessCalls::verifyFirstBlockFinality,
                            )
                    }
                    verifyFirstBlockFinality
                },
                {
                    fn srIndexPlus1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessCalls> {
                        <srIndexPlus1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderGasHarnessCalls::srIndexPlus1)
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
                Self::parseTriggerSmartContract(inner) => {
                    <parseTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
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
                Self::verifyFirstBlockFinality(inner) => {
                    <verifyFirstBlockFinalityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyInclusion(inner) => {
                    <verifyInclusionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::parseTriggerSmartContract(inner) => {
                    <parseTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
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
                Self::verifyFirstBlockFinality(inner) => {
                    <verifyFirstBlockFinalityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyInclusion(inner) => {
                    <verifyInclusionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
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
    ///Container for all the [`StatefulTronTxReaderGasHarness`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum StatefulTronTxReaderGasHarnessErrors {
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
    impl StatefulTronTxReaderGasHarnessErrors {
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
    impl alloy_sol_types::SolInterface for StatefulTronTxReaderGasHarnessErrors {
        const NAME: &'static str = "StatefulTronTxReaderGasHarnessErrors";
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
            ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors>] = &[
                {
                    fn InvalidTxMerkleProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <InvalidTxMerkleProof as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::InvalidTxMerkleProof,
                            )
                    }
                    InvalidTxMerkleProof
                },
                {
                    fn InvalidWitnessSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <InvalidWitnessSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::InvalidWitnessSignature,
                            )
                    }
                    InvalidWitnessSignature
                },
                {
                    fn ProtoTruncated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <ProtoTruncated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderGasHarnessErrors::ProtoTruncated)
                    }
                    ProtoTruncated
                },
                {
                    fn SrSetNotSorted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <SrSetNotSorted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderGasHarnessErrors::SrSetNotSorted)
                    }
                    SrSetNotSorted
                },
                {
                    fn TimestampOverflow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <TimestampOverflow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderGasHarnessErrors::TimestampOverflow)
                    }
                    TimestampOverflow
                },
                {
                    fn InvalidEncodedBlockLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <InvalidEncodedBlockLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::InvalidEncodedBlockLength,
                            )
                    }
                    InvalidEncodedBlockLength
                },
                {
                    fn TronInvalidOwnerLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <TronInvalidOwnerLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::TronInvalidOwnerLength,
                            )
                    }
                    TronInvalidOwnerLength
                },
                {
                    fn InvalidWitnessAddressPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <InvalidWitnessAddressPrefix as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::InvalidWitnessAddressPrefix,
                            )
                    }
                    InvalidWitnessAddressPrefix
                },
                {
                    fn TronInvalidOwnerPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <TronInvalidOwnerPrefix as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::TronInvalidOwnerPrefix,
                            )
                    }
                    TronInvalidOwnerPrefix
                },
                {
                    fn ProtoInvalidWireType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <ProtoInvalidWireType as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::ProtoInvalidWireType,
                            )
                    }
                    ProtoInvalidWireType
                },
                {
                    fn TronInvalidContractPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <TronInvalidContractPrefix as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::TronInvalidContractPrefix,
                            )
                    }
                    TronInvalidContractPrefix
                },
                {
                    fn DuplicateSr(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <DuplicateSr as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(StatefulTronTxReaderGasHarnessErrors::DuplicateSr)
                    }
                    DuplicateSr
                },
                {
                    fn NotTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <NotTriggerSmartContract as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::NotTriggerSmartContract,
                            )
                    }
                    NotTriggerSmartContract
                },
                {
                    fn TronTxNotSuccessful(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <TronTxNotSuccessful as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::TronTxNotSuccessful,
                            )
                    }
                    TronTxNotSuccessful
                },
                {
                    fn UnknownSr(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <UnknownSr as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(StatefulTronTxReaderGasHarnessErrors::UnknownSr)
                    }
                    UnknownSr
                },
                {
                    fn TronInvalidContractLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <TronInvalidContractLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::TronInvalidContractLength,
                            )
                    }
                    TronInvalidContractLength
                },
                {
                    fn InvalidBlockSequence(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <InvalidBlockSequence as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::InvalidBlockSequence,
                            )
                    }
                    InvalidBlockSequence
                },
                {
                    fn InvalidHeaderPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <InvalidHeaderPrefix as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::InvalidHeaderPrefix,
                            )
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
            ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors>] = &[
                {
                    fn InvalidTxMerkleProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <InvalidTxMerkleProof as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::InvalidTxMerkleProof,
                            )
                    }
                    InvalidTxMerkleProof
                },
                {
                    fn InvalidWitnessSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <InvalidWitnessSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::InvalidWitnessSignature,
                            )
                    }
                    InvalidWitnessSignature
                },
                {
                    fn ProtoTruncated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <ProtoTruncated as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderGasHarnessErrors::ProtoTruncated)
                    }
                    ProtoTruncated
                },
                {
                    fn SrSetNotSorted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <SrSetNotSorted as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderGasHarnessErrors::SrSetNotSorted)
                    }
                    SrSetNotSorted
                },
                {
                    fn TimestampOverflow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <TimestampOverflow as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderGasHarnessErrors::TimestampOverflow)
                    }
                    TimestampOverflow
                },
                {
                    fn InvalidEncodedBlockLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <InvalidEncodedBlockLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::InvalidEncodedBlockLength,
                            )
                    }
                    InvalidEncodedBlockLength
                },
                {
                    fn TronInvalidOwnerLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <TronInvalidOwnerLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::TronInvalidOwnerLength,
                            )
                    }
                    TronInvalidOwnerLength
                },
                {
                    fn InvalidWitnessAddressPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <InvalidWitnessAddressPrefix as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::InvalidWitnessAddressPrefix,
                            )
                    }
                    InvalidWitnessAddressPrefix
                },
                {
                    fn TronInvalidOwnerPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <TronInvalidOwnerPrefix as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::TronInvalidOwnerPrefix,
                            )
                    }
                    TronInvalidOwnerPrefix
                },
                {
                    fn ProtoInvalidWireType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <ProtoInvalidWireType as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::ProtoInvalidWireType,
                            )
                    }
                    ProtoInvalidWireType
                },
                {
                    fn TronInvalidContractPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <TronInvalidContractPrefix as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::TronInvalidContractPrefix,
                            )
                    }
                    TronInvalidContractPrefix
                },
                {
                    fn DuplicateSr(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <DuplicateSr as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderGasHarnessErrors::DuplicateSr)
                    }
                    DuplicateSr
                },
                {
                    fn NotTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <NotTriggerSmartContract as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::NotTriggerSmartContract,
                            )
                    }
                    NotTriggerSmartContract
                },
                {
                    fn TronTxNotSuccessful(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <TronTxNotSuccessful as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::TronTxNotSuccessful,
                            )
                    }
                    TronTxNotSuccessful
                },
                {
                    fn UnknownSr(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <UnknownSr as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderGasHarnessErrors::UnknownSr)
                    }
                    UnknownSr
                },
                {
                    fn TronInvalidContractLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <TronInvalidContractLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::TronInvalidContractLength,
                            )
                    }
                    TronInvalidContractLength
                },
                {
                    fn InvalidBlockSequence(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <InvalidBlockSequence as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::InvalidBlockSequence,
                            )
                    }
                    InvalidBlockSequence
                },
                {
                    fn InvalidHeaderPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderGasHarnessErrors> {
                        <InvalidHeaderPrefix as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderGasHarnessErrors::InvalidHeaderPrefix,
                            )
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
    /**Creates a new wrapper around an on-chain [`StatefulTronTxReaderGasHarness`](self) contract instance.

See the [wrapper's documentation](`StatefulTronTxReaderGasHarnessInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> StatefulTronTxReaderGasHarnessInstance<P, N> {
        StatefulTronTxReaderGasHarnessInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<StatefulTronTxReaderGasHarnessInstance<P, N>>,
    > {
        StatefulTronTxReaderGasHarnessInstance::<
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
        StatefulTronTxReaderGasHarnessInstance::<
            P,
            N,
        >::deploy_builder(__provider, _srs, _witnessDelegatees)
    }
    /**A [`StatefulTronTxReaderGasHarness`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`StatefulTronTxReaderGasHarness`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct StatefulTronTxReaderGasHarnessInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for StatefulTronTxReaderGasHarnessInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("StatefulTronTxReaderGasHarnessInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StatefulTronTxReaderGasHarnessInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`StatefulTronTxReaderGasHarness`](self) contract instance.

See the [wrapper's documentation](`StatefulTronTxReaderGasHarnessInstance`) for more details.*/
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
        ) -> alloy_contract::Result<StatefulTronTxReaderGasHarnessInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> StatefulTronTxReaderGasHarnessInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> StatefulTronTxReaderGasHarnessInstance<P, N> {
            StatefulTronTxReaderGasHarnessInstance {
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
    > StatefulTronTxReaderGasHarnessInstance<P, N> {
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
        ///Creates a new call builder for the [`parseTriggerSmartContract`] function.
        pub fn parseTriggerSmartContract(
            &self,
            encodedTx: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, parseTriggerSmartContractCall, N> {
            self.call_builder(
                &parseTriggerSmartContractCall {
                    encodedTx,
                },
            )
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
        ///Creates a new call builder for the [`verifyFirstBlockFinality`] function.
        pub fn verifyFirstBlockFinality(
            &self,
            blocks: [alloy::sol_types::private::Bytes; 20usize],
        ) -> alloy_contract::SolCallBuilder<&P, verifyFirstBlockFinalityCall, N> {
            self.call_builder(
                &verifyFirstBlockFinalityCall {
                    blocks,
                },
            )
        }
        ///Creates a new call builder for the [`verifyInclusion`] function.
        pub fn verifyInclusion(
            &self,
            root: alloy::sol_types::private::FixedBytes<32>,
            encodedTx: alloy::sol_types::private::Bytes,
            proof: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, verifyInclusionCall, N> {
            self.call_builder(
                &verifyInclusionCall {
                    root,
                    encodedTx,
                    proof,
                    index,
                },
            )
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
    > StatefulTronTxReaderGasHarnessInstance<P, N> {
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
