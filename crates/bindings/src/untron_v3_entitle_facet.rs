///Module containing a contract's types and functions.
/**

```solidity
library UntronV3Base {
    struct PayoutConfig { uint256 targetChainId; address targetToken; address beneficiary; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod UntronV3Base {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct PayoutConfig { uint256 targetChainId; address targetToken; address beneficiary; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PayoutConfig {
        #[allow(missing_docs)]
        pub targetChainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub targetToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub beneficiary: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<PayoutConfig> for UnderlyingRustTuple<'_> {
            fn from(value: PayoutConfig) -> Self {
                (value.targetChainId, value.targetToken, value.beneficiary)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PayoutConfig {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    targetChainId: tuple.0,
                    targetToken: tuple.1,
                    beneficiary: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PayoutConfig {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PayoutConfig {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.targetChainId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.targetToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.beneficiary,
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
        impl alloy_sol_types::SolType for PayoutConfig {
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
        impl alloy_sol_types::SolStruct for PayoutConfig {
            const NAME: &'static str = "PayoutConfig";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PayoutConfig(uint256 targetChainId,address targetToken,address beneficiary)",
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.targetChainId)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.targetToken,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.beneficiary,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PayoutConfig {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.targetChainId,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.targetToken,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.beneficiary,
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.targetChainId,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.targetToken,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.beneficiary,
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
    /**Creates a new wrapper around an on-chain [`UntronV3Base`](self) contract instance.

See the [wrapper's documentation](`UntronV3BaseInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> UntronV3BaseInstance<P, N> {
        UntronV3BaseInstance::<P, N>::new(address, __provider)
    }
    /**A [`UntronV3Base`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`UntronV3Base`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct UntronV3BaseInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for UntronV3BaseInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("UntronV3BaseInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > UntronV3BaseInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`UntronV3Base`](self) contract instance.

See the [wrapper's documentation](`UntronV3BaseInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> UntronV3BaseInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> UntronV3BaseInstance<P, N> {
            UntronV3BaseInstance {
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
    > UntronV3BaseInstance<P, N> {
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
    > UntronV3BaseInstance<P, N> {
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
///Module containing a contract's types and functions.
/**

```solidity
library UntronV3Index {
    type ClaimOrigin is uint8;
    type PnlReason is uint8;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod UntronV3Index {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ClaimOrigin(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<ClaimOrigin> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl ClaimOrigin {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u8> for ClaimOrigin {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<ClaimOrigin> for u8 {
            fn from(value: ClaimOrigin) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ClaimOrigin {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ClaimOrigin {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PnlReason(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<PnlReason> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl PnlReason {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u8> for PnlReason {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<PnlReason> for u8 {
            fn from(value: PnlReason) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for PnlReason {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PnlReason {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`UntronV3Index`](self) contract instance.

See the [wrapper's documentation](`UntronV3IndexInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> UntronV3IndexInstance<P, N> {
        UntronV3IndexInstance::<P, N>::new(address, __provider)
    }
    /**A [`UntronV3Index`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`UntronV3Index`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct UntronV3IndexInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for UntronV3IndexInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("UntronV3IndexInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > UntronV3IndexInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`UntronV3Index`](self) contract instance.

See the [wrapper's documentation](`UntronV3IndexInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> UntronV3IndexInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> UntronV3IndexInstance<P, N> {
            UntronV3IndexInstance {
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
    > UntronV3IndexInstance<P, N> {
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
    > UntronV3IndexInstance<P, N> {
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
library UntronV3Base {
    struct PayoutConfig {
        uint256 targetChainId;
        address targetToken;
        address beneficiary;
    }
}

library UntronV3Index {
    type ClaimOrigin is uint8;
    type PnlReason is uint8;
}

interface UntronV3EntitleFacet {
    error AlreadyInitialized();
    error AmountTooLargeForInt();
    error CannotRescueUSDT();
    error ChainDeprecated();
    error DepositAlreadyProcessed();
    error DepositNotAfterLastReceiverPull();
    error EnforcedPause();
    error EventRelayNoProgress();
    error EventTipMismatch();
    error ExpectedPause();
    error InsufficientLpPrincipal();
    error InsufficientProtocolProfit();
    error InsufficientUsdtBalance();
    error InvalidLeaseId();
    error InvalidLeaseTimeframe();
    error InvalidReceiverForSalt();
    error InvalidSignature();
    error InvalidTargetToken();
    error LeaseDurationTooLong();
    error LeaseFeeTooLow();
    error LeaseFlatFeeTooLow();
    error LeaseNotNukeableYet();
    error LeaseRateLimitConfigInvalid();
    error LeaseRateLimitExceeded();
    error LpNotAllowlisted();
    error NewOwnerIsZeroAddress();
    error NoActiveLease();
    error NoBridger();
    error NotATrc20Transfer();
    error NotEventChainTip();
    error NotLessee();
    error NotRealtor();
    error NotTronUsdt();
    error PayoutConfigRateLimitConfigInvalid();
    error PayoutConfigRateLimitExceeded();
    error RateNotSet();
    error Reentrancy();
    error SignatureExpired();
    error SubjectiveNetOutZero();
    error SubjectivePreEntitlementAlreadyExists();
    error TronInvalidCalldataLength();
    error TronInvalidTrc20DataLength();
    error Unauthorized();
    error WithdrawExceedsPrincipal();
    error ZeroAmount();

    event BridgerSet(address indexed targetToken, uint256 indexed targetChainId, address bridger);
    event ChainDeprecatedSet(uint256 indexed targetChainId, bool deprecated);
    event ClaimCreated(uint256 indexed leaseId, uint256 indexed claimId, address targetToken, uint256 queueIndex, uint256 amountUsdt, uint256 targetChainId, address beneficiary, UntronV3Index.ClaimOrigin origin, bytes32 originId, address originActor, address originToken, uint64 originTimestamp, uint256 originRawAmount);
    event ClaimFilled(uint256 indexed leaseId, uint256 indexed claimId, address targetToken, uint256 queueIndex, uint256 amountUsdt, uint256 targetChainId, address beneficiary);
    event ControllerEventChainTipUpdated(bytes32 previousTip, uint256 indexed blockNumber, uint256 blockTimestamp, bytes32 indexed eventSignature, bytes abiEncodedEventData);
    event ControllerEventProcessed(uint256 indexed eventIndex, uint256 indexed blockNumber, uint256 blockTimestamp, bytes32 indexed eventSignature, bytes abiEncodedEventData);
    event EventAppended(uint256 indexed eventSeq, bytes32 indexed prevTip, bytes32 indexed newTip, bytes32 eventSignature, bytes abiEncodedEventData);
    event LeaseCreated(uint256 indexed leaseId, bytes32 indexed receiverSalt, uint256 indexed leaseNumber, address realtor, address lessee, uint64 startTime, uint64 nukeableAfter, uint32 leaseFeePpm, uint64 flatFee);
    event LeaseNonceUpdated(uint256 indexed leaseId, uint256 nonce);
    event LesseePayoutConfigRateLimitSet(uint256 maxUpdates, uint256 windowSeconds);
    event LpDeposited(address indexed lp, uint256 amount);
    event LpSet(address indexed lp, bool allowed);
    event LpWithdrawn(address indexed lp, uint256 amount);
    event OwnershipTransferred(address indexed oldOwner, address indexed newOwner);
    event Paused(address account);
    event PayoutConfigUpdated(uint256 indexed leaseId, uint256 targetChainId, address targetToken, address beneficiary);
    event ProtocolFlatFeeFloorSet(uint64 floorFlatFee);
    event ProtocolFloorSet(uint256 floorPpm);
    event ProtocolMaxLeaseDurationSet(uint32 maxLeaseDurationSeconds);
    event ProtocolPnlUpdated(int256 pnl, int256 delta, UntronV3Index.PnlReason reason);
    event RealtorLeaseRateLimitSet(address indexed realtor, uint256 maxLeases, uint256 windowSeconds);
    event RealtorMaxLeaseDurationSet(address indexed realtor, uint32 maxLeaseDurationSeconds);
    event RealtorMinFeeSet(address indexed realtor, uint256 minFeePpm);
    event RealtorMinFlatFeeSet(address indexed realtor, uint64 minFlatFee);
    event RealtorSet(address indexed realtor, bool allowed);
    event SwapRateSet(address indexed targetToken, uint256 ratePpm);
    event TokensRescued(address token, uint256 amount);
    event TronReaderSet(address indexed reader);
    event TronUsdtSet(address indexed tronUsdt);
    event Unpaused(address account);
    event UsdtSet(address indexed usdt);

    function CONTROLLER_ADDRESS() external view returns (address);
    function RECEIVER_IMPL() external view returns (address);
    function SWAP_EXECUTOR() external view returns (address);
    function bridgers(address, uint256) external view returns (address);
    function claimLocatorByLease(uint256, uint256) external view returns (address targetToken, uint256 queueIndex);
    function claimsByTargetToken(address, uint256) external view returns (uint256 claimId, uint256 amountUsdt, uint256 leaseId, uint256 targetChainId, address beneficiary);
    function depositProcessed(bytes32) external view returns (bool);
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function eventChainTip() external view returns (bytes32);
    function eventSeq() external view returns (uint256);
    function isChainDeprecated(uint256) external view returns (bool);
    function isLpAllowed(address) external view returns (bool);
    function isRealtor(address) external view returns (bool);
    function lastControllerEventSeq() external view returns (uint256);
    function lastControllerEventTip() external view returns (bytes32);
    function lastReceiverPullTimestampByToken(bytes32, address) external view returns (uint64);
    function leaseNonces(uint256) external view returns (uint256);
    function leasesByReceiver(bytes32, uint256) external view returns (bytes32 receiverSalt, address realtor, address lessee, uint64 startTime, uint64 nukeableAfter, uint32 leaseFeePpm, uint64 flatFee, uint256 recognizedRaw, uint256 backedRaw, uint256 unbackedRaw, UntronV3Base.PayoutConfig memory payout);
    function lpPrincipal(address) external view returns (uint256);
    function nextClaimIdByLease(uint256) external view returns (uint256);
    function nextControllerEventIndex() external view returns (uint256);
    function nextIndexByTargetToken(address) external view returns (uint256);
    function nextLeaseId() external view returns (uint256);
    function owner() external view returns (address result);
    function paused() external view returns (bool);
    function preEntitle(bytes32 receiverSalt, bytes[20] memory blocks, bytes memory encodedTx, bytes32[] memory proof, uint256 index) external returns (uint256 queueIndex, uint256 leaseId, uint256 netOut);
    function predictReceiverAddress(bytes32 salt) external view returns (address predicted);
    function predictReceiverAddress(address controller, bytes32 salt) external view returns (address predicted);
    function protocolPnl() external view returns (int256);
    function receiverBytecode() external view returns (bytes memory);
    function renounceOwnership() external payable;
    function subjectivePreEntitle(bytes32 txId, uint256 leaseId, uint256 rawAmount) external returns (uint256 queueIndex, uint256 netOut);
    function subjectivePreEntitlementByTxId(bytes32) external view returns (address sponsor, uint256 leaseId, uint256 rawAmount, uint256 queueIndex, uint256 claimId);
    function swapRatePpm(address) external view returns (uint256);
    function transferOwnership(address newOwner) external payable;
    function tronReader() external view returns (address);
    function tronUsdt() external view returns (address);
    function usdt() external view returns (address);
    function usdtBalance() external view returns (uint256);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "CONTROLLER_ADDRESS",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "RECEIVER_IMPL",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "SWAP_EXECUTOR",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract SwapExecutor"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "bridgers",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IBridger"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "claimLocatorByLease",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "targetToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "queueIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "claimsByTargetToken",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "claimId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountUsdt",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "leaseId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "targetChainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "beneficiary",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "depositProcessed",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eip712Domain",
    "inputs": [],
    "outputs": [
      {
        "name": "fields",
        "type": "bytes1",
        "internalType": "bytes1"
      },
      {
        "name": "name",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "version",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "chainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "verifyingContract",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "extensions",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eventChainTip",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eventSeq",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isChainDeprecated",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isLpAllowed",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isRealtor",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "lastControllerEventSeq",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "lastControllerEventTip",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "lastReceiverPullTimestampByToken",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "leaseNonces",
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
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "leasesByReceiver",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "receiverSalt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "realtor",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "lessee",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "startTime",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "nukeableAfter",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "leaseFeePpm",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "flatFee",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "recognizedRaw",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "backedRaw",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "unbackedRaw",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "payout",
        "type": "tuple",
        "internalType": "struct UntronV3Base.PayoutConfig",
        "components": [
          {
            "name": "targetChainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "targetToken",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "beneficiary",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "lpPrincipal",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "nextClaimIdByLease",
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
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "nextControllerEventIndex",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "nextIndexByTargetToken",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "nextLeaseId",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "result",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "paused",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "preEntitle",
    "inputs": [
      {
        "name": "receiverSalt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
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
        "name": "queueIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "leaseId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "netOut",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "predictReceiverAddress",
    "inputs": [
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "predicted",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "predictReceiverAddress",
    "inputs": [
      {
        "name": "controller",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "predicted",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "protocolPnl",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "int256",
        "internalType": "int256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "receiverBytecode",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "subjectivePreEntitle",
    "inputs": [
      {
        "name": "txId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "leaseId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "rawAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "queueIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "netOut",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "subjectivePreEntitlementByTxId",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "sponsor",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "leaseId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "rawAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "queueIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "claimId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "swapRatePpm",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "tronReader",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ITronTxReader"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "tronUsdt",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "usdt",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "usdtBalance",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "BridgerSet",
    "inputs": [
      {
        "name": "targetToken",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "targetChainId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "bridger",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ChainDeprecatedSet",
    "inputs": [
      {
        "name": "targetChainId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "deprecated",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ClaimCreated",
    "inputs": [
      {
        "name": "leaseId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "claimId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "targetToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "queueIndex",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "amountUsdt",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "targetChainId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "beneficiary",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "origin",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum UntronV3Index.ClaimOrigin"
      },
      {
        "name": "originId",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "originActor",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "originToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "originTimestamp",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      },
      {
        "name": "originRawAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ClaimFilled",
    "inputs": [
      {
        "name": "leaseId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "claimId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "targetToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "queueIndex",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "amountUsdt",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "targetChainId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "beneficiary",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ControllerEventChainTipUpdated",
    "inputs": [
      {
        "name": "previousTip",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "blockNumber",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "blockTimestamp",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "eventSignature",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "abiEncodedEventData",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ControllerEventProcessed",
    "inputs": [
      {
        "name": "eventIndex",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "blockNumber",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "blockTimestamp",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "eventSignature",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "abiEncodedEventData",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EventAppended",
    "inputs": [
      {
        "name": "eventSeq",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "prevTip",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "newTip",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "eventSignature",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "abiEncodedEventData",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "LeaseCreated",
    "inputs": [
      {
        "name": "leaseId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "receiverSalt",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "leaseNumber",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "realtor",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "lessee",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "startTime",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      },
      {
        "name": "nukeableAfter",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      },
      {
        "name": "leaseFeePpm",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "flatFee",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "LeaseNonceUpdated",
    "inputs": [
      {
        "name": "leaseId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "nonce",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "LesseePayoutConfigRateLimitSet",
    "inputs": [
      {
        "name": "maxUpdates",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "windowSeconds",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "LpDeposited",
    "inputs": [
      {
        "name": "lp",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "LpSet",
    "inputs": [
      {
        "name": "lp",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "allowed",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "LpWithdrawn",
    "inputs": [
      {
        "name": "lp",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "oldOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Paused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PayoutConfigUpdated",
    "inputs": [
      {
        "name": "leaseId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "targetChainId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "targetToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "beneficiary",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ProtocolFlatFeeFloorSet",
    "inputs": [
      {
        "name": "floorFlatFee",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ProtocolFloorSet",
    "inputs": [
      {
        "name": "floorPpm",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ProtocolMaxLeaseDurationSet",
    "inputs": [
      {
        "name": "maxLeaseDurationSeconds",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ProtocolPnlUpdated",
    "inputs": [
      {
        "name": "pnl",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      },
      {
        "name": "delta",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      },
      {
        "name": "reason",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum UntronV3Index.PnlReason"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RealtorLeaseRateLimitSet",
    "inputs": [
      {
        "name": "realtor",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "maxLeases",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "windowSeconds",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RealtorMaxLeaseDurationSet",
    "inputs": [
      {
        "name": "realtor",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "maxLeaseDurationSeconds",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RealtorMinFeeSet",
    "inputs": [
      {
        "name": "realtor",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "minFeePpm",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RealtorMinFlatFeeSet",
    "inputs": [
      {
        "name": "realtor",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "minFlatFee",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RealtorSet",
    "inputs": [
      {
        "name": "realtor",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "allowed",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SwapRateSet",
    "inputs": [
      {
        "name": "targetToken",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "ratePpm",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TokensRescued",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TronReaderSet",
    "inputs": [
      {
        "name": "reader",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TronUsdtSet",
    "inputs": [
      {
        "name": "tronUsdt",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Unpaused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UsdtSet",
    "inputs": [
      {
        "name": "usdt",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AlreadyInitialized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "AmountTooLargeForInt",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CannotRescueUSDT",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ChainDeprecated",
    "inputs": []
  },
  {
    "type": "error",
    "name": "DepositAlreadyProcessed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "DepositNotAfterLastReceiverPull",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EnforcedPause",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EventRelayNoProgress",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EventTipMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ExpectedPause",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientLpPrincipal",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientProtocolProfit",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientUsdtBalance",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidLeaseId",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidLeaseTimeframe",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidReceiverForSalt",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidTargetToken",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LeaseDurationTooLong",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LeaseFeeTooLow",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LeaseFlatFeeTooLow",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LeaseNotNukeableYet",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LeaseRateLimitConfigInvalid",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LeaseRateLimitExceeded",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LpNotAllowlisted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NewOwnerIsZeroAddress",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NoActiveLease",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NoBridger",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotATrc20Transfer",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotEventChainTip",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotLessee",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotRealtor",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotTronUsdt",
    "inputs": []
  },
  {
    "type": "error",
    "name": "PayoutConfigRateLimitConfigInvalid",
    "inputs": []
  },
  {
    "type": "error",
    "name": "PayoutConfigRateLimitExceeded",
    "inputs": []
  },
  {
    "type": "error",
    "name": "RateNotSet",
    "inputs": []
  },
  {
    "type": "error",
    "name": "Reentrancy",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SignatureExpired",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SubjectiveNetOutZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SubjectivePreEntitlementAlreadyExists",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TronInvalidCalldataLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TronInvalidTrc20DataLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "Unauthorized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WithdrawExceedsPrincipal",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroAmount",
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
pub mod UntronV3EntitleFacet {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6102c06040526101626101208181526002916127e36101403960405160200161002891906101e2565b60408051601f19818403018152908290526100429161020b565b602060405180830381855afa15801561005d573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906100809190610216565b6001555f6009556002604051806101a0016040528061016281526020016127e361016291396040516020016100b5919061022d565b60408051601f19818403018152908290526100cf9161020b565b602060405180830381855afa1580156100ea573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061010d9190610216565b60185534801561011b575f5ffd5b50306080524660a05260608061016260408051808201825260068152652ab73a3937b760d11b602080830191909152825180840190935260018352603160f81b9083015291565b815160209283012081519183019190912060c082905260e0819052604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f8152938401929092529082015246606082015230608082015260a09020610100525061025e9050565b5f81518060208401855e5f93019283525090919050565b6d2ab73a3937b72b19a4b73232bc0560911b81525f610204600e8301846101cb565b9392505050565b5f61020482846101cb565b5f60208284031215610226575f5ffd5b5051919050565b7f556e74726f6e436f6e74726f6c6c6572496e6465780a0000000000000000000081525f61020460168301846101cb565b60805160a05160c05160e0516101005161255761028c5f395f50505f50505f50505f50505f50506125575ff3fe608060405260043610610228575f3560e01c80638da5cb5b11610129578063c63bbf29116100a8578063f04e02c01161006d578063f04e02c0146107e5578063f127a9b314610810578063f2fde38b14610825578063f516a5b414610838578063f9fdcae614610863575f5ffd5b8063c63bbf2914610710578063dc8f863314610767578063de40d89f14610786578063e24d5c35146107a5578063eeb90259146107ba575f5ffd5b8063aa94360c116100ee578063aa94360c1461067a578063b371fa6914610699578063b7ed020e146106ae578063b98e631d146106c3578063bc5c5950146106e2575f5ffd5b80638da5cb5b146105e3578063902238e1146105fb5780639efaca7914610610578063a630255914610631578063a7ec9df914610646575f5ffd5b80634da2f899116101b5578063718fbc251161017a578063718fbc25146104b857806378aaf25e1461052057806380a72c8b1461057057806384b0196e1461058f57806388927296146105b6575f5ffd5b80634da2f899146104145780635c975abb1461043f57806360b6bfdd146104555780636c835a8214610483578063715018a6146104ae575f5ffd5b80632f83d9af116101fb5780632f83d9af146103045780633d92af841461033a5780633fea3488146103be578063482edb07146103dd5780634d53e931146103ff575f5ffd5b806304ec42941461022c5780630b3458791461026f5780631dbf4c61146102a65780632f48ab7d146102e5575b5f5ffd5b348015610237575f5ffd5b5061025a610246366004611bc2565b60176020525f908152604090205460ff1681565b60405190151581526020015b60405180910390f35b34801561027a575f5ffd5b5060085461028e906001600160a01b031681565b6040516001600160a01b039091168152602001610266565b3480156102b1575f5ffd5b5061028e6102c0366004611bf4565b601660209081525f92835260408084209091529082529020546001600160a01b031681565b3480156102f0575f5ffd5b5060065461028e906001600160a01b031681565b34801561030f575f5ffd5b5061032361031e366004611c1c565b61089d565b6040516102669b9a99989796959493929190611c3c565b348015610345575f5ffd5b5061038c610354366004611bc2565b60216020525f9081526040902080546001820154600283015460038401546004909401546001600160a01b0390931693919290919085565b604080516001600160a01b0390961686526020860194909452928401919091526060830152608082015260a001610266565b3480156103c9575f5ffd5b5061028e6103d8366004611bc2565b61095e565b3480156103e8575f5ffd5b506103f161096f565b604051908152602001610266565b34801561040a575f5ffd5b506103f160015481565b34801561041f575f5ffd5b506103f161042e366004611cde565b60146020525f908152604090205481565b34801561044a575f5ffd5b505f5460ff1661025a565b348015610460575f5ffd5b5061025a61046f366004611cde565b600d6020525f908152604090205460ff1681565b34801561048e575f5ffd5b506103f161049d366004611bc2565b60236020525f908152604090205481565b6104b6610999565b005b3480156104c3575f5ffd5b506105016104d2366004611c1c565b601f60209081525f9283526040808420909152908252902080546001909101546001600160a01b039091169082565b604080516001600160a01b039093168352602083019190915201610266565b34801561052b575f5ffd5b5061053f61053a366004611bf4565b6109ac565b6040805195865260208601949094529284019190915260608301526001600160a01b0316608082015260a001610266565b34801561057b575f5ffd5b5060055461028e906001600160a01b031681565b34801561059a575f5ffd5b506105a36109ff565b6040516102669796959493929190611d25565b3480156105c1575f5ffd5b5061025a6105d0366004611bc2565b602080525f908152604090205460ff1681565b3480156105ee575f5ffd5b50638b78c6d8195461028e565b348015610606575f5ffd5b506103f160095481565b34801561061b575f5ffd5b50610624610a58565b6040516102669190611dbb565b34801561063c575f5ffd5b506103f160185481565b348015610651575f5ffd5b50610665610660366004611dcd565b610ac3565b60408051928352602083019190915201610266565b348015610685575f5ffd5b5061028e610694366004611bf4565b610cd2565b3480156106a4575f5ffd5b506103f160195481565b3480156106b9575f5ffd5b506103f160135481565b3480156106ce575f5ffd5b5060045461028e906001600160a01b031681565b3480156106ed575f5ffd5b5061025a6106fc366004611cde565b600e6020525f908152604090205460ff1681565b34801561071b575f5ffd5b5061074f61072a366004611df6565b602260209081525f92835260408084209091529082529020546001600160401b031681565b6040516001600160401b039091168152602001610266565b348015610772575f5ffd5b5060075461028e906001600160a01b031681565b348015610791575f5ffd5b5060035461028e906001600160a01b031681565b3480156107b0575f5ffd5b506103f160025481565b3480156107c5575f5ffd5b506103f16107d4366004611cde565b601d6020525f908152604090205481565b3480156107f0575f5ffd5b506103f16107ff366004611cde565b60156020525f908152604090205481565b34801561081b575f5ffd5b506103f1601b5481565b6104b6610833366004611cde565b610d56565b348015610843575f5ffd5b506103f1610852366004611bc2565b601e6020525f908152604090205481565b34801561086e575f5ffd5b5061088261087d366004611e67565b610d7f565b60408051938452602084019290925290820152606001610266565b600a602052815f5260405f2081815481106108b6575f80fd5b5f918252602091829020600a9091020180546001820154600283015460038401546004850154600586015460068701546040805160608101825260078a0154815260088a01546001600160a01b039081169b82019b909b526009909901548a16908901529599509387169750958216956001600160401b03600160a01b9093048316958284169563ffffffff600160401b85041695600160601b90940490941693919291908b565b5f6109693083610cd2565b92915050565b6006545f906001600160a01b031680610989575f91505090565b610993813061106f565b91505090565b6109a16110fe565b6109aa5f611118565b565b601c602052815f5260405f2081815481106109c5575f80fd5b5f9182526020909120600590910201805460018201546002830154600384015460049094015492955090935091906001600160a01b031685565b600f60f81b6060805f808083610a4660408051808201825260068152652ab73a3937b760d11b602080830191909152825180840190935260018352603160f81b9083015291565b97989097965046955030945091925090565b600354604051733d602d80600a3d3981f3363d3d373d3d3d363d7360601b60208201526bffffffffffffffffffffffff19606092831b1660348201526e5af43d82803e903d91602b57fd5bf360881b6048820152605701604051602081830303815290604052905090565b5f5f610acd611142565b5f85815260208052604090205460ff1615610afb57604051631beb710560e11b815260040160405180910390fd5b5f858152602160205260409020546001600160a01b031615610b305760405163cc0bb0c160e01b815260040160405180910390fd5b825f03610b5057604051631f2a200560e01b815260040160405180910390fd5b5f610b5a85611165565b9050610b6681856111d5565b9150815f03610b8857604051633f41ae0160e01b815260040160405180910390fd5b6007810180546008830154610ba691906001600160a01b0316611243565b335f9081526014602052604090205483811015610bd65760405163acf2da9360e01b815260040160405180910390fd5b610be08482611f56565b335f90815260146020526040812091909155600183015483546002850154610c18926001600160a01b039081169289928d921661133b565b80925081975050506040518060a00160405280336001600160a01b031681526020018981526020018881526020018781526020018281525060215f8b81526020019081526020015f205f820151815f015f6101000a8154816001600160a01b0302191690836001600160a01b0316021790555060208201518160010155604082015181600201556060820151816003015560808201518160040155905050610cc688828589898e338e611457565b50505050935093915050565b6003545f90600160a01b900460f81b8383610ceb610a58565b8051602091820120604051610d3795949392016001600160f81b031994909416845260609290921b6bffffffffffffffffffffffff191660018401526015830152603582015260550190565b60408051601f1981840301815291905280516020909101209392505050565b610d5e6110fe565b8060601b610d7357637448fbae5f526004601cfd5b610d7c81611118565b50565b5f5f5f610d8a611142565b600554604051630939b3f360e31b81525f916001600160a01b0316906349cd9f9890610dc4908d908d908d908d908d908d90600401611fc1565b5f60405180830381865afa158015610dde573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610e059190810190612169565b80515f8181526020805260409020549192509060ff1615610e3957604051631beb710560e11b815260040160405180910390fd5b600160205f8381526020019081526020015f205f6101000a81548160ff0219169083151502179055505f60075f9054906101000a90046001600160a01b031690505f610e948e856080015186606001518760a00151866114e8565b9050610ea58e8386604001516115a2565b610eb98e856040015163ffffffff16611609565b9550855f03610edb57604051632503bb3360e11b815260040160405180910390fd5b5f610ee587611165565b905081816004015f828254610efa9190612224565b9250508190555081816006015f828254610f149190612224565b90915550610f24905081836111d5565b9550610f3082876116bd565b5f84815260216020526040902080546001600160a01b03161561100757805460018201546001600160a01b03909116905f908a148015610f735750848360020154145b6003808501545f8a815260216020526040812080546001600160a01b031916815560018101829055600281018290559283018190556004909201919091559091508115610fed578915610fed576001600160a01b0383165f90815260146020526040812080548c9290610fe7908490612224565b90915550505b8115611003579a50611062975050505050505050565b5050505b861561105b576008820154600783018054600985015491925f9261103c926001600160a01b03928316928d928f92911661133b565b809250819c5050506110588a82848e8d8c8e604001518c6116d8565b50505b5050505050505b9750975097945050505050565b5f6001600160a01b03831661108f57506001600160a01b03811631610969565b6040516370a0823160e01b81526001600160a01b0383811660048301528416906370a0823190602401602060405180830381865afa1580156110d3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110f79190612237565b9392505050565b638b78c6d8195433146109aa576382b429005f526004601cfd5b638b78c6d819546001600160a01b03909116638b78c6d8198190559061113e818361175d565b5050565b5f5460ff16156109aa5760405163d93c066560e01b815260040160405180910390fd5b5f818152600b602052604081206001810154808303611196576040516290ed3d60e61b815260040160405180910390fd5b81545f908152600a602052604090206111b0600183611f56565b815481106111c0576111c061224e565b905f5260205f2090600a020192505050919050565b60038201545f90600160401b900463ffffffff1681620f42406111f88382611f56565b6112029086612262565b61120c9190612279565b6003860154909150600160601b90046001600160401b03168082111561123657808203935061123a565b5f93505b50505092915050565b5f8281526017602052604090205460ff16156112725760405163d9259ca960e01b815260040160405180910390fd5b6001600160a01b03811661129957604051638562eb4560e01b815260040160405180910390fd5b6006546001600160a01b0382811691161480159046841415906112ed576001600160a01b0383165f9081526015602052604081205490036112ed5760405163047e3fe760e11b815260040160405180910390fd5b8015611335576001600160a01b038381165f908152601660209081526040808320888452909152902054166113355760405163b37c79ed60e01b815260040160405180910390fd5b50505050565b5f838152601e60205260408120805482918261135683612298565b909155506001600160a01b038881165f908152601c60209081526040808320815160a0810183528681528084018d81529281018c8152606082018c81528b881660808401908152845460018082018755868a529790982093516005909802909301968755935186860155516002860155915160038501559051600490930180546001600160a01b03191693909416929092179092558054929350916113fb9190611f56565b6040805180820182526001600160a01b03808c16825260208083018581525f8c8152601f83528581208982529092529390209151825491166001600160a01b031990911617815590516001909101559250509550959350505050565b604080516101a0810182528981526020810189905260018801546001600160a01b03908116928201929092526060810187905260808101869052875460a0820152600288015490911660c08201526114de9060e081015f8152602081018690526001600160a01b03851660408201525f60608201819052608082015260a0018390526117fd565b5050505050505050565b5f6114f2826118b5565b6affffffffffffffffffffff1986811691161461152257604051633395765b60e01b815260040160405180910390fd5b6004545f9061153a906001600160a01b031688610cd2565b90505f611546826118b5565b90505f5f61155487896118ca565b9250925050826affffffffffffffffffffff1916826affffffffffffffffffffff191614611595576040516340650ea960e11b815260040160405180910390fd5b9998505050505050505050565b5f8381526022602090815260408083206001600160a01b03861684529091529020546001600160401b031680158015906115eb57506001600160401b03811663ffffffff831611155b1561133557604051633e6c071160e21b815260040160405180910390fd5b5f828152600c602052604081208054808303611629575f92505050610969565b805b801561123a575f868152600a6020526040812080545f1990930192839081106116565761165661224e565b905f5260205f2090600a02019050856001600160401b03168160020160149054906101000a90046001600160401b03166001600160401b0316116116b7578382815481106116a6576116a661224e565b905f5260205f20015494505061123a565b5061162b565b61113e6116d26116cd8385611f56565b611978565b5f6119a5565b604080516101a081018252898152602081018990526001808901546001600160a01b03908116938301939093526060820188905260808201879052885460a0830152600289015490921660c08201526114de9160e08201908152602081018690525f60408201819052606082015263ffffffff8516608082015260a0018390526117fd565b806001600160a01b0316826001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a361113e7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e083836040516020016117e99291906001600160a01b0392831681529116602082015260400190565b6040516020818303038152906040526119d4565b602081015181516040808401516060850151608086015160a087015160c088015160e08901516101008a01516101208b01516101408c01516101608d01516101808e01519a517f77242fbd573af5a5f3518da92600e96795ebfff993606b4fb54dea2dcd2dfe859b6118789b9a9998979695949392916122d8565b60405180910390a3610d7c7f77242fbd573af5a5f3518da92600e96795ebfff993606b4fb54dea2dcd2dfe85826040516020016117e99190612353565b6001600160a01b0316604160a01b1760581b90565b5f5f5f6004855110156118f057604051631279950360e01b815260040160405180910390fd5b5f6118fa86612442565b90506356fa634560e01b6001600160e01b031982160161192a5761191d86611aa2565b8695509093509150611970565b63dc478d2360e01b6001600160e01b03198216016119575761194b86611aea565b91955093509150611970565b604051630915b87b60e21b815260040160405180910390fd5b509250925092565b5f6001600160ff1b038211156119a157604051630599f71d60e21b815260040160405180910390fd5b5090565b815f036119b0575050565b8160135f8282546119c19190612480565b909155505060135461113e908383611b49565b60028054600190810180835590546040519092916119ff9184919043904290899089906020016124be565b60408051601f1981840301815290829052611a19916124f3565b602060405180830381855afa158015611a34573d5f5f3e3d5ffd5b5050506040513d601f19601f82011682018060405250810190611a579190612237565b600181905550600154816002547f78160f0b1b2b32b52a0076d8f0f70888687ba702a4d993d55ac8d9327d57a1278686604051611a959291906124fe565b60405180910390a4505050565b80515f90819060448114611ac957604051639ac60c2d60e01b815260040160405180910390fd5b6024840151604485015181611add816118b5565b9791965090945050505050565b5f5f5f5f8451905080606414611b1357604051639ac60c2d60e01b815260040160405180910390fd5b6024850151604486015160648701518282611b2d826118b5565b9850611b38816118b5565b989a98995091979650505050505050565b7f43991e1e1cfb2eed6c9dc37a7a848622f8e3f75bc38d532a00eaf026cd87a014838383604051611b7c9392919061251e565b60405180910390a1611bbd7f43991e1e1cfb2eed6c9dc37a7a848622f8e3f75bc38d532a00eaf026cd87a0148484846040516020016117e99392919061251e565b505050565b5f60208284031215611bd2575f5ffd5b5035919050565b80356001600160a01b0381168114611bef575f5ffd5b919050565b5f5f60408385031215611c05575f5ffd5b611c0e83611bd9565b946020939093013593505050565b5f5f60408385031215611c2d575f5ffd5b50508035926020909101359150565b8b81526001600160a01b038b811660208301528a1660408201526001600160401b038981166060830152888116608083015263ffffffff881660a0830152861660c082015260e08101859052610100810184905261012081018390526101a08101611cce610140830184805182526020808201516001600160a01b039081169184019190915260409182015116910152565b9c9b505050505050505050505050565b5f60208284031215611cee575f5ffd5b6110f782611bd9565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b60ff60f81b8816815260e060208201525f611d4360e0830189611cf7565b8281036040840152611d558189611cf7565b606084018890526001600160a01b038716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b81811015611daa578351835260209384019390920191600101611d8c565b50909b9a5050505050505050505050565b602081525f6110f76020830184611cf7565b5f5f5f60608486031215611ddf575f5ffd5b505081359360208301359350604090920135919050565b5f5f60408385031215611e07575f5ffd5b82359150611e1760208401611bd9565b90509250929050565b5f5f83601f840112611e30575f5ffd5b5081356001600160401b03811115611e46575f5ffd5b6020830191508360208260051b8501011115611e60575f5ffd5b9250929050565b5f5f5f5f5f5f5f60a0888a031215611e7d575f5ffd5b8735965060208801356001600160401b03811115611e99575f5ffd5b880161028081018a1015611eab575f5ffd5b955060408801356001600160401b03811115611ec5575f5ffd5b8801601f81018a13611ed5575f5ffd5b80356001600160401b03811115611eea575f5ffd5b8a6020828401011115611efb575f5ffd5b6020919091019550935060608801356001600160401b03811115611f1d575f5ffd5b611f298a828b01611e20565b989b979a50959894979596608090950135949350505050565b634e487b7160e01b5f52601160045260245ffd5b8181038181111561096957610969611f42565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b8183525f6001600160fb1b03831115611fa8575f5ffd5b8260051b80836020870137939093016020019392505050565b60808082525f906103008301908301898336829003601e19015b601482101561204957868503607f190184528235818112611ffa575f5ffd5b8d016020810190356001600160401b03811115612015575f5ffd5b803603821315612023575f5ffd5b61202e878284611f69565b96505050602083019250602084019350600182019150611fdb565b50505050828103602084015261206081888a611f69565b90508281036040840152612075818688611f91565b915050826060830152979650505050505050565b634e487b7160e01b5f52604160045260245ffd5b60405160c081016001600160401b03811182821017156120bf576120bf612089565b60405290565b80516affffffffffffffffffffff1981168114611bef575f5ffd5b5f82601f8301126120ef575f5ffd5b81516001600160401b0381111561210857612108612089565b604051601f8201601f19908116603f011681016001600160401b038111828210171561213657612136612089565b60405281815283820160200185101561214d575f5ffd5b8160208501602083015e5f918101602001919091529392505050565b5f60208284031215612179575f5ffd5b81516001600160401b0381111561218e575f5ffd5b820160c0818503121561219f575f5ffd5b6121a761209d565b8151815260208083015190820152604082015163ffffffff811681146121cb575f5ffd5b60408201526121dc606083016120c5565b60608201526121ed608083016120c5565b608082015260a08201516001600160401b0381111561220a575f5ffd5b612216868285016120e0565b60a083015250949350505050565b8082018082111561096957610969611f42565b5f60208284031215612247575f5ffd5b5051919050565b634e487b7160e01b5f52603260045260245ffd5b808202811582820484141761096957610969611f42565b5f8261229357634e487b7160e01b5f52601260045260245ffd5b500490565b5f600182016122a9576122a9611f42565b5060010190565b634e487b7160e01b5f52602160045260245ffd5b600381106122d4576122d46122b0565b9052565b6001600160a01b038c81168252602082018c9052604082018b9052606082018a905288166080820152610160810161231360a08301896122c4565b60c08201969096526001600160a01b0394851660e0820152929093166101008301526001600160401b031661012082015261014001529695505050505050565b5f6101a0820190508251825260208301516020830152604083015161238360408401826001600160a01b03169052565b50606083015160608301526080830151608083015260a083015160a083015260c08301516123bc60c08401826001600160a01b03169052565b5060e08301516123cf60e08401826122c4565b506101008301516101008301526101208301516123f86101208401826001600160a01b03169052565b506101408301516124156101408401826001600160a01b03169052565b506101608301516124326101608401826001600160401b03169052565b5061018092830151919092015290565b805160208201516001600160e01b0319811691906004821015612479576001600160e01b0319600483900360031b81901b82161692505b5050919050565b8082018281125f83128015821682158216171561249f5761249f611f42565b505092915050565b5f81518060208401855e5f93019283525090919050565b8681528560208201528460408201528360608201528260808201525f6124e760a08301846124a7565b98975050505050505050565b5f6110f782846124a7565b828152604060208201525f6125166040830184611cf7565b949350505050565b83815260208101839052606081016006831061253c5761253c6122b0565b82604083015294935050505056fea164736f6c634300081b000a4a757374696e2053756e20697320726573706f6e7369626c6520666f722073657474696e67206261636b2074686520696e6576697461626c6520676c6f62616c20737461626c65636f696e207265766f6c7574696f6e206279207965617273207468726f756768206578706c6f6974696e672054726f6e20555344542773206e6574776f726b206566666563747320616e6420696d706f73696e672076656e646f72206c6f636b2d696e206f6e2068756e6472656473206f66206d696c6c696f6e73206f662070656f706c6520696e2074686520546869726420576f726c642c2077686f2072656c79206f6e20737461626c65636f696e7320666f722072656d697474616e63657320616e6420746f2073746f726520746865697220736176696e677320696e20756e737461626c652c206f766572726567756c617465642065636f6e6f6d6965732e204c6574277320556e74726f6e207468652050656f706c652e
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x02\xC0`@Ra\x01ba\x01 \x81\x81R`\x02\x91a'\xE3a\x01@9`@Q` \x01a\0(\x91\x90a\x01\xE2V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\0B\x91a\x02\x0BV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\0]W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\x80\x91\x90a\x02\x16V[`\x01U_`\tU`\x02`@Q\x80a\x01\xA0\x01`@R\x80a\x01b\x81R` \x01a'\xE3a\x01b\x919`@Q` \x01a\0\xB5\x91\x90a\x02-V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\0\xCF\x91a\x02\x0BV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\0\xEAW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\r\x91\x90a\x02\x16V[`\x18U4\x80\x15a\x01\x1BW__\xFD[P0`\x80RF`\xA0R``\x80a\x01b`@\x80Q\x80\x82\x01\x82R`\x06\x81Re*\xB7:97\xB7`\xD1\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x01\x83R`1`\xF8\x1B\x90\x83\x01R\x91V[\x81Q` \x92\x83\x01 \x81Q\x91\x83\x01\x91\x90\x91 `\xC0\x82\x90R`\xE0\x81\x90R`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x93\x84\x01\x92\x90\x92R\x90\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 a\x01\0RPa\x02^\x90PV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[m*\xB7:97\xB7+\x19\xA4\xB722\xBC\x05`\x91\x1B\x81R_a\x02\x04`\x0E\x83\x01\x84a\x01\xCBV[\x93\x92PPPV[_a\x02\x04\x82\x84a\x01\xCBV[_` \x82\x84\x03\x12\x15a\x02&W__\xFD[PQ\x91\x90PV[\x7FUntronControllerIndex\n\0\0\0\0\0\0\0\0\0\0\x81R_a\x02\x04`\x16\x83\x01\x84a\x01\xCBV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa%Wa\x02\x8C_9_PP_PP_PP_PP_PPa%W_\xF3\xFE`\x80`@R`\x046\x10a\x02(W_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\x01)W\x80c\xC6;\xBF)\x11a\0\xA8W\x80c\xF0N\x02\xC0\x11a\0mW\x80c\xF0N\x02\xC0\x14a\x07\xE5W\x80c\xF1'\xA9\xB3\x14a\x08\x10W\x80c\xF2\xFD\xE3\x8B\x14a\x08%W\x80c\xF5\x16\xA5\xB4\x14a\x088W\x80c\xF9\xFD\xCA\xE6\x14a\x08cW__\xFD[\x80c\xC6;\xBF)\x14a\x07\x10W\x80c\xDC\x8F\x863\x14a\x07gW\x80c\xDE@\xD8\x9F\x14a\x07\x86W\x80c\xE2M\\5\x14a\x07\xA5W\x80c\xEE\xB9\x02Y\x14a\x07\xBAW__\xFD[\x80c\xAA\x946\x0C\x11a\0\xEEW\x80c\xAA\x946\x0C\x14a\x06zW\x80c\xB3q\xFAi\x14a\x06\x99W\x80c\xB7\xED\x02\x0E\x14a\x06\xAEW\x80c\xB9\x8Ec\x1D\x14a\x06\xC3W\x80c\xBC\\YP\x14a\x06\xE2W__\xFD[\x80c\x8D\xA5\xCB[\x14a\x05\xE3W\x80c\x90\"8\xE1\x14a\x05\xFBW\x80c\x9E\xFA\xCAy\x14a\x06\x10W\x80c\xA60%Y\x14a\x061W\x80c\xA7\xEC\x9D\xF9\x14a\x06FW__\xFD[\x80cM\xA2\xF8\x99\x11a\x01\xB5W\x80cq\x8F\xBC%\x11a\x01zW\x80cq\x8F\xBC%\x14a\x04\xB8W\x80cx\xAA\xF2^\x14a\x05 W\x80c\x80\xA7,\x8B\x14a\x05pW\x80c\x84\xB0\x19n\x14a\x05\x8FW\x80c\x88\x92r\x96\x14a\x05\xB6W__\xFD[\x80cM\xA2\xF8\x99\x14a\x04\x14W\x80c\\\x97Z\xBB\x14a\x04?W\x80c`\xB6\xBF\xDD\x14a\x04UW\x80cl\x83Z\x82\x14a\x04\x83W\x80cqP\x18\xA6\x14a\x04\xAEW__\xFD[\x80c/\x83\xD9\xAF\x11a\x01\xFBW\x80c/\x83\xD9\xAF\x14a\x03\x04W\x80c=\x92\xAF\x84\x14a\x03:W\x80c?\xEA4\x88\x14a\x03\xBEW\x80cH.\xDB\x07\x14a\x03\xDDW\x80cMS\xE91\x14a\x03\xFFW__\xFD[\x80c\x04\xECB\x94\x14a\x02,W\x80c\x0B4Xy\x14a\x02oW\x80c\x1D\xBFLa\x14a\x02\xA6W\x80c/H\xAB}\x14a\x02\xE5W[__\xFD[4\x80\x15a\x027W__\xFD[Pa\x02Za\x02F6`\x04a\x1B\xC2V[`\x17` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02zW__\xFD[P`\x08Ta\x02\x8E\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02fV[4\x80\x15a\x02\xB1W__\xFD[Pa\x02\x8Ea\x02\xC06`\x04a\x1B\xF4V[`\x16` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02\xF0W__\xFD[P`\x06Ta\x02\x8E\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x03\x0FW__\xFD[Pa\x03#a\x03\x1E6`\x04a\x1C\x1CV[a\x08\x9DV[`@Qa\x02f\x9B\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x1C<V[4\x80\x15a\x03EW__\xFD[Pa\x03\x8Ca\x03T6`\x04a\x1B\xC2V[`!` R_\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x93\x91\x92\x90\x91\x90\x85V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x02fV[4\x80\x15a\x03\xC9W__\xFD[Pa\x02\x8Ea\x03\xD86`\x04a\x1B\xC2V[a\t^V[4\x80\x15a\x03\xE8W__\xFD[Pa\x03\xF1a\toV[`@Q\x90\x81R` \x01a\x02fV[4\x80\x15a\x04\nW__\xFD[Pa\x03\xF1`\x01T\x81V[4\x80\x15a\x04\x1FW__\xFD[Pa\x03\xF1a\x04.6`\x04a\x1C\xDEV[`\x14` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x04JW__\xFD[P_T`\xFF\x16a\x02ZV[4\x80\x15a\x04`W__\xFD[Pa\x02Za\x04o6`\x04a\x1C\xDEV[`\r` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x04\x8EW__\xFD[Pa\x03\xF1a\x04\x9D6`\x04a\x1B\xC2V[`#` R_\x90\x81R`@\x90 T\x81V[a\x04\xB6a\t\x99V[\0[4\x80\x15a\x04\xC3W__\xFD[Pa\x05\x01a\x04\xD26`\x04a\x1C\x1CV[`\x1F` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80T`\x01\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x82V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x02fV[4\x80\x15a\x05+W__\xFD[Pa\x05?a\x05:6`\x04a\x1B\xF4V[a\t\xACV[`@\x80Q\x95\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16`\x80\x82\x01R`\xA0\x01a\x02fV[4\x80\x15a\x05{W__\xFD[P`\x05Ta\x02\x8E\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05\x9AW__\xFD[Pa\x05\xA3a\t\xFFV[`@Qa\x02f\x97\x96\x95\x94\x93\x92\x91\x90a\x1D%V[4\x80\x15a\x05\xC1W__\xFD[Pa\x02Za\x05\xD06`\x04a\x1B\xC2V[` \x80R_\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x05\xEEW__\xFD[Pc\x8Bx\xC6\xD8\x19Ta\x02\x8EV[4\x80\x15a\x06\x06W__\xFD[Pa\x03\xF1`\tT\x81V[4\x80\x15a\x06\x1BW__\xFD[Pa\x06$a\nXV[`@Qa\x02f\x91\x90a\x1D\xBBV[4\x80\x15a\x06<W__\xFD[Pa\x03\xF1`\x18T\x81V[4\x80\x15a\x06QW__\xFD[Pa\x06ea\x06`6`\x04a\x1D\xCDV[a\n\xC3V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02fV[4\x80\x15a\x06\x85W__\xFD[Pa\x02\x8Ea\x06\x946`\x04a\x1B\xF4V[a\x0C\xD2V[4\x80\x15a\x06\xA4W__\xFD[Pa\x03\xF1`\x19T\x81V[4\x80\x15a\x06\xB9W__\xFD[Pa\x03\xF1`\x13T\x81V[4\x80\x15a\x06\xCEW__\xFD[P`\x04Ta\x02\x8E\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06\xEDW__\xFD[Pa\x02Za\x06\xFC6`\x04a\x1C\xDEV[`\x0E` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x07\x1BW__\xFD[Pa\x07Oa\x07*6`\x04a\x1D\xF6V[`\"` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02fV[4\x80\x15a\x07rW__\xFD[P`\x07Ta\x02\x8E\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07\x91W__\xFD[P`\x03Ta\x02\x8E\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07\xB0W__\xFD[Pa\x03\xF1`\x02T\x81V[4\x80\x15a\x07\xC5W__\xFD[Pa\x03\xF1a\x07\xD46`\x04a\x1C\xDEV[`\x1D` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x07\xF0W__\xFD[Pa\x03\xF1a\x07\xFF6`\x04a\x1C\xDEV[`\x15` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x08\x1BW__\xFD[Pa\x03\xF1`\x1BT\x81V[a\x04\xB6a\x0836`\x04a\x1C\xDEV[a\rVV[4\x80\x15a\x08CW__\xFD[Pa\x03\xF1a\x08R6`\x04a\x1B\xC2V[`\x1E` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x08nW__\xFD[Pa\x08\x82a\x08}6`\x04a\x1EgV[a\r\x7FV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02fV[`\n` R\x81_R`@_ \x81\x81T\x81\x10a\x08\xB6W_\x80\xFD[_\x91\x82R` \x91\x82\x90 `\n\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x87\x01T`@\x80Q``\x81\x01\x82R`\x07\x8A\x01T\x81R`\x08\x8A\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x9B\x82\x01\x9B\x90\x9BR`\t\x90\x99\x01T\x8A\x16\x90\x89\x01R\x95\x99P\x93\x87\x16\x97P\x95\x82\x16\x95`\x01`\x01`@\x1B\x03`\x01`\xA0\x1B\x90\x93\x04\x83\x16\x95\x82\x84\x16\x95c\xFF\xFF\xFF\xFF`\x01`@\x1B\x85\x04\x16\x95`\x01``\x1B\x90\x94\x04\x90\x94\x16\x93\x91\x92\x91\x90\x8BV[_a\ti0\x83a\x0C\xD2V[\x92\x91PPV[`\x06T_\x90`\x01`\x01`\xA0\x1B\x03\x16\x80a\t\x89W_\x91PP\x90V[a\t\x93\x810a\x10oV[\x91PP\x90V[a\t\xA1a\x10\xFEV[a\t\xAA_a\x11\x18V[V[`\x1C` R\x81_R`@_ \x81\x81T\x81\x10a\t\xC5W_\x80\xFD[_\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01T\x92\x95P\x90\x93P\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x85V[`\x0F`\xF8\x1B``\x80_\x80\x80\x83a\nF`@\x80Q\x80\x82\x01\x82R`\x06\x81Re*\xB7:97\xB7`\xD1\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x01\x83R`1`\xF8\x1B\x90\x83\x01R\x91V[\x97\x98\x90\x97\x96PF\x95P0\x94P\x91\x92P\x90V[`\x03T`@Qs=`-\x80`\n=9\x81\xF36==7===6=s``\x1B` \x82\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x92\x83\x1B\x16`4\x82\x01RnZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3`\x88\x1B`H\x82\x01R`W\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[__a\n\xCDa\x11BV[_\x85\x81R` \x80R`@\x90 T`\xFF\x16\x15a\n\xFBW`@Qc\x1B\xEBq\x05`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x85\x81R`!` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0B0W`@Qc\xCC\x0B\xB0\xC1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82_\x03a\x0BPW`@Qc\x1F* \x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0BZ\x85a\x11eV[\x90Pa\x0Bf\x81\x85a\x11\xD5V[\x91P\x81_\x03a\x0B\x88W`@Qc?A\xAE\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07\x81\x01\x80T`\x08\x83\x01Ta\x0B\xA6\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a\x12CV[3_\x90\x81R`\x14` R`@\x90 T\x83\x81\x10\x15a\x0B\xD6W`@Qc\xAC\xF2\xDA\x93`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\xE0\x84\x82a\x1FVV[3_\x90\x81R`\x14` R`@\x81 \x91\x90\x91U`\x01\x83\x01T\x83T`\x02\x85\x01Ta\x0C\x18\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x89\x92\x8D\x92\x16a\x13;V[\x80\x92P\x81\x97PPP`@Q\x80`\xA0\x01`@R\x803`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x89\x81R` \x01\x88\x81R` \x01\x87\x81R` \x01\x82\x81RP`!_\x8B\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01U\x90PPa\x0C\xC6\x88\x82\x85\x89\x89\x8E3\x8Ea\x14WV[PPPP\x93P\x93\x91PPV[`\x03T_\x90`\x01`\xA0\x1B\x90\x04`\xF8\x1B\x83\x83a\x0C\xEBa\nXV[\x80Q` \x91\x82\x01 `@Qa\r7\x95\x94\x93\x92\x01`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84R``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x84\x01R`\x15\x83\x01R`5\x82\x01R`U\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[a\r^a\x10\xFEV[\x80``\x1Ba\rsWctH\xFB\xAE_R`\x04`\x1C\xFD[a\r|\x81a\x11\x18V[PV[___a\r\x8Aa\x11BV[`\x05T`@Qc\t9\xB3\xF3`\xE3\x1B\x81R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cI\xCD\x9F\x98\x90a\r\xC4\x90\x8D\x90\x8D\x90\x8D\x90\x8D\x90\x8D\x90\x8D\x90`\x04\x01a\x1F\xC1V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xDEW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\x05\x91\x90\x81\x01\x90a!iV[\x80Q_\x81\x81R` \x80R`@\x90 T\x91\x92P\x90`\xFF\x16\x15a\x0E9W`@Qc\x1B\xEBq\x05`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01` _\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_`\x07_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90P_a\x0E\x94\x8E\x85`\x80\x01Q\x86``\x01Q\x87`\xA0\x01Q\x86a\x14\xE8V[\x90Pa\x0E\xA5\x8E\x83\x86`@\x01Qa\x15\xA2V[a\x0E\xB9\x8E\x85`@\x01Qc\xFF\xFF\xFF\xFF\x16a\x16\tV[\x95P\x85_\x03a\x0E\xDBW`@Qc%\x03\xBB3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0E\xE5\x87a\x11eV[\x90P\x81\x81`\x04\x01_\x82\x82Ta\x0E\xFA\x91\x90a\"$V[\x92PP\x81\x90UP\x81\x81`\x06\x01_\x82\x82Ta\x0F\x14\x91\x90a\"$V[\x90\x91UPa\x0F$\x90P\x81\x83a\x11\xD5V[\x95Pa\x0F0\x82\x87a\x16\xBDV[_\x84\x81R`!` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x10\x07W\x80T`\x01\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90_\x90\x8A\x14\x80\x15a\x0FsWP\x84\x83`\x02\x01T\x14[`\x03\x80\x85\x01T_\x8A\x81R`!` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x81\x01\x82\x90U`\x02\x81\x01\x82\x90U\x92\x83\x01\x81\x90U`\x04\x90\x92\x01\x91\x90\x91U\x90\x91P\x81\x15a\x0F\xEDW\x89\x15a\x0F\xEDW`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x14` R`@\x81 \x80T\x8C\x92\x90a\x0F\xE7\x90\x84\x90a\"$V[\x90\x91UPP[\x81\x15a\x10\x03W\x9APa\x10b\x97PPPPPPPPV[PPP[\x86\x15a\x10[W`\x08\x82\x01T`\x07\x83\x01\x80T`\t\x85\x01T\x91\x92_\x92a\x10<\x92`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92\x8D\x92\x8F\x92\x91\x16a\x13;V[\x80\x92P\x81\x9CPPPa\x10X\x8A\x82\x84\x8E\x8D\x8C\x8E`@\x01Q\x8Ca\x16\xD8V[PP[PPPPPP[\x97P\x97P\x97\x94PPPPPV[_`\x01`\x01`\xA0\x1B\x03\x83\x16a\x10\x8FWP`\x01`\x01`\xA0\x1B\x03\x81\x161a\tiV[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xD3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xF7\x91\x90a\"7V[\x93\x92PPPV[c\x8Bx\xC6\xD8\x19T3\x14a\t\xAAWc\x82\xB4)\0_R`\x04`\x1C\xFD[c\x8Bx\xC6\xD8\x19T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16c\x8Bx\xC6\xD8\x19\x81\x90U\x90a\x11>\x81\x83a\x17]V[PPV[_T`\xFF\x16\x15a\t\xAAW`@Qc\xD9<\x06e`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x0B` R`@\x81 `\x01\x81\x01T\x80\x83\x03a\x11\x96W`@Qb\x90\xED=`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81T_\x90\x81R`\n` R`@\x90 a\x11\xB0`\x01\x83a\x1FVV[\x81T\x81\x10a\x11\xC0Wa\x11\xC0a\"NV[\x90_R` _ \x90`\n\x02\x01\x92PPP\x91\x90PV[`\x03\x82\x01T_\x90`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81b\x0FB@a\x11\xF8\x83\x82a\x1FVV[a\x12\x02\x90\x86a\"bV[a\x12\x0C\x91\x90a\"yV[`\x03\x86\x01T\x90\x91P`\x01``\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x80\x82\x11\x15a\x126W\x80\x82\x03\x93Pa\x12:V[_\x93P[PPP\x92\x91PPV[_\x82\x81R`\x17` R`@\x90 T`\xFF\x16\x15a\x12rW`@Qc\xD9%\x9C\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x12\x99W`@Qc\x85b\xEBE`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06T`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14\x80\x15\x90F\x84\x14\x15\x90a\x12\xEDW`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x15` R`@\x81 T\x90\x03a\x12\xEDW`@Qc\x04~?\xE7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x135W`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x16` \x90\x81R`@\x80\x83 \x88\x84R\x90\x91R\x90 T\x16a\x135W`@Qc\xB3|y\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[_\x83\x81R`\x1E` R`@\x81 \x80T\x82\x91\x82a\x13V\x83a\"\x98V[\x90\x91UP`\x01`\x01`\xA0\x1B\x03\x88\x81\x16_\x90\x81R`\x1C` \x90\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x86\x81R\x80\x84\x01\x8D\x81R\x92\x81\x01\x8C\x81R``\x82\x01\x8C\x81R\x8B\x88\x16`\x80\x84\x01\x90\x81R\x84T`\x01\x80\x82\x01\x87U\x86\x8AR\x97\x90\x98 \x93Q`\x05\x90\x98\x02\x90\x93\x01\x96\x87U\x93Q\x86\x86\x01UQ`\x02\x86\x01U\x91Q`\x03\x85\x01U\x90Q`\x04\x90\x93\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x93\x90\x94\x16\x92\x90\x92\x17\x90\x92U\x80T\x92\x93P\x91a\x13\xFB\x91\x90a\x1FVV[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x8C\x16\x82R` \x80\x83\x01\x85\x81R_\x8C\x81R`\x1F\x83R\x85\x81 \x89\x82R\x90\x92R\x93\x90 \x91Q\x82T\x91\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x17\x81U\x90Q`\x01\x90\x91\x01U\x92PP\x95P\x95\x93PPPPV[`@\x80Qa\x01\xA0\x81\x01\x82R\x89\x81R` \x81\x01\x89\x90R`\x01\x88\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R\x87T`\xA0\x82\x01R`\x02\x88\x01T\x90\x91\x16`\xC0\x82\x01Ra\x14\xDE\x90`\xE0\x81\x01_\x81R` \x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x85\x16`@\x82\x01R_``\x82\x01\x81\x90R`\x80\x82\x01R`\xA0\x01\x83\x90Ra\x17\xFDV[PPPPPPPPV[_a\x14\xF2\x82a\x18\xB5V[j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x86\x81\x16\x91\x16\x14a\x15\"W`@Qc3\x95v[`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04T_\x90a\x15:\x90`\x01`\x01`\xA0\x1B\x03\x16\x88a\x0C\xD2V[\x90P_a\x15F\x82a\x18\xB5V[\x90P__a\x15T\x87\x89a\x18\xCAV[\x92P\x92PP\x82j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14a\x15\x95W`@Qc@e\x0E\xA9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x99\x98PPPPPPPPPV[_\x83\x81R`\"` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 T`\x01`\x01`@\x1B\x03\x16\x80\x15\x80\x15\x90a\x15\xEBWP`\x01`\x01`@\x1B\x03\x81\x16c\xFF\xFF\xFF\xFF\x83\x16\x11\x15[\x15a\x135W`@Qc>l\x07\x11`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82\x81R`\x0C` R`@\x81 \x80T\x80\x83\x03a\x16)W_\x92PPPa\tiV[\x80[\x80\x15a\x12:W_\x86\x81R`\n` R`@\x81 \x80T_\x19\x90\x93\x01\x92\x83\x90\x81\x10a\x16VWa\x16Va\"NV[\x90_R` _ \x90`\n\x02\x01\x90P\x85`\x01`\x01`@\x1B\x03\x16\x81`\x02\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16\x11a\x16\xB7W\x83\x82\x81T\x81\x10a\x16\xA6Wa\x16\xA6a\"NV[\x90_R` _ \x01T\x94PPa\x12:V[Pa\x16+V[a\x11>a\x16\xD2a\x16\xCD\x83\x85a\x1FVV[a\x19xV[_a\x19\xA5V[`@\x80Qa\x01\xA0\x81\x01\x82R\x89\x81R` \x81\x01\x89\x90R`\x01\x80\x89\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x93\x83\x01\x93\x90\x93R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R\x88T`\xA0\x83\x01R`\x02\x89\x01T\x90\x92\x16`\xC0\x82\x01Ra\x14\xDE\x91`\xE0\x82\x01\x90\x81R` \x81\x01\x86\x90R_`@\x82\x01\x81\x90R``\x82\x01Rc\xFF\xFF\xFF\xFF\x85\x16`\x80\x82\x01R`\xA0\x01\x83\x90Ra\x17\xFDV[\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3a\x11>\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x83\x83`@Q` \x01a\x17\xE9\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x19\xD4V[` \x81\x01Q\x81Q`@\x80\x84\x01Q``\x85\x01Q`\x80\x86\x01Q`\xA0\x87\x01Q`\xC0\x88\x01Q`\xE0\x89\x01Qa\x01\0\x8A\x01Qa\x01 \x8B\x01Qa\x01@\x8C\x01Qa\x01`\x8D\x01Qa\x01\x80\x8E\x01Q\x9AQ\x7Fw$/\xBDW:\xF5\xA5\xF3Q\x8D\xA9&\0\xE9g\x95\xEB\xFF\xF9\x93`kO\xB5M\xEA-\xCD-\xFE\x85\x9Ba\x18x\x9B\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91a\"\xD8V[`@Q\x80\x91\x03\x90\xA3a\r|\x7Fw$/\xBDW:\xF5\xA5\xF3Q\x8D\xA9&\0\xE9g\x95\xEB\xFF\xF9\x93`kO\xB5M\xEA-\xCD-\xFE\x85\x82`@Q` \x01a\x17\xE9\x91\x90a#SV[`\x01`\x01`\xA0\x1B\x03\x16`A`\xA0\x1B\x17`X\x1B\x90V[___`\x04\x85Q\x10\x15a\x18\xF0W`@Qc\x12y\x95\x03`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x18\xFA\x86a$BV[\x90PcV\xFAcE`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\x19*Wa\x19\x1D\x86a\x1A\xA2V[\x86\x95P\x90\x93P\x91Pa\x19pV[c\xDCG\x8D#`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\x19WWa\x19K\x86a\x1A\xEAV[\x91\x95P\x93P\x91Pa\x19pV[`@Qc\t\x15\xB8{`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x92P\x92P\x92V[_`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\x19\xA1W`@Qc\x05\x99\xF7\x1D`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x90V[\x81_\x03a\x19\xB0WPPV[\x81`\x13_\x82\x82Ta\x19\xC1\x91\x90a$\x80V[\x90\x91UPP`\x13Ta\x11>\x90\x83\x83a\x1BIV[`\x02\x80T`\x01\x90\x81\x01\x80\x83U\x90T`@Q\x90\x92\x91a\x19\xFF\x91\x84\x91\x90C\x90B\x90\x89\x90\x89\x90` \x01a$\xBEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1A\x19\x91a$\xF3V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x1A4W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1AW\x91\x90a\"7V[`\x01\x81\x90UP`\x01T\x81`\x02T\x7Fx\x16\x0F\x0B\x1B+2\xB5*\0v\xD8\xF0\xF7\x08\x88h{\xA7\x02\xA4\xD9\x93\xD5Z\xC8\xD92}W\xA1'\x86\x86`@Qa\x1A\x95\x92\x91\x90a$\xFEV[`@Q\x80\x91\x03\x90\xA4PPPV[\x80Q_\x90\x81\x90`D\x81\x14a\x1A\xC9W`@Qc\x9A\xC6\x0C-`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`$\x84\x01Q`D\x85\x01Q\x81a\x1A\xDD\x81a\x18\xB5V[\x97\x91\x96P\x90\x94PPPPPV[____\x84Q\x90P\x80`d\x14a\x1B\x13W`@Qc\x9A\xC6\x0C-`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`$\x85\x01Q`D\x86\x01Q`d\x87\x01Q\x82\x82a\x1B-\x82a\x18\xB5V[\x98Pa\x1B8\x81a\x18\xB5V[\x98\x9A\x98\x99P\x91\x97\x96PPPPPPPV[\x7FC\x99\x1E\x1E\x1C\xFB.\xEDl\x9D\xC3zz\x84\x86\"\xF8\xE3\xF7[\xC3\x8DS*\0\xEA\xF0&\xCD\x87\xA0\x14\x83\x83\x83`@Qa\x1B|\x93\x92\x91\x90a%\x1EV[`@Q\x80\x91\x03\x90\xA1a\x1B\xBD\x7FC\x99\x1E\x1E\x1C\xFB.\xEDl\x9D\xC3zz\x84\x86\"\xF8\xE3\xF7[\xC3\x8DS*\0\xEA\xF0&\xCD\x87\xA0\x14\x84\x84\x84`@Q` \x01a\x17\xE9\x93\x92\x91\x90a%\x1EV[PPPV[_` \x82\x84\x03\x12\x15a\x1B\xD2W__\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B\xEFW__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x1C\x05W__\xFD[a\x1C\x0E\x83a\x1B\xD9V[\x94` \x93\x90\x93\x015\x93PPPV[__`@\x83\x85\x03\x12\x15a\x1C-W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x8B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16` \x83\x01R\x8A\x16`@\x82\x01R`\x01`\x01`@\x1B\x03\x89\x81\x16``\x83\x01R\x88\x81\x16`\x80\x83\x01Rc\xFF\xFF\xFF\xFF\x88\x16`\xA0\x83\x01R\x86\x16`\xC0\x82\x01R`\xE0\x81\x01\x85\x90Ra\x01\0\x81\x01\x84\x90Ra\x01 \x81\x01\x83\x90Ra\x01\xA0\x81\x01a\x1C\xCEa\x01@\x83\x01\x84\x80Q\x82R` \x80\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x84\x01\x91\x90\x91R`@\x91\x82\x01Q\x16\x91\x01RV[\x9C\x9BPPPPPPPPPPPPV[_` \x82\x84\x03\x12\x15a\x1C\xEEW__\xFD[a\x10\xF7\x82a\x1B\xD9V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R_a\x1DC`\xE0\x83\x01\x89a\x1C\xF7V[\x82\x81\x03`@\x84\x01Ra\x1DU\x81\x89a\x1C\xF7V[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15a\x1D\xAAW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1D\x8CV[P\x90\x9B\x9APPPPPPPPPPPV[` \x81R_a\x10\xF7` \x83\x01\x84a\x1C\xF7V[___``\x84\x86\x03\x12\x15a\x1D\xDFW__\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x1E\x07W__\xFD[\x825\x91Pa\x1E\x17` \x84\x01a\x1B\xD9V[\x90P\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a\x1E0W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1EFW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1E`W__\xFD[\x92P\x92\x90PV[_______`\xA0\x88\x8A\x03\x12\x15a\x1E}W__\xFD[\x875\x96P` \x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\x99W__\xFD[\x88\x01a\x02\x80\x81\x01\x8A\x10\x15a\x1E\xABW__\xFD[\x95P`@\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xC5W__\xFD[\x88\x01`\x1F\x81\x01\x8A\x13a\x1E\xD5W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xEAW__\xFD[\x8A` \x82\x84\x01\x01\x11\x15a\x1E\xFBW__\xFD[` \x91\x90\x91\x01\x95P\x93P``\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\x1DW__\xFD[a\x1F)\x8A\x82\x8B\x01a\x1E V[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x95\x96`\x80\x90\x95\x015\x94\x93PPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\tiWa\tia\x1FBV[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x81\x83R_`\x01`\x01`\xFB\x1B\x03\x83\x11\x15a\x1F\xA8W__\xFD[\x82`\x05\x1B\x80\x83` \x87\x017\x93\x90\x93\x01` \x01\x93\x92PPPV[`\x80\x80\x82R_\x90a\x03\0\x83\x01\x90\x83\x01\x89\x836\x82\x90\x03`\x1E\x19\x01[`\x14\x82\x10\x15a IW\x86\x85\x03`\x7F\x19\x01\x84R\x825\x81\x81\x12a\x1F\xFAW__\xFD[\x8D\x01` \x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15a \x15W__\xFD[\x806\x03\x82\x13\x15a #W__\xFD[a .\x87\x82\x84a\x1FiV[\x96PPP` \x83\x01\x92P` \x84\x01\x93P`\x01\x82\x01\x91Pa\x1F\xDBV[PPPP\x82\x81\x03` \x84\x01Ra `\x81\x88\x8Aa\x1FiV[\x90P\x82\x81\x03`@\x84\x01Ra u\x81\x86\x88a\x1F\x91V[\x91PP\x82``\x83\x01R\x97\x96PPPPPPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a \xBFWa \xBFa \x89V[`@R\x90V[\x80Qj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x14a\x1B\xEFW__\xFD[_\x82`\x1F\x83\x01\x12a \xEFW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a!\x08Wa!\x08a \x89V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a!6Wa!6a \x89V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a!MW__\xFD[\x81` \x85\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a!yW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a!\x8EW__\xFD[\x82\x01`\xC0\x81\x85\x03\x12\x15a!\x9FW__\xFD[a!\xA7a \x9DV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x82\x01Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a!\xCBW__\xFD[`@\x82\x01Ra!\xDC``\x83\x01a \xC5V[``\x82\x01Ra!\xED`\x80\x83\x01a \xC5V[`\x80\x82\x01R`\xA0\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\nW__\xFD[a\"\x16\x86\x82\x85\x01a \xE0V[`\xA0\x83\x01RP\x94\x93PPPPV[\x80\x82\x01\x80\x82\x11\x15a\tiWa\tia\x1FBV[_` \x82\x84\x03\x12\x15a\"GW__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\tiWa\tia\x1FBV[_\x82a\"\x93WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[_`\x01\x82\x01a\"\xA9Wa\"\xA9a\x1FBV[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x81\x10a\"\xD4Wa\"\xD4a\"\xB0V[\x90RV[`\x01`\x01`\xA0\x1B\x03\x8C\x81\x16\x82R` \x82\x01\x8C\x90R`@\x82\x01\x8B\x90R``\x82\x01\x8A\x90R\x88\x16`\x80\x82\x01Ra\x01`\x81\x01a#\x13`\xA0\x83\x01\x89a\"\xC4V[`\xC0\x82\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\xE0\x82\x01R\x92\x90\x93\x16a\x01\0\x83\x01R`\x01`\x01`@\x1B\x03\x16a\x01 \x82\x01Ra\x01@\x01R\x96\x95PPPPPPV[_a\x01\xA0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Qa#\x83`@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R`\xC0\x83\x01Qa#\xBC`\xC0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xE0\x83\x01Qa#\xCF`\xE0\x84\x01\x82a\"\xC4V[Pa\x01\0\x83\x01Qa\x01\0\x83\x01Ra\x01 \x83\x01Qa#\xF8a\x01 \x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01@\x83\x01Qa$\x15a\x01@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01`\x83\x01Qa$2a\x01`\x84\x01\x82`\x01`\x01`@\x1B\x03\x16\x90RV[Pa\x01\x80\x92\x83\x01Q\x91\x90\x92\x01R\x90V[\x80Q` \x82\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x91\x90`\x04\x82\x10\x15a$yW`\x01`\x01`\xE0\x1B\x03\x19`\x04\x83\x90\x03`\x03\x1B\x81\x90\x1B\x82\x16\x16\x92P[PP\x91\x90PV[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a$\x9FWa$\x9Fa\x1FBV[PP\x92\x91PPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x86\x81R\x85` \x82\x01R\x84`@\x82\x01R\x83``\x82\x01R\x82`\x80\x82\x01R_a$\xE7`\xA0\x83\x01\x84a$\xA7V[\x98\x97PPPPPPPPV[_a\x10\xF7\x82\x84a$\xA7V[\x82\x81R`@` \x82\x01R_a%\x16`@\x83\x01\x84a\x1C\xF7V[\x94\x93PPPPV[\x83\x81R` \x81\x01\x83\x90R``\x81\x01`\x06\x83\x10a%<Wa%<a\"\xB0V[\x82`@\x83\x01R\x94\x93PPPPV\xFE\xA1dsolcC\0\x08\x1B\0\nJustin Sun is responsible for setting back the inevitable global stablecoin revolution by years through exploiting Tron USDT's network effects and imposing vendor lock-in on hundreds of millions of people in the Third World, who rely on stablecoins for remittances and to store their savings in unstable, overregulated economies. Let's Untron the People.",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610228575f3560e01c80638da5cb5b11610129578063c63bbf29116100a8578063f04e02c01161006d578063f04e02c0146107e5578063f127a9b314610810578063f2fde38b14610825578063f516a5b414610838578063f9fdcae614610863575f5ffd5b8063c63bbf2914610710578063dc8f863314610767578063de40d89f14610786578063e24d5c35146107a5578063eeb90259146107ba575f5ffd5b8063aa94360c116100ee578063aa94360c1461067a578063b371fa6914610699578063b7ed020e146106ae578063b98e631d146106c3578063bc5c5950146106e2575f5ffd5b80638da5cb5b146105e3578063902238e1146105fb5780639efaca7914610610578063a630255914610631578063a7ec9df914610646575f5ffd5b80634da2f899116101b5578063718fbc251161017a578063718fbc25146104b857806378aaf25e1461052057806380a72c8b1461057057806384b0196e1461058f57806388927296146105b6575f5ffd5b80634da2f899146104145780635c975abb1461043f57806360b6bfdd146104555780636c835a8214610483578063715018a6146104ae575f5ffd5b80632f83d9af116101fb5780632f83d9af146103045780633d92af841461033a5780633fea3488146103be578063482edb07146103dd5780634d53e931146103ff575f5ffd5b806304ec42941461022c5780630b3458791461026f5780631dbf4c61146102a65780632f48ab7d146102e5575b5f5ffd5b348015610237575f5ffd5b5061025a610246366004611bc2565b60176020525f908152604090205460ff1681565b60405190151581526020015b60405180910390f35b34801561027a575f5ffd5b5060085461028e906001600160a01b031681565b6040516001600160a01b039091168152602001610266565b3480156102b1575f5ffd5b5061028e6102c0366004611bf4565b601660209081525f92835260408084209091529082529020546001600160a01b031681565b3480156102f0575f5ffd5b5060065461028e906001600160a01b031681565b34801561030f575f5ffd5b5061032361031e366004611c1c565b61089d565b6040516102669b9a99989796959493929190611c3c565b348015610345575f5ffd5b5061038c610354366004611bc2565b60216020525f9081526040902080546001820154600283015460038401546004909401546001600160a01b0390931693919290919085565b604080516001600160a01b0390961686526020860194909452928401919091526060830152608082015260a001610266565b3480156103c9575f5ffd5b5061028e6103d8366004611bc2565b61095e565b3480156103e8575f5ffd5b506103f161096f565b604051908152602001610266565b34801561040a575f5ffd5b506103f160015481565b34801561041f575f5ffd5b506103f161042e366004611cde565b60146020525f908152604090205481565b34801561044a575f5ffd5b505f5460ff1661025a565b348015610460575f5ffd5b5061025a61046f366004611cde565b600d6020525f908152604090205460ff1681565b34801561048e575f5ffd5b506103f161049d366004611bc2565b60236020525f908152604090205481565b6104b6610999565b005b3480156104c3575f5ffd5b506105016104d2366004611c1c565b601f60209081525f9283526040808420909152908252902080546001909101546001600160a01b039091169082565b604080516001600160a01b039093168352602083019190915201610266565b34801561052b575f5ffd5b5061053f61053a366004611bf4565b6109ac565b6040805195865260208601949094529284019190915260608301526001600160a01b0316608082015260a001610266565b34801561057b575f5ffd5b5060055461028e906001600160a01b031681565b34801561059a575f5ffd5b506105a36109ff565b6040516102669796959493929190611d25565b3480156105c1575f5ffd5b5061025a6105d0366004611bc2565b602080525f908152604090205460ff1681565b3480156105ee575f5ffd5b50638b78c6d8195461028e565b348015610606575f5ffd5b506103f160095481565b34801561061b575f5ffd5b50610624610a58565b6040516102669190611dbb565b34801561063c575f5ffd5b506103f160185481565b348015610651575f5ffd5b50610665610660366004611dcd565b610ac3565b60408051928352602083019190915201610266565b348015610685575f5ffd5b5061028e610694366004611bf4565b610cd2565b3480156106a4575f5ffd5b506103f160195481565b3480156106b9575f5ffd5b506103f160135481565b3480156106ce575f5ffd5b5060045461028e906001600160a01b031681565b3480156106ed575f5ffd5b5061025a6106fc366004611cde565b600e6020525f908152604090205460ff1681565b34801561071b575f5ffd5b5061074f61072a366004611df6565b602260209081525f92835260408084209091529082529020546001600160401b031681565b6040516001600160401b039091168152602001610266565b348015610772575f5ffd5b5060075461028e906001600160a01b031681565b348015610791575f5ffd5b5060035461028e906001600160a01b031681565b3480156107b0575f5ffd5b506103f160025481565b3480156107c5575f5ffd5b506103f16107d4366004611cde565b601d6020525f908152604090205481565b3480156107f0575f5ffd5b506103f16107ff366004611cde565b60156020525f908152604090205481565b34801561081b575f5ffd5b506103f1601b5481565b6104b6610833366004611cde565b610d56565b348015610843575f5ffd5b506103f1610852366004611bc2565b601e6020525f908152604090205481565b34801561086e575f5ffd5b5061088261087d366004611e67565b610d7f565b60408051938452602084019290925290820152606001610266565b600a602052815f5260405f2081815481106108b6575f80fd5b5f918252602091829020600a9091020180546001820154600283015460038401546004850154600586015460068701546040805160608101825260078a0154815260088a01546001600160a01b039081169b82019b909b526009909901548a16908901529599509387169750958216956001600160401b03600160a01b9093048316958284169563ffffffff600160401b85041695600160601b90940490941693919291908b565b5f6109693083610cd2565b92915050565b6006545f906001600160a01b031680610989575f91505090565b610993813061106f565b91505090565b6109a16110fe565b6109aa5f611118565b565b601c602052815f5260405f2081815481106109c5575f80fd5b5f9182526020909120600590910201805460018201546002830154600384015460049094015492955090935091906001600160a01b031685565b600f60f81b6060805f808083610a4660408051808201825260068152652ab73a3937b760d11b602080830191909152825180840190935260018352603160f81b9083015291565b97989097965046955030945091925090565b600354604051733d602d80600a3d3981f3363d3d373d3d3d363d7360601b60208201526bffffffffffffffffffffffff19606092831b1660348201526e5af43d82803e903d91602b57fd5bf360881b6048820152605701604051602081830303815290604052905090565b5f5f610acd611142565b5f85815260208052604090205460ff1615610afb57604051631beb710560e11b815260040160405180910390fd5b5f858152602160205260409020546001600160a01b031615610b305760405163cc0bb0c160e01b815260040160405180910390fd5b825f03610b5057604051631f2a200560e01b815260040160405180910390fd5b5f610b5a85611165565b9050610b6681856111d5565b9150815f03610b8857604051633f41ae0160e01b815260040160405180910390fd5b6007810180546008830154610ba691906001600160a01b0316611243565b335f9081526014602052604090205483811015610bd65760405163acf2da9360e01b815260040160405180910390fd5b610be08482611f56565b335f90815260146020526040812091909155600183015483546002850154610c18926001600160a01b039081169289928d921661133b565b80925081975050506040518060a00160405280336001600160a01b031681526020018981526020018881526020018781526020018281525060215f8b81526020019081526020015f205f820151815f015f6101000a8154816001600160a01b0302191690836001600160a01b0316021790555060208201518160010155604082015181600201556060820151816003015560808201518160040155905050610cc688828589898e338e611457565b50505050935093915050565b6003545f90600160a01b900460f81b8383610ceb610a58565b8051602091820120604051610d3795949392016001600160f81b031994909416845260609290921b6bffffffffffffffffffffffff191660018401526015830152603582015260550190565b60408051601f1981840301815291905280516020909101209392505050565b610d5e6110fe565b8060601b610d7357637448fbae5f526004601cfd5b610d7c81611118565b50565b5f5f5f610d8a611142565b600554604051630939b3f360e31b81525f916001600160a01b0316906349cd9f9890610dc4908d908d908d908d908d908d90600401611fc1565b5f60405180830381865afa158015610dde573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610e059190810190612169565b80515f8181526020805260409020549192509060ff1615610e3957604051631beb710560e11b815260040160405180910390fd5b600160205f8381526020019081526020015f205f6101000a81548160ff0219169083151502179055505f60075f9054906101000a90046001600160a01b031690505f610e948e856080015186606001518760a00151866114e8565b9050610ea58e8386604001516115a2565b610eb98e856040015163ffffffff16611609565b9550855f03610edb57604051632503bb3360e11b815260040160405180910390fd5b5f610ee587611165565b905081816004015f828254610efa9190612224565b9250508190555081816006015f828254610f149190612224565b90915550610f24905081836111d5565b9550610f3082876116bd565b5f84815260216020526040902080546001600160a01b03161561100757805460018201546001600160a01b03909116905f908a148015610f735750848360020154145b6003808501545f8a815260216020526040812080546001600160a01b031916815560018101829055600281018290559283018190556004909201919091559091508115610fed578915610fed576001600160a01b0383165f90815260146020526040812080548c9290610fe7908490612224565b90915550505b8115611003579a50611062975050505050505050565b5050505b861561105b576008820154600783018054600985015491925f9261103c926001600160a01b03928316928d928f92911661133b565b809250819c5050506110588a82848e8d8c8e604001518c6116d8565b50505b5050505050505b9750975097945050505050565b5f6001600160a01b03831661108f57506001600160a01b03811631610969565b6040516370a0823160e01b81526001600160a01b0383811660048301528416906370a0823190602401602060405180830381865afa1580156110d3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110f79190612237565b9392505050565b638b78c6d8195433146109aa576382b429005f526004601cfd5b638b78c6d819546001600160a01b03909116638b78c6d8198190559061113e818361175d565b5050565b5f5460ff16156109aa5760405163d93c066560e01b815260040160405180910390fd5b5f818152600b602052604081206001810154808303611196576040516290ed3d60e61b815260040160405180910390fd5b81545f908152600a602052604090206111b0600183611f56565b815481106111c0576111c061224e565b905f5260205f2090600a020192505050919050565b60038201545f90600160401b900463ffffffff1681620f42406111f88382611f56565b6112029086612262565b61120c9190612279565b6003860154909150600160601b90046001600160401b03168082111561123657808203935061123a565b5f93505b50505092915050565b5f8281526017602052604090205460ff16156112725760405163d9259ca960e01b815260040160405180910390fd5b6001600160a01b03811661129957604051638562eb4560e01b815260040160405180910390fd5b6006546001600160a01b0382811691161480159046841415906112ed576001600160a01b0383165f9081526015602052604081205490036112ed5760405163047e3fe760e11b815260040160405180910390fd5b8015611335576001600160a01b038381165f908152601660209081526040808320888452909152902054166113355760405163b37c79ed60e01b815260040160405180910390fd5b50505050565b5f838152601e60205260408120805482918261135683612298565b909155506001600160a01b038881165f908152601c60209081526040808320815160a0810183528681528084018d81529281018c8152606082018c81528b881660808401908152845460018082018755868a529790982093516005909802909301968755935186860155516002860155915160038501559051600490930180546001600160a01b03191693909416929092179092558054929350916113fb9190611f56565b6040805180820182526001600160a01b03808c16825260208083018581525f8c8152601f83528581208982529092529390209151825491166001600160a01b031990911617815590516001909101559250509550959350505050565b604080516101a0810182528981526020810189905260018801546001600160a01b03908116928201929092526060810187905260808101869052875460a0820152600288015490911660c08201526114de9060e081015f8152602081018690526001600160a01b03851660408201525f60608201819052608082015260a0018390526117fd565b5050505050505050565b5f6114f2826118b5565b6affffffffffffffffffffff1986811691161461152257604051633395765b60e01b815260040160405180910390fd5b6004545f9061153a906001600160a01b031688610cd2565b90505f611546826118b5565b90505f5f61155487896118ca565b9250925050826affffffffffffffffffffff1916826affffffffffffffffffffff191614611595576040516340650ea960e11b815260040160405180910390fd5b9998505050505050505050565b5f8381526022602090815260408083206001600160a01b03861684529091529020546001600160401b031680158015906115eb57506001600160401b03811663ffffffff831611155b1561133557604051633e6c071160e21b815260040160405180910390fd5b5f828152600c602052604081208054808303611629575f92505050610969565b805b801561123a575f868152600a6020526040812080545f1990930192839081106116565761165661224e565b905f5260205f2090600a02019050856001600160401b03168160020160149054906101000a90046001600160401b03166001600160401b0316116116b7578382815481106116a6576116a661224e565b905f5260205f20015494505061123a565b5061162b565b61113e6116d26116cd8385611f56565b611978565b5f6119a5565b604080516101a081018252898152602081018990526001808901546001600160a01b03908116938301939093526060820188905260808201879052885460a0830152600289015490921660c08201526114de9160e08201908152602081018690525f60408201819052606082015263ffffffff8516608082015260a0018390526117fd565b806001600160a01b0316826001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a361113e7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e083836040516020016117e99291906001600160a01b0392831681529116602082015260400190565b6040516020818303038152906040526119d4565b602081015181516040808401516060850151608086015160a087015160c088015160e08901516101008a01516101208b01516101408c01516101608d01516101808e01519a517f77242fbd573af5a5f3518da92600e96795ebfff993606b4fb54dea2dcd2dfe859b6118789b9a9998979695949392916122d8565b60405180910390a3610d7c7f77242fbd573af5a5f3518da92600e96795ebfff993606b4fb54dea2dcd2dfe85826040516020016117e99190612353565b6001600160a01b0316604160a01b1760581b90565b5f5f5f6004855110156118f057604051631279950360e01b815260040160405180910390fd5b5f6118fa86612442565b90506356fa634560e01b6001600160e01b031982160161192a5761191d86611aa2565b8695509093509150611970565b63dc478d2360e01b6001600160e01b03198216016119575761194b86611aea565b91955093509150611970565b604051630915b87b60e21b815260040160405180910390fd5b509250925092565b5f6001600160ff1b038211156119a157604051630599f71d60e21b815260040160405180910390fd5b5090565b815f036119b0575050565b8160135f8282546119c19190612480565b909155505060135461113e908383611b49565b60028054600190810180835590546040519092916119ff9184919043904290899089906020016124be565b60408051601f1981840301815290829052611a19916124f3565b602060405180830381855afa158015611a34573d5f5f3e3d5ffd5b5050506040513d601f19601f82011682018060405250810190611a579190612237565b600181905550600154816002547f78160f0b1b2b32b52a0076d8f0f70888687ba702a4d993d55ac8d9327d57a1278686604051611a959291906124fe565b60405180910390a4505050565b80515f90819060448114611ac957604051639ac60c2d60e01b815260040160405180910390fd5b6024840151604485015181611add816118b5565b9791965090945050505050565b5f5f5f5f8451905080606414611b1357604051639ac60c2d60e01b815260040160405180910390fd5b6024850151604486015160648701518282611b2d826118b5565b9850611b38816118b5565b989a98995091979650505050505050565b7f43991e1e1cfb2eed6c9dc37a7a848622f8e3f75bc38d532a00eaf026cd87a014838383604051611b7c9392919061251e565b60405180910390a1611bbd7f43991e1e1cfb2eed6c9dc37a7a848622f8e3f75bc38d532a00eaf026cd87a0148484846040516020016117e99392919061251e565b505050565b5f60208284031215611bd2575f5ffd5b5035919050565b80356001600160a01b0381168114611bef575f5ffd5b919050565b5f5f60408385031215611c05575f5ffd5b611c0e83611bd9565b946020939093013593505050565b5f5f60408385031215611c2d575f5ffd5b50508035926020909101359150565b8b81526001600160a01b038b811660208301528a1660408201526001600160401b038981166060830152888116608083015263ffffffff881660a0830152861660c082015260e08101859052610100810184905261012081018390526101a08101611cce610140830184805182526020808201516001600160a01b039081169184019190915260409182015116910152565b9c9b505050505050505050505050565b5f60208284031215611cee575f5ffd5b6110f782611bd9565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b60ff60f81b8816815260e060208201525f611d4360e0830189611cf7565b8281036040840152611d558189611cf7565b606084018890526001600160a01b038716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b81811015611daa578351835260209384019390920191600101611d8c565b50909b9a5050505050505050505050565b602081525f6110f76020830184611cf7565b5f5f5f60608486031215611ddf575f5ffd5b505081359360208301359350604090920135919050565b5f5f60408385031215611e07575f5ffd5b82359150611e1760208401611bd9565b90509250929050565b5f5f83601f840112611e30575f5ffd5b5081356001600160401b03811115611e46575f5ffd5b6020830191508360208260051b8501011115611e60575f5ffd5b9250929050565b5f5f5f5f5f5f5f60a0888a031215611e7d575f5ffd5b8735965060208801356001600160401b03811115611e99575f5ffd5b880161028081018a1015611eab575f5ffd5b955060408801356001600160401b03811115611ec5575f5ffd5b8801601f81018a13611ed5575f5ffd5b80356001600160401b03811115611eea575f5ffd5b8a6020828401011115611efb575f5ffd5b6020919091019550935060608801356001600160401b03811115611f1d575f5ffd5b611f298a828b01611e20565b989b979a50959894979596608090950135949350505050565b634e487b7160e01b5f52601160045260245ffd5b8181038181111561096957610969611f42565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b8183525f6001600160fb1b03831115611fa8575f5ffd5b8260051b80836020870137939093016020019392505050565b60808082525f906103008301908301898336829003601e19015b601482101561204957868503607f190184528235818112611ffa575f5ffd5b8d016020810190356001600160401b03811115612015575f5ffd5b803603821315612023575f5ffd5b61202e878284611f69565b96505050602083019250602084019350600182019150611fdb565b50505050828103602084015261206081888a611f69565b90508281036040840152612075818688611f91565b915050826060830152979650505050505050565b634e487b7160e01b5f52604160045260245ffd5b60405160c081016001600160401b03811182821017156120bf576120bf612089565b60405290565b80516affffffffffffffffffffff1981168114611bef575f5ffd5b5f82601f8301126120ef575f5ffd5b81516001600160401b0381111561210857612108612089565b604051601f8201601f19908116603f011681016001600160401b038111828210171561213657612136612089565b60405281815283820160200185101561214d575f5ffd5b8160208501602083015e5f918101602001919091529392505050565b5f60208284031215612179575f5ffd5b81516001600160401b0381111561218e575f5ffd5b820160c0818503121561219f575f5ffd5b6121a761209d565b8151815260208083015190820152604082015163ffffffff811681146121cb575f5ffd5b60408201526121dc606083016120c5565b60608201526121ed608083016120c5565b608082015260a08201516001600160401b0381111561220a575f5ffd5b612216868285016120e0565b60a083015250949350505050565b8082018082111561096957610969611f42565b5f60208284031215612247575f5ffd5b5051919050565b634e487b7160e01b5f52603260045260245ffd5b808202811582820484141761096957610969611f42565b5f8261229357634e487b7160e01b5f52601260045260245ffd5b500490565b5f600182016122a9576122a9611f42565b5060010190565b634e487b7160e01b5f52602160045260245ffd5b600381106122d4576122d46122b0565b9052565b6001600160a01b038c81168252602082018c9052604082018b9052606082018a905288166080820152610160810161231360a08301896122c4565b60c08201969096526001600160a01b0394851660e0820152929093166101008301526001600160401b031661012082015261014001529695505050505050565b5f6101a0820190508251825260208301516020830152604083015161238360408401826001600160a01b03169052565b50606083015160608301526080830151608083015260a083015160a083015260c08301516123bc60c08401826001600160a01b03169052565b5060e08301516123cf60e08401826122c4565b506101008301516101008301526101208301516123f86101208401826001600160a01b03169052565b506101408301516124156101408401826001600160a01b03169052565b506101608301516124326101608401826001600160401b03169052565b5061018092830151919092015290565b805160208201516001600160e01b0319811691906004821015612479576001600160e01b0319600483900360031b81901b82161692505b5050919050565b8082018281125f83128015821682158216171561249f5761249f611f42565b505092915050565b5f81518060208401855e5f93019283525090919050565b8681528560208201528460408201528360608201528260808201525f6124e760a08301846124a7565b98975050505050505050565b5f6110f782846124a7565b828152604060208201525f6125166040830184611cf7565b949350505050565b83815260208101839052606081016006831061253c5761253c6122b0565b82604083015294935050505056fea164736f6c634300081b000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x02(W_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\x01)W\x80c\xC6;\xBF)\x11a\0\xA8W\x80c\xF0N\x02\xC0\x11a\0mW\x80c\xF0N\x02\xC0\x14a\x07\xE5W\x80c\xF1'\xA9\xB3\x14a\x08\x10W\x80c\xF2\xFD\xE3\x8B\x14a\x08%W\x80c\xF5\x16\xA5\xB4\x14a\x088W\x80c\xF9\xFD\xCA\xE6\x14a\x08cW__\xFD[\x80c\xC6;\xBF)\x14a\x07\x10W\x80c\xDC\x8F\x863\x14a\x07gW\x80c\xDE@\xD8\x9F\x14a\x07\x86W\x80c\xE2M\\5\x14a\x07\xA5W\x80c\xEE\xB9\x02Y\x14a\x07\xBAW__\xFD[\x80c\xAA\x946\x0C\x11a\0\xEEW\x80c\xAA\x946\x0C\x14a\x06zW\x80c\xB3q\xFAi\x14a\x06\x99W\x80c\xB7\xED\x02\x0E\x14a\x06\xAEW\x80c\xB9\x8Ec\x1D\x14a\x06\xC3W\x80c\xBC\\YP\x14a\x06\xE2W__\xFD[\x80c\x8D\xA5\xCB[\x14a\x05\xE3W\x80c\x90\"8\xE1\x14a\x05\xFBW\x80c\x9E\xFA\xCAy\x14a\x06\x10W\x80c\xA60%Y\x14a\x061W\x80c\xA7\xEC\x9D\xF9\x14a\x06FW__\xFD[\x80cM\xA2\xF8\x99\x11a\x01\xB5W\x80cq\x8F\xBC%\x11a\x01zW\x80cq\x8F\xBC%\x14a\x04\xB8W\x80cx\xAA\xF2^\x14a\x05 W\x80c\x80\xA7,\x8B\x14a\x05pW\x80c\x84\xB0\x19n\x14a\x05\x8FW\x80c\x88\x92r\x96\x14a\x05\xB6W__\xFD[\x80cM\xA2\xF8\x99\x14a\x04\x14W\x80c\\\x97Z\xBB\x14a\x04?W\x80c`\xB6\xBF\xDD\x14a\x04UW\x80cl\x83Z\x82\x14a\x04\x83W\x80cqP\x18\xA6\x14a\x04\xAEW__\xFD[\x80c/\x83\xD9\xAF\x11a\x01\xFBW\x80c/\x83\xD9\xAF\x14a\x03\x04W\x80c=\x92\xAF\x84\x14a\x03:W\x80c?\xEA4\x88\x14a\x03\xBEW\x80cH.\xDB\x07\x14a\x03\xDDW\x80cMS\xE91\x14a\x03\xFFW__\xFD[\x80c\x04\xECB\x94\x14a\x02,W\x80c\x0B4Xy\x14a\x02oW\x80c\x1D\xBFLa\x14a\x02\xA6W\x80c/H\xAB}\x14a\x02\xE5W[__\xFD[4\x80\x15a\x027W__\xFD[Pa\x02Za\x02F6`\x04a\x1B\xC2V[`\x17` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02zW__\xFD[P`\x08Ta\x02\x8E\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02fV[4\x80\x15a\x02\xB1W__\xFD[Pa\x02\x8Ea\x02\xC06`\x04a\x1B\xF4V[`\x16` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02\xF0W__\xFD[P`\x06Ta\x02\x8E\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x03\x0FW__\xFD[Pa\x03#a\x03\x1E6`\x04a\x1C\x1CV[a\x08\x9DV[`@Qa\x02f\x9B\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x1C<V[4\x80\x15a\x03EW__\xFD[Pa\x03\x8Ca\x03T6`\x04a\x1B\xC2V[`!` R_\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x93\x91\x92\x90\x91\x90\x85V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\x02fV[4\x80\x15a\x03\xC9W__\xFD[Pa\x02\x8Ea\x03\xD86`\x04a\x1B\xC2V[a\t^V[4\x80\x15a\x03\xE8W__\xFD[Pa\x03\xF1a\toV[`@Q\x90\x81R` \x01a\x02fV[4\x80\x15a\x04\nW__\xFD[Pa\x03\xF1`\x01T\x81V[4\x80\x15a\x04\x1FW__\xFD[Pa\x03\xF1a\x04.6`\x04a\x1C\xDEV[`\x14` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x04JW__\xFD[P_T`\xFF\x16a\x02ZV[4\x80\x15a\x04`W__\xFD[Pa\x02Za\x04o6`\x04a\x1C\xDEV[`\r` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x04\x8EW__\xFD[Pa\x03\xF1a\x04\x9D6`\x04a\x1B\xC2V[`#` R_\x90\x81R`@\x90 T\x81V[a\x04\xB6a\t\x99V[\0[4\x80\x15a\x04\xC3W__\xFD[Pa\x05\x01a\x04\xD26`\x04a\x1C\x1CV[`\x1F` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80T`\x01\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x82V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x02fV[4\x80\x15a\x05+W__\xFD[Pa\x05?a\x05:6`\x04a\x1B\xF4V[a\t\xACV[`@\x80Q\x95\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16`\x80\x82\x01R`\xA0\x01a\x02fV[4\x80\x15a\x05{W__\xFD[P`\x05Ta\x02\x8E\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05\x9AW__\xFD[Pa\x05\xA3a\t\xFFV[`@Qa\x02f\x97\x96\x95\x94\x93\x92\x91\x90a\x1D%V[4\x80\x15a\x05\xC1W__\xFD[Pa\x02Za\x05\xD06`\x04a\x1B\xC2V[` \x80R_\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x05\xEEW__\xFD[Pc\x8Bx\xC6\xD8\x19Ta\x02\x8EV[4\x80\x15a\x06\x06W__\xFD[Pa\x03\xF1`\tT\x81V[4\x80\x15a\x06\x1BW__\xFD[Pa\x06$a\nXV[`@Qa\x02f\x91\x90a\x1D\xBBV[4\x80\x15a\x06<W__\xFD[Pa\x03\xF1`\x18T\x81V[4\x80\x15a\x06QW__\xFD[Pa\x06ea\x06`6`\x04a\x1D\xCDV[a\n\xC3V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02fV[4\x80\x15a\x06\x85W__\xFD[Pa\x02\x8Ea\x06\x946`\x04a\x1B\xF4V[a\x0C\xD2V[4\x80\x15a\x06\xA4W__\xFD[Pa\x03\xF1`\x19T\x81V[4\x80\x15a\x06\xB9W__\xFD[Pa\x03\xF1`\x13T\x81V[4\x80\x15a\x06\xCEW__\xFD[P`\x04Ta\x02\x8E\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06\xEDW__\xFD[Pa\x02Za\x06\xFC6`\x04a\x1C\xDEV[`\x0E` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x07\x1BW__\xFD[Pa\x07Oa\x07*6`\x04a\x1D\xF6V[`\"` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02fV[4\x80\x15a\x07rW__\xFD[P`\x07Ta\x02\x8E\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07\x91W__\xFD[P`\x03Ta\x02\x8E\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x07\xB0W__\xFD[Pa\x03\xF1`\x02T\x81V[4\x80\x15a\x07\xC5W__\xFD[Pa\x03\xF1a\x07\xD46`\x04a\x1C\xDEV[`\x1D` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x07\xF0W__\xFD[Pa\x03\xF1a\x07\xFF6`\x04a\x1C\xDEV[`\x15` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x08\x1BW__\xFD[Pa\x03\xF1`\x1BT\x81V[a\x04\xB6a\x0836`\x04a\x1C\xDEV[a\rVV[4\x80\x15a\x08CW__\xFD[Pa\x03\xF1a\x08R6`\x04a\x1B\xC2V[`\x1E` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x08nW__\xFD[Pa\x08\x82a\x08}6`\x04a\x1EgV[a\r\x7FV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02fV[`\n` R\x81_R`@_ \x81\x81T\x81\x10a\x08\xB6W_\x80\xFD[_\x91\x82R` \x91\x82\x90 `\n\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x87\x01T`@\x80Q``\x81\x01\x82R`\x07\x8A\x01T\x81R`\x08\x8A\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x9B\x82\x01\x9B\x90\x9BR`\t\x90\x99\x01T\x8A\x16\x90\x89\x01R\x95\x99P\x93\x87\x16\x97P\x95\x82\x16\x95`\x01`\x01`@\x1B\x03`\x01`\xA0\x1B\x90\x93\x04\x83\x16\x95\x82\x84\x16\x95c\xFF\xFF\xFF\xFF`\x01`@\x1B\x85\x04\x16\x95`\x01``\x1B\x90\x94\x04\x90\x94\x16\x93\x91\x92\x91\x90\x8BV[_a\ti0\x83a\x0C\xD2V[\x92\x91PPV[`\x06T_\x90`\x01`\x01`\xA0\x1B\x03\x16\x80a\t\x89W_\x91PP\x90V[a\t\x93\x810a\x10oV[\x91PP\x90V[a\t\xA1a\x10\xFEV[a\t\xAA_a\x11\x18V[V[`\x1C` R\x81_R`@_ \x81\x81T\x81\x10a\t\xC5W_\x80\xFD[_\x91\x82R` \x90\x91 `\x05\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01T\x92\x95P\x90\x93P\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x85V[`\x0F`\xF8\x1B``\x80_\x80\x80\x83a\nF`@\x80Q\x80\x82\x01\x82R`\x06\x81Re*\xB7:97\xB7`\xD1\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x01\x83R`1`\xF8\x1B\x90\x83\x01R\x91V[\x97\x98\x90\x97\x96PF\x95P0\x94P\x91\x92P\x90V[`\x03T`@Qs=`-\x80`\n=9\x81\xF36==7===6=s``\x1B` \x82\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x92\x83\x1B\x16`4\x82\x01RnZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3`\x88\x1B`H\x82\x01R`W\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[__a\n\xCDa\x11BV[_\x85\x81R` \x80R`@\x90 T`\xFF\x16\x15a\n\xFBW`@Qc\x1B\xEBq\x05`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x85\x81R`!` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0B0W`@Qc\xCC\x0B\xB0\xC1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82_\x03a\x0BPW`@Qc\x1F* \x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0BZ\x85a\x11eV[\x90Pa\x0Bf\x81\x85a\x11\xD5V[\x91P\x81_\x03a\x0B\x88W`@Qc?A\xAE\x01`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07\x81\x01\x80T`\x08\x83\x01Ta\x0B\xA6\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a\x12CV[3_\x90\x81R`\x14` R`@\x90 T\x83\x81\x10\x15a\x0B\xD6W`@Qc\xAC\xF2\xDA\x93`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\xE0\x84\x82a\x1FVV[3_\x90\x81R`\x14` R`@\x81 \x91\x90\x91U`\x01\x83\x01T\x83T`\x02\x85\x01Ta\x0C\x18\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x89\x92\x8D\x92\x16a\x13;V[\x80\x92P\x81\x97PPP`@Q\x80`\xA0\x01`@R\x803`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x89\x81R` \x01\x88\x81R` \x01\x87\x81R` \x01\x82\x81RP`!_\x8B\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01U\x90PPa\x0C\xC6\x88\x82\x85\x89\x89\x8E3\x8Ea\x14WV[PPPP\x93P\x93\x91PPV[`\x03T_\x90`\x01`\xA0\x1B\x90\x04`\xF8\x1B\x83\x83a\x0C\xEBa\nXV[\x80Q` \x91\x82\x01 `@Qa\r7\x95\x94\x93\x92\x01`\x01`\x01`\xF8\x1B\x03\x19\x94\x90\x94\x16\x84R``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x84\x01R`\x15\x83\x01R`5\x82\x01R`U\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[a\r^a\x10\xFEV[\x80``\x1Ba\rsWctH\xFB\xAE_R`\x04`\x1C\xFD[a\r|\x81a\x11\x18V[PV[___a\r\x8Aa\x11BV[`\x05T`@Qc\t9\xB3\xF3`\xE3\x1B\x81R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cI\xCD\x9F\x98\x90a\r\xC4\x90\x8D\x90\x8D\x90\x8D\x90\x8D\x90\x8D\x90\x8D\x90`\x04\x01a\x1F\xC1V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xDEW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\x05\x91\x90\x81\x01\x90a!iV[\x80Q_\x81\x81R` \x80R`@\x90 T\x91\x92P\x90`\xFF\x16\x15a\x0E9W`@Qc\x1B\xEBq\x05`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01` _\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_`\x07_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90P_a\x0E\x94\x8E\x85`\x80\x01Q\x86``\x01Q\x87`\xA0\x01Q\x86a\x14\xE8V[\x90Pa\x0E\xA5\x8E\x83\x86`@\x01Qa\x15\xA2V[a\x0E\xB9\x8E\x85`@\x01Qc\xFF\xFF\xFF\xFF\x16a\x16\tV[\x95P\x85_\x03a\x0E\xDBW`@Qc%\x03\xBB3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0E\xE5\x87a\x11eV[\x90P\x81\x81`\x04\x01_\x82\x82Ta\x0E\xFA\x91\x90a\"$V[\x92PP\x81\x90UP\x81\x81`\x06\x01_\x82\x82Ta\x0F\x14\x91\x90a\"$V[\x90\x91UPa\x0F$\x90P\x81\x83a\x11\xD5V[\x95Pa\x0F0\x82\x87a\x16\xBDV[_\x84\x81R`!` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x10\x07W\x80T`\x01\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90_\x90\x8A\x14\x80\x15a\x0FsWP\x84\x83`\x02\x01T\x14[`\x03\x80\x85\x01T_\x8A\x81R`!` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x81\x01\x82\x90U`\x02\x81\x01\x82\x90U\x92\x83\x01\x81\x90U`\x04\x90\x92\x01\x91\x90\x91U\x90\x91P\x81\x15a\x0F\xEDW\x89\x15a\x0F\xEDW`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x14` R`@\x81 \x80T\x8C\x92\x90a\x0F\xE7\x90\x84\x90a\"$V[\x90\x91UPP[\x81\x15a\x10\x03W\x9APa\x10b\x97PPPPPPPPV[PPP[\x86\x15a\x10[W`\x08\x82\x01T`\x07\x83\x01\x80T`\t\x85\x01T\x91\x92_\x92a\x10<\x92`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92\x8D\x92\x8F\x92\x91\x16a\x13;V[\x80\x92P\x81\x9CPPPa\x10X\x8A\x82\x84\x8E\x8D\x8C\x8E`@\x01Q\x8Ca\x16\xD8V[PP[PPPPPP[\x97P\x97P\x97\x94PPPPPV[_`\x01`\x01`\xA0\x1B\x03\x83\x16a\x10\x8FWP`\x01`\x01`\xA0\x1B\x03\x81\x161a\tiV[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xD3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xF7\x91\x90a\"7V[\x93\x92PPPV[c\x8Bx\xC6\xD8\x19T3\x14a\t\xAAWc\x82\xB4)\0_R`\x04`\x1C\xFD[c\x8Bx\xC6\xD8\x19T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16c\x8Bx\xC6\xD8\x19\x81\x90U\x90a\x11>\x81\x83a\x17]V[PPV[_T`\xFF\x16\x15a\t\xAAW`@Qc\xD9<\x06e`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x0B` R`@\x81 `\x01\x81\x01T\x80\x83\x03a\x11\x96W`@Qb\x90\xED=`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81T_\x90\x81R`\n` R`@\x90 a\x11\xB0`\x01\x83a\x1FVV[\x81T\x81\x10a\x11\xC0Wa\x11\xC0a\"NV[\x90_R` _ \x90`\n\x02\x01\x92PPP\x91\x90PV[`\x03\x82\x01T_\x90`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81b\x0FB@a\x11\xF8\x83\x82a\x1FVV[a\x12\x02\x90\x86a\"bV[a\x12\x0C\x91\x90a\"yV[`\x03\x86\x01T\x90\x91P`\x01``\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x80\x82\x11\x15a\x126W\x80\x82\x03\x93Pa\x12:V[_\x93P[PPP\x92\x91PPV[_\x82\x81R`\x17` R`@\x90 T`\xFF\x16\x15a\x12rW`@Qc\xD9%\x9C\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x12\x99W`@Qc\x85b\xEBE`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06T`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14\x80\x15\x90F\x84\x14\x15\x90a\x12\xEDW`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x15` R`@\x81 T\x90\x03a\x12\xEDW`@Qc\x04~?\xE7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x135W`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x16` \x90\x81R`@\x80\x83 \x88\x84R\x90\x91R\x90 T\x16a\x135W`@Qc\xB3|y\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[_\x83\x81R`\x1E` R`@\x81 \x80T\x82\x91\x82a\x13V\x83a\"\x98V[\x90\x91UP`\x01`\x01`\xA0\x1B\x03\x88\x81\x16_\x90\x81R`\x1C` \x90\x81R`@\x80\x83 \x81Q`\xA0\x81\x01\x83R\x86\x81R\x80\x84\x01\x8D\x81R\x92\x81\x01\x8C\x81R``\x82\x01\x8C\x81R\x8B\x88\x16`\x80\x84\x01\x90\x81R\x84T`\x01\x80\x82\x01\x87U\x86\x8AR\x97\x90\x98 \x93Q`\x05\x90\x98\x02\x90\x93\x01\x96\x87U\x93Q\x86\x86\x01UQ`\x02\x86\x01U\x91Q`\x03\x85\x01U\x90Q`\x04\x90\x93\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x93\x90\x94\x16\x92\x90\x92\x17\x90\x92U\x80T\x92\x93P\x91a\x13\xFB\x91\x90a\x1FVV[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x8C\x16\x82R` \x80\x83\x01\x85\x81R_\x8C\x81R`\x1F\x83R\x85\x81 \x89\x82R\x90\x92R\x93\x90 \x91Q\x82T\x91\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x17\x81U\x90Q`\x01\x90\x91\x01U\x92PP\x95P\x95\x93PPPPV[`@\x80Qa\x01\xA0\x81\x01\x82R\x89\x81R` \x81\x01\x89\x90R`\x01\x88\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R\x87T`\xA0\x82\x01R`\x02\x88\x01T\x90\x91\x16`\xC0\x82\x01Ra\x14\xDE\x90`\xE0\x81\x01_\x81R` \x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x85\x16`@\x82\x01R_``\x82\x01\x81\x90R`\x80\x82\x01R`\xA0\x01\x83\x90Ra\x17\xFDV[PPPPPPPPV[_a\x14\xF2\x82a\x18\xB5V[j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x86\x81\x16\x91\x16\x14a\x15\"W`@Qc3\x95v[`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04T_\x90a\x15:\x90`\x01`\x01`\xA0\x1B\x03\x16\x88a\x0C\xD2V[\x90P_a\x15F\x82a\x18\xB5V[\x90P__a\x15T\x87\x89a\x18\xCAV[\x92P\x92PP\x82j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14a\x15\x95W`@Qc@e\x0E\xA9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x99\x98PPPPPPPPPV[_\x83\x81R`\"` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 T`\x01`\x01`@\x1B\x03\x16\x80\x15\x80\x15\x90a\x15\xEBWP`\x01`\x01`@\x1B\x03\x81\x16c\xFF\xFF\xFF\xFF\x83\x16\x11\x15[\x15a\x135W`@Qc>l\x07\x11`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82\x81R`\x0C` R`@\x81 \x80T\x80\x83\x03a\x16)W_\x92PPPa\tiV[\x80[\x80\x15a\x12:W_\x86\x81R`\n` R`@\x81 \x80T_\x19\x90\x93\x01\x92\x83\x90\x81\x10a\x16VWa\x16Va\"NV[\x90_R` _ \x90`\n\x02\x01\x90P\x85`\x01`\x01`@\x1B\x03\x16\x81`\x02\x01`\x14\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16\x11a\x16\xB7W\x83\x82\x81T\x81\x10a\x16\xA6Wa\x16\xA6a\"NV[\x90_R` _ \x01T\x94PPa\x12:V[Pa\x16+V[a\x11>a\x16\xD2a\x16\xCD\x83\x85a\x1FVV[a\x19xV[_a\x19\xA5V[`@\x80Qa\x01\xA0\x81\x01\x82R\x89\x81R` \x81\x01\x89\x90R`\x01\x80\x89\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x93\x83\x01\x93\x90\x93R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R\x88T`\xA0\x83\x01R`\x02\x89\x01T\x90\x92\x16`\xC0\x82\x01Ra\x14\xDE\x91`\xE0\x82\x01\x90\x81R` \x81\x01\x86\x90R_`@\x82\x01\x81\x90R``\x82\x01Rc\xFF\xFF\xFF\xFF\x85\x16`\x80\x82\x01R`\xA0\x01\x83\x90Ra\x17\xFDV[\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3a\x11>\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x83\x83`@Q` \x01a\x17\xE9\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x19\xD4V[` \x81\x01Q\x81Q`@\x80\x84\x01Q``\x85\x01Q`\x80\x86\x01Q`\xA0\x87\x01Q`\xC0\x88\x01Q`\xE0\x89\x01Qa\x01\0\x8A\x01Qa\x01 \x8B\x01Qa\x01@\x8C\x01Qa\x01`\x8D\x01Qa\x01\x80\x8E\x01Q\x9AQ\x7Fw$/\xBDW:\xF5\xA5\xF3Q\x8D\xA9&\0\xE9g\x95\xEB\xFF\xF9\x93`kO\xB5M\xEA-\xCD-\xFE\x85\x9Ba\x18x\x9B\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91a\"\xD8V[`@Q\x80\x91\x03\x90\xA3a\r|\x7Fw$/\xBDW:\xF5\xA5\xF3Q\x8D\xA9&\0\xE9g\x95\xEB\xFF\xF9\x93`kO\xB5M\xEA-\xCD-\xFE\x85\x82`@Q` \x01a\x17\xE9\x91\x90a#SV[`\x01`\x01`\xA0\x1B\x03\x16`A`\xA0\x1B\x17`X\x1B\x90V[___`\x04\x85Q\x10\x15a\x18\xF0W`@Qc\x12y\x95\x03`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x18\xFA\x86a$BV[\x90PcV\xFAcE`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\x19*Wa\x19\x1D\x86a\x1A\xA2V[\x86\x95P\x90\x93P\x91Pa\x19pV[c\xDCG\x8D#`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\x19WWa\x19K\x86a\x1A\xEAV[\x91\x95P\x93P\x91Pa\x19pV[`@Qc\t\x15\xB8{`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x92P\x92P\x92V[_`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\x19\xA1W`@Qc\x05\x99\xF7\x1D`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x90V[\x81_\x03a\x19\xB0WPPV[\x81`\x13_\x82\x82Ta\x19\xC1\x91\x90a$\x80V[\x90\x91UPP`\x13Ta\x11>\x90\x83\x83a\x1BIV[`\x02\x80T`\x01\x90\x81\x01\x80\x83U\x90T`@Q\x90\x92\x91a\x19\xFF\x91\x84\x91\x90C\x90B\x90\x89\x90\x89\x90` \x01a$\xBEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1A\x19\x91a$\xF3V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x1A4W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1AW\x91\x90a\"7V[`\x01\x81\x90UP`\x01T\x81`\x02T\x7Fx\x16\x0F\x0B\x1B+2\xB5*\0v\xD8\xF0\xF7\x08\x88h{\xA7\x02\xA4\xD9\x93\xD5Z\xC8\xD92}W\xA1'\x86\x86`@Qa\x1A\x95\x92\x91\x90a$\xFEV[`@Q\x80\x91\x03\x90\xA4PPPV[\x80Q_\x90\x81\x90`D\x81\x14a\x1A\xC9W`@Qc\x9A\xC6\x0C-`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`$\x84\x01Q`D\x85\x01Q\x81a\x1A\xDD\x81a\x18\xB5V[\x97\x91\x96P\x90\x94PPPPPV[____\x84Q\x90P\x80`d\x14a\x1B\x13W`@Qc\x9A\xC6\x0C-`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`$\x85\x01Q`D\x86\x01Q`d\x87\x01Q\x82\x82a\x1B-\x82a\x18\xB5V[\x98Pa\x1B8\x81a\x18\xB5V[\x98\x9A\x98\x99P\x91\x97\x96PPPPPPPV[\x7FC\x99\x1E\x1E\x1C\xFB.\xEDl\x9D\xC3zz\x84\x86\"\xF8\xE3\xF7[\xC3\x8DS*\0\xEA\xF0&\xCD\x87\xA0\x14\x83\x83\x83`@Qa\x1B|\x93\x92\x91\x90a%\x1EV[`@Q\x80\x91\x03\x90\xA1a\x1B\xBD\x7FC\x99\x1E\x1E\x1C\xFB.\xEDl\x9D\xC3zz\x84\x86\"\xF8\xE3\xF7[\xC3\x8DS*\0\xEA\xF0&\xCD\x87\xA0\x14\x84\x84\x84`@Q` \x01a\x17\xE9\x93\x92\x91\x90a%\x1EV[PPPV[_` \x82\x84\x03\x12\x15a\x1B\xD2W__\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B\xEFW__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x1C\x05W__\xFD[a\x1C\x0E\x83a\x1B\xD9V[\x94` \x93\x90\x93\x015\x93PPPV[__`@\x83\x85\x03\x12\x15a\x1C-W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x8B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16` \x83\x01R\x8A\x16`@\x82\x01R`\x01`\x01`@\x1B\x03\x89\x81\x16``\x83\x01R\x88\x81\x16`\x80\x83\x01Rc\xFF\xFF\xFF\xFF\x88\x16`\xA0\x83\x01R\x86\x16`\xC0\x82\x01R`\xE0\x81\x01\x85\x90Ra\x01\0\x81\x01\x84\x90Ra\x01 \x81\x01\x83\x90Ra\x01\xA0\x81\x01a\x1C\xCEa\x01@\x83\x01\x84\x80Q\x82R` \x80\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x84\x01\x91\x90\x91R`@\x91\x82\x01Q\x16\x91\x01RV[\x9C\x9BPPPPPPPPPPPPV[_` \x82\x84\x03\x12\x15a\x1C\xEEW__\xFD[a\x10\xF7\x82a\x1B\xD9V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R_a\x1DC`\xE0\x83\x01\x89a\x1C\xF7V[\x82\x81\x03`@\x84\x01Ra\x1DU\x81\x89a\x1C\xF7V[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15a\x1D\xAAW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1D\x8CV[P\x90\x9B\x9APPPPPPPPPPPV[` \x81R_a\x10\xF7` \x83\x01\x84a\x1C\xF7V[___``\x84\x86\x03\x12\x15a\x1D\xDFW__\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x1E\x07W__\xFD[\x825\x91Pa\x1E\x17` \x84\x01a\x1B\xD9V[\x90P\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a\x1E0W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1EFW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1E`W__\xFD[\x92P\x92\x90PV[_______`\xA0\x88\x8A\x03\x12\x15a\x1E}W__\xFD[\x875\x96P` \x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\x99W__\xFD[\x88\x01a\x02\x80\x81\x01\x8A\x10\x15a\x1E\xABW__\xFD[\x95P`@\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xC5W__\xFD[\x88\x01`\x1F\x81\x01\x8A\x13a\x1E\xD5W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xEAW__\xFD[\x8A` \x82\x84\x01\x01\x11\x15a\x1E\xFBW__\xFD[` \x91\x90\x91\x01\x95P\x93P``\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\x1DW__\xFD[a\x1F)\x8A\x82\x8B\x01a\x1E V[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x95\x96`\x80\x90\x95\x015\x94\x93PPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\tiWa\tia\x1FBV[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x81\x83R_`\x01`\x01`\xFB\x1B\x03\x83\x11\x15a\x1F\xA8W__\xFD[\x82`\x05\x1B\x80\x83` \x87\x017\x93\x90\x93\x01` \x01\x93\x92PPPV[`\x80\x80\x82R_\x90a\x03\0\x83\x01\x90\x83\x01\x89\x836\x82\x90\x03`\x1E\x19\x01[`\x14\x82\x10\x15a IW\x86\x85\x03`\x7F\x19\x01\x84R\x825\x81\x81\x12a\x1F\xFAW__\xFD[\x8D\x01` \x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15a \x15W__\xFD[\x806\x03\x82\x13\x15a #W__\xFD[a .\x87\x82\x84a\x1FiV[\x96PPP` \x83\x01\x92P` \x84\x01\x93P`\x01\x82\x01\x91Pa\x1F\xDBV[PPPP\x82\x81\x03` \x84\x01Ra `\x81\x88\x8Aa\x1FiV[\x90P\x82\x81\x03`@\x84\x01Ra u\x81\x86\x88a\x1F\x91V[\x91PP\x82``\x83\x01R\x97\x96PPPPPPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a \xBFWa \xBFa \x89V[`@R\x90V[\x80Qj\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x14a\x1B\xEFW__\xFD[_\x82`\x1F\x83\x01\x12a \xEFW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a!\x08Wa!\x08a \x89V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a!6Wa!6a \x89V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a!MW__\xFD[\x81` \x85\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a!yW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a!\x8EW__\xFD[\x82\x01`\xC0\x81\x85\x03\x12\x15a!\x9FW__\xFD[a!\xA7a \x9DV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x82\x01Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a!\xCBW__\xFD[`@\x82\x01Ra!\xDC``\x83\x01a \xC5V[``\x82\x01Ra!\xED`\x80\x83\x01a \xC5V[`\x80\x82\x01R`\xA0\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\nW__\xFD[a\"\x16\x86\x82\x85\x01a \xE0V[`\xA0\x83\x01RP\x94\x93PPPPV[\x80\x82\x01\x80\x82\x11\x15a\tiWa\tia\x1FBV[_` \x82\x84\x03\x12\x15a\"GW__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\tiWa\tia\x1FBV[_\x82a\"\x93WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[_`\x01\x82\x01a\"\xA9Wa\"\xA9a\x1FBV[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x81\x10a\"\xD4Wa\"\xD4a\"\xB0V[\x90RV[`\x01`\x01`\xA0\x1B\x03\x8C\x81\x16\x82R` \x82\x01\x8C\x90R`@\x82\x01\x8B\x90R``\x82\x01\x8A\x90R\x88\x16`\x80\x82\x01Ra\x01`\x81\x01a#\x13`\xA0\x83\x01\x89a\"\xC4V[`\xC0\x82\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\xE0\x82\x01R\x92\x90\x93\x16a\x01\0\x83\x01R`\x01`\x01`@\x1B\x03\x16a\x01 \x82\x01Ra\x01@\x01R\x96\x95PPPPPPV[_a\x01\xA0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Qa#\x83`@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R`\xC0\x83\x01Qa#\xBC`\xC0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xE0\x83\x01Qa#\xCF`\xE0\x84\x01\x82a\"\xC4V[Pa\x01\0\x83\x01Qa\x01\0\x83\x01Ra\x01 \x83\x01Qa#\xF8a\x01 \x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01@\x83\x01Qa$\x15a\x01@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01`\x83\x01Qa$2a\x01`\x84\x01\x82`\x01`\x01`@\x1B\x03\x16\x90RV[Pa\x01\x80\x92\x83\x01Q\x91\x90\x92\x01R\x90V[\x80Q` \x82\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x91\x90`\x04\x82\x10\x15a$yW`\x01`\x01`\xE0\x1B\x03\x19`\x04\x83\x90\x03`\x03\x1B\x81\x90\x1B\x82\x16\x16\x92P[PP\x91\x90PV[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a$\x9FWa$\x9Fa\x1FBV[PP\x92\x91PPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[\x86\x81R\x85` \x82\x01R\x84`@\x82\x01R\x83``\x82\x01R\x82`\x80\x82\x01R_a$\xE7`\xA0\x83\x01\x84a$\xA7V[\x98\x97PPPPPPPPV[_a\x10\xF7\x82\x84a$\xA7V[\x82\x81R`@` \x82\x01R_a%\x16`@\x83\x01\x84a\x1C\xF7V[\x94\x93PPPPV[\x83\x81R` \x81\x01\x83\x90R``\x81\x01`\x06\x83\x10a%<Wa%<a\"\xB0V[\x82`@\x83\x01R\x94\x93PPPPV\xFE\xA1dsolcC\0\x08\x1B\0\n",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AlreadyInitialized()` and selector `0x0dc149f0`.
```solidity
error AlreadyInitialized();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadyInitialized;
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
        impl ::core::convert::From<AlreadyInitialized> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyInitialized) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyInitialized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyInitialized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyInitialized()";
            const SELECTOR: [u8; 4] = [13u8, 193u8, 73u8, 240u8];
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
    /**Custom error with signature `AmountTooLargeForInt()` and selector `0x1667dc74`.
```solidity
error AmountTooLargeForInt();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AmountTooLargeForInt;
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
        impl ::core::convert::From<AmountTooLargeForInt> for UnderlyingRustTuple<'_> {
            fn from(value: AmountTooLargeForInt) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AmountTooLargeForInt {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AmountTooLargeForInt {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AmountTooLargeForInt()";
            const SELECTOR: [u8; 4] = [22u8, 103u8, 220u8, 116u8];
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
    /**Custom error with signature `CannotRescueUSDT()` and selector `0x1c4e1bff`.
```solidity
error CannotRescueUSDT();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotRescueUSDT;
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
        impl ::core::convert::From<CannotRescueUSDT> for UnderlyingRustTuple<'_> {
            fn from(value: CannotRescueUSDT) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CannotRescueUSDT {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotRescueUSDT {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotRescueUSDT()";
            const SELECTOR: [u8; 4] = [28u8, 78u8, 27u8, 255u8];
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
    /**Custom error with signature `ChainDeprecated()` and selector `0xd9259ca9`.
```solidity
error ChainDeprecated();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ChainDeprecated;
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
        impl ::core::convert::From<ChainDeprecated> for UnderlyingRustTuple<'_> {
            fn from(value: ChainDeprecated) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ChainDeprecated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ChainDeprecated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ChainDeprecated()";
            const SELECTOR: [u8; 4] = [217u8, 37u8, 156u8, 169u8];
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
    /**Custom error with signature `DepositAlreadyProcessed()` and selector `0x37d6e20a`.
```solidity
error DepositAlreadyProcessed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DepositAlreadyProcessed;
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
        impl ::core::convert::From<DepositAlreadyProcessed> for UnderlyingRustTuple<'_> {
            fn from(value: DepositAlreadyProcessed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DepositAlreadyProcessed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DepositAlreadyProcessed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DepositAlreadyProcessed()";
            const SELECTOR: [u8; 4] = [55u8, 214u8, 226u8, 10u8];
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
    /**Custom error with signature `DepositNotAfterLastReceiverPull()` and selector `0xf9b01c44`.
```solidity
error DepositNotAfterLastReceiverPull();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DepositNotAfterLastReceiverPull;
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
        impl ::core::convert::From<DepositNotAfterLastReceiverPull>
        for UnderlyingRustTuple<'_> {
            fn from(value: DepositNotAfterLastReceiverPull) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for DepositNotAfterLastReceiverPull {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DepositNotAfterLastReceiverPull {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DepositNotAfterLastReceiverPull()";
            const SELECTOR: [u8; 4] = [249u8, 176u8, 28u8, 68u8];
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
    /**Custom error with signature `EnforcedPause()` and selector `0xd93c0665`.
```solidity
error EnforcedPause();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EnforcedPause;
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
        impl ::core::convert::From<EnforcedPause> for UnderlyingRustTuple<'_> {
            fn from(value: EnforcedPause) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EnforcedPause {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EnforcedPause {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EnforcedPause()";
            const SELECTOR: [u8; 4] = [217u8, 60u8, 6u8, 101u8];
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
    /**Custom error with signature `EventRelayNoProgress()` and selector `0xd6c1467c`.
```solidity
error EventRelayNoProgress();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EventRelayNoProgress;
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
        impl ::core::convert::From<EventRelayNoProgress> for UnderlyingRustTuple<'_> {
            fn from(value: EventRelayNoProgress) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EventRelayNoProgress {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EventRelayNoProgress {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EventRelayNoProgress()";
            const SELECTOR: [u8; 4] = [214u8, 193u8, 70u8, 124u8];
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
    /**Custom error with signature `EventTipMismatch()` and selector `0x0e521c43`.
```solidity
error EventTipMismatch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EventTipMismatch;
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
        impl ::core::convert::From<EventTipMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: EventTipMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EventTipMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EventTipMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EventTipMismatch()";
            const SELECTOR: [u8; 4] = [14u8, 82u8, 28u8, 67u8];
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
    /**Custom error with signature `ExpectedPause()` and selector `0x8dfc202b`.
```solidity
error ExpectedPause();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExpectedPause;
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
        impl ::core::convert::From<ExpectedPause> for UnderlyingRustTuple<'_> {
            fn from(value: ExpectedPause) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExpectedPause {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExpectedPause {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExpectedPause()";
            const SELECTOR: [u8; 4] = [141u8, 252u8, 32u8, 43u8];
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
    /**Custom error with signature `InsufficientLpPrincipal()` and selector `0xacf2da93`.
```solidity
error InsufficientLpPrincipal();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientLpPrincipal;
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
        impl ::core::convert::From<InsufficientLpPrincipal> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientLpPrincipal) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientLpPrincipal {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientLpPrincipal {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientLpPrincipal()";
            const SELECTOR: [u8; 4] = [172u8, 242u8, 218u8, 147u8];
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
    /**Custom error with signature `InsufficientProtocolProfit()` and selector `0x9d8c7d99`.
```solidity
error InsufficientProtocolProfit();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientProtocolProfit;
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
        impl ::core::convert::From<InsufficientProtocolProfit>
        for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientProtocolProfit) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InsufficientProtocolProfit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientProtocolProfit {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientProtocolProfit()";
            const SELECTOR: [u8; 4] = [157u8, 140u8, 125u8, 153u8];
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
    /**Custom error with signature `InsufficientUsdtBalance()` and selector `0xb55c1bad`.
```solidity
error InsufficientUsdtBalance();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientUsdtBalance;
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
        impl ::core::convert::From<InsufficientUsdtBalance> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientUsdtBalance) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientUsdtBalance {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientUsdtBalance {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientUsdtBalance()";
            const SELECTOR: [u8; 4] = [181u8, 92u8, 27u8, 173u8];
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
    /**Custom error with signature `InvalidLeaseId()` and selector `0x243b4f40`.
```solidity
error InvalidLeaseId();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidLeaseId;
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
        impl ::core::convert::From<InvalidLeaseId> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidLeaseId) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidLeaseId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidLeaseId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidLeaseId()";
            const SELECTOR: [u8; 4] = [36u8, 59u8, 79u8, 64u8];
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
    /**Custom error with signature `InvalidLeaseTimeframe()` and selector `0x501d13ed`.
```solidity
error InvalidLeaseTimeframe();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidLeaseTimeframe;
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
        impl ::core::convert::From<InvalidLeaseTimeframe> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidLeaseTimeframe) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidLeaseTimeframe {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidLeaseTimeframe {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidLeaseTimeframe()";
            const SELECTOR: [u8; 4] = [80u8, 29u8, 19u8, 237u8];
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
    /**Custom error with signature `InvalidReceiverForSalt()` and selector `0x80ca1d52`.
```solidity
error InvalidReceiverForSalt();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidReceiverForSalt;
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
        impl ::core::convert::From<InvalidReceiverForSalt> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidReceiverForSalt) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidReceiverForSalt {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidReceiverForSalt {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidReceiverForSalt()";
            const SELECTOR: [u8; 4] = [128u8, 202u8, 29u8, 82u8];
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
    /**Custom error with signature `InvalidSignature()` and selector `0x8baa579f`.
```solidity
error InvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignature;
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
        impl ::core::convert::From<InvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignature()";
            const SELECTOR: [u8; 4] = [139u8, 170u8, 87u8, 159u8];
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
    /**Custom error with signature `InvalidTargetToken()` and selector `0x8562eb45`.
```solidity
error InvalidTargetToken();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidTargetToken;
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
        impl ::core::convert::From<InvalidTargetToken> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidTargetToken) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidTargetToken {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidTargetToken {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidTargetToken()";
            const SELECTOR: [u8; 4] = [133u8, 98u8, 235u8, 69u8];
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
    /**Custom error with signature `LeaseDurationTooLong()` and selector `0xaae1311c`.
```solidity
error LeaseDurationTooLong();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LeaseDurationTooLong;
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
        impl ::core::convert::From<LeaseDurationTooLong> for UnderlyingRustTuple<'_> {
            fn from(value: LeaseDurationTooLong) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LeaseDurationTooLong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LeaseDurationTooLong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LeaseDurationTooLong()";
            const SELECTOR: [u8; 4] = [170u8, 225u8, 49u8, 28u8];
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
    /**Custom error with signature `LeaseFeeTooLow()` and selector `0x94ab28d6`.
```solidity
error LeaseFeeTooLow();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LeaseFeeTooLow;
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
        impl ::core::convert::From<LeaseFeeTooLow> for UnderlyingRustTuple<'_> {
            fn from(value: LeaseFeeTooLow) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LeaseFeeTooLow {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LeaseFeeTooLow {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LeaseFeeTooLow()";
            const SELECTOR: [u8; 4] = [148u8, 171u8, 40u8, 214u8];
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
    /**Custom error with signature `LeaseFlatFeeTooLow()` and selector `0x4c4b6f02`.
```solidity
error LeaseFlatFeeTooLow();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LeaseFlatFeeTooLow;
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
        impl ::core::convert::From<LeaseFlatFeeTooLow> for UnderlyingRustTuple<'_> {
            fn from(value: LeaseFlatFeeTooLow) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LeaseFlatFeeTooLow {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LeaseFlatFeeTooLow {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LeaseFlatFeeTooLow()";
            const SELECTOR: [u8; 4] = [76u8, 75u8, 111u8, 2u8];
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
    /**Custom error with signature `LeaseNotNukeableYet()` and selector `0xb4ed1c35`.
```solidity
error LeaseNotNukeableYet();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LeaseNotNukeableYet;
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
        impl ::core::convert::From<LeaseNotNukeableYet> for UnderlyingRustTuple<'_> {
            fn from(value: LeaseNotNukeableYet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LeaseNotNukeableYet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LeaseNotNukeableYet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LeaseNotNukeableYet()";
            const SELECTOR: [u8; 4] = [180u8, 237u8, 28u8, 53u8];
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
    /**Custom error with signature `LeaseRateLimitConfigInvalid()` and selector `0x42658232`.
```solidity
error LeaseRateLimitConfigInvalid();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LeaseRateLimitConfigInvalid;
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
        impl ::core::convert::From<LeaseRateLimitConfigInvalid>
        for UnderlyingRustTuple<'_> {
            fn from(value: LeaseRateLimitConfigInvalid) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for LeaseRateLimitConfigInvalid {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LeaseRateLimitConfigInvalid {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LeaseRateLimitConfigInvalid()";
            const SELECTOR: [u8; 4] = [66u8, 101u8, 130u8, 50u8];
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
    /**Custom error with signature `LeaseRateLimitExceeded()` and selector `0x4c542ae4`.
```solidity
error LeaseRateLimitExceeded();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LeaseRateLimitExceeded;
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
        impl ::core::convert::From<LeaseRateLimitExceeded> for UnderlyingRustTuple<'_> {
            fn from(value: LeaseRateLimitExceeded) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LeaseRateLimitExceeded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LeaseRateLimitExceeded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LeaseRateLimitExceeded()";
            const SELECTOR: [u8; 4] = [76u8, 84u8, 42u8, 228u8];
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
    /**Custom error with signature `LpNotAllowlisted()` and selector `0x0dad8e34`.
```solidity
error LpNotAllowlisted();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LpNotAllowlisted;
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
        impl ::core::convert::From<LpNotAllowlisted> for UnderlyingRustTuple<'_> {
            fn from(value: LpNotAllowlisted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LpNotAllowlisted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LpNotAllowlisted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LpNotAllowlisted()";
            const SELECTOR: [u8; 4] = [13u8, 173u8, 142u8, 52u8];
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
    /**Custom error with signature `NewOwnerIsZeroAddress()` and selector `0x7448fbae`.
```solidity
error NewOwnerIsZeroAddress();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NewOwnerIsZeroAddress;
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
        impl ::core::convert::From<NewOwnerIsZeroAddress> for UnderlyingRustTuple<'_> {
            fn from(value: NewOwnerIsZeroAddress) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NewOwnerIsZeroAddress {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NewOwnerIsZeroAddress {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NewOwnerIsZeroAddress()";
            const SELECTOR: [u8; 4] = [116u8, 72u8, 251u8, 174u8];
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
    /**Custom error with signature `NoActiveLease()` and selector `0x4a077666`.
```solidity
error NoActiveLease();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoActiveLease;
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
        impl ::core::convert::From<NoActiveLease> for UnderlyingRustTuple<'_> {
            fn from(value: NoActiveLease) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoActiveLease {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoActiveLease {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoActiveLease()";
            const SELECTOR: [u8; 4] = [74u8, 7u8, 118u8, 102u8];
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
    /**Custom error with signature `NoBridger()` and selector `0xb37c79ed`.
```solidity
error NoBridger();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoBridger;
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
        impl ::core::convert::From<NoBridger> for UnderlyingRustTuple<'_> {
            fn from(value: NoBridger) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoBridger {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoBridger {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoBridger()";
            const SELECTOR: [u8; 4] = [179u8, 124u8, 121u8, 237u8];
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
    /**Custom error with signature `NotATrc20Transfer()` and selector `0x2456e1ec`.
```solidity
error NotATrc20Transfer();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotATrc20Transfer;
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
        impl ::core::convert::From<NotATrc20Transfer> for UnderlyingRustTuple<'_> {
            fn from(value: NotATrc20Transfer) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotATrc20Transfer {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotATrc20Transfer {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotATrc20Transfer()";
            const SELECTOR: [u8; 4] = [36u8, 86u8, 225u8, 236u8];
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
    /**Custom error with signature `NotEventChainTip()` and selector `0x6cb67ca6`.
```solidity
error NotEventChainTip();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotEventChainTip;
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
        impl ::core::convert::From<NotEventChainTip> for UnderlyingRustTuple<'_> {
            fn from(value: NotEventChainTip) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotEventChainTip {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotEventChainTip {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotEventChainTip()";
            const SELECTOR: [u8; 4] = [108u8, 182u8, 124u8, 166u8];
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
    /**Custom error with signature `NotLessee()` and selector `0x7136f4f2`.
```solidity
error NotLessee();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotLessee;
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
        impl ::core::convert::From<NotLessee> for UnderlyingRustTuple<'_> {
            fn from(value: NotLessee) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotLessee {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotLessee {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotLessee()";
            const SELECTOR: [u8; 4] = [113u8, 54u8, 244u8, 242u8];
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
    /**Custom error with signature `NotRealtor()` and selector `0x8a28437a`.
```solidity
error NotRealtor();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotRealtor;
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
        impl ::core::convert::From<NotRealtor> for UnderlyingRustTuple<'_> {
            fn from(value: NotRealtor) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotRealtor {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotRealtor {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotRealtor()";
            const SELECTOR: [u8; 4] = [138u8, 40u8, 67u8, 122u8];
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
    /**Custom error with signature `NotTronUsdt()` and selector `0x3395765b`.
```solidity
error NotTronUsdt();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotTronUsdt;
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
        impl ::core::convert::From<NotTronUsdt> for UnderlyingRustTuple<'_> {
            fn from(value: NotTronUsdt) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotTronUsdt {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotTronUsdt {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotTronUsdt()";
            const SELECTOR: [u8; 4] = [51u8, 149u8, 118u8, 91u8];
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
    /**Custom error with signature `PayoutConfigRateLimitConfigInvalid()` and selector `0xa17eea77`.
```solidity
error PayoutConfigRateLimitConfigInvalid();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PayoutConfigRateLimitConfigInvalid;
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
        impl ::core::convert::From<PayoutConfigRateLimitConfigInvalid>
        for UnderlyingRustTuple<'_> {
            fn from(value: PayoutConfigRateLimitConfigInvalid) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for PayoutConfigRateLimitConfigInvalid {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PayoutConfigRateLimitConfigInvalid {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PayoutConfigRateLimitConfigInvalid()";
            const SELECTOR: [u8; 4] = [161u8, 126u8, 234u8, 119u8];
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
    /**Custom error with signature `PayoutConfigRateLimitExceeded()` and selector `0x0a83b084`.
```solidity
error PayoutConfigRateLimitExceeded();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PayoutConfigRateLimitExceeded;
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
        impl ::core::convert::From<PayoutConfigRateLimitExceeded>
        for UnderlyingRustTuple<'_> {
            fn from(value: PayoutConfigRateLimitExceeded) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for PayoutConfigRateLimitExceeded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PayoutConfigRateLimitExceeded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PayoutConfigRateLimitExceeded()";
            const SELECTOR: [u8; 4] = [10u8, 131u8, 176u8, 132u8];
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
    /**Custom error with signature `RateNotSet()` and selector `0x08fc7fce`.
```solidity
error RateNotSet();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RateNotSet;
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
        impl ::core::convert::From<RateNotSet> for UnderlyingRustTuple<'_> {
            fn from(value: RateNotSet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RateNotSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RateNotSet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RateNotSet()";
            const SELECTOR: [u8; 4] = [8u8, 252u8, 127u8, 206u8];
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
    /**Custom error with signature `Reentrancy()` and selector `0xab143c06`.
```solidity
error Reentrancy();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Reentrancy;
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
        impl ::core::convert::From<Reentrancy> for UnderlyingRustTuple<'_> {
            fn from(value: Reentrancy) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Reentrancy {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Reentrancy {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Reentrancy()";
            const SELECTOR: [u8; 4] = [171u8, 20u8, 60u8, 6u8];
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
    /**Custom error with signature `SignatureExpired()` and selector `0x0819bdcd`.
```solidity
error SignatureExpired();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureExpired;
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
        impl ::core::convert::From<SignatureExpired> for UnderlyingRustTuple<'_> {
            fn from(value: SignatureExpired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignatureExpired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SignatureExpired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SignatureExpired()";
            const SELECTOR: [u8; 4] = [8u8, 25u8, 189u8, 205u8];
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
    /**Custom error with signature `SubjectiveNetOutZero()` and selector `0x3f41ae01`.
```solidity
error SubjectiveNetOutZero();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SubjectiveNetOutZero;
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
        impl ::core::convert::From<SubjectiveNetOutZero> for UnderlyingRustTuple<'_> {
            fn from(value: SubjectiveNetOutZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SubjectiveNetOutZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SubjectiveNetOutZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SubjectiveNetOutZero()";
            const SELECTOR: [u8; 4] = [63u8, 65u8, 174u8, 1u8];
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
    /**Custom error with signature `SubjectivePreEntitlementAlreadyExists()` and selector `0xcc0bb0c1`.
```solidity
error SubjectivePreEntitlementAlreadyExists();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SubjectivePreEntitlementAlreadyExists;
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
        impl ::core::convert::From<SubjectivePreEntitlementAlreadyExists>
        for UnderlyingRustTuple<'_> {
            fn from(value: SubjectivePreEntitlementAlreadyExists) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SubjectivePreEntitlementAlreadyExists {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SubjectivePreEntitlementAlreadyExists {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SubjectivePreEntitlementAlreadyExists()";
            const SELECTOR: [u8; 4] = [204u8, 11u8, 176u8, 193u8];
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
    /**Custom error with signature `TronInvalidCalldataLength()` and selector `0x12799503`.
```solidity
error TronInvalidCalldataLength();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TronInvalidCalldataLength;
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
        impl ::core::convert::From<TronInvalidCalldataLength>
        for UnderlyingRustTuple<'_> {
            fn from(value: TronInvalidCalldataLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TronInvalidCalldataLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TronInvalidCalldataLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TronInvalidCalldataLength()";
            const SELECTOR: [u8; 4] = [18u8, 121u8, 149u8, 3u8];
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
    /**Custom error with signature `TronInvalidTrc20DataLength()` and selector `0x9ac60c2d`.
```solidity
error TronInvalidTrc20DataLength();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TronInvalidTrc20DataLength;
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
        impl ::core::convert::From<TronInvalidTrc20DataLength>
        for UnderlyingRustTuple<'_> {
            fn from(value: TronInvalidTrc20DataLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TronInvalidTrc20DataLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TronInvalidTrc20DataLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TronInvalidTrc20DataLength()";
            const SELECTOR: [u8; 4] = [154u8, 198u8, 12u8, 45u8];
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
    /**Custom error with signature `Unauthorized()` and selector `0x82b42900`.
```solidity
error Unauthorized();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Unauthorized;
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
        impl ::core::convert::From<Unauthorized> for UnderlyingRustTuple<'_> {
            fn from(value: Unauthorized) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Unauthorized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Unauthorized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Unauthorized()";
            const SELECTOR: [u8; 4] = [130u8, 180u8, 41u8, 0u8];
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
    /**Custom error with signature `WithdrawExceedsPrincipal()` and selector `0xd4708ca5`.
```solidity
error WithdrawExceedsPrincipal();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawExceedsPrincipal;
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
        impl ::core::convert::From<WithdrawExceedsPrincipal>
        for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawExceedsPrincipal) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for WithdrawExceedsPrincipal {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WithdrawExceedsPrincipal {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WithdrawExceedsPrincipal()";
            const SELECTOR: [u8; 4] = [212u8, 112u8, 140u8, 165u8];
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
    /**Custom error with signature `ZeroAmount()` and selector `0x1f2a2005`.
```solidity
error ZeroAmount();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroAmount;
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
        impl ::core::convert::From<ZeroAmount> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroAmount) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroAmount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroAmount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroAmount()";
            const SELECTOR: [u8; 4] = [31u8, 42u8, 32u8, 5u8];
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
    /**Event with signature `BridgerSet(address,uint256,address)` and selector `0x427db48f8cd0cf7fdabcdbc1327c7db26ce3a544292ba380ef07d5175af729cd`.
```solidity
event BridgerSet(address indexed targetToken, uint256 indexed targetChainId, address bridger);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct BridgerSet {
        #[allow(missing_docs)]
        pub targetToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub targetChainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub bridger: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for BridgerSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "BridgerSet(address,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                66u8, 125u8, 180u8, 143u8, 140u8, 208u8, 207u8, 127u8, 218u8, 188u8,
                219u8, 193u8, 50u8, 124u8, 125u8, 178u8, 108u8, 227u8, 165u8, 68u8, 41u8,
                43u8, 163u8, 128u8, 239u8, 7u8, 213u8, 23u8, 90u8, 247u8, 41u8, 205u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    targetToken: topics.1,
                    targetChainId: topics.2,
                    bridger: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.bridger,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.targetToken.clone(),
                    self.targetChainId.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.targetToken,
                );
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.targetChainId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for BridgerSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BridgerSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BridgerSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ChainDeprecatedSet(uint256,bool)` and selector `0xc8b7fe24dc2e9e731141fe1dd74d6ce470bd6b5371c12f42c9ce46bac15424c5`.
```solidity
event ChainDeprecatedSet(uint256 indexed targetChainId, bool deprecated);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ChainDeprecatedSet {
        #[allow(missing_docs)]
        pub targetChainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub deprecated: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ChainDeprecatedSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "ChainDeprecatedSet(uint256,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                200u8, 183u8, 254u8, 36u8, 220u8, 46u8, 158u8, 115u8, 17u8, 65u8, 254u8,
                29u8, 215u8, 77u8, 108u8, 228u8, 112u8, 189u8, 107u8, 83u8, 113u8, 193u8,
                47u8, 66u8, 201u8, 206u8, 70u8, 186u8, 193u8, 84u8, 36u8, 197u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    targetChainId: topics.1,
                    deprecated: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.deprecated,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.targetChainId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.targetChainId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ChainDeprecatedSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChainDeprecatedSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ChainDeprecatedSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ClaimCreated(uint256,uint256,address,uint256,uint256,uint256,address,uint8,bytes32,address,address,uint64,uint256)` and selector `0x77242fbd573af5a5f3518da92600e96795ebfff993606b4fb54dea2dcd2dfe85`.
```solidity
event ClaimCreated(uint256 indexed leaseId, uint256 indexed claimId, address targetToken, uint256 queueIndex, uint256 amountUsdt, uint256 targetChainId, address beneficiary, UntronV3Index.ClaimOrigin origin, bytes32 originId, address originActor, address originToken, uint64 originTimestamp, uint256 originRawAmount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ClaimCreated {
        #[allow(missing_docs)]
        pub leaseId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub claimId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub targetToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub queueIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountUsdt: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub targetChainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub beneficiary: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub origin: <UntronV3Index::ClaimOrigin as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub originId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub originActor: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub originToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub originTimestamp: u64,
        #[allow(missing_docs)]
        pub originRawAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ClaimCreated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                UntronV3Index::ClaimOrigin,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "ClaimCreated(uint256,uint256,address,uint256,uint256,uint256,address,uint8,bytes32,address,address,uint64,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                119u8, 36u8, 47u8, 189u8, 87u8, 58u8, 245u8, 165u8, 243u8, 81u8, 141u8,
                169u8, 38u8, 0u8, 233u8, 103u8, 149u8, 235u8, 255u8, 249u8, 147u8, 96u8,
                107u8, 79u8, 181u8, 77u8, 234u8, 45u8, 205u8, 45u8, 254u8, 133u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    leaseId: topics.1,
                    claimId: topics.2,
                    targetToken: data.0,
                    queueIndex: data.1,
                    amountUsdt: data.2,
                    targetChainId: data.3,
                    beneficiary: data.4,
                    origin: data.5,
                    originId: data.6,
                    originActor: data.7,
                    originToken: data.8,
                    originTimestamp: data.9,
                    originRawAmount: data.10,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.targetToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.queueIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountUsdt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.targetChainId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.beneficiary,
                    ),
                    <UntronV3Index::ClaimOrigin as alloy_sol_types::SolType>::tokenize(
                        &self.origin,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.originId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.originActor,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.originToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.originTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.originRawAmount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.leaseId.clone(), self.claimId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.leaseId);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.claimId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ClaimCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ClaimCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ClaimCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ClaimFilled(uint256,uint256,address,uint256,uint256,uint256,address)` and selector `0xb62b4e6f1ec5970a29274e747835f444a5ccd48049698eff9c9cfdca2e1a5eaf`.
```solidity
event ClaimFilled(uint256 indexed leaseId, uint256 indexed claimId, address targetToken, uint256 queueIndex, uint256 amountUsdt, uint256 targetChainId, address beneficiary);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ClaimFilled {
        #[allow(missing_docs)]
        pub leaseId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub claimId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub targetToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub queueIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountUsdt: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub targetChainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub beneficiary: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ClaimFilled {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "ClaimFilled(uint256,uint256,address,uint256,uint256,uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                182u8, 43u8, 78u8, 111u8, 30u8, 197u8, 151u8, 10u8, 41u8, 39u8, 78u8,
                116u8, 120u8, 53u8, 244u8, 68u8, 165u8, 204u8, 212u8, 128u8, 73u8, 105u8,
                142u8, 255u8, 156u8, 156u8, 253u8, 202u8, 46u8, 26u8, 94u8, 175u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    leaseId: topics.1,
                    claimId: topics.2,
                    targetToken: data.0,
                    queueIndex: data.1,
                    amountUsdt: data.2,
                    targetChainId: data.3,
                    beneficiary: data.4,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.targetToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.queueIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountUsdt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.targetChainId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.beneficiary,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.leaseId.clone(), self.claimId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.leaseId);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.claimId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ClaimFilled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ClaimFilled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ClaimFilled) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ControllerEventChainTipUpdated(bytes32,uint256,uint256,bytes32,bytes)` and selector `0x9d611b5b34cb76131c4fb413eb74119b2c0c3a6aa6fcd8e740cf70ac3085d87b`.
```solidity
event ControllerEventChainTipUpdated(bytes32 previousTip, uint256 indexed blockNumber, uint256 blockTimestamp, bytes32 indexed eventSignature, bytes abiEncodedEventData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ControllerEventChainTipUpdated {
        #[allow(missing_docs)]
        pub previousTip: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub blockTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub eventSignature: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub abiEncodedEventData: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ControllerEventChainTipUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ControllerEventChainTipUpdated(bytes32,uint256,uint256,bytes32,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                157u8, 97u8, 27u8, 91u8, 52u8, 203u8, 118u8, 19u8, 28u8, 79u8, 180u8,
                19u8, 235u8, 116u8, 17u8, 155u8, 44u8, 12u8, 58u8, 106u8, 166u8, 252u8,
                216u8, 231u8, 64u8, 207u8, 112u8, 172u8, 48u8, 133u8, 216u8, 123u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousTip: data.0,
                    blockNumber: topics.1,
                    blockTimestamp: data.1,
                    eventSignature: topics.2,
                    abiEncodedEventData: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.previousTip),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockTimestamp),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.abiEncodedEventData,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.blockNumber.clone(),
                    self.eventSignature.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.blockNumber);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.eventSignature);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ControllerEventChainTipUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ControllerEventChainTipUpdated>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ControllerEventChainTipUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ControllerEventProcessed(uint256,uint256,uint256,bytes32,bytes)` and selector `0xdca16b0af6e10f5dfb7d4ea91055951419a0c8ffc5925acffdc52a95fcc67133`.
```solidity
event ControllerEventProcessed(uint256 indexed eventIndex, uint256 indexed blockNumber, uint256 blockTimestamp, bytes32 indexed eventSignature, bytes abiEncodedEventData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ControllerEventProcessed {
        #[allow(missing_docs)]
        pub eventIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub blockTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub eventSignature: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub abiEncodedEventData: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ControllerEventProcessed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ControllerEventProcessed(uint256,uint256,uint256,bytes32,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                220u8, 161u8, 107u8, 10u8, 246u8, 225u8, 15u8, 93u8, 251u8, 125u8, 78u8,
                169u8, 16u8, 85u8, 149u8, 20u8, 25u8, 160u8, 200u8, 255u8, 197u8, 146u8,
                90u8, 207u8, 253u8, 197u8, 42u8, 149u8, 252u8, 198u8, 113u8, 51u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    eventIndex: topics.1,
                    blockNumber: topics.2,
                    blockTimestamp: data.0,
                    eventSignature: topics.3,
                    abiEncodedEventData: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockTimestamp),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.abiEncodedEventData,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.eventIndex.clone(),
                    self.blockNumber.clone(),
                    self.eventSignature.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.eventIndex);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.blockNumber);
                out[3usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.eventSignature);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ControllerEventProcessed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ControllerEventProcessed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ControllerEventProcessed,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `EventAppended(uint256,bytes32,bytes32,bytes32,bytes)` and selector `0x78160f0b1b2b32b52a0076d8f0f70888687ba702a4d993d55ac8d9327d57a127`.
```solidity
event EventAppended(uint256 indexed eventSeq, bytes32 indexed prevTip, bytes32 indexed newTip, bytes32 eventSignature, bytes abiEncodedEventData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EventAppended {
        #[allow(missing_docs)]
        pub eventSeq: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub prevTip: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub newTip: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub eventSignature: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub abiEncodedEventData: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for EventAppended {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "EventAppended(uint256,bytes32,bytes32,bytes32,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                120u8, 22u8, 15u8, 11u8, 27u8, 43u8, 50u8, 181u8, 42u8, 0u8, 118u8,
                216u8, 240u8, 247u8, 8u8, 136u8, 104u8, 123u8, 167u8, 2u8, 164u8, 217u8,
                147u8, 213u8, 90u8, 200u8, 217u8, 50u8, 125u8, 87u8, 161u8, 39u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    eventSeq: topics.1,
                    prevTip: topics.2,
                    newTip: topics.3,
                    eventSignature: data.0,
                    abiEncodedEventData: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.eventSignature),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.abiEncodedEventData,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.eventSeq.clone(),
                    self.prevTip.clone(),
                    self.newTip.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.eventSeq);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.prevTip);
                out[3usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.newTip);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EventAppended {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EventAppended> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EventAppended) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `LeaseCreated(uint256,bytes32,uint256,address,address,uint64,uint64,uint32,uint64)` and selector `0xe431502bd0df5880397ec13c0a260f0c749c8f3a30c6caf3ffe0f08a2ed13942`.
```solidity
event LeaseCreated(uint256 indexed leaseId, bytes32 indexed receiverSalt, uint256 indexed leaseNumber, address realtor, address lessee, uint64 startTime, uint64 nukeableAfter, uint32 leaseFeePpm, uint64 flatFee);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct LeaseCreated {
        #[allow(missing_docs)]
        pub leaseId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub receiverSalt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub leaseNumber: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub realtor: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub lessee: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub startTime: u64,
        #[allow(missing_docs)]
        pub nukeableAfter: u64,
        #[allow(missing_docs)]
        pub leaseFeePpm: u32,
        #[allow(missing_docs)]
        pub flatFee: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for LeaseCreated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "LeaseCreated(uint256,bytes32,uint256,address,address,uint64,uint64,uint32,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                228u8, 49u8, 80u8, 43u8, 208u8, 223u8, 88u8, 128u8, 57u8, 126u8, 193u8,
                60u8, 10u8, 38u8, 15u8, 12u8, 116u8, 156u8, 143u8, 58u8, 48u8, 198u8,
                202u8, 243u8, 255u8, 224u8, 240u8, 138u8, 46u8, 209u8, 57u8, 66u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    leaseId: topics.1,
                    receiverSalt: topics.2,
                    leaseNumber: topics.3,
                    realtor: data.0,
                    lessee: data.1,
                    startTime: data.2,
                    nukeableAfter: data.3,
                    leaseFeePpm: data.4,
                    flatFee: data.5,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.realtor,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.lessee,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTime),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.nukeableAfter),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.leaseFeePpm),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.flatFee),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.leaseId.clone(),
                    self.receiverSalt.clone(),
                    self.leaseNumber.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.leaseId);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.receiverSalt);
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.leaseNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for LeaseCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&LeaseCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &LeaseCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `LeaseNonceUpdated(uint256,uint256)` and selector `0x8e98a0734420e57ef102cab0b1af0809a5bbaf4fde966fd173c322490644a5d0`.
```solidity
event LeaseNonceUpdated(uint256 indexed leaseId, uint256 nonce);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct LeaseNonceUpdated {
        #[allow(missing_docs)]
        pub leaseId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for LeaseNonceUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "LeaseNonceUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                142u8, 152u8, 160u8, 115u8, 68u8, 32u8, 229u8, 126u8, 241u8, 2u8, 202u8,
                176u8, 177u8, 175u8, 8u8, 9u8, 165u8, 187u8, 175u8, 79u8, 222u8, 150u8,
                111u8, 209u8, 115u8, 195u8, 34u8, 73u8, 6u8, 68u8, 165u8, 208u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    leaseId: topics.1,
                    nonce: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.leaseId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.leaseId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for LeaseNonceUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&LeaseNonceUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &LeaseNonceUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `LesseePayoutConfigRateLimitSet(uint256,uint256)` and selector `0xab63c25aa07a4d10a1842d61026a08d95115c4e6895288683015ca4464c7f50f`.
```solidity
event LesseePayoutConfigRateLimitSet(uint256 maxUpdates, uint256 windowSeconds);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct LesseePayoutConfigRateLimitSet {
        #[allow(missing_docs)]
        pub maxUpdates: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub windowSeconds: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for LesseePayoutConfigRateLimitSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "LesseePayoutConfigRateLimitSet(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                171u8, 99u8, 194u8, 90u8, 160u8, 122u8, 77u8, 16u8, 161u8, 132u8, 45u8,
                97u8, 2u8, 106u8, 8u8, 217u8, 81u8, 21u8, 196u8, 230u8, 137u8, 82u8,
                136u8, 104u8, 48u8, 21u8, 202u8, 68u8, 100u8, 199u8, 245u8, 15u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    maxUpdates: data.0,
                    windowSeconds: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxUpdates),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.windowSeconds),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for LesseePayoutConfigRateLimitSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&LesseePayoutConfigRateLimitSet>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &LesseePayoutConfigRateLimitSet,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `LpDeposited(address,uint256)` and selector `0xdae4417c25a93cfb86ce5fbbb8fc1630945b61afaeab4f29cf301b9a058ba914`.
```solidity
event LpDeposited(address indexed lp, uint256 amount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct LpDeposited {
        #[allow(missing_docs)]
        pub lp: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for LpDeposited {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "LpDeposited(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                218u8, 228u8, 65u8, 124u8, 37u8, 169u8, 60u8, 251u8, 134u8, 206u8, 95u8,
                187u8, 184u8, 252u8, 22u8, 48u8, 148u8, 91u8, 97u8, 175u8, 174u8, 171u8,
                79u8, 41u8, 207u8, 48u8, 27u8, 154u8, 5u8, 139u8, 169u8, 20u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    lp: topics.1,
                    amount: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.lp.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.lp,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for LpDeposited {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&LpDeposited> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &LpDeposited) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `LpSet(address,bool)` and selector `0x442f7ab7d26bf7ba74f23e237d12876251f91d61aeb2899ad95f334fa5e7c633`.
```solidity
event LpSet(address indexed lp, bool allowed);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct LpSet {
        #[allow(missing_docs)]
        pub lp: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub allowed: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for LpSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "LpSet(address,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                68u8, 47u8, 122u8, 183u8, 210u8, 107u8, 247u8, 186u8, 116u8, 242u8, 62u8,
                35u8, 125u8, 18u8, 135u8, 98u8, 81u8, 249u8, 29u8, 97u8, 174u8, 178u8,
                137u8, 154u8, 217u8, 95u8, 51u8, 79u8, 165u8, 231u8, 198u8, 51u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    lp: topics.1,
                    allowed: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.allowed,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.lp.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.lp,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for LpSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&LpSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &LpSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `LpWithdrawn(address,uint256)` and selector `0x44282e2a4bbb86b9f8089d1086f4e0df273054eab62cdbda18f647cfd22a1eff`.
```solidity
event LpWithdrawn(address indexed lp, uint256 amount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct LpWithdrawn {
        #[allow(missing_docs)]
        pub lp: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for LpWithdrawn {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "LpWithdrawn(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                68u8, 40u8, 46u8, 42u8, 75u8, 187u8, 134u8, 185u8, 248u8, 8u8, 157u8,
                16u8, 134u8, 244u8, 224u8, 223u8, 39u8, 48u8, 84u8, 234u8, 182u8, 44u8,
                219u8, 218u8, 24u8, 246u8, 71u8, 207u8, 210u8, 42u8, 30u8, 255u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    lp: topics.1,
                    amount: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.lp.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.lp,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for LpWithdrawn {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&LpWithdrawn> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &LpWithdrawn) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed oldOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub oldOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.oldOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.oldOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Paused(address)` and selector `0x62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a258`.
```solidity
event Paused(address account);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Paused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Paused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Paused(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                98u8, 231u8, 140u8, 234u8, 1u8, 190u8, 227u8, 32u8, 205u8, 78u8, 66u8,
                2u8, 112u8, 181u8, 234u8, 116u8, 0u8, 13u8, 17u8, 176u8, 201u8, 247u8,
                71u8, 84u8, 235u8, 219u8, 252u8, 84u8, 75u8, 5u8, 162u8, 88u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { account: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Paused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Paused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Paused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PayoutConfigUpdated(uint256,uint256,address,address)` and selector `0xd7cf75dc193207f6484b7bd6c4fc469f3b6b733d2cbcee403975287b015dc499`.
```solidity
event PayoutConfigUpdated(uint256 indexed leaseId, uint256 targetChainId, address targetToken, address beneficiary);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PayoutConfigUpdated {
        #[allow(missing_docs)]
        pub leaseId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub targetChainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub targetToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub beneficiary: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for PayoutConfigUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "PayoutConfigUpdated(uint256,uint256,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                215u8, 207u8, 117u8, 220u8, 25u8, 50u8, 7u8, 246u8, 72u8, 75u8, 123u8,
                214u8, 196u8, 252u8, 70u8, 159u8, 59u8, 107u8, 115u8, 61u8, 44u8, 188u8,
                238u8, 64u8, 57u8, 117u8, 40u8, 123u8, 1u8, 93u8, 196u8, 153u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    leaseId: topics.1,
                    targetChainId: data.0,
                    targetToken: data.1,
                    beneficiary: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.targetChainId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.targetToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.beneficiary,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.leaseId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.leaseId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PayoutConfigUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PayoutConfigUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PayoutConfigUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ProtocolFlatFeeFloorSet(uint64)` and selector `0x286cd7c5781e6df376daa59ab207e1a0b1ece95fafc298f9ab69a188f49ff2d5`.
```solidity
event ProtocolFlatFeeFloorSet(uint64 floorFlatFee);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ProtocolFlatFeeFloorSet {
        #[allow(missing_docs)]
        pub floorFlatFee: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ProtocolFlatFeeFloorSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ProtocolFlatFeeFloorSet(uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                40u8, 108u8, 215u8, 197u8, 120u8, 30u8, 109u8, 243u8, 118u8, 218u8,
                165u8, 154u8, 178u8, 7u8, 225u8, 160u8, 177u8, 236u8, 233u8, 95u8, 175u8,
                194u8, 152u8, 249u8, 171u8, 105u8, 161u8, 136u8, 244u8, 159u8, 242u8,
                213u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { floorFlatFee: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.floorFlatFee),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ProtocolFlatFeeFloorSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProtocolFlatFeeFloorSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ProtocolFlatFeeFloorSet,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ProtocolFloorSet(uint256)` and selector `0xf7cfae9870e1307a0791d6418d9e78abd1731a1c03606813906b474d307ead56`.
```solidity
event ProtocolFloorSet(uint256 floorPpm);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ProtocolFloorSet {
        #[allow(missing_docs)]
        pub floorPpm: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ProtocolFloorSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ProtocolFloorSet(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                247u8, 207u8, 174u8, 152u8, 112u8, 225u8, 48u8, 122u8, 7u8, 145u8, 214u8,
                65u8, 141u8, 158u8, 120u8, 171u8, 209u8, 115u8, 26u8, 28u8, 3u8, 96u8,
                104u8, 19u8, 144u8, 107u8, 71u8, 77u8, 48u8, 126u8, 173u8, 86u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { floorPpm: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.floorPpm),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ProtocolFloorSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProtocolFloorSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ProtocolFloorSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ProtocolMaxLeaseDurationSet(uint32)` and selector `0x1fb3b38e402009f1bbd24d0a70005a7a9a055b3f5b546c8b21f815470a9c2ec4`.
```solidity
event ProtocolMaxLeaseDurationSet(uint32 maxLeaseDurationSeconds);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ProtocolMaxLeaseDurationSet {
        #[allow(missing_docs)]
        pub maxLeaseDurationSeconds: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ProtocolMaxLeaseDurationSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ProtocolMaxLeaseDurationSet(uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                31u8, 179u8, 179u8, 142u8, 64u8, 32u8, 9u8, 241u8, 187u8, 210u8, 77u8,
                10u8, 112u8, 0u8, 90u8, 122u8, 154u8, 5u8, 91u8, 63u8, 91u8, 84u8, 108u8,
                139u8, 33u8, 248u8, 21u8, 71u8, 10u8, 156u8, 46u8, 196u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    maxLeaseDurationSeconds: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.maxLeaseDurationSeconds,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ProtocolMaxLeaseDurationSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProtocolMaxLeaseDurationSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ProtocolMaxLeaseDurationSet,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ProtocolPnlUpdated(int256,int256,uint8)` and selector `0x43991e1e1cfb2eed6c9dc37a7a848622f8e3f75bc38d532a00eaf026cd87a014`.
```solidity
event ProtocolPnlUpdated(int256 pnl, int256 delta, UntronV3Index.PnlReason reason);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ProtocolPnlUpdated {
        #[allow(missing_docs)]
        pub pnl: alloy::sol_types::private::primitives::aliases::I256,
        #[allow(missing_docs)]
        pub delta: alloy::sol_types::private::primitives::aliases::I256,
        #[allow(missing_docs)]
        pub reason: <UntronV3Index::PnlReason as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ProtocolPnlUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Int<256>,
                alloy::sol_types::sol_data::Int<256>,
                UntronV3Index::PnlReason,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ProtocolPnlUpdated(int256,int256,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                67u8, 153u8, 30u8, 30u8, 28u8, 251u8, 46u8, 237u8, 108u8, 157u8, 195u8,
                122u8, 122u8, 132u8, 134u8, 34u8, 248u8, 227u8, 247u8, 91u8, 195u8,
                141u8, 83u8, 42u8, 0u8, 234u8, 240u8, 38u8, 205u8, 135u8, 160u8, 20u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    pnl: data.0,
                    delta: data.1,
                    reason: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.pnl),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.delta),
                    <UntronV3Index::PnlReason as alloy_sol_types::SolType>::tokenize(
                        &self.reason,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ProtocolPnlUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProtocolPnlUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ProtocolPnlUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RealtorLeaseRateLimitSet(address,uint256,uint256)` and selector `0x61c49ab6bb1ef59191a746afce7d2731d4d9b0fafdab8d63f02fe44169532dc5`.
```solidity
event RealtorLeaseRateLimitSet(address indexed realtor, uint256 maxLeases, uint256 windowSeconds);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RealtorLeaseRateLimitSet {
        #[allow(missing_docs)]
        pub realtor: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub maxLeases: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub windowSeconds: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RealtorLeaseRateLimitSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RealtorLeaseRateLimitSet(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                97u8, 196u8, 154u8, 182u8, 187u8, 30u8, 245u8, 145u8, 145u8, 167u8, 70u8,
                175u8, 206u8, 125u8, 39u8, 49u8, 212u8, 217u8, 176u8, 250u8, 253u8,
                171u8, 141u8, 99u8, 240u8, 47u8, 228u8, 65u8, 105u8, 83u8, 45u8, 197u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    realtor: topics.1,
                    maxLeases: data.0,
                    windowSeconds: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxLeases),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.windowSeconds),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.realtor.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.realtor,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RealtorLeaseRateLimitSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RealtorLeaseRateLimitSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &RealtorLeaseRateLimitSet,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RealtorMaxLeaseDurationSet(address,uint32)` and selector `0xb1d8eff9485816bb38ceecf317a3fda05597bc5b38f2f22df96c0ebd5dc236ee`.
```solidity
event RealtorMaxLeaseDurationSet(address indexed realtor, uint32 maxLeaseDurationSeconds);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RealtorMaxLeaseDurationSet {
        #[allow(missing_docs)]
        pub realtor: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub maxLeaseDurationSeconds: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RealtorMaxLeaseDurationSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RealtorMaxLeaseDurationSet(address,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                177u8, 216u8, 239u8, 249u8, 72u8, 88u8, 22u8, 187u8, 56u8, 206u8, 236u8,
                243u8, 23u8, 163u8, 253u8, 160u8, 85u8, 151u8, 188u8, 91u8, 56u8, 242u8,
                242u8, 45u8, 249u8, 108u8, 14u8, 189u8, 93u8, 194u8, 54u8, 238u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    realtor: topics.1,
                    maxLeaseDurationSeconds: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.maxLeaseDurationSeconds,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.realtor.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.realtor,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RealtorMaxLeaseDurationSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RealtorMaxLeaseDurationSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &RealtorMaxLeaseDurationSet,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RealtorMinFeeSet(address,uint256)` and selector `0x0aa82626f19bb5d4196202b01f2fee431b95e88cb054484b987d650d91928ada`.
```solidity
event RealtorMinFeeSet(address indexed realtor, uint256 minFeePpm);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RealtorMinFeeSet {
        #[allow(missing_docs)]
        pub realtor: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub minFeePpm: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RealtorMinFeeSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RealtorMinFeeSet(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                10u8, 168u8, 38u8, 38u8, 241u8, 155u8, 181u8, 212u8, 25u8, 98u8, 2u8,
                176u8, 31u8, 47u8, 238u8, 67u8, 27u8, 149u8, 232u8, 140u8, 176u8, 84u8,
                72u8, 75u8, 152u8, 125u8, 101u8, 13u8, 145u8, 146u8, 138u8, 218u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    realtor: topics.1,
                    minFeePpm: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.minFeePpm),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.realtor.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.realtor,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RealtorMinFeeSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RealtorMinFeeSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RealtorMinFeeSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RealtorMinFlatFeeSet(address,uint64)` and selector `0x2f4817318242d1b86700a426f7ced049c68059253c0a721e01b9718f4527bef6`.
```solidity
event RealtorMinFlatFeeSet(address indexed realtor, uint64 minFlatFee);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RealtorMinFlatFeeSet {
        #[allow(missing_docs)]
        pub realtor: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub minFlatFee: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RealtorMinFlatFeeSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RealtorMinFlatFeeSet(address,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8, 72u8, 23u8, 49u8, 130u8, 66u8, 209u8, 184u8, 103u8, 0u8, 164u8,
                38u8, 247u8, 206u8, 208u8, 73u8, 198u8, 128u8, 89u8, 37u8, 60u8, 10u8,
                114u8, 30u8, 1u8, 185u8, 113u8, 143u8, 69u8, 39u8, 190u8, 246u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    realtor: topics.1,
                    minFlatFee: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.minFlatFee),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.realtor.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.realtor,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RealtorMinFlatFeeSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RealtorMinFlatFeeSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RealtorMinFlatFeeSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RealtorSet(address,bool)` and selector `0x2080a756444d2af0f9fb4f87db7af63cd6e23b7a1203cbe9ab0972ee05d1eafd`.
```solidity
event RealtorSet(address indexed realtor, bool allowed);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RealtorSet {
        #[allow(missing_docs)]
        pub realtor: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub allowed: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RealtorSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RealtorSet(address,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                32u8, 128u8, 167u8, 86u8, 68u8, 77u8, 42u8, 240u8, 249u8, 251u8, 79u8,
                135u8, 219u8, 122u8, 246u8, 60u8, 214u8, 226u8, 59u8, 122u8, 18u8, 3u8,
                203u8, 233u8, 171u8, 9u8, 114u8, 238u8, 5u8, 209u8, 234u8, 253u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    realtor: topics.1,
                    allowed: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.allowed,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.realtor.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.realtor,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RealtorSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RealtorSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RealtorSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `SwapRateSet(address,uint256)` and selector `0xa8d3b3213b7f8244e5b04551db9aaabccc21d6212c43c3b3143bcde97ca853ce`.
```solidity
event SwapRateSet(address indexed targetToken, uint256 ratePpm);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SwapRateSet {
        #[allow(missing_docs)]
        pub targetToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ratePpm: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SwapRateSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SwapRateSet(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                168u8, 211u8, 179u8, 33u8, 59u8, 127u8, 130u8, 68u8, 229u8, 176u8, 69u8,
                81u8, 219u8, 154u8, 170u8, 188u8, 204u8, 33u8, 214u8, 33u8, 44u8, 67u8,
                195u8, 179u8, 20u8, 59u8, 205u8, 233u8, 124u8, 168u8, 83u8, 206u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    targetToken: topics.1,
                    ratePpm: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.ratePpm),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.targetToken.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.targetToken,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SwapRateSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SwapRateSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SwapRateSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TokensRescued(address,uint256)` and selector `0x68f67de89e96b13a3ea058af5fd44cc125efceb528482d539c7b43db2faa066e`.
```solidity
event TokensRescued(address token, uint256 amount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TokensRescued {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TokensRescued {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TokensRescued(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                104u8, 246u8, 125u8, 232u8, 158u8, 150u8, 177u8, 58u8, 62u8, 160u8, 88u8,
                175u8, 95u8, 212u8, 76u8, 193u8, 37u8, 239u8, 206u8, 181u8, 40u8, 72u8,
                45u8, 83u8, 156u8, 123u8, 67u8, 219u8, 47u8, 170u8, 6u8, 110u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    token: data.0,
                    amount: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TokensRescued {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TokensRescued> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TokensRescued) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TronReaderSet(address)` and selector `0x5d29693e63b3a084e56893ce8c7f5ef5dc8213da41070084e6b1c4370abd64c4`.
```solidity
event TronReaderSet(address indexed reader);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TronReaderSet {
        #[allow(missing_docs)]
        pub reader: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TronReaderSet {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "TronReaderSet(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                93u8, 41u8, 105u8, 62u8, 99u8, 179u8, 160u8, 132u8, 229u8, 104u8, 147u8,
                206u8, 140u8, 127u8, 94u8, 245u8, 220u8, 130u8, 19u8, 218u8, 65u8, 7u8,
                0u8, 132u8, 230u8, 177u8, 196u8, 55u8, 10u8, 189u8, 100u8, 196u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { reader: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.reader.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.reader,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TronReaderSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TronReaderSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TronReaderSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TronUsdtSet(address)` and selector `0x9f5e1d13045d272fbe74ce4d08e91982a5c57784391ae6a199eecdcf63949ffe`.
```solidity
event TronUsdtSet(address indexed tronUsdt);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TronUsdtSet {
        #[allow(missing_docs)]
        pub tronUsdt: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TronUsdtSet {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "TronUsdtSet(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                159u8, 94u8, 29u8, 19u8, 4u8, 93u8, 39u8, 47u8, 190u8, 116u8, 206u8,
                77u8, 8u8, 233u8, 25u8, 130u8, 165u8, 197u8, 119u8, 132u8, 57u8, 26u8,
                230u8, 161u8, 153u8, 238u8, 205u8, 207u8, 99u8, 148u8, 159u8, 254u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { tronUsdt: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.tronUsdt.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.tronUsdt,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TronUsdtSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TronUsdtSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TronUsdtSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Unpaused(address)` and selector `0x5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa`.
```solidity
event Unpaused(address account);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Unpaused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Unpaused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Unpaused(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                93u8, 185u8, 238u8, 10u8, 73u8, 91u8, 242u8, 230u8, 255u8, 156u8, 145u8,
                167u8, 131u8, 76u8, 27u8, 164u8, 253u8, 210u8, 68u8, 165u8, 232u8, 170u8,
                78u8, 83u8, 123u8, 211u8, 138u8, 234u8, 228u8, 176u8, 115u8, 170u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { account: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Unpaused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Unpaused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Unpaused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UsdtSet(address)` and selector `0xa44f293dfa9228916345a6016220f304fd4e10c2f25ef62c896b4946926a70f4`.
```solidity
event UsdtSet(address indexed usdt);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UsdtSet {
        #[allow(missing_docs)]
        pub usdt: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for UsdtSet {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "UsdtSet(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                164u8, 79u8, 41u8, 61u8, 250u8, 146u8, 40u8, 145u8, 99u8, 69u8, 166u8,
                1u8, 98u8, 32u8, 243u8, 4u8, 253u8, 78u8, 16u8, 194u8, 242u8, 94u8,
                246u8, 44u8, 137u8, 107u8, 73u8, 70u8, 146u8, 106u8, 112u8, 244u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { usdt: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.usdt.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.usdt,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UsdtSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UsdtSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UsdtSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `CONTROLLER_ADDRESS()` and selector `0xb98e631d`.
```solidity
function CONTROLLER_ADDRESS() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CONTROLLER_ADDRESSCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`CONTROLLER_ADDRESS()`](CONTROLLER_ADDRESSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CONTROLLER_ADDRESSReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<CONTROLLER_ADDRESSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: CONTROLLER_ADDRESSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for CONTROLLER_ADDRESSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<CONTROLLER_ADDRESSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: CONTROLLER_ADDRESSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for CONTROLLER_ADDRESSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for CONTROLLER_ADDRESSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CONTROLLER_ADDRESS()";
            const SELECTOR: [u8; 4] = [185u8, 142u8, 99u8, 29u8];
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
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
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
                        let r: CONTROLLER_ADDRESSReturn = r.into();
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
                        let r: CONTROLLER_ADDRESSReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `RECEIVER_IMPL()` and selector `0xde40d89f`.
```solidity
function RECEIVER_IMPL() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RECEIVER_IMPLCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`RECEIVER_IMPL()`](RECEIVER_IMPLCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RECEIVER_IMPLReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<RECEIVER_IMPLCall> for UnderlyingRustTuple<'_> {
                fn from(value: RECEIVER_IMPLCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for RECEIVER_IMPLCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<RECEIVER_IMPLReturn> for UnderlyingRustTuple<'_> {
                fn from(value: RECEIVER_IMPLReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for RECEIVER_IMPLReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for RECEIVER_IMPLCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RECEIVER_IMPL()";
            const SELECTOR: [u8; 4] = [222u8, 64u8, 216u8, 159u8];
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
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
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
                        let r: RECEIVER_IMPLReturn = r.into();
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
                        let r: RECEIVER_IMPLReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `SWAP_EXECUTOR()` and selector `0x0b345879`.
```solidity
function SWAP_EXECUTOR() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SWAP_EXECUTORCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`SWAP_EXECUTOR()`](SWAP_EXECUTORCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SWAP_EXECUTORReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<SWAP_EXECUTORCall> for UnderlyingRustTuple<'_> {
                fn from(value: SWAP_EXECUTORCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for SWAP_EXECUTORCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<SWAP_EXECUTORReturn> for UnderlyingRustTuple<'_> {
                fn from(value: SWAP_EXECUTORReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for SWAP_EXECUTORReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SWAP_EXECUTORCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SWAP_EXECUTOR()";
            const SELECTOR: [u8; 4] = [11u8, 52u8, 88u8, 121u8];
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
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
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
                        let r: SWAP_EXECUTORReturn = r.into();
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
                        let r: SWAP_EXECUTORReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `bridgers(address,uint256)` and selector `0x1dbf4c61`.
```solidity
function bridgers(address, uint256) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bridgersCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`bridgers(address,uint256)`](bridgersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bridgersReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<bridgersCall> for UnderlyingRustTuple<'_> {
                fn from(value: bridgersCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bridgersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<bridgersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: bridgersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bridgersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bridgersCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "bridgers(address,uint256)";
            const SELECTOR: [u8; 4] = [29u8, 191u8, 76u8, 97u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
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
                        let r: bridgersReturn = r.into();
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
                        let r: bridgersReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `claimLocatorByLease(uint256,uint256)` and selector `0x718fbc25`.
```solidity
function claimLocatorByLease(uint256, uint256) external view returns (address targetToken, uint256 queueIndex);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimLocatorByLeaseCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`claimLocatorByLease(uint256,uint256)`](claimLocatorByLeaseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimLocatorByLeaseReturn {
        #[allow(missing_docs)]
        pub targetToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub queueIndex: alloy::sol_types::private::primitives::aliases::U256,
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<claimLocatorByLeaseCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: claimLocatorByLeaseCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for claimLocatorByLeaseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<claimLocatorByLeaseReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: claimLocatorByLeaseReturn) -> Self {
                    (value.targetToken, value.queueIndex)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for claimLocatorByLeaseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetToken: tuple.0,
                        queueIndex: tuple.1,
                    }
                }
            }
        }
        impl claimLocatorByLeaseReturn {
            fn _tokenize(
                &self,
            ) -> <claimLocatorByLeaseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.targetToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.queueIndex),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimLocatorByLeaseCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = claimLocatorByLeaseReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claimLocatorByLease(uint256,uint256)";
            const SELECTOR: [u8; 4] = [113u8, 143u8, 188u8, 37u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                claimLocatorByLeaseReturn::_tokenize(ret)
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
    /**Function with signature `claimsByTargetToken(address,uint256)` and selector `0x78aaf25e`.
```solidity
function claimsByTargetToken(address, uint256) external view returns (uint256 claimId, uint256 amountUsdt, uint256 leaseId, uint256 targetChainId, address beneficiary);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimsByTargetTokenCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`claimsByTargetToken(address,uint256)`](claimsByTargetTokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimsByTargetTokenReturn {
        #[allow(missing_docs)]
        pub claimId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountUsdt: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub leaseId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub targetChainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub beneficiary: alloy::sol_types::private::Address,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<claimsByTargetTokenCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: claimsByTargetTokenCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for claimsByTargetTokenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<claimsByTargetTokenReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: claimsByTargetTokenReturn) -> Self {
                    (
                        value.claimId,
                        value.amountUsdt,
                        value.leaseId,
                        value.targetChainId,
                        value.beneficiary,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for claimsByTargetTokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        claimId: tuple.0,
                        amountUsdt: tuple.1,
                        leaseId: tuple.2,
                        targetChainId: tuple.3,
                        beneficiary: tuple.4,
                    }
                }
            }
        }
        impl claimsByTargetTokenReturn {
            fn _tokenize(
                &self,
            ) -> <claimsByTargetTokenCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.claimId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountUsdt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.leaseId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.targetChainId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.beneficiary,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimsByTargetTokenCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = claimsByTargetTokenReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claimsByTargetToken(address,uint256)";
            const SELECTOR: [u8; 4] = [120u8, 170u8, 242u8, 94u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                claimsByTargetTokenReturn::_tokenize(ret)
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
    /**Function with signature `depositProcessed(bytes32)` and selector `0x88927296`.
```solidity
function depositProcessed(bytes32) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositProcessedCall(pub alloy::sol_types::private::FixedBytes<32>);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`depositProcessed(bytes32)`](depositProcessedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositProcessedReturn {
        #[allow(missing_docs)]
        pub _0: bool,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<depositProcessedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: depositProcessedCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for depositProcessedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
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
            impl ::core::convert::From<depositProcessedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: depositProcessedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for depositProcessedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositProcessedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "depositProcessed(bytes32)";
            const SELECTOR: [u8; 4] = [136u8, 146u8, 114u8, 150u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
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
                        let r: depositProcessedReturn = r.into();
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
                        let r: depositProcessedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `eip712Domain()` and selector `0x84b0196e`.
```solidity
function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`eip712Domain()`](eip712DomainCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainReturn {
        #[allow(missing_docs)]
        pub fields: alloy::sol_types::private::FixedBytes<1>,
        #[allow(missing_docs)]
        pub name: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub version: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub verifyingContract: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub extensions: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
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
            impl ::core::convert::From<eip712DomainCall> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<1>,
                alloy::sol_types::private::String,
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
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
            impl ::core::convert::From<eip712DomainReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainReturn) -> Self {
                    (
                        value.fields,
                        value.name,
                        value.version,
                        value.chainId,
                        value.verifyingContract,
                        value.salt,
                        value.extensions,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        fields: tuple.0,
                        name: tuple.1,
                        version: tuple.2,
                        chainId: tuple.3,
                        verifyingContract: tuple.4,
                        salt: tuple.5,
                        extensions: tuple.6,
                    }
                }
            }
        }
        impl eip712DomainReturn {
            fn _tokenize(
                &self,
            ) -> <eip712DomainCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        1,
                    > as alloy_sol_types::SolType>::tokenize(&self.fields),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.name,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.version,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.verifyingContract,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.extensions),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eip712DomainCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eip712DomainReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eip712Domain()";
            const SELECTOR: [u8; 4] = [132u8, 176u8, 25u8, 110u8];
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
                eip712DomainReturn::_tokenize(ret)
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
    /**Function with signature `eventChainTip()` and selector `0x4d53e931`.
```solidity
function eventChainTip() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eventChainTipCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`eventChainTip()`](eventChainTipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eventChainTipReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<eventChainTipCall> for UnderlyingRustTuple<'_> {
                fn from(value: eventChainTipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eventChainTipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<eventChainTipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eventChainTipReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eventChainTipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eventChainTipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eventChainTip()";
            const SELECTOR: [u8; 4] = [77u8, 83u8, 233u8, 49u8];
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
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: eventChainTipReturn = r.into();
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
                        let r: eventChainTipReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `eventSeq()` and selector `0xe24d5c35`.
```solidity
function eventSeq() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eventSeqCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`eventSeq()`](eventSeqCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eventSeqReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<eventSeqCall> for UnderlyingRustTuple<'_> {
                fn from(value: eventSeqCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eventSeqCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
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
            impl ::core::convert::From<eventSeqReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eventSeqReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eventSeqReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eventSeqCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eventSeq()";
            const SELECTOR: [u8; 4] = [226u8, 77u8, 92u8, 53u8];
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
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: eventSeqReturn = r.into();
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
                        let r: eventSeqReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isChainDeprecated(uint256)` and selector `0x04ec4294`.
```solidity
function isChainDeprecated(uint256) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isChainDeprecatedCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isChainDeprecated(uint256)`](isChainDeprecatedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isChainDeprecatedReturn {
        #[allow(missing_docs)]
        pub _0: bool,
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
            impl ::core::convert::From<isChainDeprecatedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isChainDeprecatedCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isChainDeprecatedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
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
            impl ::core::convert::From<isChainDeprecatedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isChainDeprecatedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isChainDeprecatedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isChainDeprecatedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isChainDeprecated(uint256)";
            const SELECTOR: [u8; 4] = [4u8, 236u8, 66u8, 148u8];
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
                        let r: isChainDeprecatedReturn = r.into();
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
                        let r: isChainDeprecatedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isLpAllowed(address)` and selector `0xbc5c5950`.
```solidity
function isLpAllowed(address) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isLpAllowedCall(pub alloy::sol_types::private::Address);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isLpAllowed(address)`](isLpAllowedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isLpAllowedReturn {
        #[allow(missing_docs)]
        pub _0: bool,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<isLpAllowedCall> for UnderlyingRustTuple<'_> {
                fn from(value: isLpAllowedCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isLpAllowedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
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
            impl ::core::convert::From<isLpAllowedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isLpAllowedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isLpAllowedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isLpAllowedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isLpAllowed(address)";
            const SELECTOR: [u8; 4] = [188u8, 92u8, 89u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.0,
                    ),
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
                        let r: isLpAllowedReturn = r.into();
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
                        let r: isLpAllowedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isRealtor(address)` and selector `0x60b6bfdd`.
```solidity
function isRealtor(address) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRealtorCall(pub alloy::sol_types::private::Address);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isRealtor(address)`](isRealtorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRealtorReturn {
        #[allow(missing_docs)]
        pub _0: bool,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<isRealtorCall> for UnderlyingRustTuple<'_> {
                fn from(value: isRealtorCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isRealtorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
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
            impl ::core::convert::From<isRealtorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isRealtorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isRealtorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isRealtorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isRealtor(address)";
            const SELECTOR: [u8; 4] = [96u8, 182u8, 191u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.0,
                    ),
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
                        let r: isRealtorReturn = r.into();
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
                        let r: isRealtorReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `lastControllerEventSeq()` and selector `0xb371fa69`.
```solidity
function lastControllerEventSeq() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastControllerEventSeqCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`lastControllerEventSeq()`](lastControllerEventSeqCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastControllerEventSeqReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<lastControllerEventSeqCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastControllerEventSeqCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastControllerEventSeqCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
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
            impl ::core::convert::From<lastControllerEventSeqReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastControllerEventSeqReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastControllerEventSeqReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastControllerEventSeqCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastControllerEventSeq()";
            const SELECTOR: [u8; 4] = [179u8, 113u8, 250u8, 105u8];
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
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: lastControllerEventSeqReturn = r.into();
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
                        let r: lastControllerEventSeqReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `lastControllerEventTip()` and selector `0xa6302559`.
```solidity
function lastControllerEventTip() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastControllerEventTipCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`lastControllerEventTip()`](lastControllerEventTipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastControllerEventTipReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<lastControllerEventTipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastControllerEventTipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastControllerEventTipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<lastControllerEventTipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastControllerEventTipReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastControllerEventTipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastControllerEventTipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastControllerEventTip()";
            const SELECTOR: [u8; 4] = [166u8, 48u8, 37u8, 89u8];
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
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: lastControllerEventTipReturn = r.into();
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
                        let r: lastControllerEventTipReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `lastReceiverPullTimestampByToken(bytes32,address)` and selector `0xc63bbf29`.
```solidity
function lastReceiverPullTimestampByToken(bytes32, address) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastReceiverPullTimestampByTokenCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`lastReceiverPullTimestampByToken(bytes32,address)`](lastReceiverPullTimestampByTokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastReceiverPullTimestampByTokenReturn {
        #[allow(missing_docs)]
        pub _0: u64,
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
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<lastReceiverPullTimestampByTokenCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastReceiverPullTimestampByTokenCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastReceiverPullTimestampByTokenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<lastReceiverPullTimestampByTokenReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastReceiverPullTimestampByTokenReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastReceiverPullTimestampByTokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastReceiverPullTimestampByTokenCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u64;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastReceiverPullTimestampByToken(bytes32,address)";
            const SELECTOR: [u8; 4] = [198u8, 59u8, 191u8, 41u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: lastReceiverPullTimestampByTokenReturn = r.into();
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
                        let r: lastReceiverPullTimestampByTokenReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `leaseNonces(uint256)` and selector `0x6c835a82`.
```solidity
function leaseNonces(uint256) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct leaseNoncesCall(pub alloy::sol_types::private::primitives::aliases::U256);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`leaseNonces(uint256)`](leaseNoncesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct leaseNoncesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<leaseNoncesCall> for UnderlyingRustTuple<'_> {
                fn from(value: leaseNoncesCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for leaseNoncesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
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
            impl ::core::convert::From<leaseNoncesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: leaseNoncesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for leaseNoncesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for leaseNoncesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "leaseNonces(uint256)";
            const SELECTOR: [u8; 4] = [108u8, 131u8, 90u8, 130u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: leaseNoncesReturn = r.into();
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
                        let r: leaseNoncesReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `leasesByReceiver(bytes32,uint256)` and selector `0x2f83d9af`.
```solidity
function leasesByReceiver(bytes32, uint256) external view returns (bytes32 receiverSalt, address realtor, address lessee, uint64 startTime, uint64 nukeableAfter, uint32 leaseFeePpm, uint64 flatFee, uint256 recognizedRaw, uint256 backedRaw, uint256 unbackedRaw, UntronV3Base.PayoutConfig memory payout);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct leasesByReceiverCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`leasesByReceiver(bytes32,uint256)`](leasesByReceiverCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct leasesByReceiverReturn {
        #[allow(missing_docs)]
        pub receiverSalt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub realtor: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub lessee: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub startTime: u64,
        #[allow(missing_docs)]
        pub nukeableAfter: u64,
        #[allow(missing_docs)]
        pub leaseFeePpm: u32,
        #[allow(missing_docs)]
        pub flatFee: u64,
        #[allow(missing_docs)]
        pub recognizedRaw: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub backedRaw: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub unbackedRaw: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub payout: <UntronV3Base::PayoutConfig as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<leasesByReceiverCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: leasesByReceiverCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for leasesByReceiverCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                UntronV3Base::PayoutConfig,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u64,
                u64,
                u32,
                u64,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                <UntronV3Base::PayoutConfig as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<leasesByReceiverReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: leasesByReceiverReturn) -> Self {
                    (
                        value.receiverSalt,
                        value.realtor,
                        value.lessee,
                        value.startTime,
                        value.nukeableAfter,
                        value.leaseFeePpm,
                        value.flatFee,
                        value.recognizedRaw,
                        value.backedRaw,
                        value.unbackedRaw,
                        value.payout,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for leasesByReceiverReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        receiverSalt: tuple.0,
                        realtor: tuple.1,
                        lessee: tuple.2,
                        startTime: tuple.3,
                        nukeableAfter: tuple.4,
                        leaseFeePpm: tuple.5,
                        flatFee: tuple.6,
                        recognizedRaw: tuple.7,
                        backedRaw: tuple.8,
                        unbackedRaw: tuple.9,
                        payout: tuple.10,
                    }
                }
            }
        }
        impl leasesByReceiverReturn {
            fn _tokenize(
                &self,
            ) -> <leasesByReceiverCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.receiverSalt),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.realtor,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.lessee,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTime),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.nukeableAfter),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.leaseFeePpm),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.flatFee),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.recognizedRaw),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.backedRaw),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.unbackedRaw),
                    <UntronV3Base::PayoutConfig as alloy_sol_types::SolType>::tokenize(
                        &self.payout,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for leasesByReceiverCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = leasesByReceiverReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                UntronV3Base::PayoutConfig,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "leasesByReceiver(bytes32,uint256)";
            const SELECTOR: [u8; 4] = [47u8, 131u8, 217u8, 175u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                leasesByReceiverReturn::_tokenize(ret)
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
    /**Function with signature `lpPrincipal(address)` and selector `0x4da2f899`.
```solidity
function lpPrincipal(address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lpPrincipalCall(pub alloy::sol_types::private::Address);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`lpPrincipal(address)`](lpPrincipalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lpPrincipalReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<lpPrincipalCall> for UnderlyingRustTuple<'_> {
                fn from(value: lpPrincipalCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lpPrincipalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
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
            impl ::core::convert::From<lpPrincipalReturn> for UnderlyingRustTuple<'_> {
                fn from(value: lpPrincipalReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lpPrincipalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lpPrincipalCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lpPrincipal(address)";
            const SELECTOR: [u8; 4] = [77u8, 162u8, 248u8, 153u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.0,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: lpPrincipalReturn = r.into();
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
                        let r: lpPrincipalReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `nextClaimIdByLease(uint256)` and selector `0xf516a5b4`.
```solidity
function nextClaimIdByLease(uint256) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextClaimIdByLeaseCall(
        pub alloy::sol_types::private::primitives::aliases::U256,
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`nextClaimIdByLease(uint256)`](nextClaimIdByLeaseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextClaimIdByLeaseReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<nextClaimIdByLeaseCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: nextClaimIdByLeaseCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for nextClaimIdByLeaseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
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
            impl ::core::convert::From<nextClaimIdByLeaseReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: nextClaimIdByLeaseReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for nextClaimIdByLeaseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nextClaimIdByLeaseCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nextClaimIdByLease(uint256)";
            const SELECTOR: [u8; 4] = [245u8, 22u8, 165u8, 180u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: nextClaimIdByLeaseReturn = r.into();
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
                        let r: nextClaimIdByLeaseReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `nextControllerEventIndex()` and selector `0xf127a9b3`.
```solidity
function nextControllerEventIndex() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextControllerEventIndexCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`nextControllerEventIndex()`](nextControllerEventIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextControllerEventIndexReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<nextControllerEventIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: nextControllerEventIndexCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for nextControllerEventIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
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
            impl ::core::convert::From<nextControllerEventIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: nextControllerEventIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for nextControllerEventIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nextControllerEventIndexCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nextControllerEventIndex()";
            const SELECTOR: [u8; 4] = [241u8, 39u8, 169u8, 179u8];
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
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: nextControllerEventIndexReturn = r.into();
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
                        let r: nextControllerEventIndexReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `nextIndexByTargetToken(address)` and selector `0xeeb90259`.
```solidity
function nextIndexByTargetToken(address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextIndexByTargetTokenCall(pub alloy::sol_types::private::Address);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`nextIndexByTargetToken(address)`](nextIndexByTargetTokenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextIndexByTargetTokenReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<nextIndexByTargetTokenCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: nextIndexByTargetTokenCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for nextIndexByTargetTokenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
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
            impl ::core::convert::From<nextIndexByTargetTokenReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: nextIndexByTargetTokenReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for nextIndexByTargetTokenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nextIndexByTargetTokenCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nextIndexByTargetToken(address)";
            const SELECTOR: [u8; 4] = [238u8, 185u8, 2u8, 89u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.0,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: nextIndexByTargetTokenReturn = r.into();
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
                        let r: nextIndexByTargetTokenReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `nextLeaseId()` and selector `0x902238e1`.
```solidity
function nextLeaseId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextLeaseIdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`nextLeaseId()`](nextLeaseIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextLeaseIdReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<nextLeaseIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: nextLeaseIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nextLeaseIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
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
            impl ::core::convert::From<nextLeaseIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: nextLeaseIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nextLeaseIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nextLeaseIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nextLeaseId()";
            const SELECTOR: [u8; 4] = [144u8, 34u8, 56u8, 225u8];
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
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: nextLeaseIdReturn = r.into();
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
                        let r: nextLeaseIdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address result);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        #[allow(missing_docs)]
        pub result: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value.result,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { result: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
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
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
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
                        let r: ownerReturn = r.into();
                        r.result
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
                        let r: ownerReturn = r.into();
                        r.result
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `paused()` and selector `0x5c975abb`.
```solidity
function paused() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`paused()`](pausedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedReturn {
        #[allow(missing_docs)]
        pub _0: bool,
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
            impl ::core::convert::From<pausedCall> for UnderlyingRustTuple<'_> {
                fn from(value: pausedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<pausedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pausedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pausedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 90u8, 187u8];
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
                        let r: pausedReturn = r.into();
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
                        let r: pausedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `preEntitle(bytes32,bytes[20],bytes,bytes32[],uint256)` and selector `0xf9fdcae6`.
```solidity
function preEntitle(bytes32 receiverSalt, bytes[20] memory blocks, bytes memory encodedTx, bytes32[] memory proof, uint256 index) external returns (uint256 queueIndex, uint256 leaseId, uint256 netOut);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct preEntitleCall {
        #[allow(missing_docs)]
        pub receiverSalt: alloy::sol_types::private::FixedBytes<32>,
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
    ///Container type for the return parameters of the [`preEntitle(bytes32,bytes[20],bytes,bytes32[],uint256)`](preEntitleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct preEntitleReturn {
        #[allow(missing_docs)]
        pub queueIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub leaseId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub netOut: alloy::sol_types::private::primitives::aliases::U256,
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
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<preEntitleCall> for UnderlyingRustTuple<'_> {
                fn from(value: preEntitleCall) -> Self {
                    (
                        value.receiverSalt,
                        value.blocks,
                        value.encodedTx,
                        value.proof,
                        value.index,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for preEntitleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        receiverSalt: tuple.0,
                        blocks: tuple.1,
                        encodedTx: tuple.2,
                        proof: tuple.3,
                        index: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<preEntitleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: preEntitleReturn) -> Self {
                    (value.queueIndex, value.leaseId, value.netOut)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for preEntitleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        queueIndex: tuple.0,
                        leaseId: tuple.1,
                        netOut: tuple.2,
                    }
                }
            }
        }
        impl preEntitleReturn {
            fn _tokenize(
                &self,
            ) -> <preEntitleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.queueIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.leaseId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.netOut),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for preEntitleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
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
            type Return = preEntitleReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "preEntitle(bytes32,bytes[20],bytes,bytes32[],uint256)";
            const SELECTOR: [u8; 4] = [249u8, 253u8, 202u8, 230u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.receiverSalt),
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
                preEntitleReturn::_tokenize(ret)
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
    /**Function with signature `predictReceiverAddress(bytes32)` and selector `0x3fea3488`.
```solidity
function predictReceiverAddress(bytes32 salt) external view returns (address predicted);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct predictReceiverAddress_0Call {
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`predictReceiverAddress(bytes32)`](predictReceiverAddress_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct predictReceiverAddress_0Return {
        #[allow(missing_docs)]
        pub predicted: alloy::sol_types::private::Address,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<predictReceiverAddress_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: predictReceiverAddress_0Call) -> Self {
                    (value.salt,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for predictReceiverAddress_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { salt: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<predictReceiverAddress_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: predictReceiverAddress_0Return) -> Self {
                    (value.predicted,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for predictReceiverAddress_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { predicted: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for predictReceiverAddress_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "predictReceiverAddress(bytes32)";
            const SELECTOR: [u8; 4] = [63u8, 234u8, 52u8, 136u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
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
                        let r: predictReceiverAddress_0Return = r.into();
                        r.predicted
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
                        let r: predictReceiverAddress_0Return = r.into();
                        r.predicted
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `predictReceiverAddress(address,bytes32)` and selector `0xaa94360c`.
```solidity
function predictReceiverAddress(address controller, bytes32 salt) external view returns (address predicted);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct predictReceiverAddress_1Call {
        #[allow(missing_docs)]
        pub controller: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`predictReceiverAddress(address,bytes32)`](predictReceiverAddress_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct predictReceiverAddress_1Return {
        #[allow(missing_docs)]
        pub predicted: alloy::sol_types::private::Address,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<predictReceiverAddress_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: predictReceiverAddress_1Call) -> Self {
                    (value.controller, value.salt)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for predictReceiverAddress_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        controller: tuple.0,
                        salt: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<predictReceiverAddress_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: predictReceiverAddress_1Return) -> Self {
                    (value.predicted,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for predictReceiverAddress_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { predicted: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for predictReceiverAddress_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "predictReceiverAddress(address,bytes32)";
            const SELECTOR: [u8; 4] = [170u8, 148u8, 54u8, 12u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.controller,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
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
                        let r: predictReceiverAddress_1Return = r.into();
                        r.predicted
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
                        let r: predictReceiverAddress_1Return = r.into();
                        r.predicted
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `protocolPnl()` and selector `0xb7ed020e`.
```solidity
function protocolPnl() external view returns (int256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct protocolPnlCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`protocolPnl()`](protocolPnlCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct protocolPnlReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::I256,
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
            impl ::core::convert::From<protocolPnlCall> for UnderlyingRustTuple<'_> {
                fn from(value: protocolPnlCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for protocolPnlCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::I256,
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
            impl ::core::convert::From<protocolPnlReturn> for UnderlyingRustTuple<'_> {
                fn from(value: protocolPnlReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for protocolPnlReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for protocolPnlCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::I256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "protocolPnl()";
            const SELECTOR: [u8; 4] = [183u8, 237u8, 2u8, 14u8];
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
                (
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: protocolPnlReturn = r.into();
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
                        let r: protocolPnlReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `receiverBytecode()` and selector `0x9efaca79`.
```solidity
function receiverBytecode() external view returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct receiverBytecodeCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`receiverBytecode()`](receiverBytecodeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct receiverBytecodeReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<receiverBytecodeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: receiverBytecodeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for receiverBytecodeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
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
            impl ::core::convert::From<receiverBytecodeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: receiverBytecodeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for receiverBytecodeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for receiverBytecodeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Bytes;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "receiverBytecode()";
            const SELECTOR: [u8; 4] = [158u8, 250u8, 202u8, 121u8];
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
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
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
                        let r: receiverBytecodeReturn = r.into();
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
                        let r: receiverBytecodeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall;
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
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
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
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
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl renounceOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <renounceOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
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
                renounceOwnershipReturn::_tokenize(ret)
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
    /**Function with signature `subjectivePreEntitle(bytes32,uint256,uint256)` and selector `0xa7ec9df9`.
```solidity
function subjectivePreEntitle(bytes32 txId, uint256 leaseId, uint256 rawAmount) external returns (uint256 queueIndex, uint256 netOut);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct subjectivePreEntitleCall {
        #[allow(missing_docs)]
        pub txId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub leaseId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub rawAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`subjectivePreEntitle(bytes32,uint256,uint256)`](subjectivePreEntitleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct subjectivePreEntitleReturn {
        #[allow(missing_docs)]
        pub queueIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub netOut: alloy::sol_types::private::primitives::aliases::U256,
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<subjectivePreEntitleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: subjectivePreEntitleCall) -> Self {
                    (value.txId, value.leaseId, value.rawAmount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for subjectivePreEntitleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        txId: tuple.0,
                        leaseId: tuple.1,
                        rawAmount: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<subjectivePreEntitleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: subjectivePreEntitleReturn) -> Self {
                    (value.queueIndex, value.netOut)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for subjectivePreEntitleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        queueIndex: tuple.0,
                        netOut: tuple.1,
                    }
                }
            }
        }
        impl subjectivePreEntitleReturn {
            fn _tokenize(
                &self,
            ) -> <subjectivePreEntitleCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.queueIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.netOut),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for subjectivePreEntitleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = subjectivePreEntitleReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "subjectivePreEntitle(bytes32,uint256,uint256)";
            const SELECTOR: [u8; 4] = [167u8, 236u8, 157u8, 249u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.leaseId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.rawAmount),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                subjectivePreEntitleReturn::_tokenize(ret)
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
    /**Function with signature `subjectivePreEntitlementByTxId(bytes32)` and selector `0x3d92af84`.
```solidity
function subjectivePreEntitlementByTxId(bytes32) external view returns (address sponsor, uint256 leaseId, uint256 rawAmount, uint256 queueIndex, uint256 claimId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct subjectivePreEntitlementByTxIdCall(
        pub alloy::sol_types::private::FixedBytes<32>,
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`subjectivePreEntitlementByTxId(bytes32)`](subjectivePreEntitlementByTxIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct subjectivePreEntitlementByTxIdReturn {
        #[allow(missing_docs)]
        pub sponsor: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub leaseId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub rawAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub queueIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub claimId: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<subjectivePreEntitlementByTxIdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: subjectivePreEntitlementByTxIdCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for subjectivePreEntitlementByTxIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<subjectivePreEntitlementByTxIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: subjectivePreEntitlementByTxIdReturn) -> Self {
                    (
                        value.sponsor,
                        value.leaseId,
                        value.rawAmount,
                        value.queueIndex,
                        value.claimId,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for subjectivePreEntitlementByTxIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sponsor: tuple.0,
                        leaseId: tuple.1,
                        rawAmount: tuple.2,
                        queueIndex: tuple.3,
                        claimId: tuple.4,
                    }
                }
            }
        }
        impl subjectivePreEntitlementByTxIdReturn {
            fn _tokenize(
                &self,
            ) -> <subjectivePreEntitlementByTxIdCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sponsor,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.leaseId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.rawAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.queueIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.claimId),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for subjectivePreEntitlementByTxIdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = subjectivePreEntitlementByTxIdReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "subjectivePreEntitlementByTxId(bytes32)";
            const SELECTOR: [u8; 4] = [61u8, 146u8, 175u8, 132u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.0),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                subjectivePreEntitlementByTxIdReturn::_tokenize(ret)
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
    /**Function with signature `swapRatePpm(address)` and selector `0xf04e02c0`.
```solidity
function swapRatePpm(address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct swapRatePpmCall(pub alloy::sol_types::private::Address);
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`swapRatePpm(address)`](swapRatePpmCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct swapRatePpmReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<swapRatePpmCall> for UnderlyingRustTuple<'_> {
                fn from(value: swapRatePpmCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for swapRatePpmCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
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
            impl ::core::convert::From<swapRatePpmReturn> for UnderlyingRustTuple<'_> {
                fn from(value: swapRatePpmReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for swapRatePpmReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for swapRatePpmCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "swapRatePpm(address)";
            const SELECTOR: [u8; 4] = [240u8, 78u8, 2u8, 192u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.0,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: swapRatePpmReturn = r.into();
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
                        let r: swapRatePpmReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
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
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl transferOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <transferOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                transferOwnershipReturn::_tokenize(ret)
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
    /**Function with signature `tronReader()` and selector `0x80a72c8b`.
```solidity
function tronReader() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tronReaderCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`tronReader()`](tronReaderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tronReaderReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<tronReaderCall> for UnderlyingRustTuple<'_> {
                fn from(value: tronReaderCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tronReaderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<tronReaderReturn> for UnderlyingRustTuple<'_> {
                fn from(value: tronReaderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tronReaderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tronReaderCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "tronReader()";
            const SELECTOR: [u8; 4] = [128u8, 167u8, 44u8, 139u8];
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
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
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
                        let r: tronReaderReturn = r.into();
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
                        let r: tronReaderReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `tronUsdt()` and selector `0xdc8f8633`.
```solidity
function tronUsdt() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tronUsdtCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`tronUsdt()`](tronUsdtCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tronUsdtReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<tronUsdtCall> for UnderlyingRustTuple<'_> {
                fn from(value: tronUsdtCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tronUsdtCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<tronUsdtReturn> for UnderlyingRustTuple<'_> {
                fn from(value: tronUsdtReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tronUsdtReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tronUsdtCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "tronUsdt()";
            const SELECTOR: [u8; 4] = [220u8, 143u8, 134u8, 51u8];
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
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
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
                        let r: tronUsdtReturn = r.into();
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
                        let r: tronUsdtReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `usdt()` and selector `0x2f48ab7d`.
```solidity
function usdt() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct usdtCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`usdt()`](usdtCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct usdtReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<usdtCall> for UnderlyingRustTuple<'_> {
                fn from(value: usdtCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for usdtCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<usdtReturn> for UnderlyingRustTuple<'_> {
                fn from(value: usdtReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for usdtReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for usdtCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "usdt()";
            const SELECTOR: [u8; 4] = [47u8, 72u8, 171u8, 125u8];
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
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
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
                        let r: usdtReturn = r.into();
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
                        let r: usdtReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `usdtBalance()` and selector `0x482edb07`.
```solidity
function usdtBalance() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct usdtBalanceCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`usdtBalance()`](usdtBalanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct usdtBalanceReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<usdtBalanceCall> for UnderlyingRustTuple<'_> {
                fn from(value: usdtBalanceCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for usdtBalanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
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
            impl ::core::convert::From<usdtBalanceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: usdtBalanceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for usdtBalanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for usdtBalanceCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "usdtBalance()";
            const SELECTOR: [u8; 4] = [72u8, 46u8, 219u8, 7u8];
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
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: usdtBalanceReturn = r.into();
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
                        let r: usdtBalanceReturn = r.into();
                        r._0
                    })
            }
        }
    };
    ///Container for all the [`UntronV3EntitleFacet`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum UntronV3EntitleFacetCalls {
        #[allow(missing_docs)]
        CONTROLLER_ADDRESS(CONTROLLER_ADDRESSCall),
        #[allow(missing_docs)]
        RECEIVER_IMPL(RECEIVER_IMPLCall),
        #[allow(missing_docs)]
        SWAP_EXECUTOR(SWAP_EXECUTORCall),
        #[allow(missing_docs)]
        bridgers(bridgersCall),
        #[allow(missing_docs)]
        claimLocatorByLease(claimLocatorByLeaseCall),
        #[allow(missing_docs)]
        claimsByTargetToken(claimsByTargetTokenCall),
        #[allow(missing_docs)]
        depositProcessed(depositProcessedCall),
        #[allow(missing_docs)]
        eip712Domain(eip712DomainCall),
        #[allow(missing_docs)]
        eventChainTip(eventChainTipCall),
        #[allow(missing_docs)]
        eventSeq(eventSeqCall),
        #[allow(missing_docs)]
        isChainDeprecated(isChainDeprecatedCall),
        #[allow(missing_docs)]
        isLpAllowed(isLpAllowedCall),
        #[allow(missing_docs)]
        isRealtor(isRealtorCall),
        #[allow(missing_docs)]
        lastControllerEventSeq(lastControllerEventSeqCall),
        #[allow(missing_docs)]
        lastControllerEventTip(lastControllerEventTipCall),
        #[allow(missing_docs)]
        lastReceiverPullTimestampByToken(lastReceiverPullTimestampByTokenCall),
        #[allow(missing_docs)]
        leaseNonces(leaseNoncesCall),
        #[allow(missing_docs)]
        leasesByReceiver(leasesByReceiverCall),
        #[allow(missing_docs)]
        lpPrincipal(lpPrincipalCall),
        #[allow(missing_docs)]
        nextClaimIdByLease(nextClaimIdByLeaseCall),
        #[allow(missing_docs)]
        nextControllerEventIndex(nextControllerEventIndexCall),
        #[allow(missing_docs)]
        nextIndexByTargetToken(nextIndexByTargetTokenCall),
        #[allow(missing_docs)]
        nextLeaseId(nextLeaseIdCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        paused(pausedCall),
        #[allow(missing_docs)]
        preEntitle(preEntitleCall),
        #[allow(missing_docs)]
        predictReceiverAddress_0(predictReceiverAddress_0Call),
        #[allow(missing_docs)]
        predictReceiverAddress_1(predictReceiverAddress_1Call),
        #[allow(missing_docs)]
        protocolPnl(protocolPnlCall),
        #[allow(missing_docs)]
        receiverBytecode(receiverBytecodeCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        subjectivePreEntitle(subjectivePreEntitleCall),
        #[allow(missing_docs)]
        subjectivePreEntitlementByTxId(subjectivePreEntitlementByTxIdCall),
        #[allow(missing_docs)]
        swapRatePpm(swapRatePpmCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        tronReader(tronReaderCall),
        #[allow(missing_docs)]
        tronUsdt(tronUsdtCall),
        #[allow(missing_docs)]
        usdt(usdtCall),
        #[allow(missing_docs)]
        usdtBalance(usdtBalanceCall),
    }
    impl UntronV3EntitleFacetCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [4u8, 236u8, 66u8, 148u8],
            [11u8, 52u8, 88u8, 121u8],
            [29u8, 191u8, 76u8, 97u8],
            [47u8, 72u8, 171u8, 125u8],
            [47u8, 131u8, 217u8, 175u8],
            [61u8, 146u8, 175u8, 132u8],
            [63u8, 234u8, 52u8, 136u8],
            [72u8, 46u8, 219u8, 7u8],
            [77u8, 83u8, 233u8, 49u8],
            [77u8, 162u8, 248u8, 153u8],
            [92u8, 151u8, 90u8, 187u8],
            [96u8, 182u8, 191u8, 221u8],
            [108u8, 131u8, 90u8, 130u8],
            [113u8, 80u8, 24u8, 166u8],
            [113u8, 143u8, 188u8, 37u8],
            [120u8, 170u8, 242u8, 94u8],
            [128u8, 167u8, 44u8, 139u8],
            [132u8, 176u8, 25u8, 110u8],
            [136u8, 146u8, 114u8, 150u8],
            [141u8, 165u8, 203u8, 91u8],
            [144u8, 34u8, 56u8, 225u8],
            [158u8, 250u8, 202u8, 121u8],
            [166u8, 48u8, 37u8, 89u8],
            [167u8, 236u8, 157u8, 249u8],
            [170u8, 148u8, 54u8, 12u8],
            [179u8, 113u8, 250u8, 105u8],
            [183u8, 237u8, 2u8, 14u8],
            [185u8, 142u8, 99u8, 29u8],
            [188u8, 92u8, 89u8, 80u8],
            [198u8, 59u8, 191u8, 41u8],
            [220u8, 143u8, 134u8, 51u8],
            [222u8, 64u8, 216u8, 159u8],
            [226u8, 77u8, 92u8, 53u8],
            [238u8, 185u8, 2u8, 89u8],
            [240u8, 78u8, 2u8, 192u8],
            [241u8, 39u8, 169u8, 179u8],
            [242u8, 253u8, 227u8, 139u8],
            [245u8, 22u8, 165u8, 180u8],
            [249u8, 253u8, 202u8, 230u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(isChainDeprecated),
            ::core::stringify!(SWAP_EXECUTOR),
            ::core::stringify!(bridgers),
            ::core::stringify!(usdt),
            ::core::stringify!(leasesByReceiver),
            ::core::stringify!(subjectivePreEntitlementByTxId),
            ::core::stringify!(predictReceiverAddress_0),
            ::core::stringify!(usdtBalance),
            ::core::stringify!(eventChainTip),
            ::core::stringify!(lpPrincipal),
            ::core::stringify!(paused),
            ::core::stringify!(isRealtor),
            ::core::stringify!(leaseNonces),
            ::core::stringify!(renounceOwnership),
            ::core::stringify!(claimLocatorByLease),
            ::core::stringify!(claimsByTargetToken),
            ::core::stringify!(tronReader),
            ::core::stringify!(eip712Domain),
            ::core::stringify!(depositProcessed),
            ::core::stringify!(owner),
            ::core::stringify!(nextLeaseId),
            ::core::stringify!(receiverBytecode),
            ::core::stringify!(lastControllerEventTip),
            ::core::stringify!(subjectivePreEntitle),
            ::core::stringify!(predictReceiverAddress_1),
            ::core::stringify!(lastControllerEventSeq),
            ::core::stringify!(protocolPnl),
            ::core::stringify!(CONTROLLER_ADDRESS),
            ::core::stringify!(isLpAllowed),
            ::core::stringify!(lastReceiverPullTimestampByToken),
            ::core::stringify!(tronUsdt),
            ::core::stringify!(RECEIVER_IMPL),
            ::core::stringify!(eventSeq),
            ::core::stringify!(nextIndexByTargetToken),
            ::core::stringify!(swapRatePpm),
            ::core::stringify!(nextControllerEventIndex),
            ::core::stringify!(transferOwnership),
            ::core::stringify!(nextClaimIdByLease),
            ::core::stringify!(preEntitle),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <isChainDeprecatedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <SWAP_EXECUTORCall as alloy_sol_types::SolCall>::SIGNATURE,
            <bridgersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <usdtCall as alloy_sol_types::SolCall>::SIGNATURE,
            <leasesByReceiverCall as alloy_sol_types::SolCall>::SIGNATURE,
            <subjectivePreEntitlementByTxIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <predictReceiverAddress_0Call as alloy_sol_types::SolCall>::SIGNATURE,
            <usdtBalanceCall as alloy_sol_types::SolCall>::SIGNATURE,
            <eventChainTipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <lpPrincipalCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pausedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isRealtorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <leaseNoncesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <renounceOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimLocatorByLeaseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <claimsByTargetTokenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <tronReaderCall as alloy_sol_types::SolCall>::SIGNATURE,
            <eip712DomainCall as alloy_sol_types::SolCall>::SIGNATURE,
            <depositProcessedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <nextLeaseIdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <receiverBytecodeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <lastControllerEventTipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <subjectivePreEntitleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <predictReceiverAddress_1Call as alloy_sol_types::SolCall>::SIGNATURE,
            <lastControllerEventSeqCall as alloy_sol_types::SolCall>::SIGNATURE,
            <protocolPnlCall as alloy_sol_types::SolCall>::SIGNATURE,
            <CONTROLLER_ADDRESSCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isLpAllowedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <lastReceiverPullTimestampByTokenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <tronUsdtCall as alloy_sol_types::SolCall>::SIGNATURE,
            <RECEIVER_IMPLCall as alloy_sol_types::SolCall>::SIGNATURE,
            <eventSeqCall as alloy_sol_types::SolCall>::SIGNATURE,
            <nextIndexByTargetTokenCall as alloy_sol_types::SolCall>::SIGNATURE,
            <swapRatePpmCall as alloy_sol_types::SolCall>::SIGNATURE,
            <nextControllerEventIndexCall as alloy_sol_types::SolCall>::SIGNATURE,
            <transferOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
            <nextClaimIdByLeaseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <preEntitleCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for UntronV3EntitleFacetCalls {
        const NAME: &'static str = "UntronV3EntitleFacetCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 39usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::CONTROLLER_ADDRESS(_) => {
                    <CONTROLLER_ADDRESSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::RECEIVER_IMPL(_) => {
                    <RECEIVER_IMPLCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::SWAP_EXECUTOR(_) => {
                    <SWAP_EXECUTORCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::bridgers(_) => <bridgersCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::claimLocatorByLease(_) => {
                    <claimLocatorByLeaseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::claimsByTargetToken(_) => {
                    <claimsByTargetTokenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::depositProcessed(_) => {
                    <depositProcessedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eventChainTip(_) => {
                    <eventChainTipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eventSeq(_) => <eventSeqCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::isChainDeprecated(_) => {
                    <isChainDeprecatedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isLpAllowed(_) => {
                    <isLpAllowedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isRealtor(_) => {
                    <isRealtorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastControllerEventSeq(_) => {
                    <lastControllerEventSeqCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastControllerEventTip(_) => {
                    <lastControllerEventTipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastReceiverPullTimestampByToken(_) => {
                    <lastReceiverPullTimestampByTokenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::leaseNonces(_) => {
                    <leaseNoncesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::leasesByReceiver(_) => {
                    <leasesByReceiverCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lpPrincipal(_) => {
                    <lpPrincipalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nextClaimIdByLease(_) => {
                    <nextClaimIdByLeaseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nextControllerEventIndex(_) => {
                    <nextControllerEventIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nextIndexByTargetToken(_) => {
                    <nextIndexByTargetTokenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nextLeaseId(_) => {
                    <nextLeaseIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused(_) => <pausedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::preEntitle(_) => {
                    <preEntitleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::predictReceiverAddress_0(_) => {
                    <predictReceiverAddress_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::predictReceiverAddress_1(_) => {
                    <predictReceiverAddress_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::protocolPnl(_) => {
                    <protocolPnlCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::receiverBytecode(_) => {
                    <receiverBytecodeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::subjectivePreEntitle(_) => {
                    <subjectivePreEntitleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::subjectivePreEntitlementByTxId(_) => {
                    <subjectivePreEntitlementByTxIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::swapRatePpm(_) => {
                    <swapRatePpmCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::tronReader(_) => {
                    <tronReaderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::tronUsdt(_) => <tronUsdtCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::usdt(_) => <usdtCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::usdtBalance(_) => {
                    <usdtBalanceCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls>] = &[
                {
                    fn isChainDeprecated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <isChainDeprecatedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::isChainDeprecated)
                    }
                    isChainDeprecated
                },
                {
                    fn SWAP_EXECUTOR(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <SWAP_EXECUTORCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::SWAP_EXECUTOR)
                    }
                    SWAP_EXECUTOR
                },
                {
                    fn bridgers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <bridgersCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(UntronV3EntitleFacetCalls::bridgers)
                    }
                    bridgers
                },
                {
                    fn usdt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <usdtCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(UntronV3EntitleFacetCalls::usdt)
                    }
                    usdt
                },
                {
                    fn leasesByReceiver(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <leasesByReceiverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::leasesByReceiver)
                    }
                    leasesByReceiver
                },
                {
                    fn subjectivePreEntitlementByTxId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <subjectivePreEntitlementByTxIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                UntronV3EntitleFacetCalls::subjectivePreEntitlementByTxId,
                            )
                    }
                    subjectivePreEntitlementByTxId
                },
                {
                    fn predictReceiverAddress_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <predictReceiverAddress_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::predictReceiverAddress_0)
                    }
                    predictReceiverAddress_0
                },
                {
                    fn usdtBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <usdtBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::usdtBalance)
                    }
                    usdtBalance
                },
                {
                    fn eventChainTip(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <eventChainTipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::eventChainTip)
                    }
                    eventChainTip
                },
                {
                    fn lpPrincipal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <lpPrincipalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::lpPrincipal)
                    }
                    lpPrincipal
                },
                {
                    fn paused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(UntronV3EntitleFacetCalls::paused)
                    }
                    paused
                },
                {
                    fn isRealtor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <isRealtorCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(UntronV3EntitleFacetCalls::isRealtor)
                    }
                    isRealtor
                },
                {
                    fn leaseNonces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <leaseNoncesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::leaseNonces)
                    }
                    leaseNonces
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn claimLocatorByLease(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <claimLocatorByLeaseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::claimLocatorByLease)
                    }
                    claimLocatorByLease
                },
                {
                    fn claimsByTargetToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <claimsByTargetTokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::claimsByTargetToken)
                    }
                    claimsByTargetToken
                },
                {
                    fn tronReader(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <tronReaderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::tronReader)
                    }
                    tronReader
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn depositProcessed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <depositProcessedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::depositProcessed)
                    }
                    depositProcessed
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(UntronV3EntitleFacetCalls::owner)
                    }
                    owner
                },
                {
                    fn nextLeaseId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <nextLeaseIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::nextLeaseId)
                    }
                    nextLeaseId
                },
                {
                    fn receiverBytecode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <receiverBytecodeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::receiverBytecode)
                    }
                    receiverBytecode
                },
                {
                    fn lastControllerEventTip(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <lastControllerEventTipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::lastControllerEventTip)
                    }
                    lastControllerEventTip
                },
                {
                    fn subjectivePreEntitle(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <subjectivePreEntitleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::subjectivePreEntitle)
                    }
                    subjectivePreEntitle
                },
                {
                    fn predictReceiverAddress_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <predictReceiverAddress_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::predictReceiverAddress_1)
                    }
                    predictReceiverAddress_1
                },
                {
                    fn lastControllerEventSeq(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <lastControllerEventSeqCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::lastControllerEventSeq)
                    }
                    lastControllerEventSeq
                },
                {
                    fn protocolPnl(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <protocolPnlCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::protocolPnl)
                    }
                    protocolPnl
                },
                {
                    fn CONTROLLER_ADDRESS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <CONTROLLER_ADDRESSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::CONTROLLER_ADDRESS)
                    }
                    CONTROLLER_ADDRESS
                },
                {
                    fn isLpAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <isLpAllowedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::isLpAllowed)
                    }
                    isLpAllowed
                },
                {
                    fn lastReceiverPullTimestampByToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <lastReceiverPullTimestampByTokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                UntronV3EntitleFacetCalls::lastReceiverPullTimestampByToken,
                            )
                    }
                    lastReceiverPullTimestampByToken
                },
                {
                    fn tronUsdt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <tronUsdtCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(UntronV3EntitleFacetCalls::tronUsdt)
                    }
                    tronUsdt
                },
                {
                    fn RECEIVER_IMPL(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <RECEIVER_IMPLCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::RECEIVER_IMPL)
                    }
                    RECEIVER_IMPL
                },
                {
                    fn eventSeq(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <eventSeqCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(UntronV3EntitleFacetCalls::eventSeq)
                    }
                    eventSeq
                },
                {
                    fn nextIndexByTargetToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <nextIndexByTargetTokenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::nextIndexByTargetToken)
                    }
                    nextIndexByTargetToken
                },
                {
                    fn swapRatePpm(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <swapRatePpmCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::swapRatePpm)
                    }
                    swapRatePpm
                },
                {
                    fn nextControllerEventIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <nextControllerEventIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::nextControllerEventIndex)
                    }
                    nextControllerEventIndex
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn nextClaimIdByLease(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <nextClaimIdByLeaseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::nextClaimIdByLease)
                    }
                    nextClaimIdByLease
                },
                {
                    fn preEntitle(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <preEntitleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::preEntitle)
                    }
                    preEntitle
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
            ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls>] = &[
                {
                    fn isChainDeprecated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <isChainDeprecatedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::isChainDeprecated)
                    }
                    isChainDeprecated
                },
                {
                    fn SWAP_EXECUTOR(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <SWAP_EXECUTORCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::SWAP_EXECUTOR)
                    }
                    SWAP_EXECUTOR
                },
                {
                    fn bridgers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <bridgersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::bridgers)
                    }
                    bridgers
                },
                {
                    fn usdt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <usdtCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::usdt)
                    }
                    usdt
                },
                {
                    fn leasesByReceiver(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <leasesByReceiverCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::leasesByReceiver)
                    }
                    leasesByReceiver
                },
                {
                    fn subjectivePreEntitlementByTxId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <subjectivePreEntitlementByTxIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                UntronV3EntitleFacetCalls::subjectivePreEntitlementByTxId,
                            )
                    }
                    subjectivePreEntitlementByTxId
                },
                {
                    fn predictReceiverAddress_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <predictReceiverAddress_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::predictReceiverAddress_0)
                    }
                    predictReceiverAddress_0
                },
                {
                    fn usdtBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <usdtBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::usdtBalance)
                    }
                    usdtBalance
                },
                {
                    fn eventChainTip(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <eventChainTipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::eventChainTip)
                    }
                    eventChainTip
                },
                {
                    fn lpPrincipal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <lpPrincipalCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::lpPrincipal)
                    }
                    lpPrincipal
                },
                {
                    fn paused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::paused)
                    }
                    paused
                },
                {
                    fn isRealtor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <isRealtorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::isRealtor)
                    }
                    isRealtor
                },
                {
                    fn leaseNonces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <leaseNoncesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::leaseNonces)
                    }
                    leaseNonces
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn claimLocatorByLease(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <claimLocatorByLeaseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::claimLocatorByLease)
                    }
                    claimLocatorByLease
                },
                {
                    fn claimsByTargetToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <claimsByTargetTokenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::claimsByTargetToken)
                    }
                    claimsByTargetToken
                },
                {
                    fn tronReader(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <tronReaderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::tronReader)
                    }
                    tronReader
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn depositProcessed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <depositProcessedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::depositProcessed)
                    }
                    depositProcessed
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::owner)
                    }
                    owner
                },
                {
                    fn nextLeaseId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <nextLeaseIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::nextLeaseId)
                    }
                    nextLeaseId
                },
                {
                    fn receiverBytecode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <receiverBytecodeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::receiverBytecode)
                    }
                    receiverBytecode
                },
                {
                    fn lastControllerEventTip(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <lastControllerEventTipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::lastControllerEventTip)
                    }
                    lastControllerEventTip
                },
                {
                    fn subjectivePreEntitle(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <subjectivePreEntitleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::subjectivePreEntitle)
                    }
                    subjectivePreEntitle
                },
                {
                    fn predictReceiverAddress_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <predictReceiverAddress_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::predictReceiverAddress_1)
                    }
                    predictReceiverAddress_1
                },
                {
                    fn lastControllerEventSeq(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <lastControllerEventSeqCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::lastControllerEventSeq)
                    }
                    lastControllerEventSeq
                },
                {
                    fn protocolPnl(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <protocolPnlCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::protocolPnl)
                    }
                    protocolPnl
                },
                {
                    fn CONTROLLER_ADDRESS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <CONTROLLER_ADDRESSCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::CONTROLLER_ADDRESS)
                    }
                    CONTROLLER_ADDRESS
                },
                {
                    fn isLpAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <isLpAllowedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::isLpAllowed)
                    }
                    isLpAllowed
                },
                {
                    fn lastReceiverPullTimestampByToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <lastReceiverPullTimestampByTokenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                UntronV3EntitleFacetCalls::lastReceiverPullTimestampByToken,
                            )
                    }
                    lastReceiverPullTimestampByToken
                },
                {
                    fn tronUsdt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <tronUsdtCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::tronUsdt)
                    }
                    tronUsdt
                },
                {
                    fn RECEIVER_IMPL(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <RECEIVER_IMPLCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::RECEIVER_IMPL)
                    }
                    RECEIVER_IMPL
                },
                {
                    fn eventSeq(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <eventSeqCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::eventSeq)
                    }
                    eventSeq
                },
                {
                    fn nextIndexByTargetToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <nextIndexByTargetTokenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::nextIndexByTargetToken)
                    }
                    nextIndexByTargetToken
                },
                {
                    fn swapRatePpm(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <swapRatePpmCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::swapRatePpm)
                    }
                    swapRatePpm
                },
                {
                    fn nextControllerEventIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <nextControllerEventIndexCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::nextControllerEventIndex)
                    }
                    nextControllerEventIndex
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn nextClaimIdByLease(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <nextClaimIdByLeaseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::nextClaimIdByLease)
                    }
                    nextClaimIdByLease
                },
                {
                    fn preEntitle(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetCalls> {
                        <preEntitleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetCalls::preEntitle)
                    }
                    preEntitle
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
                Self::CONTROLLER_ADDRESS(inner) => {
                    <CONTROLLER_ADDRESSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::RECEIVER_IMPL(inner) => {
                    <RECEIVER_IMPLCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SWAP_EXECUTOR(inner) => {
                    <SWAP_EXECUTORCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::bridgers(inner) => {
                    <bridgersCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::claimLocatorByLease(inner) => {
                    <claimLocatorByLeaseCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::claimsByTargetToken(inner) => {
                    <claimsByTargetTokenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::depositProcessed(inner) => {
                    <depositProcessedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eventChainTip(inner) => {
                    <eventChainTipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eventSeq(inner) => {
                    <eventSeqCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isChainDeprecated(inner) => {
                    <isChainDeprecatedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isLpAllowed(inner) => {
                    <isLpAllowedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isRealtor(inner) => {
                    <isRealtorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::lastControllerEventSeq(inner) => {
                    <lastControllerEventSeqCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lastControllerEventTip(inner) => {
                    <lastControllerEventTipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lastReceiverPullTimestampByToken(inner) => {
                    <lastReceiverPullTimestampByTokenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::leaseNonces(inner) => {
                    <leaseNoncesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::leasesByReceiver(inner) => {
                    <leasesByReceiverCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lpPrincipal(inner) => {
                    <lpPrincipalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::nextClaimIdByLease(inner) => {
                    <nextClaimIdByLeaseCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::nextControllerEventIndex(inner) => {
                    <nextControllerEventIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::nextIndexByTargetToken(inner) => {
                    <nextIndexByTargetTokenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::nextLeaseId(inner) => {
                    <nextLeaseIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::preEntitle(inner) => {
                    <preEntitleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::predictReceiverAddress_0(inner) => {
                    <predictReceiverAddress_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::predictReceiverAddress_1(inner) => {
                    <predictReceiverAddress_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::protocolPnl(inner) => {
                    <protocolPnlCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::receiverBytecode(inner) => {
                    <receiverBytecodeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::subjectivePreEntitle(inner) => {
                    <subjectivePreEntitleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::subjectivePreEntitlementByTxId(inner) => {
                    <subjectivePreEntitlementByTxIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::swapRatePpm(inner) => {
                    <swapRatePpmCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::tronReader(inner) => {
                    <tronReaderCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::tronUsdt(inner) => {
                    <tronUsdtCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::usdt(inner) => {
                    <usdtCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::usdtBalance(inner) => {
                    <usdtBalanceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::CONTROLLER_ADDRESS(inner) => {
                    <CONTROLLER_ADDRESSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::RECEIVER_IMPL(inner) => {
                    <RECEIVER_IMPLCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SWAP_EXECUTOR(inner) => {
                    <SWAP_EXECUTORCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::bridgers(inner) => {
                    <bridgersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::claimLocatorByLease(inner) => {
                    <claimLocatorByLeaseCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::claimsByTargetToken(inner) => {
                    <claimsByTargetTokenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::depositProcessed(inner) => {
                    <depositProcessedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eventChainTip(inner) => {
                    <eventChainTipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eventSeq(inner) => {
                    <eventSeqCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isChainDeprecated(inner) => {
                    <isChainDeprecatedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isLpAllowed(inner) => {
                    <isLpAllowedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isRealtor(inner) => {
                    <isRealtorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lastControllerEventSeq(inner) => {
                    <lastControllerEventSeqCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lastControllerEventTip(inner) => {
                    <lastControllerEventTipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lastReceiverPullTimestampByToken(inner) => {
                    <lastReceiverPullTimestampByTokenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::leaseNonces(inner) => {
                    <leaseNoncesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::leasesByReceiver(inner) => {
                    <leasesByReceiverCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lpPrincipal(inner) => {
                    <lpPrincipalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::nextClaimIdByLease(inner) => {
                    <nextClaimIdByLeaseCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::nextControllerEventIndex(inner) => {
                    <nextControllerEventIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::nextIndexByTargetToken(inner) => {
                    <nextIndexByTargetTokenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::nextLeaseId(inner) => {
                    <nextLeaseIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::preEntitle(inner) => {
                    <preEntitleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::predictReceiverAddress_0(inner) => {
                    <predictReceiverAddress_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::predictReceiverAddress_1(inner) => {
                    <predictReceiverAddress_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::protocolPnl(inner) => {
                    <protocolPnlCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::receiverBytecode(inner) => {
                    <receiverBytecodeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::subjectivePreEntitle(inner) => {
                    <subjectivePreEntitleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::subjectivePreEntitlementByTxId(inner) => {
                    <subjectivePreEntitlementByTxIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::swapRatePpm(inner) => {
                    <swapRatePpmCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::tronReader(inner) => {
                    <tronReaderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::tronUsdt(inner) => {
                    <tronUsdtCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::usdt(inner) => {
                    <usdtCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::usdtBalance(inner) => {
                    <usdtBalanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`UntronV3EntitleFacet`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum UntronV3EntitleFacetErrors {
        #[allow(missing_docs)]
        AlreadyInitialized(AlreadyInitialized),
        #[allow(missing_docs)]
        AmountTooLargeForInt(AmountTooLargeForInt),
        #[allow(missing_docs)]
        CannotRescueUSDT(CannotRescueUSDT),
        #[allow(missing_docs)]
        ChainDeprecated(ChainDeprecated),
        #[allow(missing_docs)]
        DepositAlreadyProcessed(DepositAlreadyProcessed),
        #[allow(missing_docs)]
        DepositNotAfterLastReceiverPull(DepositNotAfterLastReceiverPull),
        #[allow(missing_docs)]
        EnforcedPause(EnforcedPause),
        #[allow(missing_docs)]
        EventRelayNoProgress(EventRelayNoProgress),
        #[allow(missing_docs)]
        EventTipMismatch(EventTipMismatch),
        #[allow(missing_docs)]
        ExpectedPause(ExpectedPause),
        #[allow(missing_docs)]
        InsufficientLpPrincipal(InsufficientLpPrincipal),
        #[allow(missing_docs)]
        InsufficientProtocolProfit(InsufficientProtocolProfit),
        #[allow(missing_docs)]
        InsufficientUsdtBalance(InsufficientUsdtBalance),
        #[allow(missing_docs)]
        InvalidLeaseId(InvalidLeaseId),
        #[allow(missing_docs)]
        InvalidLeaseTimeframe(InvalidLeaseTimeframe),
        #[allow(missing_docs)]
        InvalidReceiverForSalt(InvalidReceiverForSalt),
        #[allow(missing_docs)]
        InvalidSignature(InvalidSignature),
        #[allow(missing_docs)]
        InvalidTargetToken(InvalidTargetToken),
        #[allow(missing_docs)]
        LeaseDurationTooLong(LeaseDurationTooLong),
        #[allow(missing_docs)]
        LeaseFeeTooLow(LeaseFeeTooLow),
        #[allow(missing_docs)]
        LeaseFlatFeeTooLow(LeaseFlatFeeTooLow),
        #[allow(missing_docs)]
        LeaseNotNukeableYet(LeaseNotNukeableYet),
        #[allow(missing_docs)]
        LeaseRateLimitConfigInvalid(LeaseRateLimitConfigInvalid),
        #[allow(missing_docs)]
        LeaseRateLimitExceeded(LeaseRateLimitExceeded),
        #[allow(missing_docs)]
        LpNotAllowlisted(LpNotAllowlisted),
        #[allow(missing_docs)]
        NewOwnerIsZeroAddress(NewOwnerIsZeroAddress),
        #[allow(missing_docs)]
        NoActiveLease(NoActiveLease),
        #[allow(missing_docs)]
        NoBridger(NoBridger),
        #[allow(missing_docs)]
        NotATrc20Transfer(NotATrc20Transfer),
        #[allow(missing_docs)]
        NotEventChainTip(NotEventChainTip),
        #[allow(missing_docs)]
        NotLessee(NotLessee),
        #[allow(missing_docs)]
        NotRealtor(NotRealtor),
        #[allow(missing_docs)]
        NotTronUsdt(NotTronUsdt),
        #[allow(missing_docs)]
        PayoutConfigRateLimitConfigInvalid(PayoutConfigRateLimitConfigInvalid),
        #[allow(missing_docs)]
        PayoutConfigRateLimitExceeded(PayoutConfigRateLimitExceeded),
        #[allow(missing_docs)]
        RateNotSet(RateNotSet),
        #[allow(missing_docs)]
        Reentrancy(Reentrancy),
        #[allow(missing_docs)]
        SignatureExpired(SignatureExpired),
        #[allow(missing_docs)]
        SubjectiveNetOutZero(SubjectiveNetOutZero),
        #[allow(missing_docs)]
        SubjectivePreEntitlementAlreadyExists(SubjectivePreEntitlementAlreadyExists),
        #[allow(missing_docs)]
        TronInvalidCalldataLength(TronInvalidCalldataLength),
        #[allow(missing_docs)]
        TronInvalidTrc20DataLength(TronInvalidTrc20DataLength),
        #[allow(missing_docs)]
        Unauthorized(Unauthorized),
        #[allow(missing_docs)]
        WithdrawExceedsPrincipal(WithdrawExceedsPrincipal),
        #[allow(missing_docs)]
        ZeroAmount(ZeroAmount),
    }
    impl UntronV3EntitleFacetErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 25u8, 189u8, 205u8],
            [8u8, 252u8, 127u8, 206u8],
            [10u8, 131u8, 176u8, 132u8],
            [13u8, 173u8, 142u8, 52u8],
            [13u8, 193u8, 73u8, 240u8],
            [14u8, 82u8, 28u8, 67u8],
            [18u8, 121u8, 149u8, 3u8],
            [22u8, 103u8, 220u8, 116u8],
            [28u8, 78u8, 27u8, 255u8],
            [31u8, 42u8, 32u8, 5u8],
            [36u8, 59u8, 79u8, 64u8],
            [36u8, 86u8, 225u8, 236u8],
            [51u8, 149u8, 118u8, 91u8],
            [55u8, 214u8, 226u8, 10u8],
            [63u8, 65u8, 174u8, 1u8],
            [66u8, 101u8, 130u8, 50u8],
            [74u8, 7u8, 118u8, 102u8],
            [76u8, 75u8, 111u8, 2u8],
            [76u8, 84u8, 42u8, 228u8],
            [80u8, 29u8, 19u8, 237u8],
            [108u8, 182u8, 124u8, 166u8],
            [113u8, 54u8, 244u8, 242u8],
            [116u8, 72u8, 251u8, 174u8],
            [128u8, 202u8, 29u8, 82u8],
            [130u8, 180u8, 41u8, 0u8],
            [133u8, 98u8, 235u8, 69u8],
            [138u8, 40u8, 67u8, 122u8],
            [139u8, 170u8, 87u8, 159u8],
            [141u8, 252u8, 32u8, 43u8],
            [148u8, 171u8, 40u8, 214u8],
            [154u8, 198u8, 12u8, 45u8],
            [157u8, 140u8, 125u8, 153u8],
            [161u8, 126u8, 234u8, 119u8],
            [170u8, 225u8, 49u8, 28u8],
            [171u8, 20u8, 60u8, 6u8],
            [172u8, 242u8, 218u8, 147u8],
            [179u8, 124u8, 121u8, 237u8],
            [180u8, 237u8, 28u8, 53u8],
            [181u8, 92u8, 27u8, 173u8],
            [204u8, 11u8, 176u8, 193u8],
            [212u8, 112u8, 140u8, 165u8],
            [214u8, 193u8, 70u8, 124u8],
            [217u8, 37u8, 156u8, 169u8],
            [217u8, 60u8, 6u8, 101u8],
            [249u8, 176u8, 28u8, 68u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(SignatureExpired),
            ::core::stringify!(RateNotSet),
            ::core::stringify!(PayoutConfigRateLimitExceeded),
            ::core::stringify!(LpNotAllowlisted),
            ::core::stringify!(AlreadyInitialized),
            ::core::stringify!(EventTipMismatch),
            ::core::stringify!(TronInvalidCalldataLength),
            ::core::stringify!(AmountTooLargeForInt),
            ::core::stringify!(CannotRescueUSDT),
            ::core::stringify!(ZeroAmount),
            ::core::stringify!(InvalidLeaseId),
            ::core::stringify!(NotATrc20Transfer),
            ::core::stringify!(NotTronUsdt),
            ::core::stringify!(DepositAlreadyProcessed),
            ::core::stringify!(SubjectiveNetOutZero),
            ::core::stringify!(LeaseRateLimitConfigInvalid),
            ::core::stringify!(NoActiveLease),
            ::core::stringify!(LeaseFlatFeeTooLow),
            ::core::stringify!(LeaseRateLimitExceeded),
            ::core::stringify!(InvalidLeaseTimeframe),
            ::core::stringify!(NotEventChainTip),
            ::core::stringify!(NotLessee),
            ::core::stringify!(NewOwnerIsZeroAddress),
            ::core::stringify!(InvalidReceiverForSalt),
            ::core::stringify!(Unauthorized),
            ::core::stringify!(InvalidTargetToken),
            ::core::stringify!(NotRealtor),
            ::core::stringify!(InvalidSignature),
            ::core::stringify!(ExpectedPause),
            ::core::stringify!(LeaseFeeTooLow),
            ::core::stringify!(TronInvalidTrc20DataLength),
            ::core::stringify!(InsufficientProtocolProfit),
            ::core::stringify!(PayoutConfigRateLimitConfigInvalid),
            ::core::stringify!(LeaseDurationTooLong),
            ::core::stringify!(Reentrancy),
            ::core::stringify!(InsufficientLpPrincipal),
            ::core::stringify!(NoBridger),
            ::core::stringify!(LeaseNotNukeableYet),
            ::core::stringify!(InsufficientUsdtBalance),
            ::core::stringify!(SubjectivePreEntitlementAlreadyExists),
            ::core::stringify!(WithdrawExceedsPrincipal),
            ::core::stringify!(EventRelayNoProgress),
            ::core::stringify!(ChainDeprecated),
            ::core::stringify!(EnforcedPause),
            ::core::stringify!(DepositNotAfterLastReceiverPull),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <SignatureExpired as alloy_sol_types::SolError>::SIGNATURE,
            <RateNotSet as alloy_sol_types::SolError>::SIGNATURE,
            <PayoutConfigRateLimitExceeded as alloy_sol_types::SolError>::SIGNATURE,
            <LpNotAllowlisted as alloy_sol_types::SolError>::SIGNATURE,
            <AlreadyInitialized as alloy_sol_types::SolError>::SIGNATURE,
            <EventTipMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <TronInvalidCalldataLength as alloy_sol_types::SolError>::SIGNATURE,
            <AmountTooLargeForInt as alloy_sol_types::SolError>::SIGNATURE,
            <CannotRescueUSDT as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroAmount as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidLeaseId as alloy_sol_types::SolError>::SIGNATURE,
            <NotATrc20Transfer as alloy_sol_types::SolError>::SIGNATURE,
            <NotTronUsdt as alloy_sol_types::SolError>::SIGNATURE,
            <DepositAlreadyProcessed as alloy_sol_types::SolError>::SIGNATURE,
            <SubjectiveNetOutZero as alloy_sol_types::SolError>::SIGNATURE,
            <LeaseRateLimitConfigInvalid as alloy_sol_types::SolError>::SIGNATURE,
            <NoActiveLease as alloy_sol_types::SolError>::SIGNATURE,
            <LeaseFlatFeeTooLow as alloy_sol_types::SolError>::SIGNATURE,
            <LeaseRateLimitExceeded as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidLeaseTimeframe as alloy_sol_types::SolError>::SIGNATURE,
            <NotEventChainTip as alloy_sol_types::SolError>::SIGNATURE,
            <NotLessee as alloy_sol_types::SolError>::SIGNATURE,
            <NewOwnerIsZeroAddress as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidReceiverForSalt as alloy_sol_types::SolError>::SIGNATURE,
            <Unauthorized as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidTargetToken as alloy_sol_types::SolError>::SIGNATURE,
            <NotRealtor as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidSignature as alloy_sol_types::SolError>::SIGNATURE,
            <ExpectedPause as alloy_sol_types::SolError>::SIGNATURE,
            <LeaseFeeTooLow as alloy_sol_types::SolError>::SIGNATURE,
            <TronInvalidTrc20DataLength as alloy_sol_types::SolError>::SIGNATURE,
            <InsufficientProtocolProfit as alloy_sol_types::SolError>::SIGNATURE,
            <PayoutConfigRateLimitConfigInvalid as alloy_sol_types::SolError>::SIGNATURE,
            <LeaseDurationTooLong as alloy_sol_types::SolError>::SIGNATURE,
            <Reentrancy as alloy_sol_types::SolError>::SIGNATURE,
            <InsufficientLpPrincipal as alloy_sol_types::SolError>::SIGNATURE,
            <NoBridger as alloy_sol_types::SolError>::SIGNATURE,
            <LeaseNotNukeableYet as alloy_sol_types::SolError>::SIGNATURE,
            <InsufficientUsdtBalance as alloy_sol_types::SolError>::SIGNATURE,
            <SubjectivePreEntitlementAlreadyExists as alloy_sol_types::SolError>::SIGNATURE,
            <WithdrawExceedsPrincipal as alloy_sol_types::SolError>::SIGNATURE,
            <EventRelayNoProgress as alloy_sol_types::SolError>::SIGNATURE,
            <ChainDeprecated as alloy_sol_types::SolError>::SIGNATURE,
            <EnforcedPause as alloy_sol_types::SolError>::SIGNATURE,
            <DepositNotAfterLastReceiverPull as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for UntronV3EntitleFacetErrors {
        const NAME: &'static str = "UntronV3EntitleFacetErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 45usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AlreadyInitialized(_) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AmountTooLargeForInt(_) => {
                    <AmountTooLargeForInt as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CannotRescueUSDT(_) => {
                    <CannotRescueUSDT as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ChainDeprecated(_) => {
                    <ChainDeprecated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DepositAlreadyProcessed(_) => {
                    <DepositAlreadyProcessed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DepositNotAfterLastReceiverPull(_) => {
                    <DepositNotAfterLastReceiverPull as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EnforcedPause(_) => {
                    <EnforcedPause as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EventRelayNoProgress(_) => {
                    <EventRelayNoProgress as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EventTipMismatch(_) => {
                    <EventTipMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ExpectedPause(_) => {
                    <ExpectedPause as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientLpPrincipal(_) => {
                    <InsufficientLpPrincipal as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientProtocolProfit(_) => {
                    <InsufficientProtocolProfit as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientUsdtBalance(_) => {
                    <InsufficientUsdtBalance as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidLeaseId(_) => {
                    <InvalidLeaseId as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidLeaseTimeframe(_) => {
                    <InvalidLeaseTimeframe as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidReceiverForSalt(_) => {
                    <InvalidReceiverForSalt as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidTargetToken(_) => {
                    <InvalidTargetToken as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LeaseDurationTooLong(_) => {
                    <LeaseDurationTooLong as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LeaseFeeTooLow(_) => {
                    <LeaseFeeTooLow as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LeaseFlatFeeTooLow(_) => {
                    <LeaseFlatFeeTooLow as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LeaseNotNukeableYet(_) => {
                    <LeaseNotNukeableYet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LeaseRateLimitConfigInvalid(_) => {
                    <LeaseRateLimitConfigInvalid as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LeaseRateLimitExceeded(_) => {
                    <LeaseRateLimitExceeded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LpNotAllowlisted(_) => {
                    <LpNotAllowlisted as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NewOwnerIsZeroAddress(_) => {
                    <NewOwnerIsZeroAddress as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoActiveLease(_) => {
                    <NoActiveLease as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoBridger(_) => <NoBridger as alloy_sol_types::SolError>::SELECTOR,
                Self::NotATrc20Transfer(_) => {
                    <NotATrc20Transfer as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotEventChainTip(_) => {
                    <NotEventChainTip as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotLessee(_) => <NotLessee as alloy_sol_types::SolError>::SELECTOR,
                Self::NotRealtor(_) => {
                    <NotRealtor as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotTronUsdt(_) => {
                    <NotTronUsdt as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PayoutConfigRateLimitConfigInvalid(_) => {
                    <PayoutConfigRateLimitConfigInvalid as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PayoutConfigRateLimitExceeded(_) => {
                    <PayoutConfigRateLimitExceeded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::RateNotSet(_) => {
                    <RateNotSet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Reentrancy(_) => {
                    <Reentrancy as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SignatureExpired(_) => {
                    <SignatureExpired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SubjectiveNetOutZero(_) => {
                    <SubjectiveNetOutZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SubjectivePreEntitlementAlreadyExists(_) => {
                    <SubjectivePreEntitlementAlreadyExists as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TronInvalidCalldataLength(_) => {
                    <TronInvalidCalldataLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TronInvalidTrc20DataLength(_) => {
                    <TronInvalidTrc20DataLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Unauthorized(_) => {
                    <Unauthorized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WithdrawExceedsPrincipal(_) => {
                    <WithdrawExceedsPrincipal as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroAmount(_) => {
                    <ZeroAmount as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors>] = &[
                {
                    fn SignatureExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <SignatureExpired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::SignatureExpired)
                    }
                    SignatureExpired
                },
                {
                    fn RateNotSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <RateNotSet as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(UntronV3EntitleFacetErrors::RateNotSet)
                    }
                    RateNotSet
                },
                {
                    fn PayoutConfigRateLimitExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <PayoutConfigRateLimitExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                UntronV3EntitleFacetErrors::PayoutConfigRateLimitExceeded,
                            )
                    }
                    PayoutConfigRateLimitExceeded
                },
                {
                    fn LpNotAllowlisted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <LpNotAllowlisted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::LpNotAllowlisted)
                    }
                    LpNotAllowlisted
                },
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn EventTipMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <EventTipMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::EventTipMismatch)
                    }
                    EventTipMismatch
                },
                {
                    fn TronInvalidCalldataLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <TronInvalidCalldataLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::TronInvalidCalldataLength)
                    }
                    TronInvalidCalldataLength
                },
                {
                    fn AmountTooLargeForInt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <AmountTooLargeForInt as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::AmountTooLargeForInt)
                    }
                    AmountTooLargeForInt
                },
                {
                    fn CannotRescueUSDT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <CannotRescueUSDT as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::CannotRescueUSDT)
                    }
                    CannotRescueUSDT
                },
                {
                    fn ZeroAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <ZeroAmount as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(UntronV3EntitleFacetErrors::ZeroAmount)
                    }
                    ZeroAmount
                },
                {
                    fn InvalidLeaseId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <InvalidLeaseId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::InvalidLeaseId)
                    }
                    InvalidLeaseId
                },
                {
                    fn NotATrc20Transfer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <NotATrc20Transfer as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::NotATrc20Transfer)
                    }
                    NotATrc20Transfer
                },
                {
                    fn NotTronUsdt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <NotTronUsdt as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(UntronV3EntitleFacetErrors::NotTronUsdt)
                    }
                    NotTronUsdt
                },
                {
                    fn DepositAlreadyProcessed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <DepositAlreadyProcessed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::DepositAlreadyProcessed)
                    }
                    DepositAlreadyProcessed
                },
                {
                    fn SubjectiveNetOutZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <SubjectiveNetOutZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::SubjectiveNetOutZero)
                    }
                    SubjectiveNetOutZero
                },
                {
                    fn LeaseRateLimitConfigInvalid(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <LeaseRateLimitConfigInvalid as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::LeaseRateLimitConfigInvalid)
                    }
                    LeaseRateLimitConfigInvalid
                },
                {
                    fn NoActiveLease(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <NoActiveLease as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::NoActiveLease)
                    }
                    NoActiveLease
                },
                {
                    fn LeaseFlatFeeTooLow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <LeaseFlatFeeTooLow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::LeaseFlatFeeTooLow)
                    }
                    LeaseFlatFeeTooLow
                },
                {
                    fn LeaseRateLimitExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <LeaseRateLimitExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::LeaseRateLimitExceeded)
                    }
                    LeaseRateLimitExceeded
                },
                {
                    fn InvalidLeaseTimeframe(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <InvalidLeaseTimeframe as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::InvalidLeaseTimeframe)
                    }
                    InvalidLeaseTimeframe
                },
                {
                    fn NotEventChainTip(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <NotEventChainTip as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::NotEventChainTip)
                    }
                    NotEventChainTip
                },
                {
                    fn NotLessee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <NotLessee as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(UntronV3EntitleFacetErrors::NotLessee)
                    }
                    NotLessee
                },
                {
                    fn NewOwnerIsZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <NewOwnerIsZeroAddress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::NewOwnerIsZeroAddress)
                    }
                    NewOwnerIsZeroAddress
                },
                {
                    fn InvalidReceiverForSalt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <InvalidReceiverForSalt as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::InvalidReceiverForSalt)
                    }
                    InvalidReceiverForSalt
                },
                {
                    fn Unauthorized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <Unauthorized as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(UntronV3EntitleFacetErrors::Unauthorized)
                    }
                    Unauthorized
                },
                {
                    fn InvalidTargetToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <InvalidTargetToken as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::InvalidTargetToken)
                    }
                    InvalidTargetToken
                },
                {
                    fn NotRealtor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <NotRealtor as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(UntronV3EntitleFacetErrors::NotRealtor)
                    }
                    NotRealtor
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn ExpectedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <ExpectedPause as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::ExpectedPause)
                    }
                    ExpectedPause
                },
                {
                    fn LeaseFeeTooLow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <LeaseFeeTooLow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::LeaseFeeTooLow)
                    }
                    LeaseFeeTooLow
                },
                {
                    fn TronInvalidTrc20DataLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <TronInvalidTrc20DataLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::TronInvalidTrc20DataLength)
                    }
                    TronInvalidTrc20DataLength
                },
                {
                    fn InsufficientProtocolProfit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <InsufficientProtocolProfit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::InsufficientProtocolProfit)
                    }
                    InsufficientProtocolProfit
                },
                {
                    fn PayoutConfigRateLimitConfigInvalid(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <PayoutConfigRateLimitConfigInvalid as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                UntronV3EntitleFacetErrors::PayoutConfigRateLimitConfigInvalid,
                            )
                    }
                    PayoutConfigRateLimitConfigInvalid
                },
                {
                    fn LeaseDurationTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <LeaseDurationTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::LeaseDurationTooLong)
                    }
                    LeaseDurationTooLong
                },
                {
                    fn Reentrancy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <Reentrancy as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(UntronV3EntitleFacetErrors::Reentrancy)
                    }
                    Reentrancy
                },
                {
                    fn InsufficientLpPrincipal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <InsufficientLpPrincipal as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::InsufficientLpPrincipal)
                    }
                    InsufficientLpPrincipal
                },
                {
                    fn NoBridger(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <NoBridger as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(UntronV3EntitleFacetErrors::NoBridger)
                    }
                    NoBridger
                },
                {
                    fn LeaseNotNukeableYet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <LeaseNotNukeableYet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::LeaseNotNukeableYet)
                    }
                    LeaseNotNukeableYet
                },
                {
                    fn InsufficientUsdtBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <InsufficientUsdtBalance as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::InsufficientUsdtBalance)
                    }
                    InsufficientUsdtBalance
                },
                {
                    fn SubjectivePreEntitlementAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <SubjectivePreEntitlementAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                UntronV3EntitleFacetErrors::SubjectivePreEntitlementAlreadyExists,
                            )
                    }
                    SubjectivePreEntitlementAlreadyExists
                },
                {
                    fn WithdrawExceedsPrincipal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <WithdrawExceedsPrincipal as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::WithdrawExceedsPrincipal)
                    }
                    WithdrawExceedsPrincipal
                },
                {
                    fn EventRelayNoProgress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <EventRelayNoProgress as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::EventRelayNoProgress)
                    }
                    EventRelayNoProgress
                },
                {
                    fn ChainDeprecated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <ChainDeprecated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::ChainDeprecated)
                    }
                    ChainDeprecated
                },
                {
                    fn EnforcedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <EnforcedPause as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::EnforcedPause)
                    }
                    EnforcedPause
                },
                {
                    fn DepositNotAfterLastReceiverPull(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <DepositNotAfterLastReceiverPull as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                UntronV3EntitleFacetErrors::DepositNotAfterLastReceiverPull,
                            )
                    }
                    DepositNotAfterLastReceiverPull
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
            ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors>] = &[
                {
                    fn SignatureExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <SignatureExpired as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::SignatureExpired)
                    }
                    SignatureExpired
                },
                {
                    fn RateNotSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <RateNotSet as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::RateNotSet)
                    }
                    RateNotSet
                },
                {
                    fn PayoutConfigRateLimitExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <PayoutConfigRateLimitExceeded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                UntronV3EntitleFacetErrors::PayoutConfigRateLimitExceeded,
                            )
                    }
                    PayoutConfigRateLimitExceeded
                },
                {
                    fn LpNotAllowlisted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <LpNotAllowlisted as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::LpNotAllowlisted)
                    }
                    LpNotAllowlisted
                },
                {
                    fn AlreadyInitialized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <AlreadyInitialized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::AlreadyInitialized)
                    }
                    AlreadyInitialized
                },
                {
                    fn EventTipMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <EventTipMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::EventTipMismatch)
                    }
                    EventTipMismatch
                },
                {
                    fn TronInvalidCalldataLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <TronInvalidCalldataLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::TronInvalidCalldataLength)
                    }
                    TronInvalidCalldataLength
                },
                {
                    fn AmountTooLargeForInt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <AmountTooLargeForInt as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::AmountTooLargeForInt)
                    }
                    AmountTooLargeForInt
                },
                {
                    fn CannotRescueUSDT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <CannotRescueUSDT as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::CannotRescueUSDT)
                    }
                    CannotRescueUSDT
                },
                {
                    fn ZeroAmount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <ZeroAmount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::ZeroAmount)
                    }
                    ZeroAmount
                },
                {
                    fn InvalidLeaseId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <InvalidLeaseId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::InvalidLeaseId)
                    }
                    InvalidLeaseId
                },
                {
                    fn NotATrc20Transfer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <NotATrc20Transfer as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::NotATrc20Transfer)
                    }
                    NotATrc20Transfer
                },
                {
                    fn NotTronUsdt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <NotTronUsdt as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::NotTronUsdt)
                    }
                    NotTronUsdt
                },
                {
                    fn DepositAlreadyProcessed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <DepositAlreadyProcessed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::DepositAlreadyProcessed)
                    }
                    DepositAlreadyProcessed
                },
                {
                    fn SubjectiveNetOutZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <SubjectiveNetOutZero as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::SubjectiveNetOutZero)
                    }
                    SubjectiveNetOutZero
                },
                {
                    fn LeaseRateLimitConfigInvalid(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <LeaseRateLimitConfigInvalid as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::LeaseRateLimitConfigInvalid)
                    }
                    LeaseRateLimitConfigInvalid
                },
                {
                    fn NoActiveLease(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <NoActiveLease as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::NoActiveLease)
                    }
                    NoActiveLease
                },
                {
                    fn LeaseFlatFeeTooLow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <LeaseFlatFeeTooLow as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::LeaseFlatFeeTooLow)
                    }
                    LeaseFlatFeeTooLow
                },
                {
                    fn LeaseRateLimitExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <LeaseRateLimitExceeded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::LeaseRateLimitExceeded)
                    }
                    LeaseRateLimitExceeded
                },
                {
                    fn InvalidLeaseTimeframe(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <InvalidLeaseTimeframe as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::InvalidLeaseTimeframe)
                    }
                    InvalidLeaseTimeframe
                },
                {
                    fn NotEventChainTip(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <NotEventChainTip as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::NotEventChainTip)
                    }
                    NotEventChainTip
                },
                {
                    fn NotLessee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <NotLessee as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::NotLessee)
                    }
                    NotLessee
                },
                {
                    fn NewOwnerIsZeroAddress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <NewOwnerIsZeroAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::NewOwnerIsZeroAddress)
                    }
                    NewOwnerIsZeroAddress
                },
                {
                    fn InvalidReceiverForSalt(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <InvalidReceiverForSalt as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::InvalidReceiverForSalt)
                    }
                    InvalidReceiverForSalt
                },
                {
                    fn Unauthorized(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <Unauthorized as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::Unauthorized)
                    }
                    Unauthorized
                },
                {
                    fn InvalidTargetToken(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <InvalidTargetToken as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::InvalidTargetToken)
                    }
                    InvalidTargetToken
                },
                {
                    fn NotRealtor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <NotRealtor as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::NotRealtor)
                    }
                    NotRealtor
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn ExpectedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <ExpectedPause as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::ExpectedPause)
                    }
                    ExpectedPause
                },
                {
                    fn LeaseFeeTooLow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <LeaseFeeTooLow as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::LeaseFeeTooLow)
                    }
                    LeaseFeeTooLow
                },
                {
                    fn TronInvalidTrc20DataLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <TronInvalidTrc20DataLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::TronInvalidTrc20DataLength)
                    }
                    TronInvalidTrc20DataLength
                },
                {
                    fn InsufficientProtocolProfit(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <InsufficientProtocolProfit as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::InsufficientProtocolProfit)
                    }
                    InsufficientProtocolProfit
                },
                {
                    fn PayoutConfigRateLimitConfigInvalid(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <PayoutConfigRateLimitConfigInvalid as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                UntronV3EntitleFacetErrors::PayoutConfigRateLimitConfigInvalid,
                            )
                    }
                    PayoutConfigRateLimitConfigInvalid
                },
                {
                    fn LeaseDurationTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <LeaseDurationTooLong as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::LeaseDurationTooLong)
                    }
                    LeaseDurationTooLong
                },
                {
                    fn Reentrancy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <Reentrancy as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::Reentrancy)
                    }
                    Reentrancy
                },
                {
                    fn InsufficientLpPrincipal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <InsufficientLpPrincipal as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::InsufficientLpPrincipal)
                    }
                    InsufficientLpPrincipal
                },
                {
                    fn NoBridger(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <NoBridger as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::NoBridger)
                    }
                    NoBridger
                },
                {
                    fn LeaseNotNukeableYet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <LeaseNotNukeableYet as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::LeaseNotNukeableYet)
                    }
                    LeaseNotNukeableYet
                },
                {
                    fn InsufficientUsdtBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <InsufficientUsdtBalance as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::InsufficientUsdtBalance)
                    }
                    InsufficientUsdtBalance
                },
                {
                    fn SubjectivePreEntitlementAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <SubjectivePreEntitlementAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                UntronV3EntitleFacetErrors::SubjectivePreEntitlementAlreadyExists,
                            )
                    }
                    SubjectivePreEntitlementAlreadyExists
                },
                {
                    fn WithdrawExceedsPrincipal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <WithdrawExceedsPrincipal as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::WithdrawExceedsPrincipal)
                    }
                    WithdrawExceedsPrincipal
                },
                {
                    fn EventRelayNoProgress(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <EventRelayNoProgress as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::EventRelayNoProgress)
                    }
                    EventRelayNoProgress
                },
                {
                    fn ChainDeprecated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <ChainDeprecated as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::ChainDeprecated)
                    }
                    ChainDeprecated
                },
                {
                    fn EnforcedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <EnforcedPause as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(UntronV3EntitleFacetErrors::EnforcedPause)
                    }
                    EnforcedPause
                },
                {
                    fn DepositNotAfterLastReceiverPull(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<UntronV3EntitleFacetErrors> {
                        <DepositNotAfterLastReceiverPull as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                UntronV3EntitleFacetErrors::DepositNotAfterLastReceiverPull,
                            )
                    }
                    DepositNotAfterLastReceiverPull
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
                Self::AlreadyInitialized(inner) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AmountTooLargeForInt(inner) => {
                    <AmountTooLargeForInt as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CannotRescueUSDT(inner) => {
                    <CannotRescueUSDT as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ChainDeprecated(inner) => {
                    <ChainDeprecated as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DepositAlreadyProcessed(inner) => {
                    <DepositAlreadyProcessed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DepositNotAfterLastReceiverPull(inner) => {
                    <DepositNotAfterLastReceiverPull as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EnforcedPause(inner) => {
                    <EnforcedPause as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::EventRelayNoProgress(inner) => {
                    <EventRelayNoProgress as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EventTipMismatch(inner) => {
                    <EventTipMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ExpectedPause(inner) => {
                    <ExpectedPause as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InsufficientLpPrincipal(inner) => {
                    <InsufficientLpPrincipal as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientProtocolProfit(inner) => {
                    <InsufficientProtocolProfit as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientUsdtBalance(inner) => {
                    <InsufficientUsdtBalance as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidLeaseId(inner) => {
                    <InvalidLeaseId as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidLeaseTimeframe(inner) => {
                    <InvalidLeaseTimeframe as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidReceiverForSalt(inner) => {
                    <InvalidReceiverForSalt as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidTargetToken(inner) => {
                    <InvalidTargetToken as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LeaseDurationTooLong(inner) => {
                    <LeaseDurationTooLong as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LeaseFeeTooLow(inner) => {
                    <LeaseFeeTooLow as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LeaseFlatFeeTooLow(inner) => {
                    <LeaseFlatFeeTooLow as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LeaseNotNukeableYet(inner) => {
                    <LeaseNotNukeableYet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LeaseRateLimitConfigInvalid(inner) => {
                    <LeaseRateLimitConfigInvalid as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LeaseRateLimitExceeded(inner) => {
                    <LeaseRateLimitExceeded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LpNotAllowlisted(inner) => {
                    <LpNotAllowlisted as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NewOwnerIsZeroAddress(inner) => {
                    <NewOwnerIsZeroAddress as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoActiveLease(inner) => {
                    <NoActiveLease as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NoBridger(inner) => {
                    <NoBridger as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotATrc20Transfer(inner) => {
                    <NotATrc20Transfer as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotEventChainTip(inner) => {
                    <NotEventChainTip as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotLessee(inner) => {
                    <NotLessee as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotRealtor(inner) => {
                    <NotRealtor as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotTronUsdt(inner) => {
                    <NotTronUsdt as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::PayoutConfigRateLimitConfigInvalid(inner) => {
                    <PayoutConfigRateLimitConfigInvalid as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PayoutConfigRateLimitExceeded(inner) => {
                    <PayoutConfigRateLimitExceeded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::RateNotSet(inner) => {
                    <RateNotSet as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::Reentrancy(inner) => {
                    <Reentrancy as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SubjectiveNetOutZero(inner) => {
                    <SubjectiveNetOutZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SubjectivePreEntitlementAlreadyExists(inner) => {
                    <SubjectivePreEntitlementAlreadyExists as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TronInvalidCalldataLength(inner) => {
                    <TronInvalidCalldataLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TronInvalidTrc20DataLength(inner) => {
                    <TronInvalidTrc20DataLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Unauthorized(inner) => {
                    <Unauthorized as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::WithdrawExceedsPrincipal(inner) => {
                    <WithdrawExceedsPrincipal as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ZeroAmount(inner) => {
                    <ZeroAmount as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AlreadyInitialized(inner) => {
                    <AlreadyInitialized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AmountTooLargeForInt(inner) => {
                    <AmountTooLargeForInt as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CannotRescueUSDT(inner) => {
                    <CannotRescueUSDT as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ChainDeprecated(inner) => {
                    <ChainDeprecated as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DepositAlreadyProcessed(inner) => {
                    <DepositAlreadyProcessed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DepositNotAfterLastReceiverPull(inner) => {
                    <DepositNotAfterLastReceiverPull as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EnforcedPause(inner) => {
                    <EnforcedPause as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EventRelayNoProgress(inner) => {
                    <EventRelayNoProgress as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EventTipMismatch(inner) => {
                    <EventTipMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ExpectedPause(inner) => {
                    <ExpectedPause as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientLpPrincipal(inner) => {
                    <InsufficientLpPrincipal as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientProtocolProfit(inner) => {
                    <InsufficientProtocolProfit as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientUsdtBalance(inner) => {
                    <InsufficientUsdtBalance as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidLeaseId(inner) => {
                    <InvalidLeaseId as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidLeaseTimeframe(inner) => {
                    <InvalidLeaseTimeframe as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidReceiverForSalt(inner) => {
                    <InvalidReceiverForSalt as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidTargetToken(inner) => {
                    <InvalidTargetToken as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LeaseDurationTooLong(inner) => {
                    <LeaseDurationTooLong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LeaseFeeTooLow(inner) => {
                    <LeaseFeeTooLow as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LeaseFlatFeeTooLow(inner) => {
                    <LeaseFlatFeeTooLow as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LeaseNotNukeableYet(inner) => {
                    <LeaseNotNukeableYet as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LeaseRateLimitConfigInvalid(inner) => {
                    <LeaseRateLimitConfigInvalid as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LeaseRateLimitExceeded(inner) => {
                    <LeaseRateLimitExceeded as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LpNotAllowlisted(inner) => {
                    <LpNotAllowlisted as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NewOwnerIsZeroAddress(inner) => {
                    <NewOwnerIsZeroAddress as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoActiveLease(inner) => {
                    <NoActiveLease as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoBridger(inner) => {
                    <NoBridger as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::NotATrc20Transfer(inner) => {
                    <NotATrc20Transfer as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotEventChainTip(inner) => {
                    <NotEventChainTip as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotLessee(inner) => {
                    <NotLessee as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::NotRealtor(inner) => {
                    <NotRealtor as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::NotTronUsdt(inner) => {
                    <NotTronUsdt as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PayoutConfigRateLimitConfigInvalid(inner) => {
                    <PayoutConfigRateLimitConfigInvalid as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PayoutConfigRateLimitExceeded(inner) => {
                    <PayoutConfigRateLimitExceeded as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::RateNotSet(inner) => {
                    <RateNotSet as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::Reentrancy(inner) => {
                    <Reentrancy as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SubjectiveNetOutZero(inner) => {
                    <SubjectiveNetOutZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SubjectivePreEntitlementAlreadyExists(inner) => {
                    <SubjectivePreEntitlementAlreadyExists as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TronInvalidCalldataLength(inner) => {
                    <TronInvalidCalldataLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TronInvalidTrc20DataLength(inner) => {
                    <TronInvalidTrc20DataLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Unauthorized(inner) => {
                    <Unauthorized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WithdrawExceedsPrincipal(inner) => {
                    <WithdrawExceedsPrincipal as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroAmount(inner) => {
                    <ZeroAmount as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`UntronV3EntitleFacet`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum UntronV3EntitleFacetEvents {
        #[allow(missing_docs)]
        BridgerSet(BridgerSet),
        #[allow(missing_docs)]
        ChainDeprecatedSet(ChainDeprecatedSet),
        #[allow(missing_docs)]
        ClaimCreated(ClaimCreated),
        #[allow(missing_docs)]
        ClaimFilled(ClaimFilled),
        #[allow(missing_docs)]
        ControllerEventChainTipUpdated(ControllerEventChainTipUpdated),
        #[allow(missing_docs)]
        ControllerEventProcessed(ControllerEventProcessed),
        #[allow(missing_docs)]
        EventAppended(EventAppended),
        #[allow(missing_docs)]
        LeaseCreated(LeaseCreated),
        #[allow(missing_docs)]
        LeaseNonceUpdated(LeaseNonceUpdated),
        #[allow(missing_docs)]
        LesseePayoutConfigRateLimitSet(LesseePayoutConfigRateLimitSet),
        #[allow(missing_docs)]
        LpDeposited(LpDeposited),
        #[allow(missing_docs)]
        LpSet(LpSet),
        #[allow(missing_docs)]
        LpWithdrawn(LpWithdrawn),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        PayoutConfigUpdated(PayoutConfigUpdated),
        #[allow(missing_docs)]
        ProtocolFlatFeeFloorSet(ProtocolFlatFeeFloorSet),
        #[allow(missing_docs)]
        ProtocolFloorSet(ProtocolFloorSet),
        #[allow(missing_docs)]
        ProtocolMaxLeaseDurationSet(ProtocolMaxLeaseDurationSet),
        #[allow(missing_docs)]
        ProtocolPnlUpdated(ProtocolPnlUpdated),
        #[allow(missing_docs)]
        RealtorLeaseRateLimitSet(RealtorLeaseRateLimitSet),
        #[allow(missing_docs)]
        RealtorMaxLeaseDurationSet(RealtorMaxLeaseDurationSet),
        #[allow(missing_docs)]
        RealtorMinFeeSet(RealtorMinFeeSet),
        #[allow(missing_docs)]
        RealtorMinFlatFeeSet(RealtorMinFlatFeeSet),
        #[allow(missing_docs)]
        RealtorSet(RealtorSet),
        #[allow(missing_docs)]
        SwapRateSet(SwapRateSet),
        #[allow(missing_docs)]
        TokensRescued(TokensRescued),
        #[allow(missing_docs)]
        TronReaderSet(TronReaderSet),
        #[allow(missing_docs)]
        TronUsdtSet(TronUsdtSet),
        #[allow(missing_docs)]
        Unpaused(Unpaused),
        #[allow(missing_docs)]
        UsdtSet(UsdtSet),
    }
    impl UntronV3EntitleFacetEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                10u8, 168u8, 38u8, 38u8, 241u8, 155u8, 181u8, 212u8, 25u8, 98u8, 2u8,
                176u8, 31u8, 47u8, 238u8, 67u8, 27u8, 149u8, 232u8, 140u8, 176u8, 84u8,
                72u8, 75u8, 152u8, 125u8, 101u8, 13u8, 145u8, 146u8, 138u8, 218u8,
            ],
            [
                31u8, 179u8, 179u8, 142u8, 64u8, 32u8, 9u8, 241u8, 187u8, 210u8, 77u8,
                10u8, 112u8, 0u8, 90u8, 122u8, 154u8, 5u8, 91u8, 63u8, 91u8, 84u8, 108u8,
                139u8, 33u8, 248u8, 21u8, 71u8, 10u8, 156u8, 46u8, 196u8,
            ],
            [
                32u8, 128u8, 167u8, 86u8, 68u8, 77u8, 42u8, 240u8, 249u8, 251u8, 79u8,
                135u8, 219u8, 122u8, 246u8, 60u8, 214u8, 226u8, 59u8, 122u8, 18u8, 3u8,
                203u8, 233u8, 171u8, 9u8, 114u8, 238u8, 5u8, 209u8, 234u8, 253u8,
            ],
            [
                40u8, 108u8, 215u8, 197u8, 120u8, 30u8, 109u8, 243u8, 118u8, 218u8,
                165u8, 154u8, 178u8, 7u8, 225u8, 160u8, 177u8, 236u8, 233u8, 95u8, 175u8,
                194u8, 152u8, 249u8, 171u8, 105u8, 161u8, 136u8, 244u8, 159u8, 242u8,
                213u8,
            ],
            [
                47u8, 72u8, 23u8, 49u8, 130u8, 66u8, 209u8, 184u8, 103u8, 0u8, 164u8,
                38u8, 247u8, 206u8, 208u8, 73u8, 198u8, 128u8, 89u8, 37u8, 60u8, 10u8,
                114u8, 30u8, 1u8, 185u8, 113u8, 143u8, 69u8, 39u8, 190u8, 246u8,
            ],
            [
                66u8, 125u8, 180u8, 143u8, 140u8, 208u8, 207u8, 127u8, 218u8, 188u8,
                219u8, 193u8, 50u8, 124u8, 125u8, 178u8, 108u8, 227u8, 165u8, 68u8, 41u8,
                43u8, 163u8, 128u8, 239u8, 7u8, 213u8, 23u8, 90u8, 247u8, 41u8, 205u8,
            ],
            [
                67u8, 153u8, 30u8, 30u8, 28u8, 251u8, 46u8, 237u8, 108u8, 157u8, 195u8,
                122u8, 122u8, 132u8, 134u8, 34u8, 248u8, 227u8, 247u8, 91u8, 195u8,
                141u8, 83u8, 42u8, 0u8, 234u8, 240u8, 38u8, 205u8, 135u8, 160u8, 20u8,
            ],
            [
                68u8, 40u8, 46u8, 42u8, 75u8, 187u8, 134u8, 185u8, 248u8, 8u8, 157u8,
                16u8, 134u8, 244u8, 224u8, 223u8, 39u8, 48u8, 84u8, 234u8, 182u8, 44u8,
                219u8, 218u8, 24u8, 246u8, 71u8, 207u8, 210u8, 42u8, 30u8, 255u8,
            ],
            [
                68u8, 47u8, 122u8, 183u8, 210u8, 107u8, 247u8, 186u8, 116u8, 242u8, 62u8,
                35u8, 125u8, 18u8, 135u8, 98u8, 81u8, 249u8, 29u8, 97u8, 174u8, 178u8,
                137u8, 154u8, 217u8, 95u8, 51u8, 79u8, 165u8, 231u8, 198u8, 51u8,
            ],
            [
                93u8, 41u8, 105u8, 62u8, 99u8, 179u8, 160u8, 132u8, 229u8, 104u8, 147u8,
                206u8, 140u8, 127u8, 94u8, 245u8, 220u8, 130u8, 19u8, 218u8, 65u8, 7u8,
                0u8, 132u8, 230u8, 177u8, 196u8, 55u8, 10u8, 189u8, 100u8, 196u8,
            ],
            [
                93u8, 185u8, 238u8, 10u8, 73u8, 91u8, 242u8, 230u8, 255u8, 156u8, 145u8,
                167u8, 131u8, 76u8, 27u8, 164u8, 253u8, 210u8, 68u8, 165u8, 232u8, 170u8,
                78u8, 83u8, 123u8, 211u8, 138u8, 234u8, 228u8, 176u8, 115u8, 170u8,
            ],
            [
                97u8, 196u8, 154u8, 182u8, 187u8, 30u8, 245u8, 145u8, 145u8, 167u8, 70u8,
                175u8, 206u8, 125u8, 39u8, 49u8, 212u8, 217u8, 176u8, 250u8, 253u8,
                171u8, 141u8, 99u8, 240u8, 47u8, 228u8, 65u8, 105u8, 83u8, 45u8, 197u8,
            ],
            [
                98u8, 231u8, 140u8, 234u8, 1u8, 190u8, 227u8, 32u8, 205u8, 78u8, 66u8,
                2u8, 112u8, 181u8, 234u8, 116u8, 0u8, 13u8, 17u8, 176u8, 201u8, 247u8,
                71u8, 84u8, 235u8, 219u8, 252u8, 84u8, 75u8, 5u8, 162u8, 88u8,
            ],
            [
                104u8, 246u8, 125u8, 232u8, 158u8, 150u8, 177u8, 58u8, 62u8, 160u8, 88u8,
                175u8, 95u8, 212u8, 76u8, 193u8, 37u8, 239u8, 206u8, 181u8, 40u8, 72u8,
                45u8, 83u8, 156u8, 123u8, 67u8, 219u8, 47u8, 170u8, 6u8, 110u8,
            ],
            [
                119u8, 36u8, 47u8, 189u8, 87u8, 58u8, 245u8, 165u8, 243u8, 81u8, 141u8,
                169u8, 38u8, 0u8, 233u8, 103u8, 149u8, 235u8, 255u8, 249u8, 147u8, 96u8,
                107u8, 79u8, 181u8, 77u8, 234u8, 45u8, 205u8, 45u8, 254u8, 133u8,
            ],
            [
                120u8, 22u8, 15u8, 11u8, 27u8, 43u8, 50u8, 181u8, 42u8, 0u8, 118u8,
                216u8, 240u8, 247u8, 8u8, 136u8, 104u8, 123u8, 167u8, 2u8, 164u8, 217u8,
                147u8, 213u8, 90u8, 200u8, 217u8, 50u8, 125u8, 87u8, 161u8, 39u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                142u8, 152u8, 160u8, 115u8, 68u8, 32u8, 229u8, 126u8, 241u8, 2u8, 202u8,
                176u8, 177u8, 175u8, 8u8, 9u8, 165u8, 187u8, 175u8, 79u8, 222u8, 150u8,
                111u8, 209u8, 115u8, 195u8, 34u8, 73u8, 6u8, 68u8, 165u8, 208u8,
            ],
            [
                157u8, 97u8, 27u8, 91u8, 52u8, 203u8, 118u8, 19u8, 28u8, 79u8, 180u8,
                19u8, 235u8, 116u8, 17u8, 155u8, 44u8, 12u8, 58u8, 106u8, 166u8, 252u8,
                216u8, 231u8, 64u8, 207u8, 112u8, 172u8, 48u8, 133u8, 216u8, 123u8,
            ],
            [
                159u8, 94u8, 29u8, 19u8, 4u8, 93u8, 39u8, 47u8, 190u8, 116u8, 206u8,
                77u8, 8u8, 233u8, 25u8, 130u8, 165u8, 197u8, 119u8, 132u8, 57u8, 26u8,
                230u8, 161u8, 153u8, 238u8, 205u8, 207u8, 99u8, 148u8, 159u8, 254u8,
            ],
            [
                164u8, 79u8, 41u8, 61u8, 250u8, 146u8, 40u8, 145u8, 99u8, 69u8, 166u8,
                1u8, 98u8, 32u8, 243u8, 4u8, 253u8, 78u8, 16u8, 194u8, 242u8, 94u8,
                246u8, 44u8, 137u8, 107u8, 73u8, 70u8, 146u8, 106u8, 112u8, 244u8,
            ],
            [
                168u8, 211u8, 179u8, 33u8, 59u8, 127u8, 130u8, 68u8, 229u8, 176u8, 69u8,
                81u8, 219u8, 154u8, 170u8, 188u8, 204u8, 33u8, 214u8, 33u8, 44u8, 67u8,
                195u8, 179u8, 20u8, 59u8, 205u8, 233u8, 124u8, 168u8, 83u8, 206u8,
            ],
            [
                171u8, 99u8, 194u8, 90u8, 160u8, 122u8, 77u8, 16u8, 161u8, 132u8, 45u8,
                97u8, 2u8, 106u8, 8u8, 217u8, 81u8, 21u8, 196u8, 230u8, 137u8, 82u8,
                136u8, 104u8, 48u8, 21u8, 202u8, 68u8, 100u8, 199u8, 245u8, 15u8,
            ],
            [
                177u8, 216u8, 239u8, 249u8, 72u8, 88u8, 22u8, 187u8, 56u8, 206u8, 236u8,
                243u8, 23u8, 163u8, 253u8, 160u8, 85u8, 151u8, 188u8, 91u8, 56u8, 242u8,
                242u8, 45u8, 249u8, 108u8, 14u8, 189u8, 93u8, 194u8, 54u8, 238u8,
            ],
            [
                182u8, 43u8, 78u8, 111u8, 30u8, 197u8, 151u8, 10u8, 41u8, 39u8, 78u8,
                116u8, 120u8, 53u8, 244u8, 68u8, 165u8, 204u8, 212u8, 128u8, 73u8, 105u8,
                142u8, 255u8, 156u8, 156u8, 253u8, 202u8, 46u8, 26u8, 94u8, 175u8,
            ],
            [
                200u8, 183u8, 254u8, 36u8, 220u8, 46u8, 158u8, 115u8, 17u8, 65u8, 254u8,
                29u8, 215u8, 77u8, 108u8, 228u8, 112u8, 189u8, 107u8, 83u8, 113u8, 193u8,
                47u8, 66u8, 201u8, 206u8, 70u8, 186u8, 193u8, 84u8, 36u8, 197u8,
            ],
            [
                215u8, 207u8, 117u8, 220u8, 25u8, 50u8, 7u8, 246u8, 72u8, 75u8, 123u8,
                214u8, 196u8, 252u8, 70u8, 159u8, 59u8, 107u8, 115u8, 61u8, 44u8, 188u8,
                238u8, 64u8, 57u8, 117u8, 40u8, 123u8, 1u8, 93u8, 196u8, 153u8,
            ],
            [
                218u8, 228u8, 65u8, 124u8, 37u8, 169u8, 60u8, 251u8, 134u8, 206u8, 95u8,
                187u8, 184u8, 252u8, 22u8, 48u8, 148u8, 91u8, 97u8, 175u8, 174u8, 171u8,
                79u8, 41u8, 207u8, 48u8, 27u8, 154u8, 5u8, 139u8, 169u8, 20u8,
            ],
            [
                220u8, 161u8, 107u8, 10u8, 246u8, 225u8, 15u8, 93u8, 251u8, 125u8, 78u8,
                169u8, 16u8, 85u8, 149u8, 20u8, 25u8, 160u8, 200u8, 255u8, 197u8, 146u8,
                90u8, 207u8, 253u8, 197u8, 42u8, 149u8, 252u8, 198u8, 113u8, 51u8,
            ],
            [
                228u8, 49u8, 80u8, 43u8, 208u8, 223u8, 88u8, 128u8, 57u8, 126u8, 193u8,
                60u8, 10u8, 38u8, 15u8, 12u8, 116u8, 156u8, 143u8, 58u8, 48u8, 198u8,
                202u8, 243u8, 255u8, 224u8, 240u8, 138u8, 46u8, 209u8, 57u8, 66u8,
            ],
            [
                247u8, 207u8, 174u8, 152u8, 112u8, 225u8, 48u8, 122u8, 7u8, 145u8, 214u8,
                65u8, 141u8, 158u8, 120u8, 171u8, 209u8, 115u8, 26u8, 28u8, 3u8, 96u8,
                104u8, 19u8, 144u8, 107u8, 71u8, 77u8, 48u8, 126u8, 173u8, 86u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(RealtorMinFeeSet),
            ::core::stringify!(ProtocolMaxLeaseDurationSet),
            ::core::stringify!(RealtorSet),
            ::core::stringify!(ProtocolFlatFeeFloorSet),
            ::core::stringify!(RealtorMinFlatFeeSet),
            ::core::stringify!(BridgerSet),
            ::core::stringify!(ProtocolPnlUpdated),
            ::core::stringify!(LpWithdrawn),
            ::core::stringify!(LpSet),
            ::core::stringify!(TronReaderSet),
            ::core::stringify!(Unpaused),
            ::core::stringify!(RealtorLeaseRateLimitSet),
            ::core::stringify!(Paused),
            ::core::stringify!(TokensRescued),
            ::core::stringify!(ClaimCreated),
            ::core::stringify!(EventAppended),
            ::core::stringify!(OwnershipTransferred),
            ::core::stringify!(LeaseNonceUpdated),
            ::core::stringify!(ControllerEventChainTipUpdated),
            ::core::stringify!(TronUsdtSet),
            ::core::stringify!(UsdtSet),
            ::core::stringify!(SwapRateSet),
            ::core::stringify!(LesseePayoutConfigRateLimitSet),
            ::core::stringify!(RealtorMaxLeaseDurationSet),
            ::core::stringify!(ClaimFilled),
            ::core::stringify!(ChainDeprecatedSet),
            ::core::stringify!(PayoutConfigUpdated),
            ::core::stringify!(LpDeposited),
            ::core::stringify!(ControllerEventProcessed),
            ::core::stringify!(LeaseCreated),
            ::core::stringify!(ProtocolFloorSet),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <RealtorMinFeeSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <ProtocolMaxLeaseDurationSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <RealtorSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <ProtocolFlatFeeFloorSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <RealtorMinFlatFeeSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <BridgerSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <ProtocolPnlUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <LpWithdrawn as alloy_sol_types::SolEvent>::SIGNATURE,
            <LpSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <TronReaderSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <Unpaused as alloy_sol_types::SolEvent>::SIGNATURE,
            <RealtorLeaseRateLimitSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <Paused as alloy_sol_types::SolEvent>::SIGNATURE,
            <TokensRescued as alloy_sol_types::SolEvent>::SIGNATURE,
            <ClaimCreated as alloy_sol_types::SolEvent>::SIGNATURE,
            <EventAppended as alloy_sol_types::SolEvent>::SIGNATURE,
            <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE,
            <LeaseNonceUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <ControllerEventChainTipUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <TronUsdtSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <UsdtSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <SwapRateSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <LesseePayoutConfigRateLimitSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <RealtorMaxLeaseDurationSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <ClaimFilled as alloy_sol_types::SolEvent>::SIGNATURE,
            <ChainDeprecatedSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <PayoutConfigUpdated as alloy_sol_types::SolEvent>::SIGNATURE,
            <LpDeposited as alloy_sol_types::SolEvent>::SIGNATURE,
            <ControllerEventProcessed as alloy_sol_types::SolEvent>::SIGNATURE,
            <LeaseCreated as alloy_sol_types::SolEvent>::SIGNATURE,
            <ProtocolFloorSet as alloy_sol_types::SolEvent>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 32usize],
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
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for UntronV3EntitleFacetEvents {
        const NAME: &'static str = "UntronV3EntitleFacetEvents";
        const COUNT: usize = 31usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<BridgerSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <BridgerSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::BridgerSet)
                }
                Some(
                    <ChainDeprecatedSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ChainDeprecatedSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ChainDeprecatedSet)
                }
                Some(<ClaimCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ClaimCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ClaimCreated)
                }
                Some(<ClaimFilled as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ClaimFilled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ClaimFilled)
                }
                Some(
                    <ControllerEventChainTipUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ControllerEventChainTipUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ControllerEventChainTipUpdated)
                }
                Some(
                    <ControllerEventProcessed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ControllerEventProcessed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ControllerEventProcessed)
                }
                Some(<EventAppended as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EventAppended as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::EventAppended)
                }
                Some(<LeaseCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <LeaseCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::LeaseCreated)
                }
                Some(
                    <LeaseNonceUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <LeaseNonceUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::LeaseNonceUpdated)
                }
                Some(
                    <LesseePayoutConfigRateLimitSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <LesseePayoutConfigRateLimitSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::LesseePayoutConfigRateLimitSet)
                }
                Some(<LpDeposited as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <LpDeposited as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::LpDeposited)
                }
                Some(<LpSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <LpSet as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::LpSet)
                }
                Some(<LpWithdrawn as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <LpWithdrawn as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::LpWithdrawn)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Paused)
                }
                Some(
                    <PayoutConfigUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PayoutConfigUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PayoutConfigUpdated)
                }
                Some(
                    <ProtocolFlatFeeFloorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ProtocolFlatFeeFloorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ProtocolFlatFeeFloorSet)
                }
                Some(<ProtocolFloorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ProtocolFloorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ProtocolFloorSet)
                }
                Some(
                    <ProtocolMaxLeaseDurationSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ProtocolMaxLeaseDurationSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ProtocolMaxLeaseDurationSet)
                }
                Some(
                    <ProtocolPnlUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ProtocolPnlUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ProtocolPnlUpdated)
                }
                Some(
                    <RealtorLeaseRateLimitSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RealtorLeaseRateLimitSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RealtorLeaseRateLimitSet)
                }
                Some(
                    <RealtorMaxLeaseDurationSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RealtorMaxLeaseDurationSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RealtorMaxLeaseDurationSet)
                }
                Some(<RealtorMinFeeSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RealtorMinFeeSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RealtorMinFeeSet)
                }
                Some(
                    <RealtorMinFlatFeeSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RealtorMinFlatFeeSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RealtorMinFlatFeeSet)
                }
                Some(<RealtorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RealtorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RealtorSet)
                }
                Some(<SwapRateSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SwapRateSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::SwapRateSet)
                }
                Some(<TokensRescued as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TokensRescued as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::TokensRescued)
                }
                Some(<TronReaderSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TronReaderSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::TronReaderSet)
                }
                Some(<TronUsdtSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TronUsdtSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::TronUsdtSet)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Unpaused)
                }
                Some(<UsdtSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <UsdtSet as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::UsdtSet)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for UntronV3EntitleFacetEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::BridgerSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ChainDeprecatedSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ClaimCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ClaimFilled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ControllerEventChainTipUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ControllerEventProcessed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EventAppended(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::LeaseCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::LeaseNonceUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::LesseePayoutConfigRateLimitSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::LpDeposited(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::LpSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::LpWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PayoutConfigUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ProtocolFlatFeeFloorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ProtocolFloorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ProtocolMaxLeaseDurationSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ProtocolPnlUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RealtorLeaseRateLimitSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RealtorMaxLeaseDurationSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RealtorMinFeeSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RealtorMinFlatFeeSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RealtorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SwapRateSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TokensRescued(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TronReaderSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TronUsdtSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UsdtSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::BridgerSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ChainDeprecatedSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ClaimCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ClaimFilled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ControllerEventChainTipUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ControllerEventProcessed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EventAppended(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::LeaseCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::LeaseNonceUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::LesseePayoutConfigRateLimitSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::LpDeposited(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::LpSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::LpWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PayoutConfigUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ProtocolFlatFeeFloorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ProtocolFloorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ProtocolMaxLeaseDurationSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ProtocolPnlUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RealtorLeaseRateLimitSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RealtorMaxLeaseDurationSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RealtorMinFeeSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RealtorMinFlatFeeSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RealtorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SwapRateSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TokensRescued(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TronReaderSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TronUsdtSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UsdtSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`UntronV3EntitleFacet`](self) contract instance.

See the [wrapper's documentation](`UntronV3EntitleFacetInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> UntronV3EntitleFacetInstance<P, N> {
        UntronV3EntitleFacetInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<UntronV3EntitleFacetInstance<P, N>>,
    > {
        UntronV3EntitleFacetInstance::<P, N>::deploy(__provider)
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
        UntronV3EntitleFacetInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`UntronV3EntitleFacet`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`UntronV3EntitleFacet`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct UntronV3EntitleFacetInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for UntronV3EntitleFacetInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("UntronV3EntitleFacetInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > UntronV3EntitleFacetInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`UntronV3EntitleFacet`](self) contract instance.

See the [wrapper's documentation](`UntronV3EntitleFacetInstance`) for more details.*/
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
        ) -> alloy_contract::Result<UntronV3EntitleFacetInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> UntronV3EntitleFacetInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> UntronV3EntitleFacetInstance<P, N> {
            UntronV3EntitleFacetInstance {
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
    > UntronV3EntitleFacetInstance<P, N> {
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
        ///Creates a new call builder for the [`CONTROLLER_ADDRESS`] function.
        pub fn CONTROLLER_ADDRESS(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, CONTROLLER_ADDRESSCall, N> {
            self.call_builder(&CONTROLLER_ADDRESSCall)
        }
        ///Creates a new call builder for the [`RECEIVER_IMPL`] function.
        pub fn RECEIVER_IMPL(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, RECEIVER_IMPLCall, N> {
            self.call_builder(&RECEIVER_IMPLCall)
        }
        ///Creates a new call builder for the [`SWAP_EXECUTOR`] function.
        pub fn SWAP_EXECUTOR(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, SWAP_EXECUTORCall, N> {
            self.call_builder(&SWAP_EXECUTORCall)
        }
        ///Creates a new call builder for the [`bridgers`] function.
        pub fn bridgers(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, bridgersCall, N> {
            self.call_builder(&bridgersCall { _0, _1 })
        }
        ///Creates a new call builder for the [`claimLocatorByLease`] function.
        pub fn claimLocatorByLease(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, claimLocatorByLeaseCall, N> {
            self.call_builder(&claimLocatorByLeaseCall { _0, _1 })
        }
        ///Creates a new call builder for the [`claimsByTargetToken`] function.
        pub fn claimsByTargetToken(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, claimsByTargetTokenCall, N> {
            self.call_builder(&claimsByTargetTokenCall { _0, _1 })
        }
        ///Creates a new call builder for the [`depositProcessed`] function.
        pub fn depositProcessed(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, depositProcessedCall, N> {
            self.call_builder(&depositProcessedCall(_0))
        }
        ///Creates a new call builder for the [`eip712Domain`] function.
        pub fn eip712Domain(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, eip712DomainCall, N> {
            self.call_builder(&eip712DomainCall)
        }
        ///Creates a new call builder for the [`eventChainTip`] function.
        pub fn eventChainTip(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, eventChainTipCall, N> {
            self.call_builder(&eventChainTipCall)
        }
        ///Creates a new call builder for the [`eventSeq`] function.
        pub fn eventSeq(&self) -> alloy_contract::SolCallBuilder<&P, eventSeqCall, N> {
            self.call_builder(&eventSeqCall)
        }
        ///Creates a new call builder for the [`isChainDeprecated`] function.
        pub fn isChainDeprecated(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, isChainDeprecatedCall, N> {
            self.call_builder(&isChainDeprecatedCall(_0))
        }
        ///Creates a new call builder for the [`isLpAllowed`] function.
        pub fn isLpAllowed(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isLpAllowedCall, N> {
            self.call_builder(&isLpAllowedCall(_0))
        }
        ///Creates a new call builder for the [`isRealtor`] function.
        pub fn isRealtor(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isRealtorCall, N> {
            self.call_builder(&isRealtorCall(_0))
        }
        ///Creates a new call builder for the [`lastControllerEventSeq`] function.
        pub fn lastControllerEventSeq(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, lastControllerEventSeqCall, N> {
            self.call_builder(&lastControllerEventSeqCall)
        }
        ///Creates a new call builder for the [`lastControllerEventTip`] function.
        pub fn lastControllerEventTip(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, lastControllerEventTipCall, N> {
            self.call_builder(&lastControllerEventTipCall)
        }
        ///Creates a new call builder for the [`lastReceiverPullTimestampByToken`] function.
        pub fn lastReceiverPullTimestampByToken(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
            _1: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            lastReceiverPullTimestampByTokenCall,
            N,
        > {
            self.call_builder(
                &lastReceiverPullTimestampByTokenCall {
                    _0,
                    _1,
                },
            )
        }
        ///Creates a new call builder for the [`leaseNonces`] function.
        pub fn leaseNonces(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, leaseNoncesCall, N> {
            self.call_builder(&leaseNoncesCall(_0))
        }
        ///Creates a new call builder for the [`leasesByReceiver`] function.
        pub fn leasesByReceiver(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, leasesByReceiverCall, N> {
            self.call_builder(&leasesByReceiverCall { _0, _1 })
        }
        ///Creates a new call builder for the [`lpPrincipal`] function.
        pub fn lpPrincipal(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, lpPrincipalCall, N> {
            self.call_builder(&lpPrincipalCall(_0))
        }
        ///Creates a new call builder for the [`nextClaimIdByLease`] function.
        pub fn nextClaimIdByLease(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, nextClaimIdByLeaseCall, N> {
            self.call_builder(&nextClaimIdByLeaseCall(_0))
        }
        ///Creates a new call builder for the [`nextControllerEventIndex`] function.
        pub fn nextControllerEventIndex(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, nextControllerEventIndexCall, N> {
            self.call_builder(&nextControllerEventIndexCall)
        }
        ///Creates a new call builder for the [`nextIndexByTargetToken`] function.
        pub fn nextIndexByTargetToken(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, nextIndexByTargetTokenCall, N> {
            self.call_builder(&nextIndexByTargetTokenCall(_0))
        }
        ///Creates a new call builder for the [`nextLeaseId`] function.
        pub fn nextLeaseId(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, nextLeaseIdCall, N> {
            self.call_builder(&nextLeaseIdCall)
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<&P, ownerCall, N> {
            self.call_builder(&ownerCall)
        }
        ///Creates a new call builder for the [`paused`] function.
        pub fn paused(&self) -> alloy_contract::SolCallBuilder<&P, pausedCall, N> {
            self.call_builder(&pausedCall)
        }
        ///Creates a new call builder for the [`preEntitle`] function.
        pub fn preEntitle(
            &self,
            receiverSalt: alloy::sol_types::private::FixedBytes<32>,
            blocks: [alloy::sol_types::private::Bytes; 20usize],
            encodedTx: alloy::sol_types::private::Bytes,
            proof: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, preEntitleCall, N> {
            self.call_builder(
                &preEntitleCall {
                    receiverSalt,
                    blocks,
                    encodedTx,
                    proof,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`predictReceiverAddress_0`] function.
        pub fn predictReceiverAddress_0(
            &self,
            salt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, predictReceiverAddress_0Call, N> {
            self.call_builder(
                &predictReceiverAddress_0Call {
                    salt,
                },
            )
        }
        ///Creates a new call builder for the [`predictReceiverAddress_1`] function.
        pub fn predictReceiverAddress_1(
            &self,
            controller: alloy::sol_types::private::Address,
            salt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, predictReceiverAddress_1Call, N> {
            self.call_builder(
                &predictReceiverAddress_1Call {
                    controller,
                    salt,
                },
            )
        }
        ///Creates a new call builder for the [`protocolPnl`] function.
        pub fn protocolPnl(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, protocolPnlCall, N> {
            self.call_builder(&protocolPnlCall)
        }
        ///Creates a new call builder for the [`receiverBytecode`] function.
        pub fn receiverBytecode(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, receiverBytecodeCall, N> {
            self.call_builder(&receiverBytecodeCall)
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall)
        }
        ///Creates a new call builder for the [`subjectivePreEntitle`] function.
        pub fn subjectivePreEntitle(
            &self,
            txId: alloy::sol_types::private::FixedBytes<32>,
            leaseId: alloy::sol_types::private::primitives::aliases::U256,
            rawAmount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, subjectivePreEntitleCall, N> {
            self.call_builder(
                &subjectivePreEntitleCall {
                    txId,
                    leaseId,
                    rawAmount,
                },
            )
        }
        ///Creates a new call builder for the [`subjectivePreEntitlementByTxId`] function.
        pub fn subjectivePreEntitlementByTxId(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, subjectivePreEntitlementByTxIdCall, N> {
            self.call_builder(&subjectivePreEntitlementByTxIdCall(_0))
        }
        ///Creates a new call builder for the [`swapRatePpm`] function.
        pub fn swapRatePpm(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, swapRatePpmCall, N> {
            self.call_builder(&swapRatePpmCall(_0))
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`tronReader`] function.
        pub fn tronReader(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, tronReaderCall, N> {
            self.call_builder(&tronReaderCall)
        }
        ///Creates a new call builder for the [`tronUsdt`] function.
        pub fn tronUsdt(&self) -> alloy_contract::SolCallBuilder<&P, tronUsdtCall, N> {
            self.call_builder(&tronUsdtCall)
        }
        ///Creates a new call builder for the [`usdt`] function.
        pub fn usdt(&self) -> alloy_contract::SolCallBuilder<&P, usdtCall, N> {
            self.call_builder(&usdtCall)
        }
        ///Creates a new call builder for the [`usdtBalance`] function.
        pub fn usdtBalance(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, usdtBalanceCall, N> {
            self.call_builder(&usdtBalanceCall)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > UntronV3EntitleFacetInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`BridgerSet`] event.
        pub fn BridgerSet_filter(&self) -> alloy_contract::Event<&P, BridgerSet, N> {
            self.event_filter::<BridgerSet>()
        }
        ///Creates a new event filter for the [`ChainDeprecatedSet`] event.
        pub fn ChainDeprecatedSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, ChainDeprecatedSet, N> {
            self.event_filter::<ChainDeprecatedSet>()
        }
        ///Creates a new event filter for the [`ClaimCreated`] event.
        pub fn ClaimCreated_filter(&self) -> alloy_contract::Event<&P, ClaimCreated, N> {
            self.event_filter::<ClaimCreated>()
        }
        ///Creates a new event filter for the [`ClaimFilled`] event.
        pub fn ClaimFilled_filter(&self) -> alloy_contract::Event<&P, ClaimFilled, N> {
            self.event_filter::<ClaimFilled>()
        }
        ///Creates a new event filter for the [`ControllerEventChainTipUpdated`] event.
        pub fn ControllerEventChainTipUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, ControllerEventChainTipUpdated, N> {
            self.event_filter::<ControllerEventChainTipUpdated>()
        }
        ///Creates a new event filter for the [`ControllerEventProcessed`] event.
        pub fn ControllerEventProcessed_filter(
            &self,
        ) -> alloy_contract::Event<&P, ControllerEventProcessed, N> {
            self.event_filter::<ControllerEventProcessed>()
        }
        ///Creates a new event filter for the [`EventAppended`] event.
        pub fn EventAppended_filter(
            &self,
        ) -> alloy_contract::Event<&P, EventAppended, N> {
            self.event_filter::<EventAppended>()
        }
        ///Creates a new event filter for the [`LeaseCreated`] event.
        pub fn LeaseCreated_filter(&self) -> alloy_contract::Event<&P, LeaseCreated, N> {
            self.event_filter::<LeaseCreated>()
        }
        ///Creates a new event filter for the [`LeaseNonceUpdated`] event.
        pub fn LeaseNonceUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, LeaseNonceUpdated, N> {
            self.event_filter::<LeaseNonceUpdated>()
        }
        ///Creates a new event filter for the [`LesseePayoutConfigRateLimitSet`] event.
        pub fn LesseePayoutConfigRateLimitSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, LesseePayoutConfigRateLimitSet, N> {
            self.event_filter::<LesseePayoutConfigRateLimitSet>()
        }
        ///Creates a new event filter for the [`LpDeposited`] event.
        pub fn LpDeposited_filter(&self) -> alloy_contract::Event<&P, LpDeposited, N> {
            self.event_filter::<LpDeposited>()
        }
        ///Creates a new event filter for the [`LpSet`] event.
        pub fn LpSet_filter(&self) -> alloy_contract::Event<&P, LpSet, N> {
            self.event_filter::<LpSet>()
        }
        ///Creates a new event filter for the [`LpWithdrawn`] event.
        pub fn LpWithdrawn_filter(&self) -> alloy_contract::Event<&P, LpWithdrawn, N> {
            self.event_filter::<LpWithdrawn>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`Paused`] event.
        pub fn Paused_filter(&self) -> alloy_contract::Event<&P, Paused, N> {
            self.event_filter::<Paused>()
        }
        ///Creates a new event filter for the [`PayoutConfigUpdated`] event.
        pub fn PayoutConfigUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, PayoutConfigUpdated, N> {
            self.event_filter::<PayoutConfigUpdated>()
        }
        ///Creates a new event filter for the [`ProtocolFlatFeeFloorSet`] event.
        pub fn ProtocolFlatFeeFloorSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, ProtocolFlatFeeFloorSet, N> {
            self.event_filter::<ProtocolFlatFeeFloorSet>()
        }
        ///Creates a new event filter for the [`ProtocolFloorSet`] event.
        pub fn ProtocolFloorSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, ProtocolFloorSet, N> {
            self.event_filter::<ProtocolFloorSet>()
        }
        ///Creates a new event filter for the [`ProtocolMaxLeaseDurationSet`] event.
        pub fn ProtocolMaxLeaseDurationSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, ProtocolMaxLeaseDurationSet, N> {
            self.event_filter::<ProtocolMaxLeaseDurationSet>()
        }
        ///Creates a new event filter for the [`ProtocolPnlUpdated`] event.
        pub fn ProtocolPnlUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, ProtocolPnlUpdated, N> {
            self.event_filter::<ProtocolPnlUpdated>()
        }
        ///Creates a new event filter for the [`RealtorLeaseRateLimitSet`] event.
        pub fn RealtorLeaseRateLimitSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, RealtorLeaseRateLimitSet, N> {
            self.event_filter::<RealtorLeaseRateLimitSet>()
        }
        ///Creates a new event filter for the [`RealtorMaxLeaseDurationSet`] event.
        pub fn RealtorMaxLeaseDurationSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, RealtorMaxLeaseDurationSet, N> {
            self.event_filter::<RealtorMaxLeaseDurationSet>()
        }
        ///Creates a new event filter for the [`RealtorMinFeeSet`] event.
        pub fn RealtorMinFeeSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, RealtorMinFeeSet, N> {
            self.event_filter::<RealtorMinFeeSet>()
        }
        ///Creates a new event filter for the [`RealtorMinFlatFeeSet`] event.
        pub fn RealtorMinFlatFeeSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, RealtorMinFlatFeeSet, N> {
            self.event_filter::<RealtorMinFlatFeeSet>()
        }
        ///Creates a new event filter for the [`RealtorSet`] event.
        pub fn RealtorSet_filter(&self) -> alloy_contract::Event<&P, RealtorSet, N> {
            self.event_filter::<RealtorSet>()
        }
        ///Creates a new event filter for the [`SwapRateSet`] event.
        pub fn SwapRateSet_filter(&self) -> alloy_contract::Event<&P, SwapRateSet, N> {
            self.event_filter::<SwapRateSet>()
        }
        ///Creates a new event filter for the [`TokensRescued`] event.
        pub fn TokensRescued_filter(
            &self,
        ) -> alloy_contract::Event<&P, TokensRescued, N> {
            self.event_filter::<TokensRescued>()
        }
        ///Creates a new event filter for the [`TronReaderSet`] event.
        pub fn TronReaderSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, TronReaderSet, N> {
            self.event_filter::<TronReaderSet>()
        }
        ///Creates a new event filter for the [`TronUsdtSet`] event.
        pub fn TronUsdtSet_filter(&self) -> alloy_contract::Event<&P, TronUsdtSet, N> {
            self.event_filter::<TronUsdtSet>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<&P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
        ///Creates a new event filter for the [`UsdtSet`] event.
        pub fn UsdtSet_filter(&self) -> alloy_contract::Event<&P, UsdtSet, N> {
            self.event_filter::<UsdtSet>()
        }
    }
}
