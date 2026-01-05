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

interface StatefulTronTxReaderHarness {
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
    function verifySingleBlock(bytes memory block_) external view returns (bytes32 nextBlockId, uint32 nextSeen, uint256 blockNumber, uint32 blockTimestamp, bytes32 txTrieRoot);
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
    "name": "verifySingleBlock",
    "inputs": [
      {
        "name": "block_",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "nextBlockId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "nextSeen",
        "type": "uint32",
        "internalType": "uint32"
      },
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
pub mod StatefulTronTxReaderHarness {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50604051611d65380380611d6583398101604081905261002e9161026a565b81516001600160601b0319165f908152602081905260409020805460ff19166001908117909155829082905b601b811015610142575f836100706001846102c7565b601b81106100805761008061029f565b602002015190505f8483601b811061009a5761009a61029f565b60200201519050606081811c9083901c106100e657604051624c919360e81b8152600481018490526001600160601b031980841660248301528216604482015260640160405180910390fd5b6100f18360016102e0565b5f5f8786601b81106101055761010561029f565b602090810291909101516001600160601b03191682528101919091526040015f20805460ff191660ff92909216919091179055505060010161005a565b50610150600183601b610168565b5061015e601c82601b610168565b50505050506102f3565b82601b81019282156101ab579160200282015b828111156101ab57825182546001600160a01b03191660609190911c17825560209092019160019091019061017b565b506101b79291506101bb565b5090565b5b808211156101b7575f81556001016101bc565b80516001600160601b0319811681146101e6575f5ffd5b919050565b5f82601f8301126101fa575f5ffd5b60405161036081016001600160401b038111828210171561022957634e487b7160e01b5f52604160045260245ffd5b6040528061036084018581111561023e575f5ffd5b845b8181101561025f57610251816101cf565b835260209283019201610240565b509195945050505050565b5f5f6106c0838503121561027c575f5ffd5b61028684846101eb565b91506102968461036085016101eb565b90509250929050565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b818103818111156102da576102da6102b3565b92915050565b808201808211156102da576102da6102b3565b611a65806103005f395ff3fe608060405234801561000f575f5ffd5b506004361061007a575f3560e01c806346e6d31a1161005857806346e6d31a1461010457806349cd9f981461012457806376099a06146101375780637e670eb31461016a575f5ffd5b806312d713c21461007e5780632af05fc5146100af57806339adfeff146100f1575b5f5ffd5b61009161008c366004611669565b61019e565b6040516001600160601b031990911681526020015b60405180910390f35b6100c26100bd3660046116c4565b6101b7565b6040805195865263ffffffff94851660208701528501929092529091166060830152608082015260a0016100a6565b6100916100ff366004611669565b6101db565b6101176101123660046116c4565b6101ea565b6040516100a69190611702565b6101176101323660046117a0565b610205565b61014a61014536600461186f565b6102f3565b6040805193845263ffffffff9092166020840152908201526060016100a6565b61018c6101783660046118a0565b5f6020819052908152604090205460ff1681565b60405160ff90911681526020016100a6565b601c81601b81106101ad575f80fd5b015460601b905081565b5f808080806101c88787838061030c565b939b929a50909850965090945092505050565b600181601b81106101ad575f80fd5b6101f261161e565b6101fc838361047c565b90505b92915050565b61020d61161e565b5f5f5f6102198a61055e565b9250925092506102ad8160028b8b6040516102359291906118ce565b602060405180830381855afa158015610250573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061027391906118dd565b8989808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152508b92506105da915050565b6102ca576040516301d7cdd360e21b815260040160405180910390fd5b6102d4898961047c565b60208101939093525063ffffffff166040820152979650505050505050565b5f5f5f6102ff8461055e565b9250925092509193909250565b5f5f5f5f5f5f5f61031d8b8b610710565b9098509296509094509250905088158015906103395750818914155b156103575760405163e14a793160e01b815260040160405180910390fd5b5f6103628c8c6108fb565b90505f610370828e8e610957565b6001600160601b031984165f9081526020819052604081205491925060ff909116908190036103c35760405163cd42738b60e01b81526001600160601b0319851660048201526024015b60405180910390fd5b5f6103cf600183611908565b9050600160ff82161b8c811663ffffffff161561040b5760405163583a88ff60e11b81526001600160601b0319871660048201526024016103ba565b8c81179a506001600160601b03198416601c60ff8416601b811061043157610431611921565b015460601b6001600160601b0319161461045e576040516313d6dc7360e01b815260040160405180910390fd5b6104688a866109c9565b9b5050505050505050945094509450945094565b61048461161e565b5f5f5f5f5f5f5f6104958a8a6109ec565b9850965090508581118015906104ab5750888611155b6104b7576104b7611935565b6104c38a8a8389610ac8565b9297509095509350915050811580156104da575080155b156104f85760405163306e189b60e21b815260040160405180910390fd5b61050489898781610b90565b6105215760405163c2c062d160e01b815260040160405180910390fd5b8587526affffffffffffffffffffff1980851660608901528316608088015261054c89898484610c8a565b60a08801525094979650505050505050565b5f80808080805b60148110156105d0575f5f5f5f5f61059f8c876014811061058857610588611921565b6020028101906105989190611949565b8a8a61030c565b94509450945094509450849750839650855f036105c057829a508199508098505b5050505050806001019050610565565b5050509193909250565b5f83815b8451811015610702575f8582815181106105fa576105fa611921565b60200260200101519050816001901b85165f0361068757604080516020810185905290810182905260029060600160408051601f19818403018152908290526106429161198b565b602060405180830381855afa15801561065d573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061068091906118dd565b92506106f9565b604080516020810183905290810184905260029060600160408051601f19818403018152908290526106b89161198b565b602060405180830381855afa1580156106d3573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906106f691906118dd565b92505b506001016105de565b50851490505b949350505050565b5f8080808060ae86146107395760405163600d155160e01b8152600481018790526024016103ba565b86865f81811061074b5761074b611921565b9050013560f81c60f81b6001600160f81b031916600a60f81b14158061079b57508686600181811061077f5761077f611921565b9050013560f81c60f81b6001600160f81b031916606960f81b14155b806107d057508686606b8181106107b4576107b4611921565b9050013560f81c60f81b6001600160f81b031916601260f81b14155b8061080557508686606c8181106107e9576107e9611921565b9050013560f81c60f81b6001600160f81b031916604160f81b14155b156108235760405163ef02c9bb60e01b815260040160405180910390fd5b5f610832888860036009610d72565b506001600160401b031690505f61084b6103e8836119a1565b905063ffffffff8111156108725760405163549a019760e01b815260040160405180910390fd5b9550600b8801359450602d8801359350855f6108928a8a604e6052610d72565b6001600160401b03909116955090505f8a8a60548181106108b5576108b5611921565b919091013560f81c915050604181146108e657604051634fa88d5f60e11b815260ff821660048201526024016103ba565b50969995989497509295505050506055013590565b5f600261090b606b8285876119c0565b6040516109199291906118ce565b602060405180830381855afa158015610934573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906101fc91906118dd565b5f60ad830135811a606d840135608d850135601b83101561097957601b830192505b601c8314601b84141761098d575f5f525f5ffd5b60405187815283602082015282604082015281606082015260208160808360015afa6109ba575f5f525f5ffd5b5160601b979650505050505050565b5f806109da6001600160c01b6119e7565b60c085901b9084161791505092915050565b5f808083801580610a18575085855f818110610a0a57610a0a611921565b9091013560f81c600a141590505b15610a365760405163306e189b60e21b815260040160405180910390fd5b60015f610a4588888486610d72565b96508692509050610a60826001600160401b03831685610e2f565b94506002610a7086888a8c6119c0565b604051610a7e9291906118ce565b602060405180830381855afa158015610a99573d5f5f3e3d5ffd5b5050506040513d601f19601f82011682018060405250810190610abc91906118dd565b93505050509250925092565b5f5f5f5f5f5f5f610adb8b8b8b8b610e64565b919450925090508183108015610af15750878211155b610afd57610afd611935565b6001600160401b038116601f14610b275760405163306e189b60e21b815260040160405180910390fd5b5f5f610b358d8d8787610f6f565b909250905081158015610b46575080155b15610b645760405163306e189b60e21b815260040160405180910390fd5b610b708d8d8484611029565b809950819a50829b50839c50505050505050505050945094509450949050565b5f825b8284108015610bbc5750858585818110610baf57610baf611921565b9091013560f81c60121490505b15610c0a575082610bcc816119fa565b93505f610bdb87878787610d72565b95509050818511610bee57610bee611935565b610c0285826001600160401b031686610e2f565b945050610b93565b5f5b8385108015610c355750868686818110610c2857610c28611921565b9091013560f81c602a1490505b15610c8057506001610c46856119fa565b94505f5f610c56898989896111f4565b98509092509050610c698989848461125a565b610c79575f945050505050610708565b5050610c0c565b9695505050505050565b606082821080610c9957508382115b15610cb757604051633ffd665960e01b815260040160405180910390fd5b5f610cc284846119e7565b9050806001600160401b03811115610cdc57610cdc611a12565b6040519080825280601f01601f191660200182016040528015610d06576020820181803683370190505b5091505f5b81811015610d68578686610d1f8388611a26565b818110610d2e57610d2e611921565b9050013560f81c60f81b838281518110610d4a57610d4a611921565b60200101906001600160f81b03191690815f1a905350600101610d0b565b5050949350505050565b5f808080805b600a811015610e0c57858710610da157604051633ffd665960e01b815260040160405180910390fd5b5f898989610dae816119fa565b9a50818110610dbf57610dbf611921565b607f92013560f81c9182166001600160401b0386161b9590951794509050608081165f03610df65783889550955050505050610e26565b610e01600784611a39565b925050600101610d78565b50604051633ffd665960e01b815260040160405180910390fd5b94509492505050565b5f610e3a84836119e7565b831115610e5a57604051633ffd665960e01b815260040160405180910390fd5b6107088385611a26565b5f80808481815b86831015610f445750815f80610e838c8c858c61133a565b96509092509050828511610e9957610e99611935565b816001600160401b0316600b148015610ebb57506001600160401b0381166002145b15610f2e578315610edf5760405163306e189b60e21b815260040160405180910390fd5b60019350610eef8c8c878c6111f4565b919950975094505f610f038d8d8b8b61136e565b909750905080610f265760405163306e189b60e21b815260040160405180910390fd5b505050610f44565b610f3b8c8c878c85611404565b94505050610e6b565b81610f625760405163306e189b60e21b815260040160405180910390fd5b5050509450945094915050565b5f80838180825b8684101561101c5750825f80610f8e8c8c858c61133a565b97509092509050828611610fa457610fa4611935565b816001600160401b03166002148015610fc657506001600160401b0381166002145b1561100557610fd78c8c888c6111f4565b975090955093505f80610fec8e8e89896114ad565b91509150815f14610ffe578199508098505b5050611015565b6110128c8c888c85611404565b95505b5050610f76565b5050505094509492505050565b5f80808085805b868210156111e75750805f806110488c8c858c61133a565b9550909250905082841161105e5761105e611935565b816001600160401b0316600114801561108057506001600160401b0381166002145b156110fb575f5f6110938e8e888e6111f4565b975090925090506110a482826119e7565b6015146110c45760405163902757b160e01b815260040160405180910390fd5b6110cf8e8e84611575565b9950895f1a6041146110f45760405163a4645d6560e01b815260040160405180910390fd5b50506111e0565b816001600160401b0316600214801561111d57506001600160401b0381166002145b15611191575f5f6111308e8e888e6111f4565b9750909250905061114182826119e7565b60151461116157604051636c8ee0d960e11b815260040160405180910390fd5b61116c8e8e84611575565b9850885f1a6041146110f45760405163547793ab60e11b815260040160405180910390fd5b816001600160401b031660041480156111b357506001600160401b0381166002145b156111d0576111c48c8c868c6111f4565b919750955093506111e0565b6111dd8c8c868c85611404565b93505b5050611030565b5050945094509450949050565b5f5f5f5f61120488888888610d72565b965086945090506001600160401b03811661121f85876119e7565b81111561123f57604051633ffd665960e01b815260040160405180910390fd5b6112498188611a26565b935083925050509450945094915050565b5f808084805b858210156113175750805f806112788b8b858b61133a565b9550909250905082841161128e5761128e611935565b6001600160401b038116611300575f6112a98c8c878c610d72565b955090506001600160401b0383166002036112e1576001600160401b038116156112dc575f975050505050505050610708565b6112fa565b826001600160401b03166003036112fa57600195508096505b50611310565b61130d8b8b868b85611404565b93505b5050611260565b82801561132d5750836001600160401b03166001145b9998505050505050505050565b5f5f5f5f5f61134b89898989610d72565b600382901c671fffffffffffffff169b60079092169a5098509650505050505050565b5f8083805b848210156113f95750805f8061138b8a8a858a61133a565b955090925090508284116113a1576113a1611935565b816001600160401b031660011480156113c157506001600160401b038116155b156113e3576113d28a8a868a610d72565b50955060019450610e269350505050565b6113f08a8a868a85611404565b93505050611373565b505094509492505050565b5f6001600160401b0382166114265761141f868686866115ac565b90506114a4565b6001196001600160401b03831601611450575f611445878787876111f4565b5092506114a4915050565b6004196001600160401b0383160161146e5761141f84600485610e2f565b5f196001600160401b0383160161148b5761141f84600885610e2f565b60405163a5a5fc4360e01b815260040160405180910390fd5b95945050505050565b5f8083805b848210156113f95750805f806114ca8a8a858a61133a565b955090925090508284116114e0576114e0611935565b816001600160401b0316600114801561150257506001600160401b0381166002145b1561151d576115138a8a868a6111f4565b50945061156e9050565b816001600160401b0316600214801561153f57506001600160401b0381166002145b1561155e576115508a8a868a6111f4565b50909650945084935061156e565b61156b8a8a868a85611404565b93505b50506114b2565b5f82611582836015611a26565b11156115a157604051633ffd665960e01b815260040160405180910390fd5b509190910135919050565b5f805b600a811015610e0c578284106115d857604051633ffd665960e01b815260040160405180910390fd5b5f8686866115e5816119fa565b97508181106115f6576115f6611921565b919091013560f81c915050608081165f03611615578492505050610708565b506001016115af565b6040518060c001604052805f81526020015f81526020015f63ffffffff1681526020015f6001600160581b03191681526020015f6001600160581b0319168152602001606081525090565b5f60208284031215611679575f5ffd5b5035919050565b5f5f83601f840112611690575f5ffd5b5081356001600160401b038111156116a6575f5ffd5b6020830191508360208285010111156116bd575f5ffd5b9250929050565b5f5f602083850312156116d5575f5ffd5b82356001600160401b038111156116ea575f5ffd5b6116f685828601611680565b90969095509350505050565b60208152815160208201526020820151604082015263ffffffff60408301511660608201526001600160581b031960608301511660808201526001600160581b031960808301511660a08201525f60a083015160c08084015280518060e0850152806020830161010086015e5f6101008286010152610100601f19601f8301168501019250505092915050565b8061028081018310156101ff575f5ffd5b5f5f5f5f5f5f608087890312156117b5575f5ffd5b86356001600160401b038111156117ca575f5ffd5b6117d689828a0161178f565b96505060208701356001600160401b038111156117f1575f5ffd5b6117fd89828a01611680565b90965094505060408701356001600160401b0381111561181b575f5ffd5b8701601f8101891361182b575f5ffd5b80356001600160401b03811115611840575f5ffd5b8960208260051b8401011115611854575f5ffd5b96999598509396602090940195946060909401359392505050565b5f6020828403121561187f575f5ffd5b81356001600160401b03811115611894575f5ffd5b6107088482850161178f565b5f602082840312156118b0575f5ffd5b81356001600160601b0319811681146118c7575f5ffd5b9392505050565b818382375f9101908152919050565b5f602082840312156118ed575f5ffd5b5051919050565b634e487b7160e01b5f52601160045260245ffd5b60ff82811682821603908111156101ff576101ff6118f4565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52600160045260245ffd5b5f5f8335601e1984360301811261195e575f5ffd5b8301803591506001600160401b03821115611977575f5ffd5b6020019150368190038213156116bd575f5ffd5b5f82518060208501845e5f920191825250919050565b5f826119bb57634e487b7160e01b5f52601260045260245ffd5b500490565b5f5f858511156119ce575f5ffd5b838611156119da575f5ffd5b5050820193919092039150565b818103818111156101ff576101ff6118f4565b5f60018201611a0b57611a0b6118f4565b5060010190565b634e487b7160e01b5f52604160045260245ffd5b808201808211156101ff576101ff6118f4565b6001600160401b0381811683821601908111156101ff576101ff6118f456fea164736f6c634300081b000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x1De8\x03\x80a\x1De\x839\x81\x01`@\x81\x90Ra\0.\x91a\x02jV[\x81Q`\x01`\x01``\x1B\x03\x19\x16_\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x82\x90\x82\x90[`\x1B\x81\x10\x15a\x01BW_\x83a\0p`\x01\x84a\x02\xC7V[`\x1B\x81\x10a\0\x80Wa\0\x80a\x02\x9FV[` \x02\x01Q\x90P_\x84\x83`\x1B\x81\x10a\0\x9AWa\0\x9Aa\x02\x9FV[` \x02\x01Q\x90P``\x81\x81\x1C\x90\x83\x90\x1C\x10a\0\xE6W`@QbL\x91\x93`\xE8\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01``\x1B\x03\x19\x80\x84\x16`$\x83\x01R\x82\x16`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[a\0\xF1\x83`\x01a\x02\xE0V[__\x87\x86`\x1B\x81\x10a\x01\x05Wa\x01\x05a\x02\x9FV[` \x90\x81\x02\x91\x90\x91\x01Q`\x01`\x01``\x1B\x03\x19\x16\x82R\x81\x01\x91\x90\x91R`@\x01_ \x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UPP`\x01\x01a\0ZV[Pa\x01P`\x01\x83`\x1Ba\x01hV[Pa\x01^`\x1C\x82`\x1Ba\x01hV[PPPPPa\x02\xF3V[\x82`\x1B\x81\x01\x92\x82\x15a\x01\xABW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x01\xABW\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x91\x90\x91\x1C\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\x01{V[Pa\x01\xB7\x92\x91Pa\x01\xBBV[P\x90V[[\x80\x82\x11\x15a\x01\xB7W_\x81U`\x01\x01a\x01\xBCV[\x80Q`\x01`\x01``\x1B\x03\x19\x81\x16\x81\x14a\x01\xE6W__\xFD[\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x01\xFAW__\xFD[`@Qa\x03`\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x02)WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@R\x80a\x03`\x84\x01\x85\x81\x11\x15a\x02>W__\xFD[\x84[\x81\x81\x10\x15a\x02_Wa\x02Q\x81a\x01\xCFV[\x83R` \x92\x83\x01\x92\x01a\x02@V[P\x91\x95\x94PPPPPV[__a\x06\xC0\x83\x85\x03\x12\x15a\x02|W__\xFD[a\x02\x86\x84\x84a\x01\xEBV[\x91Pa\x02\x96\x84a\x03`\x85\x01a\x01\xEBV[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\xDAWa\x02\xDAa\x02\xB3V[\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x02\xDAWa\x02\xDAa\x02\xB3V[a\x1Ae\x80a\x03\0_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0zW_5`\xE0\x1C\x80cF\xE6\xD3\x1A\x11a\0XW\x80cF\xE6\xD3\x1A\x14a\x01\x04W\x80cI\xCD\x9F\x98\x14a\x01$W\x80cv\t\x9A\x06\x14a\x017W\x80c~g\x0E\xB3\x14a\x01jW__\xFD[\x80c\x12\xD7\x13\xC2\x14a\0~W\x80c*\xF0_\xC5\x14a\0\xAFW\x80c9\xAD\xFE\xFF\x14a\0\xF1W[__\xFD[a\0\x91a\0\x8C6`\x04a\x16iV[a\x01\x9EV[`@Q`\x01`\x01``\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xC2a\0\xBD6`\x04a\x16\xC4V[a\x01\xB7V[`@\x80Q\x95\x86Rc\xFF\xFF\xFF\xFF\x94\x85\x16` \x87\x01R\x85\x01\x92\x90\x92R\x90\x91\x16``\x83\x01R`\x80\x82\x01R`\xA0\x01a\0\xA6V[a\0\x91a\0\xFF6`\x04a\x16iV[a\x01\xDBV[a\x01\x17a\x01\x126`\x04a\x16\xC4V[a\x01\xEAV[`@Qa\0\xA6\x91\x90a\x17\x02V[a\x01\x17a\x0126`\x04a\x17\xA0V[a\x02\x05V[a\x01Ja\x01E6`\x04a\x18oV[a\x02\xF3V[`@\x80Q\x93\x84Rc\xFF\xFF\xFF\xFF\x90\x92\x16` \x84\x01R\x90\x82\x01R``\x01a\0\xA6V[a\x01\x8Ca\x01x6`\x04a\x18\xA0V[_` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\0\xA6V[`\x1C\x81`\x1B\x81\x10a\x01\xADW_\x80\xFD[\x01T``\x1B\x90P\x81V[_\x80\x80\x80\x80a\x01\xC8\x87\x87\x83\x80a\x03\x0CV[\x93\x9B\x92\x9AP\x90\x98P\x96P\x90\x94P\x92PPPV[`\x01\x81`\x1B\x81\x10a\x01\xADW_\x80\xFD[a\x01\xF2a\x16\x1EV[a\x01\xFC\x83\x83a\x04|V[\x90P[\x92\x91PPV[a\x02\ra\x16\x1EV[___a\x02\x19\x8Aa\x05^V[\x92P\x92P\x92Pa\x02\xAD\x81`\x02\x8B\x8B`@Qa\x025\x92\x91\x90a\x18\xCEV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x02PW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02s\x91\x90a\x18\xDDV[\x89\x89\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8B\x92Pa\x05\xDA\x91PPV[a\x02\xCAW`@Qc\x01\xD7\xCD\xD3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02\xD4\x89\x89a\x04|V[` \x81\x01\x93\x90\x93RPc\xFF\xFF\xFF\xFF\x16`@\x82\x01R\x97\x96PPPPPPPV[___a\x02\xFF\x84a\x05^V[\x92P\x92P\x92P\x91\x93\x90\x92PV[_______a\x03\x1D\x8B\x8Ba\x07\x10V[\x90\x98P\x92\x96P\x90\x94P\x92P\x90P\x88\x15\x80\x15\x90a\x039WP\x81\x89\x14\x15[\x15a\x03WW`@Qc\xE1Jy1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x03b\x8C\x8Ca\x08\xFBV[\x90P_a\x03p\x82\x8E\x8Ea\tWV[`\x01`\x01``\x1B\x03\x19\x84\x16_\x90\x81R` \x81\x90R`@\x81 T\x91\x92P`\xFF\x90\x91\x16\x90\x81\x90\x03a\x03\xC3W`@Qc\xCDBs\x8B`\xE0\x1B\x81R`\x01`\x01``\x1B\x03\x19\x85\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[_a\x03\xCF`\x01\x83a\x19\x08V[\x90P`\x01`\xFF\x82\x16\x1B\x8C\x81\x16c\xFF\xFF\xFF\xFF\x16\x15a\x04\x0BW`@QcX:\x88\xFF`\xE1\x1B\x81R`\x01`\x01``\x1B\x03\x19\x87\x16`\x04\x82\x01R`$\x01a\x03\xBAV[\x8C\x81\x17\x9AP`\x01`\x01``\x1B\x03\x19\x84\x16`\x1C`\xFF\x84\x16`\x1B\x81\x10a\x041Wa\x041a\x19!V[\x01T``\x1B`\x01`\x01``\x1B\x03\x19\x16\x14a\x04^W`@Qc\x13\xD6\xDCs`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04h\x8A\x86a\t\xC9V[\x9BPPPPPPPP\x94P\x94P\x94P\x94P\x94V[a\x04\x84a\x16\x1EV[_______a\x04\x95\x8A\x8Aa\t\xECV[\x98P\x96P\x90P\x85\x81\x11\x80\x15\x90a\x04\xABWP\x88\x86\x11\x15[a\x04\xB7Wa\x04\xB7a\x195V[a\x04\xC3\x8A\x8A\x83\x89a\n\xC8V[\x92\x97P\x90\x95P\x93P\x91PP\x81\x15\x80\x15a\x04\xDAWP\x80\x15[\x15a\x04\xF8W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\x04\x89\x89\x87\x81a\x0B\x90V[a\x05!W`@Qc\xC2\xC0b\xD1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x87Rj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x85\x16``\x89\x01R\x83\x16`\x80\x88\x01Ra\x05L\x89\x89\x84\x84a\x0C\x8AV[`\xA0\x88\x01RP\x94\x97\x96PPPPPPPV[_\x80\x80\x80\x80\x80[`\x14\x81\x10\x15a\x05\xD0W_____a\x05\x9F\x8C\x87`\x14\x81\x10a\x05\x88Wa\x05\x88a\x19!V[` \x02\x81\x01\x90a\x05\x98\x91\x90a\x19IV[\x8A\x8Aa\x03\x0CV[\x94P\x94P\x94P\x94P\x94P\x84\x97P\x83\x96P\x85_\x03a\x05\xC0W\x82\x9AP\x81\x99P\x80\x98P[PPPPP\x80`\x01\x01\x90Pa\x05eV[PPP\x91\x93\x90\x92PV[_\x83\x81[\x84Q\x81\x10\x15a\x07\x02W_\x85\x82\x81Q\x81\x10a\x05\xFAWa\x05\xFAa\x19!V[` \x02` \x01\x01Q\x90P\x81`\x01\x90\x1B\x85\x16_\x03a\x06\x87W`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x82\x90R`\x02\x90``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x06B\x91a\x19\x8BV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x06]W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x80\x91\x90a\x18\xDDV[\x92Pa\x06\xF9V[`@\x80Q` \x81\x01\x83\x90R\x90\x81\x01\x84\x90R`\x02\x90``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x06\xB8\x91a\x19\x8BV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x06\xD3W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xF6\x91\x90a\x18\xDDV[\x92P[P`\x01\x01a\x05\xDEV[P\x85\x14\x90P[\x94\x93PPPPV[_\x80\x80\x80\x80`\xAE\x86\x14a\x079W`@Qc`\r\x15Q`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x03\xBAV[\x86\x86_\x81\x81\x10a\x07KWa\x07Ka\x19!V[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`\n`\xF8\x1B\x14\x15\x80a\x07\x9BWP\x86\x86`\x01\x81\x81\x10a\x07\x7FWa\x07\x7Fa\x19!V[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`i`\xF8\x1B\x14\x15[\x80a\x07\xD0WP\x86\x86`k\x81\x81\x10a\x07\xB4Wa\x07\xB4a\x19!V[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`\x12`\xF8\x1B\x14\x15[\x80a\x08\x05WP\x86\x86`l\x81\x81\x10a\x07\xE9Wa\x07\xE9a\x19!V[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`A`\xF8\x1B\x14\x15[\x15a\x08#W`@Qc\xEF\x02\xC9\xBB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x082\x88\x88`\x03`\ta\rrV[P`\x01`\x01`@\x1B\x03\x16\x90P_a\x08Ka\x03\xE8\x83a\x19\xA1V[\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15a\x08rW`@QcT\x9A\x01\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x95P`\x0B\x88\x015\x94P`-\x88\x015\x93P\x85_a\x08\x92\x8A\x8A`N`Ra\rrV[`\x01`\x01`@\x1B\x03\x90\x91\x16\x95P\x90P_\x8A\x8A`T\x81\x81\x10a\x08\xB5Wa\x08\xB5a\x19!V[\x91\x90\x91\x015`\xF8\x1C\x91PP`A\x81\x14a\x08\xE6W`@QcO\xA8\x8D_`\xE1\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`$\x01a\x03\xBAV[P\x96\x99\x95\x98\x94\x97P\x92\x95PPPP`U\x015\x90V[_`\x02a\t\x0B`k\x82\x85\x87a\x19\xC0V[`@Qa\t\x19\x92\x91\x90a\x18\xCEV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\t4W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xFC\x91\x90a\x18\xDDV[_`\xAD\x83\x015\x81\x1A`m\x84\x015`\x8D\x85\x015`\x1B\x83\x10\x15a\tyW`\x1B\x83\x01\x92P[`\x1C\x83\x14`\x1B\x84\x14\x17a\t\x8DW__R__\xFD[`@Q\x87\x81R\x83` \x82\x01R\x82`@\x82\x01R\x81``\x82\x01R` \x81`\x80\x83`\x01Z\xFAa\t\xBAW__R__\xFD[Q``\x1B\x97\x96PPPPPPPV[_\x80a\t\xDA`\x01`\x01`\xC0\x1Ba\x19\xE7V[`\xC0\x85\x90\x1B\x90\x84\x16\x17\x91PP\x92\x91PPV[_\x80\x80\x83\x80\x15\x80a\n\x18WP\x85\x85_\x81\x81\x10a\n\nWa\n\na\x19!V[\x90\x91\x015`\xF8\x1C`\n\x14\x15\x90P[\x15a\n6W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01_a\nE\x88\x88\x84\x86a\rrV[\x96P\x86\x92P\x90Pa\n`\x82`\x01`\x01`@\x1B\x03\x83\x16\x85a\x0E/V[\x94P`\x02a\np\x86\x88\x8A\x8Ca\x19\xC0V[`@Qa\n~\x92\x91\x90a\x18\xCEV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\n\x99W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xBC\x91\x90a\x18\xDDV[\x93PPPP\x92P\x92P\x92V[_______a\n\xDB\x8B\x8B\x8B\x8Ba\x0EdV[\x91\x94P\x92P\x90P\x81\x83\x10\x80\x15a\n\xF1WP\x87\x82\x11\x15[a\n\xFDWa\n\xFDa\x195V[`\x01`\x01`@\x1B\x03\x81\x16`\x1F\x14a\x0B'W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\x0B5\x8D\x8D\x87\x87a\x0FoV[\x90\x92P\x90P\x81\x15\x80\x15a\x0BFWP\x80\x15[\x15a\x0BdW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Bp\x8D\x8D\x84\x84a\x10)V[\x80\x99P\x81\x9AP\x82\x9BP\x83\x9CPPPPPPPPPP\x94P\x94P\x94P\x94\x90PV[_\x82[\x82\x84\x10\x80\x15a\x0B\xBCWP\x85\x85\x85\x81\x81\x10a\x0B\xAFWa\x0B\xAFa\x19!V[\x90\x91\x015`\xF8\x1C`\x12\x14\x90P[\x15a\x0C\nWP\x82a\x0B\xCC\x81a\x19\xFAV[\x93P_a\x0B\xDB\x87\x87\x87\x87a\rrV[\x95P\x90P\x81\x85\x11a\x0B\xEEWa\x0B\xEEa\x195V[a\x0C\x02\x85\x82`\x01`\x01`@\x1B\x03\x16\x86a\x0E/V[\x94PPa\x0B\x93V[_[\x83\x85\x10\x80\x15a\x0C5WP\x86\x86\x86\x81\x81\x10a\x0C(Wa\x0C(a\x19!V[\x90\x91\x015`\xF8\x1C`*\x14\x90P[\x15a\x0C\x80WP`\x01a\x0CF\x85a\x19\xFAV[\x94P__a\x0CV\x89\x89\x89\x89a\x11\xF4V[\x98P\x90\x92P\x90Pa\x0Ci\x89\x89\x84\x84a\x12ZV[a\x0CyW_\x94PPPPPa\x07\x08V[PPa\x0C\x0CV[\x96\x95PPPPPPV[``\x82\x82\x10\x80a\x0C\x99WP\x83\x82\x11[\x15a\x0C\xB7W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0C\xC2\x84\x84a\x19\xE7V[\x90P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\xDCWa\x0C\xDCa\x1A\x12V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\r\x06W` \x82\x01\x81\x806\x837\x01\x90P[P\x91P_[\x81\x81\x10\x15a\rhW\x86\x86a\r\x1F\x83\x88a\x1A&V[\x81\x81\x10a\r.Wa\r.a\x19!V[\x90P\x015`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\rJWa\rJa\x19!V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\r\x0BV[PP\x94\x93PPPPV[_\x80\x80\x80\x80[`\n\x81\x10\x15a\x0E\x0CW\x85\x87\x10a\r\xA1W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x89\x89\x89a\r\xAE\x81a\x19\xFAV[\x9AP\x81\x81\x10a\r\xBFWa\r\xBFa\x19!V[`\x7F\x92\x015`\xF8\x1C\x91\x82\x16`\x01`\x01`@\x1B\x03\x86\x16\x1B\x95\x90\x95\x17\x94P\x90P`\x80\x81\x16_\x03a\r\xF6W\x83\x88\x95P\x95PPPPPa\x0E&V[a\x0E\x01`\x07\x84a\x1A9V[\x92PP`\x01\x01a\rxV[P`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x94P\x94\x92PPPV[_a\x0E:\x84\x83a\x19\xE7V[\x83\x11\x15a\x0EZW`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\x08\x83\x85a\x1A&V[_\x80\x80\x84\x81\x81[\x86\x83\x10\x15a\x0FDWP\x81_\x80a\x0E\x83\x8C\x8C\x85\x8Ca\x13:V[\x96P\x90\x92P\x90P\x82\x85\x11a\x0E\x99Wa\x0E\x99a\x195V[\x81`\x01`\x01`@\x1B\x03\x16`\x0B\x14\x80\x15a\x0E\xBBWP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x0F.W\x83\x15a\x0E\xDFW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x93Pa\x0E\xEF\x8C\x8C\x87\x8Ca\x11\xF4V[\x91\x99P\x97P\x94P_a\x0F\x03\x8D\x8D\x8B\x8Ba\x13nV[\x90\x97P\x90P\x80a\x0F&W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPa\x0FDV[a\x0F;\x8C\x8C\x87\x8C\x85a\x14\x04V[\x94PPPa\x0EkV[\x81a\x0FbW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPP\x94P\x94P\x94\x91PPV[_\x80\x83\x81\x80\x82[\x86\x84\x10\x15a\x10\x1CWP\x82_\x80a\x0F\x8E\x8C\x8C\x85\x8Ca\x13:V[\x97P\x90\x92P\x90P\x82\x86\x11a\x0F\xA4Wa\x0F\xA4a\x195V[\x81`\x01`\x01`@\x1B\x03\x16`\x02\x14\x80\x15a\x0F\xC6WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x10\x05Wa\x0F\xD7\x8C\x8C\x88\x8Ca\x11\xF4V[\x97P\x90\x95P\x93P_\x80a\x0F\xEC\x8E\x8E\x89\x89a\x14\xADV[\x91P\x91P\x81_\x14a\x0F\xFEW\x81\x99P\x80\x98P[PPa\x10\x15V[a\x10\x12\x8C\x8C\x88\x8C\x85a\x14\x04V[\x95P[PPa\x0FvV[PPPP\x94P\x94\x92PPPV[_\x80\x80\x80\x85\x80[\x86\x82\x10\x15a\x11\xE7WP\x80_\x80a\x10H\x8C\x8C\x85\x8Ca\x13:V[\x95P\x90\x92P\x90P\x82\x84\x11a\x10^Wa\x10^a\x195V[\x81`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x10\x80WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x10\xFBW__a\x10\x93\x8E\x8E\x88\x8Ea\x11\xF4V[\x97P\x90\x92P\x90Pa\x10\xA4\x82\x82a\x19\xE7V[`\x15\x14a\x10\xC4W`@Qc\x90'W\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xCF\x8E\x8E\x84a\x15uV[\x99P\x89_\x1A`A\x14a\x10\xF4W`@Qc\xA4d]e`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPa\x11\xE0V[\x81`\x01`\x01`@\x1B\x03\x16`\x02\x14\x80\x15a\x11\x1DWP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x11\x91W__a\x110\x8E\x8E\x88\x8Ea\x11\xF4V[\x97P\x90\x92P\x90Pa\x11A\x82\x82a\x19\xE7V[`\x15\x14a\x11aW`@Qcl\x8E\xE0\xD9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11l\x8E\x8E\x84a\x15uV[\x98P\x88_\x1A`A\x14a\x10\xF4W`@QcTw\x93\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`@\x1B\x03\x16`\x04\x14\x80\x15a\x11\xB3WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x11\xD0Wa\x11\xC4\x8C\x8C\x86\x8Ca\x11\xF4V[\x91\x97P\x95P\x93Pa\x11\xE0V[a\x11\xDD\x8C\x8C\x86\x8C\x85a\x14\x04V[\x93P[PPa\x100V[PP\x94P\x94P\x94P\x94\x90PV[____a\x12\x04\x88\x88\x88\x88a\rrV[\x96P\x86\x94P\x90P`\x01`\x01`@\x1B\x03\x81\x16a\x12\x1F\x85\x87a\x19\xE7V[\x81\x11\x15a\x12?W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12I\x81\x88a\x1A&V[\x93P\x83\x92PPP\x94P\x94P\x94\x91PPV[_\x80\x80\x84\x80[\x85\x82\x10\x15a\x13\x17WP\x80_\x80a\x12x\x8B\x8B\x85\x8Ba\x13:V[\x95P\x90\x92P\x90P\x82\x84\x11a\x12\x8EWa\x12\x8Ea\x195V[`\x01`\x01`@\x1B\x03\x81\x16a\x13\0W_a\x12\xA9\x8C\x8C\x87\x8Ca\rrV[\x95P\x90P`\x01`\x01`@\x1B\x03\x83\x16`\x02\x03a\x12\xE1W`\x01`\x01`@\x1B\x03\x81\x16\x15a\x12\xDCW_\x97PPPPPPPPa\x07\x08V[a\x12\xFAV[\x82`\x01`\x01`@\x1B\x03\x16`\x03\x03a\x12\xFAW`\x01\x95P\x80\x96P[Pa\x13\x10V[a\x13\r\x8B\x8B\x86\x8B\x85a\x14\x04V[\x93P[PPa\x12`V[\x82\x80\x15a\x13-WP\x83`\x01`\x01`@\x1B\x03\x16`\x01\x14[\x99\x98PPPPPPPPPV[_____a\x13K\x89\x89\x89\x89a\rrV[`\x03\x82\x90\x1Cg\x1F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x9B`\x07\x90\x92\x16\x9AP\x98P\x96PPPPPPPV[_\x80\x83\x80[\x84\x82\x10\x15a\x13\xF9WP\x80_\x80a\x13\x8B\x8A\x8A\x85\x8Aa\x13:V[\x95P\x90\x92P\x90P\x82\x84\x11a\x13\xA1Wa\x13\xA1a\x195V[\x81`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x13\xC1WP`\x01`\x01`@\x1B\x03\x81\x16\x15[\x15a\x13\xE3Wa\x13\xD2\x8A\x8A\x86\x8Aa\rrV[P\x95P`\x01\x94Pa\x0E&\x93PPPPV[a\x13\xF0\x8A\x8A\x86\x8A\x85a\x14\x04V[\x93PPPa\x13sV[PP\x94P\x94\x92PPPV[_`\x01`\x01`@\x1B\x03\x82\x16a\x14&Wa\x14\x1F\x86\x86\x86\x86a\x15\xACV[\x90Pa\x14\xA4V[`\x01\x19`\x01`\x01`@\x1B\x03\x83\x16\x01a\x14PW_a\x14E\x87\x87\x87\x87a\x11\xF4V[P\x92Pa\x14\xA4\x91PPV[`\x04\x19`\x01`\x01`@\x1B\x03\x83\x16\x01a\x14nWa\x14\x1F\x84`\x04\x85a\x0E/V[_\x19`\x01`\x01`@\x1B\x03\x83\x16\x01a\x14\x8BWa\x14\x1F\x84`\x08\x85a\x0E/V[`@Qc\xA5\xA5\xFCC`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x95\x94PPPPPV[_\x80\x83\x80[\x84\x82\x10\x15a\x13\xF9WP\x80_\x80a\x14\xCA\x8A\x8A\x85\x8Aa\x13:V[\x95P\x90\x92P\x90P\x82\x84\x11a\x14\xE0Wa\x14\xE0a\x195V[\x81`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x15\x02WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x15\x1DWa\x15\x13\x8A\x8A\x86\x8Aa\x11\xF4V[P\x94Pa\x15n\x90PV[\x81`\x01`\x01`@\x1B\x03\x16`\x02\x14\x80\x15a\x15?WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x15^Wa\x15P\x8A\x8A\x86\x8Aa\x11\xF4V[P\x90\x96P\x94P\x84\x93Pa\x15nV[a\x15k\x8A\x8A\x86\x8A\x85a\x14\x04V[\x93P[PPa\x14\xB2V[_\x82a\x15\x82\x83`\x15a\x1A&V[\x11\x15a\x15\xA1W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x91\x90\x91\x015\x91\x90PV[_\x80[`\n\x81\x10\x15a\x0E\x0CW\x82\x84\x10a\x15\xD8W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x86\x86\x86a\x15\xE5\x81a\x19\xFAV[\x97P\x81\x81\x10a\x15\xF6Wa\x15\xF6a\x19!V[\x91\x90\x91\x015`\xF8\x1C\x91PP`\x80\x81\x16_\x03a\x16\x15W\x84\x92PPPa\x07\x08V[P`\x01\x01a\x15\xAFV[`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\x01`\x01`X\x1B\x03\x19\x16\x81R` \x01_`\x01`\x01`X\x1B\x03\x19\x16\x81R` \x01``\x81RP\x90V[_` \x82\x84\x03\x12\x15a\x16yW__\xFD[P5\x91\x90PV[__\x83`\x1F\x84\x01\x12a\x16\x90W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xA6W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x16\xBDW__\xFD[\x92P\x92\x90PV[__` \x83\x85\x03\x12\x15a\x16\xD5W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xEAW__\xFD[a\x16\xF6\x85\x82\x86\x01a\x16\x80V[\x90\x96\x90\x95P\x93PPPPV[` \x81R\x81Q` \x82\x01R` \x82\x01Q`@\x82\x01Rc\xFF\xFF\xFF\xFF`@\x83\x01Q\x16``\x82\x01R`\x01`\x01`X\x1B\x03\x19``\x83\x01Q\x16`\x80\x82\x01R`\x01`\x01`X\x1B\x03\x19`\x80\x83\x01Q\x16`\xA0\x82\x01R_`\xA0\x83\x01Q`\xC0\x80\x84\x01R\x80Q\x80`\xE0\x85\x01R\x80` \x83\x01a\x01\0\x86\x01^_a\x01\0\x82\x86\x01\x01Ra\x01\0`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x80a\x02\x80\x81\x01\x83\x10\x15a\x01\xFFW__\xFD[______`\x80\x87\x89\x03\x12\x15a\x17\xB5W__\xFD[\x865`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xCAW__\xFD[a\x17\xD6\x89\x82\x8A\x01a\x17\x8FV[\x96PP` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xF1W__\xFD[a\x17\xFD\x89\x82\x8A\x01a\x16\x80V[\x90\x96P\x94PP`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\x1BW__\xFD[\x87\x01`\x1F\x81\x01\x89\x13a\x18+W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18@W__\xFD[\x89` \x82`\x05\x1B\x84\x01\x01\x11\x15a\x18TW__\xFD[\x96\x99\x95\x98P\x93\x96` \x90\x94\x01\x95\x94``\x90\x94\x015\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x18\x7FW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\x94W__\xFD[a\x07\x08\x84\x82\x85\x01a\x17\x8FV[_` \x82\x84\x03\x12\x15a\x18\xB0W__\xFD[\x815`\x01`\x01``\x1B\x03\x19\x81\x16\x81\x14a\x18\xC7W__\xFD[\x93\x92PPPV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_` \x82\x84\x03\x12\x15a\x18\xEDW__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x01\xFFWa\x01\xFFa\x18\xF4V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x19^W__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x19wW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x16\xBDW__\xFD[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_\x82a\x19\xBBWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[__\x85\x85\x11\x15a\x19\xCEW__\xFD[\x83\x86\x11\x15a\x19\xDAW__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x81\x81\x03\x81\x81\x11\x15a\x01\xFFWa\x01\xFFa\x18\xF4V[_`\x01\x82\x01a\x1A\x0BWa\x1A\x0Ba\x18\xF4V[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x01\xFFWa\x01\xFFa\x18\xF4V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x01\xFFWa\x01\xFFa\x18\xF4V\xFE\xA1dsolcC\0\x08\x1B\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061007a575f3560e01c806346e6d31a1161005857806346e6d31a1461010457806349cd9f981461012457806376099a06146101375780637e670eb31461016a575f5ffd5b806312d713c21461007e5780632af05fc5146100af57806339adfeff146100f1575b5f5ffd5b61009161008c366004611669565b61019e565b6040516001600160601b031990911681526020015b60405180910390f35b6100c26100bd3660046116c4565b6101b7565b6040805195865263ffffffff94851660208701528501929092529091166060830152608082015260a0016100a6565b6100916100ff366004611669565b6101db565b6101176101123660046116c4565b6101ea565b6040516100a69190611702565b6101176101323660046117a0565b610205565b61014a61014536600461186f565b6102f3565b6040805193845263ffffffff9092166020840152908201526060016100a6565b61018c6101783660046118a0565b5f6020819052908152604090205460ff1681565b60405160ff90911681526020016100a6565b601c81601b81106101ad575f80fd5b015460601b905081565b5f808080806101c88787838061030c565b939b929a50909850965090945092505050565b600181601b81106101ad575f80fd5b6101f261161e565b6101fc838361047c565b90505b92915050565b61020d61161e565b5f5f5f6102198a61055e565b9250925092506102ad8160028b8b6040516102359291906118ce565b602060405180830381855afa158015610250573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061027391906118dd565b8989808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152508b92506105da915050565b6102ca576040516301d7cdd360e21b815260040160405180910390fd5b6102d4898961047c565b60208101939093525063ffffffff166040820152979650505050505050565b5f5f5f6102ff8461055e565b9250925092509193909250565b5f5f5f5f5f5f5f61031d8b8b610710565b9098509296509094509250905088158015906103395750818914155b156103575760405163e14a793160e01b815260040160405180910390fd5b5f6103628c8c6108fb565b90505f610370828e8e610957565b6001600160601b031984165f9081526020819052604081205491925060ff909116908190036103c35760405163cd42738b60e01b81526001600160601b0319851660048201526024015b60405180910390fd5b5f6103cf600183611908565b9050600160ff82161b8c811663ffffffff161561040b5760405163583a88ff60e11b81526001600160601b0319871660048201526024016103ba565b8c81179a506001600160601b03198416601c60ff8416601b811061043157610431611921565b015460601b6001600160601b0319161461045e576040516313d6dc7360e01b815260040160405180910390fd5b6104688a866109c9565b9b5050505050505050945094509450945094565b61048461161e565b5f5f5f5f5f5f5f6104958a8a6109ec565b9850965090508581118015906104ab5750888611155b6104b7576104b7611935565b6104c38a8a8389610ac8565b9297509095509350915050811580156104da575080155b156104f85760405163306e189b60e21b815260040160405180910390fd5b61050489898781610b90565b6105215760405163c2c062d160e01b815260040160405180910390fd5b8587526affffffffffffffffffffff1980851660608901528316608088015261054c89898484610c8a565b60a08801525094979650505050505050565b5f80808080805b60148110156105d0575f5f5f5f5f61059f8c876014811061058857610588611921565b6020028101906105989190611949565b8a8a61030c565b94509450945094509450849750839650855f036105c057829a508199508098505b5050505050806001019050610565565b5050509193909250565b5f83815b8451811015610702575f8582815181106105fa576105fa611921565b60200260200101519050816001901b85165f0361068757604080516020810185905290810182905260029060600160408051601f19818403018152908290526106429161198b565b602060405180830381855afa15801561065d573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061068091906118dd565b92506106f9565b604080516020810183905290810184905260029060600160408051601f19818403018152908290526106b89161198b565b602060405180830381855afa1580156106d3573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906106f691906118dd565b92505b506001016105de565b50851490505b949350505050565b5f8080808060ae86146107395760405163600d155160e01b8152600481018790526024016103ba565b86865f81811061074b5761074b611921565b9050013560f81c60f81b6001600160f81b031916600a60f81b14158061079b57508686600181811061077f5761077f611921565b9050013560f81c60f81b6001600160f81b031916606960f81b14155b806107d057508686606b8181106107b4576107b4611921565b9050013560f81c60f81b6001600160f81b031916601260f81b14155b8061080557508686606c8181106107e9576107e9611921565b9050013560f81c60f81b6001600160f81b031916604160f81b14155b156108235760405163ef02c9bb60e01b815260040160405180910390fd5b5f610832888860036009610d72565b506001600160401b031690505f61084b6103e8836119a1565b905063ffffffff8111156108725760405163549a019760e01b815260040160405180910390fd5b9550600b8801359450602d8801359350855f6108928a8a604e6052610d72565b6001600160401b03909116955090505f8a8a60548181106108b5576108b5611921565b919091013560f81c915050604181146108e657604051634fa88d5f60e11b815260ff821660048201526024016103ba565b50969995989497509295505050506055013590565b5f600261090b606b8285876119c0565b6040516109199291906118ce565b602060405180830381855afa158015610934573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906101fc91906118dd565b5f60ad830135811a606d840135608d850135601b83101561097957601b830192505b601c8314601b84141761098d575f5f525f5ffd5b60405187815283602082015282604082015281606082015260208160808360015afa6109ba575f5f525f5ffd5b5160601b979650505050505050565b5f806109da6001600160c01b6119e7565b60c085901b9084161791505092915050565b5f808083801580610a18575085855f818110610a0a57610a0a611921565b9091013560f81c600a141590505b15610a365760405163306e189b60e21b815260040160405180910390fd5b60015f610a4588888486610d72565b96508692509050610a60826001600160401b03831685610e2f565b94506002610a7086888a8c6119c0565b604051610a7e9291906118ce565b602060405180830381855afa158015610a99573d5f5f3e3d5ffd5b5050506040513d601f19601f82011682018060405250810190610abc91906118dd565b93505050509250925092565b5f5f5f5f5f5f5f610adb8b8b8b8b610e64565b919450925090508183108015610af15750878211155b610afd57610afd611935565b6001600160401b038116601f14610b275760405163306e189b60e21b815260040160405180910390fd5b5f5f610b358d8d8787610f6f565b909250905081158015610b46575080155b15610b645760405163306e189b60e21b815260040160405180910390fd5b610b708d8d8484611029565b809950819a50829b50839c50505050505050505050945094509450949050565b5f825b8284108015610bbc5750858585818110610baf57610baf611921565b9091013560f81c60121490505b15610c0a575082610bcc816119fa565b93505f610bdb87878787610d72565b95509050818511610bee57610bee611935565b610c0285826001600160401b031686610e2f565b945050610b93565b5f5b8385108015610c355750868686818110610c2857610c28611921565b9091013560f81c602a1490505b15610c8057506001610c46856119fa565b94505f5f610c56898989896111f4565b98509092509050610c698989848461125a565b610c79575f945050505050610708565b5050610c0c565b9695505050505050565b606082821080610c9957508382115b15610cb757604051633ffd665960e01b815260040160405180910390fd5b5f610cc284846119e7565b9050806001600160401b03811115610cdc57610cdc611a12565b6040519080825280601f01601f191660200182016040528015610d06576020820181803683370190505b5091505f5b81811015610d68578686610d1f8388611a26565b818110610d2e57610d2e611921565b9050013560f81c60f81b838281518110610d4a57610d4a611921565b60200101906001600160f81b03191690815f1a905350600101610d0b565b5050949350505050565b5f808080805b600a811015610e0c57858710610da157604051633ffd665960e01b815260040160405180910390fd5b5f898989610dae816119fa565b9a50818110610dbf57610dbf611921565b607f92013560f81c9182166001600160401b0386161b9590951794509050608081165f03610df65783889550955050505050610e26565b610e01600784611a39565b925050600101610d78565b50604051633ffd665960e01b815260040160405180910390fd5b94509492505050565b5f610e3a84836119e7565b831115610e5a57604051633ffd665960e01b815260040160405180910390fd5b6107088385611a26565b5f80808481815b86831015610f445750815f80610e838c8c858c61133a565b96509092509050828511610e9957610e99611935565b816001600160401b0316600b148015610ebb57506001600160401b0381166002145b15610f2e578315610edf5760405163306e189b60e21b815260040160405180910390fd5b60019350610eef8c8c878c6111f4565b919950975094505f610f038d8d8b8b61136e565b909750905080610f265760405163306e189b60e21b815260040160405180910390fd5b505050610f44565b610f3b8c8c878c85611404565b94505050610e6b565b81610f625760405163306e189b60e21b815260040160405180910390fd5b5050509450945094915050565b5f80838180825b8684101561101c5750825f80610f8e8c8c858c61133a565b97509092509050828611610fa457610fa4611935565b816001600160401b03166002148015610fc657506001600160401b0381166002145b1561100557610fd78c8c888c6111f4565b975090955093505f80610fec8e8e89896114ad565b91509150815f14610ffe578199508098505b5050611015565b6110128c8c888c85611404565b95505b5050610f76565b5050505094509492505050565b5f80808085805b868210156111e75750805f806110488c8c858c61133a565b9550909250905082841161105e5761105e611935565b816001600160401b0316600114801561108057506001600160401b0381166002145b156110fb575f5f6110938e8e888e6111f4565b975090925090506110a482826119e7565b6015146110c45760405163902757b160e01b815260040160405180910390fd5b6110cf8e8e84611575565b9950895f1a6041146110f45760405163a4645d6560e01b815260040160405180910390fd5b50506111e0565b816001600160401b0316600214801561111d57506001600160401b0381166002145b15611191575f5f6111308e8e888e6111f4565b9750909250905061114182826119e7565b60151461116157604051636c8ee0d960e11b815260040160405180910390fd5b61116c8e8e84611575565b9850885f1a6041146110f45760405163547793ab60e11b815260040160405180910390fd5b816001600160401b031660041480156111b357506001600160401b0381166002145b156111d0576111c48c8c868c6111f4565b919750955093506111e0565b6111dd8c8c868c85611404565b93505b5050611030565b5050945094509450949050565b5f5f5f5f61120488888888610d72565b965086945090506001600160401b03811661121f85876119e7565b81111561123f57604051633ffd665960e01b815260040160405180910390fd5b6112498188611a26565b935083925050509450945094915050565b5f808084805b858210156113175750805f806112788b8b858b61133a565b9550909250905082841161128e5761128e611935565b6001600160401b038116611300575f6112a98c8c878c610d72565b955090506001600160401b0383166002036112e1576001600160401b038116156112dc575f975050505050505050610708565b6112fa565b826001600160401b03166003036112fa57600195508096505b50611310565b61130d8b8b868b85611404565b93505b5050611260565b82801561132d5750836001600160401b03166001145b9998505050505050505050565b5f5f5f5f5f61134b89898989610d72565b600382901c671fffffffffffffff169b60079092169a5098509650505050505050565b5f8083805b848210156113f95750805f8061138b8a8a858a61133a565b955090925090508284116113a1576113a1611935565b816001600160401b031660011480156113c157506001600160401b038116155b156113e3576113d28a8a868a610d72565b50955060019450610e269350505050565b6113f08a8a868a85611404565b93505050611373565b505094509492505050565b5f6001600160401b0382166114265761141f868686866115ac565b90506114a4565b6001196001600160401b03831601611450575f611445878787876111f4565b5092506114a4915050565b6004196001600160401b0383160161146e5761141f84600485610e2f565b5f196001600160401b0383160161148b5761141f84600885610e2f565b60405163a5a5fc4360e01b815260040160405180910390fd5b95945050505050565b5f8083805b848210156113f95750805f806114ca8a8a858a61133a565b955090925090508284116114e0576114e0611935565b816001600160401b0316600114801561150257506001600160401b0381166002145b1561151d576115138a8a868a6111f4565b50945061156e9050565b816001600160401b0316600214801561153f57506001600160401b0381166002145b1561155e576115508a8a868a6111f4565b50909650945084935061156e565b61156b8a8a868a85611404565b93505b50506114b2565b5f82611582836015611a26565b11156115a157604051633ffd665960e01b815260040160405180910390fd5b509190910135919050565b5f805b600a811015610e0c578284106115d857604051633ffd665960e01b815260040160405180910390fd5b5f8686866115e5816119fa565b97508181106115f6576115f6611921565b919091013560f81c915050608081165f03611615578492505050610708565b506001016115af565b6040518060c001604052805f81526020015f81526020015f63ffffffff1681526020015f6001600160581b03191681526020015f6001600160581b0319168152602001606081525090565b5f60208284031215611679575f5ffd5b5035919050565b5f5f83601f840112611690575f5ffd5b5081356001600160401b038111156116a6575f5ffd5b6020830191508360208285010111156116bd575f5ffd5b9250929050565b5f5f602083850312156116d5575f5ffd5b82356001600160401b038111156116ea575f5ffd5b6116f685828601611680565b90969095509350505050565b60208152815160208201526020820151604082015263ffffffff60408301511660608201526001600160581b031960608301511660808201526001600160581b031960808301511660a08201525f60a083015160c08084015280518060e0850152806020830161010086015e5f6101008286010152610100601f19601f8301168501019250505092915050565b8061028081018310156101ff575f5ffd5b5f5f5f5f5f5f608087890312156117b5575f5ffd5b86356001600160401b038111156117ca575f5ffd5b6117d689828a0161178f565b96505060208701356001600160401b038111156117f1575f5ffd5b6117fd89828a01611680565b90965094505060408701356001600160401b0381111561181b575f5ffd5b8701601f8101891361182b575f5ffd5b80356001600160401b03811115611840575f5ffd5b8960208260051b8401011115611854575f5ffd5b96999598509396602090940195946060909401359392505050565b5f6020828403121561187f575f5ffd5b81356001600160401b03811115611894575f5ffd5b6107088482850161178f565b5f602082840312156118b0575f5ffd5b81356001600160601b0319811681146118c7575f5ffd5b9392505050565b818382375f9101908152919050565b5f602082840312156118ed575f5ffd5b5051919050565b634e487b7160e01b5f52601160045260245ffd5b60ff82811682821603908111156101ff576101ff6118f4565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52600160045260245ffd5b5f5f8335601e1984360301811261195e575f5ffd5b8301803591506001600160401b03821115611977575f5ffd5b6020019150368190038213156116bd575f5ffd5b5f82518060208501845e5f920191825250919050565b5f826119bb57634e487b7160e01b5f52601260045260245ffd5b500490565b5f5f858511156119ce575f5ffd5b838611156119da575f5ffd5b5050820193919092039150565b818103818111156101ff576101ff6118f4565b5f60018201611a0b57611a0b6118f4565b5060010190565b634e487b7160e01b5f52604160045260245ffd5b808201808211156101ff576101ff6118f4565b6001600160401b0381811683821601908111156101ff576101ff6118f456fea164736f6c634300081b000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0zW_5`\xE0\x1C\x80cF\xE6\xD3\x1A\x11a\0XW\x80cF\xE6\xD3\x1A\x14a\x01\x04W\x80cI\xCD\x9F\x98\x14a\x01$W\x80cv\t\x9A\x06\x14a\x017W\x80c~g\x0E\xB3\x14a\x01jW__\xFD[\x80c\x12\xD7\x13\xC2\x14a\0~W\x80c*\xF0_\xC5\x14a\0\xAFW\x80c9\xAD\xFE\xFF\x14a\0\xF1W[__\xFD[a\0\x91a\0\x8C6`\x04a\x16iV[a\x01\x9EV[`@Q`\x01`\x01``\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xC2a\0\xBD6`\x04a\x16\xC4V[a\x01\xB7V[`@\x80Q\x95\x86Rc\xFF\xFF\xFF\xFF\x94\x85\x16` \x87\x01R\x85\x01\x92\x90\x92R\x90\x91\x16``\x83\x01R`\x80\x82\x01R`\xA0\x01a\0\xA6V[a\0\x91a\0\xFF6`\x04a\x16iV[a\x01\xDBV[a\x01\x17a\x01\x126`\x04a\x16\xC4V[a\x01\xEAV[`@Qa\0\xA6\x91\x90a\x17\x02V[a\x01\x17a\x0126`\x04a\x17\xA0V[a\x02\x05V[a\x01Ja\x01E6`\x04a\x18oV[a\x02\xF3V[`@\x80Q\x93\x84Rc\xFF\xFF\xFF\xFF\x90\x92\x16` \x84\x01R\x90\x82\x01R``\x01a\0\xA6V[a\x01\x8Ca\x01x6`\x04a\x18\xA0V[_` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\0\xA6V[`\x1C\x81`\x1B\x81\x10a\x01\xADW_\x80\xFD[\x01T``\x1B\x90P\x81V[_\x80\x80\x80\x80a\x01\xC8\x87\x87\x83\x80a\x03\x0CV[\x93\x9B\x92\x9AP\x90\x98P\x96P\x90\x94P\x92PPPV[`\x01\x81`\x1B\x81\x10a\x01\xADW_\x80\xFD[a\x01\xF2a\x16\x1EV[a\x01\xFC\x83\x83a\x04|V[\x90P[\x92\x91PPV[a\x02\ra\x16\x1EV[___a\x02\x19\x8Aa\x05^V[\x92P\x92P\x92Pa\x02\xAD\x81`\x02\x8B\x8B`@Qa\x025\x92\x91\x90a\x18\xCEV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x02PW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02s\x91\x90a\x18\xDDV[\x89\x89\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8B\x92Pa\x05\xDA\x91PPV[a\x02\xCAW`@Qc\x01\xD7\xCD\xD3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02\xD4\x89\x89a\x04|V[` \x81\x01\x93\x90\x93RPc\xFF\xFF\xFF\xFF\x16`@\x82\x01R\x97\x96PPPPPPPV[___a\x02\xFF\x84a\x05^V[\x92P\x92P\x92P\x91\x93\x90\x92PV[_______a\x03\x1D\x8B\x8Ba\x07\x10V[\x90\x98P\x92\x96P\x90\x94P\x92P\x90P\x88\x15\x80\x15\x90a\x039WP\x81\x89\x14\x15[\x15a\x03WW`@Qc\xE1Jy1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x03b\x8C\x8Ca\x08\xFBV[\x90P_a\x03p\x82\x8E\x8Ea\tWV[`\x01`\x01``\x1B\x03\x19\x84\x16_\x90\x81R` \x81\x90R`@\x81 T\x91\x92P`\xFF\x90\x91\x16\x90\x81\x90\x03a\x03\xC3W`@Qc\xCDBs\x8B`\xE0\x1B\x81R`\x01`\x01``\x1B\x03\x19\x85\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[_a\x03\xCF`\x01\x83a\x19\x08V[\x90P`\x01`\xFF\x82\x16\x1B\x8C\x81\x16c\xFF\xFF\xFF\xFF\x16\x15a\x04\x0BW`@QcX:\x88\xFF`\xE1\x1B\x81R`\x01`\x01``\x1B\x03\x19\x87\x16`\x04\x82\x01R`$\x01a\x03\xBAV[\x8C\x81\x17\x9AP`\x01`\x01``\x1B\x03\x19\x84\x16`\x1C`\xFF\x84\x16`\x1B\x81\x10a\x041Wa\x041a\x19!V[\x01T``\x1B`\x01`\x01``\x1B\x03\x19\x16\x14a\x04^W`@Qc\x13\xD6\xDCs`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04h\x8A\x86a\t\xC9V[\x9BPPPPPPPP\x94P\x94P\x94P\x94P\x94V[a\x04\x84a\x16\x1EV[_______a\x04\x95\x8A\x8Aa\t\xECV[\x98P\x96P\x90P\x85\x81\x11\x80\x15\x90a\x04\xABWP\x88\x86\x11\x15[a\x04\xB7Wa\x04\xB7a\x195V[a\x04\xC3\x8A\x8A\x83\x89a\n\xC8V[\x92\x97P\x90\x95P\x93P\x91PP\x81\x15\x80\x15a\x04\xDAWP\x80\x15[\x15a\x04\xF8W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\x04\x89\x89\x87\x81a\x0B\x90V[a\x05!W`@Qc\xC2\xC0b\xD1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x87Rj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x85\x16``\x89\x01R\x83\x16`\x80\x88\x01Ra\x05L\x89\x89\x84\x84a\x0C\x8AV[`\xA0\x88\x01RP\x94\x97\x96PPPPPPPV[_\x80\x80\x80\x80\x80[`\x14\x81\x10\x15a\x05\xD0W_____a\x05\x9F\x8C\x87`\x14\x81\x10a\x05\x88Wa\x05\x88a\x19!V[` \x02\x81\x01\x90a\x05\x98\x91\x90a\x19IV[\x8A\x8Aa\x03\x0CV[\x94P\x94P\x94P\x94P\x94P\x84\x97P\x83\x96P\x85_\x03a\x05\xC0W\x82\x9AP\x81\x99P\x80\x98P[PPPPP\x80`\x01\x01\x90Pa\x05eV[PPP\x91\x93\x90\x92PV[_\x83\x81[\x84Q\x81\x10\x15a\x07\x02W_\x85\x82\x81Q\x81\x10a\x05\xFAWa\x05\xFAa\x19!V[` \x02` \x01\x01Q\x90P\x81`\x01\x90\x1B\x85\x16_\x03a\x06\x87W`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x82\x90R`\x02\x90``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x06B\x91a\x19\x8BV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x06]W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x80\x91\x90a\x18\xDDV[\x92Pa\x06\xF9V[`@\x80Q` \x81\x01\x83\x90R\x90\x81\x01\x84\x90R`\x02\x90``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x06\xB8\x91a\x19\x8BV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x06\xD3W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xF6\x91\x90a\x18\xDDV[\x92P[P`\x01\x01a\x05\xDEV[P\x85\x14\x90P[\x94\x93PPPPV[_\x80\x80\x80\x80`\xAE\x86\x14a\x079W`@Qc`\r\x15Q`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x03\xBAV[\x86\x86_\x81\x81\x10a\x07KWa\x07Ka\x19!V[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`\n`\xF8\x1B\x14\x15\x80a\x07\x9BWP\x86\x86`\x01\x81\x81\x10a\x07\x7FWa\x07\x7Fa\x19!V[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`i`\xF8\x1B\x14\x15[\x80a\x07\xD0WP\x86\x86`k\x81\x81\x10a\x07\xB4Wa\x07\xB4a\x19!V[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`\x12`\xF8\x1B\x14\x15[\x80a\x08\x05WP\x86\x86`l\x81\x81\x10a\x07\xE9Wa\x07\xE9a\x19!V[\x90P\x015`\xF8\x1C`\xF8\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`A`\xF8\x1B\x14\x15[\x15a\x08#W`@Qc\xEF\x02\xC9\xBB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x082\x88\x88`\x03`\ta\rrV[P`\x01`\x01`@\x1B\x03\x16\x90P_a\x08Ka\x03\xE8\x83a\x19\xA1V[\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15a\x08rW`@QcT\x9A\x01\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x95P`\x0B\x88\x015\x94P`-\x88\x015\x93P\x85_a\x08\x92\x8A\x8A`N`Ra\rrV[`\x01`\x01`@\x1B\x03\x90\x91\x16\x95P\x90P_\x8A\x8A`T\x81\x81\x10a\x08\xB5Wa\x08\xB5a\x19!V[\x91\x90\x91\x015`\xF8\x1C\x91PP`A\x81\x14a\x08\xE6W`@QcO\xA8\x8D_`\xE1\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`$\x01a\x03\xBAV[P\x96\x99\x95\x98\x94\x97P\x92\x95PPPP`U\x015\x90V[_`\x02a\t\x0B`k\x82\x85\x87a\x19\xC0V[`@Qa\t\x19\x92\x91\x90a\x18\xCEV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\t4W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xFC\x91\x90a\x18\xDDV[_`\xAD\x83\x015\x81\x1A`m\x84\x015`\x8D\x85\x015`\x1B\x83\x10\x15a\tyW`\x1B\x83\x01\x92P[`\x1C\x83\x14`\x1B\x84\x14\x17a\t\x8DW__R__\xFD[`@Q\x87\x81R\x83` \x82\x01R\x82`@\x82\x01R\x81``\x82\x01R` \x81`\x80\x83`\x01Z\xFAa\t\xBAW__R__\xFD[Q``\x1B\x97\x96PPPPPPPV[_\x80a\t\xDA`\x01`\x01`\xC0\x1Ba\x19\xE7V[`\xC0\x85\x90\x1B\x90\x84\x16\x17\x91PP\x92\x91PPV[_\x80\x80\x83\x80\x15\x80a\n\x18WP\x85\x85_\x81\x81\x10a\n\nWa\n\na\x19!V[\x90\x91\x015`\xF8\x1C`\n\x14\x15\x90P[\x15a\n6W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01_a\nE\x88\x88\x84\x86a\rrV[\x96P\x86\x92P\x90Pa\n`\x82`\x01`\x01`@\x1B\x03\x83\x16\x85a\x0E/V[\x94P`\x02a\np\x86\x88\x8A\x8Ca\x19\xC0V[`@Qa\n~\x92\x91\x90a\x18\xCEV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\n\x99W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xBC\x91\x90a\x18\xDDV[\x93PPPP\x92P\x92P\x92V[_______a\n\xDB\x8B\x8B\x8B\x8Ba\x0EdV[\x91\x94P\x92P\x90P\x81\x83\x10\x80\x15a\n\xF1WP\x87\x82\x11\x15[a\n\xFDWa\n\xFDa\x195V[`\x01`\x01`@\x1B\x03\x81\x16`\x1F\x14a\x0B'W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\x0B5\x8D\x8D\x87\x87a\x0FoV[\x90\x92P\x90P\x81\x15\x80\x15a\x0BFWP\x80\x15[\x15a\x0BdW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Bp\x8D\x8D\x84\x84a\x10)V[\x80\x99P\x81\x9AP\x82\x9BP\x83\x9CPPPPPPPPPP\x94P\x94P\x94P\x94\x90PV[_\x82[\x82\x84\x10\x80\x15a\x0B\xBCWP\x85\x85\x85\x81\x81\x10a\x0B\xAFWa\x0B\xAFa\x19!V[\x90\x91\x015`\xF8\x1C`\x12\x14\x90P[\x15a\x0C\nWP\x82a\x0B\xCC\x81a\x19\xFAV[\x93P_a\x0B\xDB\x87\x87\x87\x87a\rrV[\x95P\x90P\x81\x85\x11a\x0B\xEEWa\x0B\xEEa\x195V[a\x0C\x02\x85\x82`\x01`\x01`@\x1B\x03\x16\x86a\x0E/V[\x94PPa\x0B\x93V[_[\x83\x85\x10\x80\x15a\x0C5WP\x86\x86\x86\x81\x81\x10a\x0C(Wa\x0C(a\x19!V[\x90\x91\x015`\xF8\x1C`*\x14\x90P[\x15a\x0C\x80WP`\x01a\x0CF\x85a\x19\xFAV[\x94P__a\x0CV\x89\x89\x89\x89a\x11\xF4V[\x98P\x90\x92P\x90Pa\x0Ci\x89\x89\x84\x84a\x12ZV[a\x0CyW_\x94PPPPPa\x07\x08V[PPa\x0C\x0CV[\x96\x95PPPPPPV[``\x82\x82\x10\x80a\x0C\x99WP\x83\x82\x11[\x15a\x0C\xB7W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0C\xC2\x84\x84a\x19\xE7V[\x90P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\xDCWa\x0C\xDCa\x1A\x12V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\r\x06W` \x82\x01\x81\x806\x837\x01\x90P[P\x91P_[\x81\x81\x10\x15a\rhW\x86\x86a\r\x1F\x83\x88a\x1A&V[\x81\x81\x10a\r.Wa\r.a\x19!V[\x90P\x015`\xF8\x1C`\xF8\x1B\x83\x82\x81Q\x81\x10a\rJWa\rJa\x19!V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\r\x0BV[PP\x94\x93PPPPV[_\x80\x80\x80\x80[`\n\x81\x10\x15a\x0E\x0CW\x85\x87\x10a\r\xA1W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x89\x89\x89a\r\xAE\x81a\x19\xFAV[\x9AP\x81\x81\x10a\r\xBFWa\r\xBFa\x19!V[`\x7F\x92\x015`\xF8\x1C\x91\x82\x16`\x01`\x01`@\x1B\x03\x86\x16\x1B\x95\x90\x95\x17\x94P\x90P`\x80\x81\x16_\x03a\r\xF6W\x83\x88\x95P\x95PPPPPa\x0E&V[a\x0E\x01`\x07\x84a\x1A9V[\x92PP`\x01\x01a\rxV[P`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x94P\x94\x92PPPV[_a\x0E:\x84\x83a\x19\xE7V[\x83\x11\x15a\x0EZW`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\x08\x83\x85a\x1A&V[_\x80\x80\x84\x81\x81[\x86\x83\x10\x15a\x0FDWP\x81_\x80a\x0E\x83\x8C\x8C\x85\x8Ca\x13:V[\x96P\x90\x92P\x90P\x82\x85\x11a\x0E\x99Wa\x0E\x99a\x195V[\x81`\x01`\x01`@\x1B\x03\x16`\x0B\x14\x80\x15a\x0E\xBBWP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x0F.W\x83\x15a\x0E\xDFW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x93Pa\x0E\xEF\x8C\x8C\x87\x8Ca\x11\xF4V[\x91\x99P\x97P\x94P_a\x0F\x03\x8D\x8D\x8B\x8Ba\x13nV[\x90\x97P\x90P\x80a\x0F&W`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPa\x0FDV[a\x0F;\x8C\x8C\x87\x8C\x85a\x14\x04V[\x94PPPa\x0EkV[\x81a\x0FbW`@Qc0n\x18\x9B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPP\x94P\x94P\x94\x91PPV[_\x80\x83\x81\x80\x82[\x86\x84\x10\x15a\x10\x1CWP\x82_\x80a\x0F\x8E\x8C\x8C\x85\x8Ca\x13:V[\x97P\x90\x92P\x90P\x82\x86\x11a\x0F\xA4Wa\x0F\xA4a\x195V[\x81`\x01`\x01`@\x1B\x03\x16`\x02\x14\x80\x15a\x0F\xC6WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x10\x05Wa\x0F\xD7\x8C\x8C\x88\x8Ca\x11\xF4V[\x97P\x90\x95P\x93P_\x80a\x0F\xEC\x8E\x8E\x89\x89a\x14\xADV[\x91P\x91P\x81_\x14a\x0F\xFEW\x81\x99P\x80\x98P[PPa\x10\x15V[a\x10\x12\x8C\x8C\x88\x8C\x85a\x14\x04V[\x95P[PPa\x0FvV[PPPP\x94P\x94\x92PPPV[_\x80\x80\x80\x85\x80[\x86\x82\x10\x15a\x11\xE7WP\x80_\x80a\x10H\x8C\x8C\x85\x8Ca\x13:V[\x95P\x90\x92P\x90P\x82\x84\x11a\x10^Wa\x10^a\x195V[\x81`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x10\x80WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x10\xFBW__a\x10\x93\x8E\x8E\x88\x8Ea\x11\xF4V[\x97P\x90\x92P\x90Pa\x10\xA4\x82\x82a\x19\xE7V[`\x15\x14a\x10\xC4W`@Qc\x90'W\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xCF\x8E\x8E\x84a\x15uV[\x99P\x89_\x1A`A\x14a\x10\xF4W`@Qc\xA4d]e`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPa\x11\xE0V[\x81`\x01`\x01`@\x1B\x03\x16`\x02\x14\x80\x15a\x11\x1DWP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x11\x91W__a\x110\x8E\x8E\x88\x8Ea\x11\xF4V[\x97P\x90\x92P\x90Pa\x11A\x82\x82a\x19\xE7V[`\x15\x14a\x11aW`@Qcl\x8E\xE0\xD9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11l\x8E\x8E\x84a\x15uV[\x98P\x88_\x1A`A\x14a\x10\xF4W`@QcTw\x93\xAB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`@\x1B\x03\x16`\x04\x14\x80\x15a\x11\xB3WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x11\xD0Wa\x11\xC4\x8C\x8C\x86\x8Ca\x11\xF4V[\x91\x97P\x95P\x93Pa\x11\xE0V[a\x11\xDD\x8C\x8C\x86\x8C\x85a\x14\x04V[\x93P[PPa\x100V[PP\x94P\x94P\x94P\x94\x90PV[____a\x12\x04\x88\x88\x88\x88a\rrV[\x96P\x86\x94P\x90P`\x01`\x01`@\x1B\x03\x81\x16a\x12\x1F\x85\x87a\x19\xE7V[\x81\x11\x15a\x12?W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12I\x81\x88a\x1A&V[\x93P\x83\x92PPP\x94P\x94P\x94\x91PPV[_\x80\x80\x84\x80[\x85\x82\x10\x15a\x13\x17WP\x80_\x80a\x12x\x8B\x8B\x85\x8Ba\x13:V[\x95P\x90\x92P\x90P\x82\x84\x11a\x12\x8EWa\x12\x8Ea\x195V[`\x01`\x01`@\x1B\x03\x81\x16a\x13\0W_a\x12\xA9\x8C\x8C\x87\x8Ca\rrV[\x95P\x90P`\x01`\x01`@\x1B\x03\x83\x16`\x02\x03a\x12\xE1W`\x01`\x01`@\x1B\x03\x81\x16\x15a\x12\xDCW_\x97PPPPPPPPa\x07\x08V[a\x12\xFAV[\x82`\x01`\x01`@\x1B\x03\x16`\x03\x03a\x12\xFAW`\x01\x95P\x80\x96P[Pa\x13\x10V[a\x13\r\x8B\x8B\x86\x8B\x85a\x14\x04V[\x93P[PPa\x12`V[\x82\x80\x15a\x13-WP\x83`\x01`\x01`@\x1B\x03\x16`\x01\x14[\x99\x98PPPPPPPPPV[_____a\x13K\x89\x89\x89\x89a\rrV[`\x03\x82\x90\x1Cg\x1F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x9B`\x07\x90\x92\x16\x9AP\x98P\x96PPPPPPPV[_\x80\x83\x80[\x84\x82\x10\x15a\x13\xF9WP\x80_\x80a\x13\x8B\x8A\x8A\x85\x8Aa\x13:V[\x95P\x90\x92P\x90P\x82\x84\x11a\x13\xA1Wa\x13\xA1a\x195V[\x81`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x13\xC1WP`\x01`\x01`@\x1B\x03\x81\x16\x15[\x15a\x13\xE3Wa\x13\xD2\x8A\x8A\x86\x8Aa\rrV[P\x95P`\x01\x94Pa\x0E&\x93PPPPV[a\x13\xF0\x8A\x8A\x86\x8A\x85a\x14\x04V[\x93PPPa\x13sV[PP\x94P\x94\x92PPPV[_`\x01`\x01`@\x1B\x03\x82\x16a\x14&Wa\x14\x1F\x86\x86\x86\x86a\x15\xACV[\x90Pa\x14\xA4V[`\x01\x19`\x01`\x01`@\x1B\x03\x83\x16\x01a\x14PW_a\x14E\x87\x87\x87\x87a\x11\xF4V[P\x92Pa\x14\xA4\x91PPV[`\x04\x19`\x01`\x01`@\x1B\x03\x83\x16\x01a\x14nWa\x14\x1F\x84`\x04\x85a\x0E/V[_\x19`\x01`\x01`@\x1B\x03\x83\x16\x01a\x14\x8BWa\x14\x1F\x84`\x08\x85a\x0E/V[`@Qc\xA5\xA5\xFCC`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x95\x94PPPPPV[_\x80\x83\x80[\x84\x82\x10\x15a\x13\xF9WP\x80_\x80a\x14\xCA\x8A\x8A\x85\x8Aa\x13:V[\x95P\x90\x92P\x90P\x82\x84\x11a\x14\xE0Wa\x14\xE0a\x195V[\x81`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x15\x02WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x15\x1DWa\x15\x13\x8A\x8A\x86\x8Aa\x11\xF4V[P\x94Pa\x15n\x90PV[\x81`\x01`\x01`@\x1B\x03\x16`\x02\x14\x80\x15a\x15?WP`\x01`\x01`@\x1B\x03\x81\x16`\x02\x14[\x15a\x15^Wa\x15P\x8A\x8A\x86\x8Aa\x11\xF4V[P\x90\x96P\x94P\x84\x93Pa\x15nV[a\x15k\x8A\x8A\x86\x8A\x85a\x14\x04V[\x93P[PPa\x14\xB2V[_\x82a\x15\x82\x83`\x15a\x1A&V[\x11\x15a\x15\xA1W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x91\x90\x91\x015\x91\x90PV[_\x80[`\n\x81\x10\x15a\x0E\x0CW\x82\x84\x10a\x15\xD8W`@Qc?\xFDfY`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x86\x86\x86a\x15\xE5\x81a\x19\xFAV[\x97P\x81\x81\x10a\x15\xF6Wa\x15\xF6a\x19!V[\x91\x90\x91\x015`\xF8\x1C\x91PP`\x80\x81\x16_\x03a\x16\x15W\x84\x92PPPa\x07\x08V[P`\x01\x01a\x15\xAFV[`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_`\x01`\x01`X\x1B\x03\x19\x16\x81R` \x01_`\x01`\x01`X\x1B\x03\x19\x16\x81R` \x01``\x81RP\x90V[_` \x82\x84\x03\x12\x15a\x16yW__\xFD[P5\x91\x90PV[__\x83`\x1F\x84\x01\x12a\x16\x90W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xA6W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x16\xBDW__\xFD[\x92P\x92\x90PV[__` \x83\x85\x03\x12\x15a\x16\xD5W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xEAW__\xFD[a\x16\xF6\x85\x82\x86\x01a\x16\x80V[\x90\x96\x90\x95P\x93PPPPV[` \x81R\x81Q` \x82\x01R` \x82\x01Q`@\x82\x01Rc\xFF\xFF\xFF\xFF`@\x83\x01Q\x16``\x82\x01R`\x01`\x01`X\x1B\x03\x19``\x83\x01Q\x16`\x80\x82\x01R`\x01`\x01`X\x1B\x03\x19`\x80\x83\x01Q\x16`\xA0\x82\x01R_`\xA0\x83\x01Q`\xC0\x80\x84\x01R\x80Q\x80`\xE0\x85\x01R\x80` \x83\x01a\x01\0\x86\x01^_a\x01\0\x82\x86\x01\x01Ra\x01\0`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x80a\x02\x80\x81\x01\x83\x10\x15a\x01\xFFW__\xFD[______`\x80\x87\x89\x03\x12\x15a\x17\xB5W__\xFD[\x865`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xCAW__\xFD[a\x17\xD6\x89\x82\x8A\x01a\x17\x8FV[\x96PP` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xF1W__\xFD[a\x17\xFD\x89\x82\x8A\x01a\x16\x80V[\x90\x96P\x94PP`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\x1BW__\xFD[\x87\x01`\x1F\x81\x01\x89\x13a\x18+W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18@W__\xFD[\x89` \x82`\x05\x1B\x84\x01\x01\x11\x15a\x18TW__\xFD[\x96\x99\x95\x98P\x93\x96` \x90\x94\x01\x95\x94``\x90\x94\x015\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x18\x7FW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\x94W__\xFD[a\x07\x08\x84\x82\x85\x01a\x17\x8FV[_` \x82\x84\x03\x12\x15a\x18\xB0W__\xFD[\x815`\x01`\x01``\x1B\x03\x19\x81\x16\x81\x14a\x18\xC7W__\xFD[\x93\x92PPPV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_` \x82\x84\x03\x12\x15a\x18\xEDW__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x01\xFFWa\x01\xFFa\x18\xF4V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x01`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x19^W__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x19wW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x16\xBDW__\xFD[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_\x82a\x19\xBBWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[__\x85\x85\x11\x15a\x19\xCEW__\xFD[\x83\x86\x11\x15a\x19\xDAW__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x81\x81\x03\x81\x81\x11\x15a\x01\xFFWa\x01\xFFa\x18\xF4V[_`\x01\x82\x01a\x1A\x0BWa\x1A\x0Ba\x18\xF4V[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x01\xFFWa\x01\xFFa\x18\xF4V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x01\xFFWa\x01\xFFa\x18\xF4V\xFE\xA1dsolcC\0\x08\x1B\0\n",
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
    /**Function with signature `verifySingleBlock(bytes)` and selector `0x2af05fc5`.
```solidity
function verifySingleBlock(bytes memory block_) external view returns (bytes32 nextBlockId, uint32 nextSeen, uint256 blockNumber, uint32 blockTimestamp, bytes32 txTrieRoot);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifySingleBlockCall {
        #[allow(missing_docs)]
        pub block_: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`verifySingleBlock(bytes)`](verifySingleBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifySingleBlockReturn {
        #[allow(missing_docs)]
        pub nextBlockId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub nextSeen: u32,
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
            impl ::core::convert::From<verifySingleBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifySingleBlockCall) -> Self {
                    (value.block_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifySingleBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { block_: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u32,
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
            impl ::core::convert::From<verifySingleBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifySingleBlockReturn) -> Self {
                    (
                        value.nextBlockId,
                        value.nextSeen,
                        value.blockNumber,
                        value.blockTimestamp,
                        value.txTrieRoot,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifySingleBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        nextBlockId: tuple.0,
                        nextSeen: tuple.1,
                        blockNumber: tuple.2,
                        blockTimestamp: tuple.3,
                        txTrieRoot: tuple.4,
                    }
                }
            }
        }
        impl verifySingleBlockReturn {
            fn _tokenize(
                &self,
            ) -> <verifySingleBlockCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.nextBlockId),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.nextSeen),
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
        impl alloy_sol_types::SolCall for verifySingleBlockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifySingleBlockReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifySingleBlock(bytes)";
            const SELECTOR: [u8; 4] = [42u8, 240u8, 95u8, 197u8];
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
                        &self.block_,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                verifySingleBlockReturn::_tokenize(ret)
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
    ///Container for all the [`StatefulTronTxReaderHarness`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum StatefulTronTxReaderHarnessCalls {
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
        verifySingleBlock(verifySingleBlockCall),
        #[allow(missing_docs)]
        witnessDelegatees(witnessDelegateesCall),
    }
    impl StatefulTronTxReaderHarnessCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [18u8, 215u8, 19u8, 194u8],
            [42u8, 240u8, 95u8, 197u8],
            [57u8, 173u8, 254u8, 255u8],
            [70u8, 230u8, 211u8, 26u8],
            [73u8, 205u8, 159u8, 152u8],
            [118u8, 9u8, 154u8, 6u8],
            [126u8, 103u8, 14u8, 179u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(witnessDelegatees),
            ::core::stringify!(verifySingleBlock),
            ::core::stringify!(srs),
            ::core::stringify!(parseTriggerSmartContract),
            ::core::stringify!(readTriggerSmartContract),
            ::core::stringify!(verifyFirstBlockFinality),
            ::core::stringify!(srIndexPlus1),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <witnessDelegateesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <verifySingleBlockCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for StatefulTronTxReaderHarnessCalls {
        const NAME: &'static str = "StatefulTronTxReaderHarnessCalls";
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
                Self::verifySingleBlock(_) => {
                    <verifySingleBlockCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessCalls>] = &[
                {
                    fn witnessDelegatees(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessCalls> {
                        <witnessDelegateesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessCalls::witnessDelegatees)
                    }
                    witnessDelegatees
                },
                {
                    fn verifySingleBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessCalls> {
                        <verifySingleBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessCalls::verifySingleBlock)
                    }
                    verifySingleBlock
                },
                {
                    fn srs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessCalls> {
                        <srsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(StatefulTronTxReaderHarnessCalls::srs)
                    }
                    srs
                },
                {
                    fn parseTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessCalls> {
                        <parseTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessCalls::parseTriggerSmartContract,
                            )
                    }
                    parseTriggerSmartContract
                },
                {
                    fn readTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessCalls> {
                        <readTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessCalls::readTriggerSmartContract,
                            )
                    }
                    readTriggerSmartContract
                },
                {
                    fn verifyFirstBlockFinality(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessCalls> {
                        <verifyFirstBlockFinalityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessCalls::verifyFirstBlockFinality,
                            )
                    }
                    verifyFirstBlockFinality
                },
                {
                    fn srIndexPlus1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessCalls> {
                        <srIndexPlus1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessCalls::srIndexPlus1)
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
            ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessCalls>] = &[
                {
                    fn witnessDelegatees(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessCalls> {
                        <witnessDelegateesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessCalls::witnessDelegatees)
                    }
                    witnessDelegatees
                },
                {
                    fn verifySingleBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessCalls> {
                        <verifySingleBlockCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessCalls::verifySingleBlock)
                    }
                    verifySingleBlock
                },
                {
                    fn srs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessCalls> {
                        <srsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessCalls::srs)
                    }
                    srs
                },
                {
                    fn parseTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessCalls> {
                        <parseTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessCalls::parseTriggerSmartContract,
                            )
                    }
                    parseTriggerSmartContract
                },
                {
                    fn readTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessCalls> {
                        <readTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessCalls::readTriggerSmartContract,
                            )
                    }
                    readTriggerSmartContract
                },
                {
                    fn verifyFirstBlockFinality(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessCalls> {
                        <verifyFirstBlockFinalityCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessCalls::verifyFirstBlockFinality,
                            )
                    }
                    verifyFirstBlockFinality
                },
                {
                    fn srIndexPlus1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessCalls> {
                        <srIndexPlus1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessCalls::srIndexPlus1)
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
                Self::verifySingleBlock(inner) => {
                    <verifySingleBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::verifySingleBlock(inner) => {
                    <verifySingleBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`StatefulTronTxReaderHarness`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum StatefulTronTxReaderHarnessErrors {
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
    impl StatefulTronTxReaderHarnessErrors {
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
    impl alloy_sol_types::SolInterface for StatefulTronTxReaderHarnessErrors {
        const NAME: &'static str = "StatefulTronTxReaderHarnessErrors";
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
            ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors>] = &[
                {
                    fn InvalidTxMerkleProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <InvalidTxMerkleProof as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessErrors::InvalidTxMerkleProof)
                    }
                    InvalidTxMerkleProof
                },
                {
                    fn InvalidWitnessSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <InvalidWitnessSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessErrors::InvalidWitnessSignature,
                            )
                    }
                    InvalidWitnessSignature
                },
                {
                    fn ProtoTruncated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <ProtoTruncated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessErrors::ProtoTruncated)
                    }
                    ProtoTruncated
                },
                {
                    fn SrSetNotSorted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <SrSetNotSorted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessErrors::SrSetNotSorted)
                    }
                    SrSetNotSorted
                },
                {
                    fn TimestampOverflow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <TimestampOverflow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessErrors::TimestampOverflow)
                    }
                    TimestampOverflow
                },
                {
                    fn InvalidEncodedBlockLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <InvalidEncodedBlockLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessErrors::InvalidEncodedBlockLength,
                            )
                    }
                    InvalidEncodedBlockLength
                },
                {
                    fn TronInvalidOwnerLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <TronInvalidOwnerLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessErrors::TronInvalidOwnerLength,
                            )
                    }
                    TronInvalidOwnerLength
                },
                {
                    fn InvalidWitnessAddressPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <InvalidWitnessAddressPrefix as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessErrors::InvalidWitnessAddressPrefix,
                            )
                    }
                    InvalidWitnessAddressPrefix
                },
                {
                    fn TronInvalidOwnerPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <TronInvalidOwnerPrefix as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessErrors::TronInvalidOwnerPrefix,
                            )
                    }
                    TronInvalidOwnerPrefix
                },
                {
                    fn ProtoInvalidWireType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <ProtoInvalidWireType as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessErrors::ProtoInvalidWireType)
                    }
                    ProtoInvalidWireType
                },
                {
                    fn TronInvalidContractPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <TronInvalidContractPrefix as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessErrors::TronInvalidContractPrefix,
                            )
                    }
                    TronInvalidContractPrefix
                },
                {
                    fn DuplicateSr(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <DuplicateSr as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(StatefulTronTxReaderHarnessErrors::DuplicateSr)
                    }
                    DuplicateSr
                },
                {
                    fn NotTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <NotTriggerSmartContract as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessErrors::NotTriggerSmartContract,
                            )
                    }
                    NotTriggerSmartContract
                },
                {
                    fn TronTxNotSuccessful(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <TronTxNotSuccessful as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessErrors::TronTxNotSuccessful)
                    }
                    TronTxNotSuccessful
                },
                {
                    fn UnknownSr(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <UnknownSr as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(StatefulTronTxReaderHarnessErrors::UnknownSr)
                    }
                    UnknownSr
                },
                {
                    fn TronInvalidContractLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <TronInvalidContractLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessErrors::TronInvalidContractLength,
                            )
                    }
                    TronInvalidContractLength
                },
                {
                    fn InvalidBlockSequence(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <InvalidBlockSequence as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessErrors::InvalidBlockSequence)
                    }
                    InvalidBlockSequence
                },
                {
                    fn InvalidHeaderPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <InvalidHeaderPrefix as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessErrors::InvalidHeaderPrefix)
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
            ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors>] = &[
                {
                    fn InvalidTxMerkleProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <InvalidTxMerkleProof as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessErrors::InvalidTxMerkleProof)
                    }
                    InvalidTxMerkleProof
                },
                {
                    fn InvalidWitnessSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <InvalidWitnessSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessErrors::InvalidWitnessSignature,
                            )
                    }
                    InvalidWitnessSignature
                },
                {
                    fn ProtoTruncated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <ProtoTruncated as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessErrors::ProtoTruncated)
                    }
                    ProtoTruncated
                },
                {
                    fn SrSetNotSorted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <SrSetNotSorted as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessErrors::SrSetNotSorted)
                    }
                    SrSetNotSorted
                },
                {
                    fn TimestampOverflow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <TimestampOverflow as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessErrors::TimestampOverflow)
                    }
                    TimestampOverflow
                },
                {
                    fn InvalidEncodedBlockLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <InvalidEncodedBlockLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessErrors::InvalidEncodedBlockLength,
                            )
                    }
                    InvalidEncodedBlockLength
                },
                {
                    fn TronInvalidOwnerLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <TronInvalidOwnerLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessErrors::TronInvalidOwnerLength,
                            )
                    }
                    TronInvalidOwnerLength
                },
                {
                    fn InvalidWitnessAddressPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <InvalidWitnessAddressPrefix as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessErrors::InvalidWitnessAddressPrefix,
                            )
                    }
                    InvalidWitnessAddressPrefix
                },
                {
                    fn TronInvalidOwnerPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <TronInvalidOwnerPrefix as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessErrors::TronInvalidOwnerPrefix,
                            )
                    }
                    TronInvalidOwnerPrefix
                },
                {
                    fn ProtoInvalidWireType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <ProtoInvalidWireType as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessErrors::ProtoInvalidWireType)
                    }
                    ProtoInvalidWireType
                },
                {
                    fn TronInvalidContractPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <TronInvalidContractPrefix as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessErrors::TronInvalidContractPrefix,
                            )
                    }
                    TronInvalidContractPrefix
                },
                {
                    fn DuplicateSr(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <DuplicateSr as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessErrors::DuplicateSr)
                    }
                    DuplicateSr
                },
                {
                    fn NotTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <NotTriggerSmartContract as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessErrors::NotTriggerSmartContract,
                            )
                    }
                    NotTriggerSmartContract
                },
                {
                    fn TronTxNotSuccessful(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <TronTxNotSuccessful as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessErrors::TronTxNotSuccessful)
                    }
                    TronTxNotSuccessful
                },
                {
                    fn UnknownSr(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <UnknownSr as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessErrors::UnknownSr)
                    }
                    UnknownSr
                },
                {
                    fn TronInvalidContractLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <TronInvalidContractLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                StatefulTronTxReaderHarnessErrors::TronInvalidContractLength,
                            )
                    }
                    TronInvalidContractLength
                },
                {
                    fn InvalidBlockSequence(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <InvalidBlockSequence as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessErrors::InvalidBlockSequence)
                    }
                    InvalidBlockSequence
                },
                {
                    fn InvalidHeaderPrefix(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<StatefulTronTxReaderHarnessErrors> {
                        <InvalidHeaderPrefix as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(StatefulTronTxReaderHarnessErrors::InvalidHeaderPrefix)
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
    /**Creates a new wrapper around an on-chain [`StatefulTronTxReaderHarness`](self) contract instance.

See the [wrapper's documentation](`StatefulTronTxReaderHarnessInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> StatefulTronTxReaderHarnessInstance<P, N> {
        StatefulTronTxReaderHarnessInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<StatefulTronTxReaderHarnessInstance<P, N>>,
    > {
        StatefulTronTxReaderHarnessInstance::<
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
        StatefulTronTxReaderHarnessInstance::<
            P,
            N,
        >::deploy_builder(__provider, _srs, _witnessDelegatees)
    }
    /**A [`StatefulTronTxReaderHarness`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`StatefulTronTxReaderHarness`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct StatefulTronTxReaderHarnessInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for StatefulTronTxReaderHarnessInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("StatefulTronTxReaderHarnessInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StatefulTronTxReaderHarnessInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`StatefulTronTxReaderHarness`](self) contract instance.

See the [wrapper's documentation](`StatefulTronTxReaderHarnessInstance`) for more details.*/
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
        ) -> alloy_contract::Result<StatefulTronTxReaderHarnessInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> StatefulTronTxReaderHarnessInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> StatefulTronTxReaderHarnessInstance<P, N> {
            StatefulTronTxReaderHarnessInstance {
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
    > StatefulTronTxReaderHarnessInstance<P, N> {
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
        ///Creates a new call builder for the [`verifySingleBlock`] function.
        pub fn verifySingleBlock(
            &self,
            block_: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, verifySingleBlockCall, N> {
            self.call_builder(&verifySingleBlockCall { block_ })
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
    > StatefulTronTxReaderHarnessInstance<P, N> {
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
