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

interface MockTronTxReader {
    function readTriggerSmartContract(bytes[20] memory, bytes memory, bytes32[] memory, uint256) external view returns (ITronTxReader.TriggerSmartContract memory callData);
    function setNextCallData(bytes32 txId, uint256 tronBlockNumber, uint32 tronBlockTimestamp, bytes21 senderTron, bytes21 toTron, bytes memory data) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "readTriggerSmartContract",
    "inputs": [
      {
        "name": "",
        "type": "bytes[20]",
        "internalType": "bytes[20]"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      },
      {
        "name": "",
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
    "name": "setNextCallData",
    "inputs": [
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
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
pub mod MockTronTxReader {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f5ffd5b5061063b8061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610034575f3560e01c806349cd9f9814610038578063f81012a014610061575b5f5ffd5b61004b6100463660046102c1565b610076565b6040516100589190610399565b60405180910390f35b61007461006f36600461044e565b61019f565b005b6040805160c0810182525f808252602082018190529181018290526060808201839052608082019290925260a08101919091526040805160c0810182525f80548252600154602083015260025463ffffffff8116938301939093526affffffffffffffffffffff19640100000000909304605890811b84166060840152600354901b90921660808201526004805460a083019190610113906104db565b80601f016020809104026020016040519081016040528092919081815260200182805461013f906104db565b801561018a5780601f106101615761010080835404028352916020019161018a565b820191905f5260205f20905b81548152906001019060200180831161016d57829003601f168201915b50505050508152509150509695505050505050565b5f82828080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201829052506040805160c0810182528e8152602081018e905263ffffffff8d169181018290526affffffffffffffffffffff19808d1660608301528b16608082015260a081018790528e835560018e90556002805460588e811c640100000000026001600160c81b031990921690941717905560038054928c901c6001600160a81b03199093169290921790915594955092506004915061026f90508482610573565b5050505050505050505050565b5f5f83601f84011261028c575f5ffd5b50813567ffffffffffffffff8111156102a3575f5ffd5b6020830191508360208285010111156102ba575f5ffd5b9250929050565b5f5f5f5f5f5f608087890312156102d6575f5ffd5b863567ffffffffffffffff8111156102ec575f5ffd5b870161028081018910156102fe575f5ffd5b9550602087013567ffffffffffffffff811115610319575f5ffd5b61032589828a0161027c565b909650945050604087013567ffffffffffffffff811115610344575f5ffd5b8701601f81018913610354575f5ffd5b803567ffffffffffffffff81111561036a575f5ffd5b8960208260051b840101111561037e575f5ffd5b96999598509396602090940195946060909401359392505050565b60208152815160208201526020820151604082015263ffffffff60408301511660608201526affffffffffffffffffffff1960608301511660808201526affffffffffffffffffffff1960808301511660a08201525f60a083015160c08084015280518060e0850152806020830161010086015e5f6101008286010152610100601f19601f8301168501019250505092915050565b80356affffffffffffffffffffff1981168114610449575f5ffd5b919050565b5f5f5f5f5f5f5f60c0888a031215610464575f5ffd5b8735965060208801359550604088013563ffffffff81168114610485575f5ffd5b94506104936060890161042e565b93506104a16080890161042e565b925060a088013567ffffffffffffffff8111156104bc575f5ffd5b6104c88a828b0161027c565b989b979a50959850939692959293505050565b600181811c908216806104ef57607f821691505b60208210810361050d57634e487b7160e01b5f52602260045260245ffd5b50919050565b634e487b7160e01b5f52604160045260245ffd5b601f82111561056e57805f5260205f20601f840160051c8101602085101561054c5750805b601f840160051c820191505b8181101561056b575f8155600101610558565b50505b505050565b815167ffffffffffffffff81111561058d5761058d610513565b6105a18161059b84546104db565b84610527565b6020601f8211600181146105d3575f83156105bc5750848201515b5f19600385901b1c1916600184901b17845561056b565b5f84815260208120601f198516915b8281101561060257878501518255602094850194600190920191016105e2565b508482101561061f57868401515f19600387901b60f8161c191681555b50505050600190811b0190555056fea164736f6c634300081b000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[Pa\x06;\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80cI\xCD\x9F\x98\x14a\08W\x80c\xF8\x10\x12\xA0\x14a\0aW[__\xFD[a\0Ka\0F6`\x04a\x02\xC1V[a\0vV[`@Qa\0X\x91\x90a\x03\x99V[`@Q\x80\x91\x03\x90\xF3[a\0ta\0o6`\x04a\x04NV[a\x01\x9FV[\0[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x80\x82\x01\x83\x90R`\x80\x82\x01\x92\x90\x92R`\xA0\x81\x01\x91\x90\x91R`@\x80Q`\xC0\x81\x01\x82R_\x80T\x82R`\x01T` \x83\x01R`\x02Tc\xFF\xFF\xFF\xFF\x81\x16\x93\x83\x01\x93\x90\x93Rj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19d\x01\0\0\0\0\x90\x93\x04`X\x90\x81\x1B\x84\x16``\x84\x01R`\x03T\x90\x1B\x90\x92\x16`\x80\x82\x01R`\x04\x80T`\xA0\x83\x01\x91\x90a\x01\x13\x90a\x04\xDBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01?\x90a\x04\xDBV[\x80\x15a\x01\x8AW\x80`\x1F\x10a\x01aWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\x8AV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01mW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RP\x91PP\x96\x95PPPPPPV[_\x82\x82\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x82\x90RP`@\x80Q`\xC0\x81\x01\x82R\x8E\x81R` \x81\x01\x8E\x90Rc\xFF\xFF\xFF\xFF\x8D\x16\x91\x81\x01\x82\x90Rj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8D\x16``\x83\x01R\x8B\x16`\x80\x82\x01R`\xA0\x81\x01\x87\x90R\x8E\x83U`\x01\x8E\x90U`\x02\x80T`X\x8E\x81\x1Cd\x01\0\0\0\0\x02`\x01`\x01`\xC8\x1B\x03\x19\x90\x92\x16\x90\x94\x17\x17\x90U`\x03\x80T\x92\x8C\x90\x1C`\x01`\x01`\xA8\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x94\x95P\x92P`\x04\x91Pa\x02o\x90P\x84\x82a\x05sV[PPPPPPPPPPPV[__\x83`\x1F\x84\x01\x12a\x02\x8CW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xA3W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x02\xBAW__\xFD[\x92P\x92\x90PV[______`\x80\x87\x89\x03\x12\x15a\x02\xD6W__\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xECW__\xFD[\x87\x01a\x02\x80\x81\x01\x89\x10\x15a\x02\xFEW__\xFD[\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\x19W__\xFD[a\x03%\x89\x82\x8A\x01a\x02|V[\x90\x96P\x94PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03DW__\xFD[\x87\x01`\x1F\x81\x01\x89\x13a\x03TW__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03jW__\xFD[\x89` \x82`\x05\x1B\x84\x01\x01\x11\x15a\x03~W__\xFD[\x96\x99\x95\x98P\x93\x96` \x90\x94\x01\x95\x94``\x90\x94\x015\x93\x92PPPV[` \x81R\x81Q` \x82\x01R` \x82\x01Q`@\x82\x01Rc\xFF\xFF\xFF\xFF`@\x83\x01Q\x16``\x82\x01Rj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x83\x01Q\x16`\x80\x82\x01Rj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\x80\x83\x01Q\x16`\xA0\x82\x01R_`\xA0\x83\x01Q`\xC0\x80\x84\x01R\x80Q\x80`\xE0\x85\x01R\x80` \x83\x01a\x01\0\x86\x01^_a\x01\0\x82\x86\x01\x01Ra\x01\0`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x805j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x14a\x04IW__\xFD[\x91\x90PV[_______`\xC0\x88\x8A\x03\x12\x15a\x04dW__\xFD[\x875\x96P` \x88\x015\x95P`@\x88\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\x85W__\xFD[\x94Pa\x04\x93``\x89\x01a\x04.V[\x93Pa\x04\xA1`\x80\x89\x01a\x04.V[\x92P`\xA0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xBCW__\xFD[a\x04\xC8\x8A\x82\x8B\x01a\x02|V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x04\xEFW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x05\rWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x1F\x82\x11\x15a\x05nW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x05LWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x05kW_\x81U`\x01\x01a\x05XV[PP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x8DWa\x05\x8Da\x05\x13V[a\x05\xA1\x81a\x05\x9B\x84Ta\x04\xDBV[\x84a\x05'V[` `\x1F\x82\x11`\x01\x81\x14a\x05\xD3W_\x83\x15a\x05\xBCWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x05kV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x06\x02W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x05\xE2V[P\x84\x82\x10\x15a\x06\x1FW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA1dsolcC\0\x08\x1B\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610034575f3560e01c806349cd9f9814610038578063f81012a014610061575b5f5ffd5b61004b6100463660046102c1565b610076565b6040516100589190610399565b60405180910390f35b61007461006f36600461044e565b61019f565b005b6040805160c0810182525f808252602082018190529181018290526060808201839052608082019290925260a08101919091526040805160c0810182525f80548252600154602083015260025463ffffffff8116938301939093526affffffffffffffffffffff19640100000000909304605890811b84166060840152600354901b90921660808201526004805460a083019190610113906104db565b80601f016020809104026020016040519081016040528092919081815260200182805461013f906104db565b801561018a5780601f106101615761010080835404028352916020019161018a565b820191905f5260205f20905b81548152906001019060200180831161016d57829003601f168201915b50505050508152509150509695505050505050565b5f82828080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201829052506040805160c0810182528e8152602081018e905263ffffffff8d169181018290526affffffffffffffffffffff19808d1660608301528b16608082015260a081018790528e835560018e90556002805460588e811c640100000000026001600160c81b031990921690941717905560038054928c901c6001600160a81b03199093169290921790915594955092506004915061026f90508482610573565b5050505050505050505050565b5f5f83601f84011261028c575f5ffd5b50813567ffffffffffffffff8111156102a3575f5ffd5b6020830191508360208285010111156102ba575f5ffd5b9250929050565b5f5f5f5f5f5f608087890312156102d6575f5ffd5b863567ffffffffffffffff8111156102ec575f5ffd5b870161028081018910156102fe575f5ffd5b9550602087013567ffffffffffffffff811115610319575f5ffd5b61032589828a0161027c565b909650945050604087013567ffffffffffffffff811115610344575f5ffd5b8701601f81018913610354575f5ffd5b803567ffffffffffffffff81111561036a575f5ffd5b8960208260051b840101111561037e575f5ffd5b96999598509396602090940195946060909401359392505050565b60208152815160208201526020820151604082015263ffffffff60408301511660608201526affffffffffffffffffffff1960608301511660808201526affffffffffffffffffffff1960808301511660a08201525f60a083015160c08084015280518060e0850152806020830161010086015e5f6101008286010152610100601f19601f8301168501019250505092915050565b80356affffffffffffffffffffff1981168114610449575f5ffd5b919050565b5f5f5f5f5f5f5f60c0888a031215610464575f5ffd5b8735965060208801359550604088013563ffffffff81168114610485575f5ffd5b94506104936060890161042e565b93506104a16080890161042e565b925060a088013567ffffffffffffffff8111156104bc575f5ffd5b6104c88a828b0161027c565b989b979a50959850939692959293505050565b600181811c908216806104ef57607f821691505b60208210810361050d57634e487b7160e01b5f52602260045260245ffd5b50919050565b634e487b7160e01b5f52604160045260245ffd5b601f82111561056e57805f5260205f20601f840160051c8101602085101561054c5750805b601f840160051c820191505b8181101561056b575f8155600101610558565b50505b505050565b815167ffffffffffffffff81111561058d5761058d610513565b6105a18161059b84546104db565b84610527565b6020601f8211600181146105d3575f83156105bc5750848201515b5f19600385901b1c1916600184901b17845561056b565b5f84815260208120601f198516915b8281101561060257878501518255602094850194600190920191016105e2565b508482101561061f57868401515f19600387901b60f8161c191681555b50505050600190811b0190555056fea164736f6c634300081b000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80cI\xCD\x9F\x98\x14a\08W\x80c\xF8\x10\x12\xA0\x14a\0aW[__\xFD[a\0Ka\0F6`\x04a\x02\xC1V[a\0vV[`@Qa\0X\x91\x90a\x03\x99V[`@Q\x80\x91\x03\x90\xF3[a\0ta\0o6`\x04a\x04NV[a\x01\x9FV[\0[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x80\x82\x01\x83\x90R`\x80\x82\x01\x92\x90\x92R`\xA0\x81\x01\x91\x90\x91R`@\x80Q`\xC0\x81\x01\x82R_\x80T\x82R`\x01T` \x83\x01R`\x02Tc\xFF\xFF\xFF\xFF\x81\x16\x93\x83\x01\x93\x90\x93Rj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19d\x01\0\0\0\0\x90\x93\x04`X\x90\x81\x1B\x84\x16``\x84\x01R`\x03T\x90\x1B\x90\x92\x16`\x80\x82\x01R`\x04\x80T`\xA0\x83\x01\x91\x90a\x01\x13\x90a\x04\xDBV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01?\x90a\x04\xDBV[\x80\x15a\x01\x8AW\x80`\x1F\x10a\x01aWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\x8AV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01mW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RP\x91PP\x96\x95PPPPPPV[_\x82\x82\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x82\x90RP`@\x80Q`\xC0\x81\x01\x82R\x8E\x81R` \x81\x01\x8E\x90Rc\xFF\xFF\xFF\xFF\x8D\x16\x91\x81\x01\x82\x90Rj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8D\x16``\x83\x01R\x8B\x16`\x80\x82\x01R`\xA0\x81\x01\x87\x90R\x8E\x83U`\x01\x8E\x90U`\x02\x80T`X\x8E\x81\x1Cd\x01\0\0\0\0\x02`\x01`\x01`\xC8\x1B\x03\x19\x90\x92\x16\x90\x94\x17\x17\x90U`\x03\x80T\x92\x8C\x90\x1C`\x01`\x01`\xA8\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x94\x95P\x92P`\x04\x91Pa\x02o\x90P\x84\x82a\x05sV[PPPPPPPPPPPV[__\x83`\x1F\x84\x01\x12a\x02\x8CW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xA3W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x02\xBAW__\xFD[\x92P\x92\x90PV[______`\x80\x87\x89\x03\x12\x15a\x02\xD6W__\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xECW__\xFD[\x87\x01a\x02\x80\x81\x01\x89\x10\x15a\x02\xFEW__\xFD[\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\x19W__\xFD[a\x03%\x89\x82\x8A\x01a\x02|V[\x90\x96P\x94PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03DW__\xFD[\x87\x01`\x1F\x81\x01\x89\x13a\x03TW__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03jW__\xFD[\x89` \x82`\x05\x1B\x84\x01\x01\x11\x15a\x03~W__\xFD[\x96\x99\x95\x98P\x93\x96` \x90\x94\x01\x95\x94``\x90\x94\x015\x93\x92PPPV[` \x81R\x81Q` \x82\x01R` \x82\x01Q`@\x82\x01Rc\xFF\xFF\xFF\xFF`@\x83\x01Q\x16``\x82\x01Rj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x83\x01Q\x16`\x80\x82\x01Rj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\x80\x83\x01Q\x16`\xA0\x82\x01R_`\xA0\x83\x01Q`\xC0\x80\x84\x01R\x80Q\x80`\xE0\x85\x01R\x80` \x83\x01a\x01\0\x86\x01^_a\x01\0\x82\x86\x01\x01Ra\x01\0`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x805j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x14a\x04IW__\xFD[\x91\x90PV[_______`\xC0\x88\x8A\x03\x12\x15a\x04dW__\xFD[\x875\x96P` \x88\x015\x95P`@\x88\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\x85W__\xFD[\x94Pa\x04\x93``\x89\x01a\x04.V[\x93Pa\x04\xA1`\x80\x89\x01a\x04.V[\x92P`\xA0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xBCW__\xFD[a\x04\xC8\x8A\x82\x8B\x01a\x02|V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x04\xEFW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x05\rWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x1F\x82\x11\x15a\x05nW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x05LWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x05kW_\x81U`\x01\x01a\x05XV[PP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x8DWa\x05\x8Da\x05\x13V[a\x05\xA1\x81a\x05\x9B\x84Ta\x04\xDBV[\x84a\x05'V[` `\x1F\x82\x11`\x01\x81\x14a\x05\xD3W_\x83\x15a\x05\xBCWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x05kV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x06\x02W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x05\xE2V[P\x84\x82\x10\x15a\x06\x1FW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA1dsolcC\0\x08\x1B\0\n",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `readTriggerSmartContract(bytes[20],bytes,bytes32[],uint256)` and selector `0x49cd9f98`.
```solidity
function readTriggerSmartContract(bytes[20] memory, bytes memory, bytes32[] memory, uint256) external view returns (ITronTxReader.TriggerSmartContract memory callData);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct readTriggerSmartContractCall {
        #[allow(missing_docs)]
        pub _0: [alloy::sol_types::private::Bytes; 20usize],
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub _2: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        #[allow(missing_docs)]
        pub _3: alloy::sol_types::private::primitives::aliases::U256,
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
                    (value._0, value._1, value._2, value._3)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for readTriggerSmartContractCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
                        _3: tuple.3,
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self._2),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._3),
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
    /**Function with signature `setNextCallData(bytes32,uint256,uint32,bytes21,bytes21,bytes)` and selector `0xf81012a0`.
```solidity
function setNextCallData(bytes32 txId, uint256 tronBlockNumber, uint32 tronBlockTimestamp, bytes21 senderTron, bytes21 toTron, bytes memory data) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setNextCallDataCall {
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
    ///Container type for the return parameters of the [`setNextCallData(bytes32,uint256,uint32,bytes21,bytes21,bytes)`](setNextCallDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setNextCallDataReturn {}
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
            impl ::core::convert::From<setNextCallDataCall> for UnderlyingRustTuple<'_> {
                fn from(value: setNextCallDataCall) -> Self {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setNextCallDataCall {
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
        }
        {
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
            impl ::core::convert::From<setNextCallDataReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setNextCallDataReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setNextCallDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setNextCallDataReturn {
            fn _tokenize(
                &self,
            ) -> <setNextCallDataCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setNextCallDataCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<21>,
                alloy::sol_types::sol_data::FixedBytes<21>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setNextCallDataReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setNextCallData(bytes32,uint256,uint32,bytes21,bytes21,bytes)";
            const SELECTOR: [u8; 4] = [248u8, 16u8, 18u8, 160u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setNextCallDataReturn::_tokenize(ret)
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
    ///Container for all the [`MockTronTxReader`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum MockTronTxReaderCalls {
        #[allow(missing_docs)]
        readTriggerSmartContract(readTriggerSmartContractCall),
        #[allow(missing_docs)]
        setNextCallData(setNextCallDataCall),
    }
    impl MockTronTxReaderCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [73u8, 205u8, 159u8, 152u8],
            [248u8, 16u8, 18u8, 160u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(readTriggerSmartContract),
            ::core::stringify!(setNextCallData),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <readTriggerSmartContractCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setNextCallDataCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for MockTronTxReaderCalls {
        const NAME: &'static str = "MockTronTxReaderCalls";
        const MIN_DATA_LENGTH: usize = 224usize;
        const COUNT: usize = 2usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::readTriggerSmartContract(_) => {
                    <readTriggerSmartContractCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setNextCallData(_) => {
                    <setNextCallDataCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<MockTronTxReaderCalls>] = &[
                {
                    fn readTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockTronTxReaderCalls> {
                        <readTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(MockTronTxReaderCalls::readTriggerSmartContract)
                    }
                    readTriggerSmartContract
                },
                {
                    fn setNextCallData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockTronTxReaderCalls> {
                        <setNextCallDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(MockTronTxReaderCalls::setNextCallData)
                    }
                    setNextCallData
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
            ) -> alloy_sol_types::Result<MockTronTxReaderCalls>] = &[
                {
                    fn readTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockTronTxReaderCalls> {
                        <readTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockTronTxReaderCalls::readTriggerSmartContract)
                    }
                    readTriggerSmartContract
                },
                {
                    fn setNextCallData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockTronTxReaderCalls> {
                        <setNextCallDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockTronTxReaderCalls::setNextCallData)
                    }
                    setNextCallData
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
                Self::setNextCallData(inner) => {
                    <setNextCallDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::setNextCallData(inner) => {
                    <setNextCallDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`MockTronTxReader`](self) contract instance.

See the [wrapper's documentation](`MockTronTxReaderInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> MockTronTxReaderInstance<P, N> {
        MockTronTxReaderInstance::<P, N>::new(address, __provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<MockTronTxReaderInstance<P, N>>,
    > {
        MockTronTxReaderInstance::<P, N>::deploy(__provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
        MockTronTxReaderInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`MockTronTxReader`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`MockTronTxReader`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct MockTronTxReaderInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for MockTronTxReaderInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("MockTronTxReaderInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > MockTronTxReaderInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`MockTronTxReader`](self) contract instance.

See the [wrapper's documentation](`MockTronTxReaderInstance`) for more details.*/
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
        ) -> alloy_contract::Result<MockTronTxReaderInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                ::core::clone::Clone::clone(&BYTECODE),
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
    impl<P: ::core::clone::Clone, N> MockTronTxReaderInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> MockTronTxReaderInstance<P, N> {
            MockTronTxReaderInstance {
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
    > MockTronTxReaderInstance<P, N> {
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
            _0: [alloy::sol_types::private::Bytes; 20usize],
            _1: alloy::sol_types::private::Bytes,
            _2: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            _3: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, readTriggerSmartContractCall, N> {
            self.call_builder(
                &readTriggerSmartContractCall {
                    _0,
                    _1,
                    _2,
                    _3,
                },
            )
        }
        ///Creates a new call builder for the [`setNextCallData`] function.
        pub fn setNextCallData(
            &self,
            txId: alloy::sol_types::private::FixedBytes<32>,
            tronBlockNumber: alloy::sol_types::private::primitives::aliases::U256,
            tronBlockTimestamp: u32,
            senderTron: alloy::sol_types::private::FixedBytes<21>,
            toTron: alloy::sol_types::private::FixedBytes<21>,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, setNextCallDataCall, N> {
            self.call_builder(
                &setNextCallDataCall {
                    txId,
                    tronBlockNumber,
                    tronBlockTimestamp,
                    senderTron,
                    toTron,
                    data,
                },
            )
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > MockTronTxReaderInstance<P, N> {
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
