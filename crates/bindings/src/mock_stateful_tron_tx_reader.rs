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

interface MockStatefulTronTxReader {
    error MockStatefulTronTxReader_TimestampTooLarge();

    function clearNext() external;
    function readTriggerSmartContract(bytes[20] memory blocks, bytes memory encodedTx, bytes32[] memory proof, uint256 index) external view returns (ITronTxReader.TriggerSmartContract memory callData);
    function setNext(ITronTxReader.TriggerSmartContract memory next_) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "clearNext",
    "inputs": [],
    "outputs": [],
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
    "name": "setNext",
    "inputs": [
      {
        "name": "next_",
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
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "error",
    "name": "MockStatefulTronTxReader_TimestampTooLarge",
    "inputs": []
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
pub mod MockStatefulTronTxReader {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f5ffd5b5061076d8061001c5f395ff3fe608060405234801561000f575f5ffd5b506004361061003f575f3560e01c806349cd9f9814610043578063e1ab27091461006c578063e3aab14114610081575b5f5ffd5b61005661005136600461031d565b610090565b60405161006391906103f3565b60405180910390f35b61007f61007a366004610488565b6102b7565b005b61007f6005805460ff19169055565b6040805160c0810182525f808252602082018190529181018290526060808201839052608082019290925260a081019190915260055460ff16156101be576040805160c0810182525f80548252600154602083015260025463ffffffff8116938301939093526001600160581b0319640100000000909304605890811b84166060840152600354901b90921660808201526004805491929160a084019190610137906104c6565b80601f0160208091040260200160405190810160405280929190818152602001828054610163906104c6565b80156101ae5780601f10610185576101008083540402835291602001916101ae565b820191905f5260205f20905b81548152906001019060200180831161019157829003601f168201915b50505050508152505090506102ad565b600286866040516101d09291906104fe565b602060405180830381855afa1580156101eb573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061020e919061050d565b81525f602082015263ffffffff42111561023b57604051633956d03360e21b815260040160405180910390fd5b4263ffffffff166040820152604160a01b331760581b6001600160581b0319166060820152604160f81b6001600160581b0319166080820152604080516020601f88018190048102820181019092528681529087908790819084018382808284375f9201919091525050505060a08201525b9695505050505050565b805f6102c3828261069f565b50506005805460ff1916600117905550565b5f5f83601f8401126102e5575f5ffd5b50813567ffffffffffffffff8111156102fc575f5ffd5b6020830191508360208260051b8501011115610316575f5ffd5b9250929050565b5f5f5f5f5f5f60808789031215610332575f5ffd5b863567ffffffffffffffff811115610348575f5ffd5b8701610280810189101561035a575f5ffd5b9550602087013567ffffffffffffffff811115610375575f5ffd5b8701601f81018913610385575f5ffd5b803567ffffffffffffffff81111561039b575f5ffd5b8960208284010111156103ac575f5ffd5b60209190910195509350604087013567ffffffffffffffff8111156103cf575f5ffd5b6103db89828a016102d5565b979a9699509497949695606090950135949350505050565b60208152815160208201526020820151604082015263ffffffff60408301511660608201526affffffffffffffffffffff1960608301511660808201526affffffffffffffffffffff1960808301511660a08201525f60a083015160c08084015280518060e0850152806020830161010086015e5f6101008286010152610100601f19601f8301168501019250505092915050565b5f60208284031215610498575f5ffd5b813567ffffffffffffffff8111156104ae575f5ffd5b820160c081850312156104bf575f5ffd5b9392505050565b600181811c908216806104da57607f821691505b6020821081036104f857634e487b7160e01b5f52602260045260245ffd5b50919050565b818382375f9101908152919050565b5f6020828403121561051d575f5ffd5b5051919050565b5f81356001600160581b03198116811461053c575f5ffd5b92915050565b5f5f8335601e19843603018112610557575f5ffd5b83018035915067ffffffffffffffff821115610571575f5ffd5b602001915036819003821315610316575f5ffd5b634e487b7160e01b5f52604160045260245ffd5b601f8211156105e057805f5260205f20601f840160051c810160208510156105be5750805b601f840160051c820191505b818110156105dd575f81556001016105ca565b50505b505050565b67ffffffffffffffff8311156105fd576105fd610585565b6106118361060b83546104c6565b83610599565b5f601f841160018114610642575f851561062b5750838201355b5f19600387901b1c1916600186901b1783556105dd565b5f83815260208120601f198716915b828110156106715786850135825560209485019460019092019101610651565b508682101561068d575f1960f88860031b161c19848701351681555b505060018560011b0183555050505050565b8135815560208201356001820155600281015f604084013563ffffffff811681146106c8575f5ffd5b825463ffffffff191663ffffffff909116178255506106e960608401610524565b8154640100000000600160c81b03191660389190911c640100000000600160c81b031617905561073f61071e60808401610524565b600383018160581c6affffffffffffffffffffff60a81b8254161781555050565b61074c60a0830183610542565b61075a8183600486016105e5565b5050505056fea164736f6c634300081b000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[Pa\x07m\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0?W_5`\xE0\x1C\x80cI\xCD\x9F\x98\x14a\0CW\x80c\xE1\xAB'\t\x14a\0lW\x80c\xE3\xAA\xB1A\x14a\0\x81W[__\xFD[a\0Va\0Q6`\x04a\x03\x1DV[a\0\x90V[`@Qa\0c\x91\x90a\x03\xF3V[`@Q\x80\x91\x03\x90\xF3[a\0\x7Fa\0z6`\x04a\x04\x88V[a\x02\xB7V[\0[a\0\x7F`\x05\x80T`\xFF\x19\x16\x90UV[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x80\x82\x01\x83\x90R`\x80\x82\x01\x92\x90\x92R`\xA0\x81\x01\x91\x90\x91R`\x05T`\xFF\x16\x15a\x01\xBEW`@\x80Q`\xC0\x81\x01\x82R_\x80T\x82R`\x01T` \x83\x01R`\x02Tc\xFF\xFF\xFF\xFF\x81\x16\x93\x83\x01\x93\x90\x93R`\x01`\x01`X\x1B\x03\x19d\x01\0\0\0\0\x90\x93\x04`X\x90\x81\x1B\x84\x16``\x84\x01R`\x03T\x90\x1B\x90\x92\x16`\x80\x82\x01R`\x04\x80T\x91\x92\x91`\xA0\x84\x01\x91\x90a\x017\x90a\x04\xC6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01c\x90a\x04\xC6V[\x80\x15a\x01\xAEW\x80`\x1F\x10a\x01\x85Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\xAEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90Pa\x02\xADV[`\x02\x86\x86`@Qa\x01\xD0\x92\x91\x90a\x04\xFEV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x01\xEBW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x0E\x91\x90a\x05\rV[\x81R_` \x82\x01Rc\xFF\xFF\xFF\xFFB\x11\x15a\x02;W`@Qc9V\xD03`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Bc\xFF\xFF\xFF\xFF\x16`@\x82\x01R`A`\xA0\x1B3\x17`X\x1B`\x01`\x01`X\x1B\x03\x19\x16``\x82\x01R`A`\xF8\x1B`\x01`\x01`X\x1B\x03\x19\x16`\x80\x82\x01R`@\x80Q` `\x1F\x88\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x86\x81R\x90\x87\x90\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPPPP`\xA0\x82\x01R[\x96\x95PPPPPPV[\x80_a\x02\xC3\x82\x82a\x06\x9FV[PP`\x05\x80T`\xFF\x19\x16`\x01\x17\x90UPV[__\x83`\x1F\x84\x01\x12a\x02\xE5W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xFCW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x03\x16W__\xFD[\x92P\x92\x90PV[______`\x80\x87\x89\x03\x12\x15a\x032W__\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03HW__\xFD[\x87\x01a\x02\x80\x81\x01\x89\x10\x15a\x03ZW__\xFD[\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03uW__\xFD[\x87\x01`\x1F\x81\x01\x89\x13a\x03\x85W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\x9BW__\xFD[\x89` \x82\x84\x01\x01\x11\x15a\x03\xACW__\xFD[` \x91\x90\x91\x01\x95P\x93P`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xCFW__\xFD[a\x03\xDB\x89\x82\x8A\x01a\x02\xD5V[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[` \x81R\x81Q` \x82\x01R` \x82\x01Q`@\x82\x01Rc\xFF\xFF\xFF\xFF`@\x83\x01Q\x16``\x82\x01Rj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x83\x01Q\x16`\x80\x82\x01Rj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\x80\x83\x01Q\x16`\xA0\x82\x01R_`\xA0\x83\x01Q`\xC0\x80\x84\x01R\x80Q\x80`\xE0\x85\x01R\x80` \x83\x01a\x01\0\x86\x01^_a\x01\0\x82\x86\x01\x01Ra\x01\0`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x04\x98W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xAEW__\xFD[\x82\x01`\xC0\x81\x85\x03\x12\x15a\x04\xBFW__\xFD[\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x04\xDAW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x04\xF8WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_` \x82\x84\x03\x12\x15a\x05\x1DW__\xFD[PQ\x91\x90PV[_\x815`\x01`\x01`X\x1B\x03\x19\x81\x16\x81\x14a\x05<W__\xFD[\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x05WW__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05qW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x03\x16W__\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x1F\x82\x11\x15a\x05\xE0W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x05\xBEWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x05\xDDW_\x81U`\x01\x01a\x05\xCAV[PP[PPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x05\xFDWa\x05\xFDa\x05\x85V[a\x06\x11\x83a\x06\x0B\x83Ta\x04\xC6V[\x83a\x05\x99V[_`\x1F\x84\x11`\x01\x81\x14a\x06BW_\x85\x15a\x06+WP\x83\x82\x015[_\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\x05\xDDV[_\x83\x81R` \x81 `\x1F\x19\x87\x16\x91[\x82\x81\x10\x15a\x06qW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x06QV[P\x86\x82\x10\x15a\x06\x8DW_\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[\x815\x81U` \x82\x015`\x01\x82\x01U`\x02\x81\x01_`@\x84\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\xC8W__\xFD[\x82Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x82UPa\x06\xE9``\x84\x01a\x05$V[\x81Td\x01\0\0\0\0`\x01`\xC8\x1B\x03\x19\x16`8\x91\x90\x91\x1Cd\x01\0\0\0\0`\x01`\xC8\x1B\x03\x16\x17\x90Ua\x07?a\x07\x1E`\x80\x84\x01a\x05$V[`\x03\x83\x01\x81`X\x1Cj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x82T\x16\x17\x81UPPV[a\x07L`\xA0\x83\x01\x83a\x05BV[a\x07Z\x81\x83`\x04\x86\x01a\x05\xE5V[PPPPV\xFE\xA1dsolcC\0\x08\x1B\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061003f575f3560e01c806349cd9f9814610043578063e1ab27091461006c578063e3aab14114610081575b5f5ffd5b61005661005136600461031d565b610090565b60405161006391906103f3565b60405180910390f35b61007f61007a366004610488565b6102b7565b005b61007f6005805460ff19169055565b6040805160c0810182525f808252602082018190529181018290526060808201839052608082019290925260a081019190915260055460ff16156101be576040805160c0810182525f80548252600154602083015260025463ffffffff8116938301939093526001600160581b0319640100000000909304605890811b84166060840152600354901b90921660808201526004805491929160a084019190610137906104c6565b80601f0160208091040260200160405190810160405280929190818152602001828054610163906104c6565b80156101ae5780601f10610185576101008083540402835291602001916101ae565b820191905f5260205f20905b81548152906001019060200180831161019157829003601f168201915b50505050508152505090506102ad565b600286866040516101d09291906104fe565b602060405180830381855afa1580156101eb573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061020e919061050d565b81525f602082015263ffffffff42111561023b57604051633956d03360e21b815260040160405180910390fd5b4263ffffffff166040820152604160a01b331760581b6001600160581b0319166060820152604160f81b6001600160581b0319166080820152604080516020601f88018190048102820181019092528681529087908790819084018382808284375f9201919091525050505060a08201525b9695505050505050565b805f6102c3828261069f565b50506005805460ff1916600117905550565b5f5f83601f8401126102e5575f5ffd5b50813567ffffffffffffffff8111156102fc575f5ffd5b6020830191508360208260051b8501011115610316575f5ffd5b9250929050565b5f5f5f5f5f5f60808789031215610332575f5ffd5b863567ffffffffffffffff811115610348575f5ffd5b8701610280810189101561035a575f5ffd5b9550602087013567ffffffffffffffff811115610375575f5ffd5b8701601f81018913610385575f5ffd5b803567ffffffffffffffff81111561039b575f5ffd5b8960208284010111156103ac575f5ffd5b60209190910195509350604087013567ffffffffffffffff8111156103cf575f5ffd5b6103db89828a016102d5565b979a9699509497949695606090950135949350505050565b60208152815160208201526020820151604082015263ffffffff60408301511660608201526affffffffffffffffffffff1960608301511660808201526affffffffffffffffffffff1960808301511660a08201525f60a083015160c08084015280518060e0850152806020830161010086015e5f6101008286010152610100601f19601f8301168501019250505092915050565b5f60208284031215610498575f5ffd5b813567ffffffffffffffff8111156104ae575f5ffd5b820160c081850312156104bf575f5ffd5b9392505050565b600181811c908216806104da57607f821691505b6020821081036104f857634e487b7160e01b5f52602260045260245ffd5b50919050565b818382375f9101908152919050565b5f6020828403121561051d575f5ffd5b5051919050565b5f81356001600160581b03198116811461053c575f5ffd5b92915050565b5f5f8335601e19843603018112610557575f5ffd5b83018035915067ffffffffffffffff821115610571575f5ffd5b602001915036819003821315610316575f5ffd5b634e487b7160e01b5f52604160045260245ffd5b601f8211156105e057805f5260205f20601f840160051c810160208510156105be5750805b601f840160051c820191505b818110156105dd575f81556001016105ca565b50505b505050565b67ffffffffffffffff8311156105fd576105fd610585565b6106118361060b83546104c6565b83610599565b5f601f841160018114610642575f851561062b5750838201355b5f19600387901b1c1916600186901b1783556105dd565b5f83815260208120601f198716915b828110156106715786850135825560209485019460019092019101610651565b508682101561068d575f1960f88860031b161c19848701351681555b505060018560011b0183555050505050565b8135815560208201356001820155600281015f604084013563ffffffff811681146106c8575f5ffd5b825463ffffffff191663ffffffff909116178255506106e960608401610524565b8154640100000000600160c81b03191660389190911c640100000000600160c81b031617905561073f61071e60808401610524565b600383018160581c6affffffffffffffffffffff60a81b8254161781555050565b61074c60a0830183610542565b61075a8183600486016105e5565b5050505056fea164736f6c634300081b000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0?W_5`\xE0\x1C\x80cI\xCD\x9F\x98\x14a\0CW\x80c\xE1\xAB'\t\x14a\0lW\x80c\xE3\xAA\xB1A\x14a\0\x81W[__\xFD[a\0Va\0Q6`\x04a\x03\x1DV[a\0\x90V[`@Qa\0c\x91\x90a\x03\xF3V[`@Q\x80\x91\x03\x90\xF3[a\0\x7Fa\0z6`\x04a\x04\x88V[a\x02\xB7V[\0[a\0\x7F`\x05\x80T`\xFF\x19\x16\x90UV[`@\x80Q`\xC0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x80\x82\x01\x83\x90R`\x80\x82\x01\x92\x90\x92R`\xA0\x81\x01\x91\x90\x91R`\x05T`\xFF\x16\x15a\x01\xBEW`@\x80Q`\xC0\x81\x01\x82R_\x80T\x82R`\x01T` \x83\x01R`\x02Tc\xFF\xFF\xFF\xFF\x81\x16\x93\x83\x01\x93\x90\x93R`\x01`\x01`X\x1B\x03\x19d\x01\0\0\0\0\x90\x93\x04`X\x90\x81\x1B\x84\x16``\x84\x01R`\x03T\x90\x1B\x90\x92\x16`\x80\x82\x01R`\x04\x80T\x91\x92\x91`\xA0\x84\x01\x91\x90a\x017\x90a\x04\xC6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01c\x90a\x04\xC6V[\x80\x15a\x01\xAEW\x80`\x1F\x10a\x01\x85Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\xAEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90Pa\x02\xADV[`\x02\x86\x86`@Qa\x01\xD0\x92\x91\x90a\x04\xFEV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x01\xEBW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x0E\x91\x90a\x05\rV[\x81R_` \x82\x01Rc\xFF\xFF\xFF\xFFB\x11\x15a\x02;W`@Qc9V\xD03`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Bc\xFF\xFF\xFF\xFF\x16`@\x82\x01R`A`\xA0\x1B3\x17`X\x1B`\x01`\x01`X\x1B\x03\x19\x16``\x82\x01R`A`\xF8\x1B`\x01`\x01`X\x1B\x03\x19\x16`\x80\x82\x01R`@\x80Q` `\x1F\x88\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x86\x81R\x90\x87\x90\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPPPP`\xA0\x82\x01R[\x96\x95PPPPPPV[\x80_a\x02\xC3\x82\x82a\x06\x9FV[PP`\x05\x80T`\xFF\x19\x16`\x01\x17\x90UPV[__\x83`\x1F\x84\x01\x12a\x02\xE5W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xFCW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x03\x16W__\xFD[\x92P\x92\x90PV[______`\x80\x87\x89\x03\x12\x15a\x032W__\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03HW__\xFD[\x87\x01a\x02\x80\x81\x01\x89\x10\x15a\x03ZW__\xFD[\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03uW__\xFD[\x87\x01`\x1F\x81\x01\x89\x13a\x03\x85W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\x9BW__\xFD[\x89` \x82\x84\x01\x01\x11\x15a\x03\xACW__\xFD[` \x91\x90\x91\x01\x95P\x93P`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xCFW__\xFD[a\x03\xDB\x89\x82\x8A\x01a\x02\xD5V[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[` \x81R\x81Q` \x82\x01R` \x82\x01Q`@\x82\x01Rc\xFF\xFF\xFF\xFF`@\x83\x01Q\x16``\x82\x01Rj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x83\x01Q\x16`\x80\x82\x01Rj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\x80\x83\x01Q\x16`\xA0\x82\x01R_`\xA0\x83\x01Q`\xC0\x80\x84\x01R\x80Q\x80`\xE0\x85\x01R\x80` \x83\x01a\x01\0\x86\x01^_a\x01\0\x82\x86\x01\x01Ra\x01\0`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x04\x98W__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xAEW__\xFD[\x82\x01`\xC0\x81\x85\x03\x12\x15a\x04\xBFW__\xFD[\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x04\xDAW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x04\xF8WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_` \x82\x84\x03\x12\x15a\x05\x1DW__\xFD[PQ\x91\x90PV[_\x815`\x01`\x01`X\x1B\x03\x19\x81\x16\x81\x14a\x05<W__\xFD[\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x05WW__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05qW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x03\x16W__\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x1F\x82\x11\x15a\x05\xE0W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x05\xBEWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x05\xDDW_\x81U`\x01\x01a\x05\xCAV[PP[PPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x05\xFDWa\x05\xFDa\x05\x85V[a\x06\x11\x83a\x06\x0B\x83Ta\x04\xC6V[\x83a\x05\x99V[_`\x1F\x84\x11`\x01\x81\x14a\x06BW_\x85\x15a\x06+WP\x83\x82\x015[_\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\x05\xDDV[_\x83\x81R` \x81 `\x1F\x19\x87\x16\x91[\x82\x81\x10\x15a\x06qW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x06QV[P\x86\x82\x10\x15a\x06\x8DW_\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[\x815\x81U` \x82\x015`\x01\x82\x01U`\x02\x81\x01_`@\x84\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\xC8W__\xFD[\x82Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x82UPa\x06\xE9``\x84\x01a\x05$V[\x81Td\x01\0\0\0\0`\x01`\xC8\x1B\x03\x19\x16`8\x91\x90\x91\x1Cd\x01\0\0\0\0`\x01`\xC8\x1B\x03\x16\x17\x90Ua\x07?a\x07\x1E`\x80\x84\x01a\x05$V[`\x03\x83\x01\x81`X\x1Cj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x82T\x16\x17\x81UPPV[a\x07L`\xA0\x83\x01\x83a\x05BV[a\x07Z\x81\x83`\x04\x86\x01a\x05\xE5V[PPPPV\xFE\xA1dsolcC\0\x08\x1B\0\n",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `MockStatefulTronTxReader_TimestampTooLarge()` and selector `0xe55b40cc`.
```solidity
error MockStatefulTronTxReader_TimestampTooLarge();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MockStatefulTronTxReader_TimestampTooLarge;
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
        impl ::core::convert::From<MockStatefulTronTxReader_TimestampTooLarge>
        for UnderlyingRustTuple<'_> {
            fn from(value: MockStatefulTronTxReader_TimestampTooLarge) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for MockStatefulTronTxReader_TimestampTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MockStatefulTronTxReader_TimestampTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MockStatefulTronTxReader_TimestampTooLarge()";
            const SELECTOR: [u8; 4] = [229u8, 91u8, 64u8, 204u8];
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
    /**Function with signature `clearNext()` and selector `0xe3aab141`.
```solidity
function clearNext() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clearNextCall;
    ///Container type for the return parameters of the [`clearNext()`](clearNextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clearNextReturn {}
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
            impl ::core::convert::From<clearNextCall> for UnderlyingRustTuple<'_> {
                fn from(value: clearNextCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clearNextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<clearNextReturn> for UnderlyingRustTuple<'_> {
                fn from(value: clearNextReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clearNextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl clearNextReturn {
            fn _tokenize(
                &self,
            ) -> <clearNextCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for clearNextCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = clearNextReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "clearNext()";
            const SELECTOR: [u8; 4] = [227u8, 170u8, 177u8, 65u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                clearNextReturn::_tokenize(ret)
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
    /**Function with signature `setNext((bytes32,uint256,uint32,bytes21,bytes21,bytes))` and selector `0xe1ab2709`.
```solidity
function setNext(ITronTxReader.TriggerSmartContract memory next_) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setNextCall {
        #[allow(missing_docs)]
        pub next_: <ITronTxReader::TriggerSmartContract as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`setNext((bytes32,uint256,uint32,bytes21,bytes21,bytes))`](setNextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setNextReturn {}
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
            impl ::core::convert::From<setNextCall> for UnderlyingRustTuple<'_> {
                fn from(value: setNextCall) -> Self {
                    (value.next_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setNextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { next_: tuple.0 }
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
            impl ::core::convert::From<setNextReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setNextReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setNextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setNextReturn {
            fn _tokenize(
                &self,
            ) -> <setNextCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setNextCall {
            type Parameters<'a> = (ITronTxReader::TriggerSmartContract,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setNextReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setNext((bytes32,uint256,uint32,bytes21,bytes21,bytes))";
            const SELECTOR: [u8; 4] = [225u8, 171u8, 39u8, 9u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ITronTxReader::TriggerSmartContract as alloy_sol_types::SolType>::tokenize(
                        &self.next_,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setNextReturn::_tokenize(ret)
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
    ///Container for all the [`MockStatefulTronTxReader`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum MockStatefulTronTxReaderCalls {
        #[allow(missing_docs)]
        clearNext(clearNextCall),
        #[allow(missing_docs)]
        readTriggerSmartContract(readTriggerSmartContractCall),
        #[allow(missing_docs)]
        setNext(setNextCall),
    }
    impl MockStatefulTronTxReaderCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [73u8, 205u8, 159u8, 152u8],
            [225u8, 171u8, 39u8, 9u8],
            [227u8, 170u8, 177u8, 65u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(readTriggerSmartContract),
            ::core::stringify!(setNext),
            ::core::stringify!(clearNext),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <readTriggerSmartContractCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setNextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <clearNextCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for MockStatefulTronTxReaderCalls {
        const NAME: &'static str = "MockStatefulTronTxReaderCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 3usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::clearNext(_) => {
                    <clearNextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::readTriggerSmartContract(_) => {
                    <readTriggerSmartContractCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setNext(_) => <setNextCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<MockStatefulTronTxReaderCalls>] = &[
                {
                    fn readTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockStatefulTronTxReaderCalls> {
                        <readTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(MockStatefulTronTxReaderCalls::readTriggerSmartContract)
                    }
                    readTriggerSmartContract
                },
                {
                    fn setNext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockStatefulTronTxReaderCalls> {
                        <setNextCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(MockStatefulTronTxReaderCalls::setNext)
                    }
                    setNext
                },
                {
                    fn clearNext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockStatefulTronTxReaderCalls> {
                        <clearNextCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(MockStatefulTronTxReaderCalls::clearNext)
                    }
                    clearNext
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
            ) -> alloy_sol_types::Result<MockStatefulTronTxReaderCalls>] = &[
                {
                    fn readTriggerSmartContract(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockStatefulTronTxReaderCalls> {
                        <readTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockStatefulTronTxReaderCalls::readTriggerSmartContract)
                    }
                    readTriggerSmartContract
                },
                {
                    fn setNext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockStatefulTronTxReaderCalls> {
                        <setNextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockStatefulTronTxReaderCalls::setNext)
                    }
                    setNext
                },
                {
                    fn clearNext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockStatefulTronTxReaderCalls> {
                        <clearNextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(MockStatefulTronTxReaderCalls::clearNext)
                    }
                    clearNext
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
                Self::clearNext(inner) => {
                    <clearNextCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::readTriggerSmartContract(inner) => {
                    <readTriggerSmartContractCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setNext(inner) => {
                    <setNextCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::clearNext(inner) => {
                    <clearNextCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setNext(inner) => {
                    <setNextCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`MockStatefulTronTxReader`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum MockStatefulTronTxReaderErrors {
        #[allow(missing_docs)]
        MockStatefulTronTxReader_TimestampTooLarge(
            MockStatefulTronTxReader_TimestampTooLarge,
        ),
    }
    impl MockStatefulTronTxReaderErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[229u8, 91u8, 64u8, 204u8]];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(MockStatefulTronTxReader_TimestampTooLarge),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <MockStatefulTronTxReader_TimestampTooLarge as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for MockStatefulTronTxReaderErrors {
        const NAME: &'static str = "MockStatefulTronTxReaderErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::MockStatefulTronTxReader_TimestampTooLarge(_) => {
                    <MockStatefulTronTxReader_TimestampTooLarge as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<MockStatefulTronTxReaderErrors>] = &[
                {
                    fn MockStatefulTronTxReader_TimestampTooLarge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockStatefulTronTxReaderErrors> {
                        <MockStatefulTronTxReader_TimestampTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                MockStatefulTronTxReaderErrors::MockStatefulTronTxReader_TimestampTooLarge,
                            )
                    }
                    MockStatefulTronTxReader_TimestampTooLarge
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
            ) -> alloy_sol_types::Result<MockStatefulTronTxReaderErrors>] = &[
                {
                    fn MockStatefulTronTxReader_TimestampTooLarge(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<MockStatefulTronTxReaderErrors> {
                        <MockStatefulTronTxReader_TimestampTooLarge as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                MockStatefulTronTxReaderErrors::MockStatefulTronTxReader_TimestampTooLarge,
                            )
                    }
                    MockStatefulTronTxReader_TimestampTooLarge
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
                Self::MockStatefulTronTxReader_TimestampTooLarge(inner) => {
                    <MockStatefulTronTxReader_TimestampTooLarge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::MockStatefulTronTxReader_TimestampTooLarge(inner) => {
                    <MockStatefulTronTxReader_TimestampTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`MockStatefulTronTxReader`](self) contract instance.

See the [wrapper's documentation](`MockStatefulTronTxReaderInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> MockStatefulTronTxReaderInstance<P, N> {
        MockStatefulTronTxReaderInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<MockStatefulTronTxReaderInstance<P, N>>,
    > {
        MockStatefulTronTxReaderInstance::<P, N>::deploy(__provider)
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
        MockStatefulTronTxReaderInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`MockStatefulTronTxReader`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`MockStatefulTronTxReader`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct MockStatefulTronTxReaderInstance<
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for MockStatefulTronTxReaderInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("MockStatefulTronTxReaderInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > MockStatefulTronTxReaderInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`MockStatefulTronTxReader`](self) contract instance.

See the [wrapper's documentation](`MockStatefulTronTxReaderInstance`) for more details.*/
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
        ) -> alloy_contract::Result<MockStatefulTronTxReaderInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> MockStatefulTronTxReaderInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> MockStatefulTronTxReaderInstance<P, N> {
            MockStatefulTronTxReaderInstance {
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
    > MockStatefulTronTxReaderInstance<P, N> {
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
        ///Creates a new call builder for the [`clearNext`] function.
        pub fn clearNext(&self) -> alloy_contract::SolCallBuilder<&P, clearNextCall, N> {
            self.call_builder(&clearNextCall)
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
        ///Creates a new call builder for the [`setNext`] function.
        pub fn setNext(
            &self,
            next_: <ITronTxReader::TriggerSmartContract as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, setNextCall, N> {
            self.call_builder(&setNextCall { next_ })
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > MockStatefulTronTxReaderInstance<P, N> {
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
