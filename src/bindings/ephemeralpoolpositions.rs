///Module containing a contract's types and functions.
/**

```solidity
library PoolUtils {
    struct PositionKey { address owner; int24 tickLower; int24 tickUpper; }
    struct Slot { uint256 slot; uint256 data; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod PoolUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct PositionKey { address owner; int24 tickLower; int24 tickUpper; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PositionKey {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tickLower: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub tickUpper: alloy::sol_types::private::primitives::aliases::I24,
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
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Int<24>,
            alloy::sol_types::sol_data::Int<24>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::I24,
            alloy::sol_types::private::primitives::aliases::I24,
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
        impl ::core::convert::From<PositionKey> for UnderlyingRustTuple<'_> {
            fn from(value: PositionKey) -> Self {
                (value.owner, value.tickLower, value.tickUpper)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PositionKey {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    owner: tuple.0,
                    tickLower: tuple.1,
                    tickUpper: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PositionKey {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PositionKey {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickLower),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickUpper),
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
        impl alloy_sol_types::SolType for PositionKey {
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
        impl alloy_sol_types::SolStruct for PositionKey {
            const NAME: &'static str = "PositionKey";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PositionKey(address owner,int24 tickLower,int24 tickUpper)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.owner,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tickLower)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tickUpper)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PositionKey {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.owner,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tickLower,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tickUpper,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.owner,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tickLower,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tickUpper,
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
    /**```solidity
struct Slot { uint256 slot; uint256 data; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Slot {
        #[allow(missing_docs)]
        pub slot: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<Slot> for UnderlyingRustTuple<'_> {
            fn from(value: Slot) -> Self {
                (value.slot, value.data)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Slot {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    slot: tuple.0,
                    data: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Slot {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Slot {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.slot),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.data),
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
        impl alloy_sol_types::SolType for Slot {
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
        impl alloy_sol_types::SolStruct for Slot {
            const NAME: &'static str = "Slot";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Slot(uint256 slot,uint256 data)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.slot)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.data)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Slot {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.slot)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.data)
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
                    &rust.slot,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
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
    /**Creates a new wrapper around an on-chain [`PoolUtils`](self) contract instance.

See the [wrapper's documentation](`PoolUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> PoolUtilsInstance<T, P, N> {
        PoolUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`PoolUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`PoolUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct PoolUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for PoolUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("PoolUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PoolUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`PoolUtils`](self) contract instance.

See the [wrapper's documentation](`PoolUtilsInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
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
    impl<T, P: ::core::clone::Clone, N> PoolUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> PoolUtilsInstance<T, P, N> {
            PoolUtilsInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PoolUtilsInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PoolUtilsInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library PoolUtils {
    struct PositionKey {
        address owner;
        int24 tickLower;
        int24 tickUpper;
    }
    struct Slot {
        uint256 slot;
        uint256 data;
    }
}

interface EphemeralPoolPositions {
    type V3PoolCallee is address;

    constructor(V3PoolCallee pool, PoolUtils.PositionKey[] keys) payable;

    function getPositions(V3PoolCallee pool, PoolUtils.PositionKey[] memory keys) external payable returns (PoolUtils.Slot[] memory slots);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "pool",
        "type": "address",
        "internalType": "V3PoolCallee"
      },
      {
        "name": "keys",
        "type": "tuple[]",
        "internalType": "struct PoolUtils.PositionKey[]",
        "components": [
          {
            "name": "owner",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "tickLower",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "tickUpper",
            "type": "int24",
            "internalType": "int24"
          }
        ]
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "getPositions",
    "inputs": [
      {
        "name": "pool",
        "type": "address",
        "internalType": "V3PoolCallee"
      },
      {
        "name": "keys",
        "type": "tuple[]",
        "internalType": "struct PoolUtils.PositionKey[]",
        "components": [
          {
            "name": "owner",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "tickLower",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "tickUpper",
            "type": "int24",
            "internalType": "int24"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "slots",
        "type": "tuple[]",
        "internalType": "struct PoolUtils.Slot[]",
        "components": [
          {
            "name": "slot",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "data",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "payable"
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
pub mod EphemeralPoolPositions {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60806040526040516104e63803806104e683398101604081905261002291610373565b600061002e838361005c565b90506000816040516020016100439190610480565b6040516020818303038152906040529050805160208201fd5b8051606090600790600281901b6001600160401b03811115610080576100806102ee565b6040519080825280602002602001820160405280156100c557816020015b604080518082019091526000808252602082015281526020019060019003908161009e5790505b5092506000805b828110156102475760006100fe8783815181106100eb576100eb6104cf565b602002602001015161025160201b60201c565b60008181526020879052604081209192506101226001600160a01b038b168461026d565b6040805180820190915283815281516001600160801b03166020820152895160018881019895019492935090918a91811061015f5761015f6104cf565b6020026020010181905250604051806040016040528083806001019450815260200182602001518152508886806001019750815181106101a1576101a16104cf565b6020026020010181905250604051806040016040528083806001019450815260200182604001518152508886806001019750815181106101e3576101e36104cf565b602002602001018190525060006060820151608083015160801b17905060405180604001604052808481526020018281525089878060010198508151811061022d5761022d6104cf565b6020026020010181905250505050508060010190506100cc565b5050505092915050565b6040810151600652602081015160035251600052601a600c2090565b6040805160a081018252600080825260208201819052918101829052606081018290526080810191909152806102ae8463514ea4bf60e01b858460a06102b5565b5092915050565b8360005282600452808260246000885afa6102cf57600080fd5b5050505050565b6001600160a01b03811681146102eb57600080fd5b50565b634e487b7160e01b600052604160045260246000fd5b604051606081016001600160401b0381118282101715610326576103266102ee565b60405290565b604051601f8201601f191681016001600160401b0381118282101715610354576103546102ee565b604052919050565b8051600281900b811461036e57600080fd5b919050565b6000806040838503121561038657600080fd5b8251610391816102d6565b60208401519092506001600160401b038111156103ad57600080fd5b8301601f810185136103be57600080fd5b80516001600160401b038111156103d7576103d76102ee565b6103e660208260051b0161032c565b8082825260208201915060206060840285010192508783111561040857600080fd5b6020840193505b82841015610472576060848903121561042757600080fd5b61042f610304565b845161043a816102d6565b81526104486020860161035c565b60208201526104596040860161035c565b604082015282526060939093019260209091019061040f565b809450505050509250929050565b602080825282518282018190526000918401906040840190835b818110156104c457835180518452602090810151818501529093019260409092019160010161049a565b509095945050505050565b634e487b7160e01b600052603260045260246000fdfe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`@Qa\x04\xE68\x03\x80a\x04\xE6\x839\x81\x01`@\x81\x90Ra\0\"\x91a\x03sV[`\0a\0.\x83\x83a\0\\V[\x90P`\0\x81`@Q` \x01a\0C\x91\x90a\x04\x80V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80Q` \x82\x01\xFD[\x80Q``\x90`\x07\x90`\x02\x81\x90\x1B`\x01`\x01`@\x1B\x03\x81\x11\x15a\0\x80Wa\0\x80a\x02\xEEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xC5W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0\x9EW\x90P[P\x92P`\0\x80[\x82\x81\x10\x15a\x02GW`\0a\0\xFE\x87\x83\x81Q\x81\x10a\0\xEBWa\0\xEBa\x04\xCFV[` \x02` \x01\x01Qa\x02Q` \x1B` \x1CV[`\0\x81\x81R` \x87\x90R`@\x81 \x91\x92Pa\x01\"`\x01`\x01`\xA0\x1B\x03\x8B\x16\x84a\x02mV[`@\x80Q\x80\x82\x01\x90\x91R\x83\x81R\x81Q`\x01`\x01`\x80\x1B\x03\x16` \x82\x01R\x89Q`\x01\x88\x81\x01\x98\x95\x01\x94\x92\x93P\x90\x91\x8A\x91\x81\x10a\x01_Wa\x01_a\x04\xCFV[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80\x83\x80`\x01\x01\x94P\x81R` \x01\x82` \x01Q\x81RP\x88\x86\x80`\x01\x01\x97P\x81Q\x81\x10a\x01\xA1Wa\x01\xA1a\x04\xCFV[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80\x83\x80`\x01\x01\x94P\x81R` \x01\x82`@\x01Q\x81RP\x88\x86\x80`\x01\x01\x97P\x81Q\x81\x10a\x01\xE3Wa\x01\xE3a\x04\xCFV[` \x02` \x01\x01\x81\x90RP`\0``\x82\x01Q`\x80\x83\x01Q`\x80\x1B\x17\x90P`@Q\x80`@\x01`@R\x80\x84\x81R` \x01\x82\x81RP\x89\x87\x80`\x01\x01\x98P\x81Q\x81\x10a\x02-Wa\x02-a\x04\xCFV[` \x02` \x01\x01\x81\x90RPPPPP\x80`\x01\x01\x90Pa\0\xCCV[PPPP\x92\x91PPV[`@\x81\x01Q`\x06R` \x81\x01Q`\x03RQ`\0R`\x1A`\x0C \x90V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x80a\x02\xAE\x84cQN\xA4\xBF`\xE0\x1B\x85\x84`\xA0a\x02\xB5V[P\x92\x91PPV[\x83`\0R\x82`\x04R\x80\x82`$`\0\x88Z\xFAa\x02\xCFW`\0\x80\xFD[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\xEBW`\0\x80\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x03&Wa\x03&a\x02\xEEV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x03TWa\x03Ta\x02\xEEV[`@R\x91\x90PV[\x80Q`\x02\x81\x90\x0B\x81\x14a\x03nW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x03\x86W`\0\x80\xFD[\x82Qa\x03\x91\x81a\x02\xD6V[` \x84\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03\xADW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x03\xBEW`\0\x80\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03\xD7Wa\x03\xD7a\x02\xEEV[a\x03\xE6` \x82`\x05\x1B\x01a\x03,V[\x80\x82\x82R` \x82\x01\x91P` ``\x84\x02\x85\x01\x01\x92P\x87\x83\x11\x15a\x04\x08W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x04rW``\x84\x89\x03\x12\x15a\x04'W`\0\x80\xFD[a\x04/a\x03\x04V[\x84Qa\x04:\x81a\x02\xD6V[\x81Ra\x04H` \x86\x01a\x03\\V[` \x82\x01Ra\x04Y`@\x86\x01a\x03\\V[`@\x82\x01R\x82R``\x93\x90\x93\x01\x92` \x90\x91\x01\x90a\x04\x0FV[\x80\x94PPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x04\xC4W\x83Q\x80Q\x84R` \x90\x81\x01Q\x81\x85\x01R\x90\x93\x01\x92`@\x90\x92\x01\x91`\x01\x01a\x04\x9AV[P\x90\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061001e5760003560e01c8063c241e27a14610023575b600080fd5b6100366100313660046103ce565b61004c565b60405161004391906104dc565b60405180910390f35b8051606090600790600281901b67ffffffffffffffff81111561007157610071610310565b6040519080825280602002602001820160405280156100b657816020015b604080518082019091526000808252602082015281526020019060019003908161008f5790505b5092506000805b8281101561025f5760006101008783815181106100dc576100dc61052b565b60200260200101516040810151600652602081015160035251600052601a600c2090565b600081815260208790526040812091925061013173ffffffffffffffffffffffffffffffffffffffff8b1684610269565b6040805180820190915283815281516fffffffffffffffffffffffffffffffff166020820152895160018881019895019492935090918a9181106101775761017761052b565b6020026020010181905250604051806040016040528083806001019450815260200182602001518152508886806001019750815181106101b9576101b961052b565b6020026020010181905250604051806040016040528083806001019450815260200182604001518152508886806001019750815181106101fb576101fb61052b565b602002602001018190525060006060820151608083015160801b1790506040518060400160405280848152602001828152508987806001019850815181106102455761024561052b565b6020026020010181905250505050508060010190506100bd565b5050505092915050565b6040805160a081018252600080825260208201819052918101829052606081018290526080810191909152806102c3847f514ea4bf00000000000000000000000000000000000000000000000000000000858460a06102ca565b5092915050565b8360005282600452808260246000885afa6102e457600080fd5b5050505050565b73ffffffffffffffffffffffffffffffffffffffff8116811461030d57600080fd5b50565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6040516060810167ffffffffffffffff8111828210171561036257610362610310565b60405290565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff811182821017156103af576103af610310565b604052919050565b8035600281900b81146103c957600080fd5b919050565b600080604083850312156103e157600080fd5b82356103ec816102eb565b9150602083013567ffffffffffffffff81111561040857600080fd5b8301601f8101851361041957600080fd5b803567ffffffffffffffff81111561043357610433610310565b61044260208260051b01610368565b8082825260208201915060206060840285010192508783111561046457600080fd5b6020840193505b828410156104ce576060848903121561048357600080fd5b61048b61033f565b8435610496816102eb565b81526104a4602086016103b7565b60208201526104b5604086016103b7565b604082015282526060939093019260209091019061046b565b809450505050509250929050565b602080825282518282018190526000918401906040840190835b818110156105205783518051845260209081015181850152909301926040909201916001016104f6565b509095945050505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0\x1EW`\x005`\xE0\x1C\x80c\xC2A\xE2z\x14a\0#W[`\0\x80\xFD[a\x006a\x0016`\x04a\x03\xCEV[a\0LV[`@Qa\0C\x91\x90a\x04\xDCV[`@Q\x80\x91\x03\x90\xF3[\x80Q``\x90`\x07\x90`\x02\x81\x90\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0qWa\0qa\x03\x10V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xB6W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0\x8FW\x90P[P\x92P`\0\x80[\x82\x81\x10\x15a\x02_W`\0a\x01\0\x87\x83\x81Q\x81\x10a\0\xDCWa\0\xDCa\x05+V[` \x02` \x01\x01Q`@\x81\x01Q`\x06R` \x81\x01Q`\x03RQ`\0R`\x1A`\x0C \x90V[`\0\x81\x81R` \x87\x90R`@\x81 \x91\x92Pa\x011s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x84a\x02iV[`@\x80Q\x80\x82\x01\x90\x91R\x83\x81R\x81Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x82\x01R\x89Q`\x01\x88\x81\x01\x98\x95\x01\x94\x92\x93P\x90\x91\x8A\x91\x81\x10a\x01wWa\x01wa\x05+V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80\x83\x80`\x01\x01\x94P\x81R` \x01\x82` \x01Q\x81RP\x88\x86\x80`\x01\x01\x97P\x81Q\x81\x10a\x01\xB9Wa\x01\xB9a\x05+V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80\x83\x80`\x01\x01\x94P\x81R` \x01\x82`@\x01Q\x81RP\x88\x86\x80`\x01\x01\x97P\x81Q\x81\x10a\x01\xFBWa\x01\xFBa\x05+V[` \x02` \x01\x01\x81\x90RP`\0``\x82\x01Q`\x80\x83\x01Q`\x80\x1B\x17\x90P`@Q\x80`@\x01`@R\x80\x84\x81R` \x01\x82\x81RP\x89\x87\x80`\x01\x01\x98P\x81Q\x81\x10a\x02EWa\x02Ea\x05+V[` \x02` \x01\x01\x81\x90RPPPPP\x80`\x01\x01\x90Pa\0\xBDV[PPPP\x92\x91PPV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x80a\x02\xC3\x84\x7FQN\xA4\xBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x84`\xA0a\x02\xCAV[P\x92\x91PPV[\x83`\0R\x82`\x04R\x80\x82`$`\0\x88Z\xFAa\x02\xE4W`\0\x80\xFD[PPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\rW`\0\x80\xFD[PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03bWa\x03ba\x03\x10V[`@R\x90V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\xAFWa\x03\xAFa\x03\x10V[`@R\x91\x90PV[\x805`\x02\x81\x90\x0B\x81\x14a\x03\xC9W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x03\xE1W`\0\x80\xFD[\x825a\x03\xEC\x81a\x02\xEBV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x08W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x04\x19W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x043Wa\x043a\x03\x10V[a\x04B` \x82`\x05\x1B\x01a\x03hV[\x80\x82\x82R` \x82\x01\x91P` ``\x84\x02\x85\x01\x01\x92P\x87\x83\x11\x15a\x04dW`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x04\xCEW``\x84\x89\x03\x12\x15a\x04\x83W`\0\x80\xFD[a\x04\x8Ba\x03?V[\x845a\x04\x96\x81a\x02\xEBV[\x81Ra\x04\xA4` \x86\x01a\x03\xB7V[` \x82\x01Ra\x04\xB5`@\x86\x01a\x03\xB7V[`@\x82\x01R\x82R``\x93\x90\x93\x01\x92` \x90\x91\x01\x90a\x04kV[\x80\x94PPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x05 W\x83Q\x80Q\x84R` \x90\x81\x01Q\x81\x85\x01R\x90\x93\x01\x92`@\x90\x92\x01\x91`\x01\x01a\x04\xF6V[P\x90\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD",
    );
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct V3PoolCallee(alloy::sol_types::private::Address);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<V3PoolCallee>
        for alloy::sol_types::private::Address {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::Token<
                '_,
            > {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Address,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        self,
                    )
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::abi_encode_packed_to(
                    self,
                    out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
            }
        }
        #[automatically_derived]
        impl V3PoolCallee {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: alloy::sol_types::private::Address) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> alloy::sol_types::private::Address {
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
        impl alloy_sol_types::SolType for V3PoolCallee {
            type RustType = alloy::sol_types::private::Address;
            type Token<'a> = <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::Token<
                'a,
            >;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::type_check(
                    token,
                )
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::detokenize(
                    token,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for V3PoolCallee {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                    rust,
                )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    out,
                )
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
            }
        }
    };
    /**Constructor`.
```solidity
constructor(V3PoolCallee pool, PoolUtils.PositionKey[] keys) payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub keys: alloy::sol_types::private::Vec<
            <PoolUtils::PositionKey as alloy::sol_types::SolType>::RustType,
        >,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                V3PoolCallee,
                alloy::sol_types::sol_data::Array<PoolUtils::PositionKey>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <V3PoolCallee as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    <PoolUtils::PositionKey as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.pool, value.keys)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pool: tuple.0,
                        keys: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                V3PoolCallee,
                alloy::sol_types::sol_data::Array<PoolUtils::PositionKey>,
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
                    <V3PoolCallee as alloy_sol_types::SolType>::tokenize(&self.pool),
                    <alloy::sol_types::sol_data::Array<
                        PoolUtils::PositionKey,
                    > as alloy_sol_types::SolType>::tokenize(&self.keys),
                )
            }
        }
    };
    /**Function with signature `getPositions(address,(address,int24,int24)[])` and selector `0xc241e27a`.
```solidity
function getPositions(V3PoolCallee pool, PoolUtils.PositionKey[] memory keys) external payable returns (PoolUtils.Slot[] memory slots);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPositionsCall {
        #[allow(missing_docs)]
        pub pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub keys: alloy::sol_types::private::Vec<
            <PoolUtils::PositionKey as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`getPositions(address,(address,int24,int24)[])`](getPositionsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPositionsReturn {
        #[allow(missing_docs)]
        pub slots: alloy::sol_types::private::Vec<
            <PoolUtils::Slot as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (
                V3PoolCallee,
                alloy::sol_types::sol_data::Array<PoolUtils::PositionKey>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <V3PoolCallee as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    <PoolUtils::PositionKey as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getPositionsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPositionsCall) -> Self {
                    (value.pool, value.keys)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPositionsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pool: tuple.0,
                        keys: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<PoolUtils::Slot>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <PoolUtils::Slot as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getPositionsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getPositionsReturn) -> Self {
                    (value.slots,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPositionsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { slots: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPositionsCall {
            type Parameters<'a> = (
                V3PoolCallee,
                alloy::sol_types::sol_data::Array<PoolUtils::PositionKey>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPositionsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<PoolUtils::Slot>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPositions(address,(address,int24,int24)[])";
            const SELECTOR: [u8; 4] = [194u8, 65u8, 226u8, 122u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <V3PoolCallee as alloy_sol_types::SolType>::tokenize(&self.pool),
                    <alloy::sol_types::sol_data::Array<
                        PoolUtils::PositionKey,
                    > as alloy_sol_types::SolType>::tokenize(&self.keys),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`EphemeralPoolPositions`](self) function calls.
    pub enum EphemeralPoolPositionsCalls {
        #[allow(missing_docs)]
        getPositions(getPositionsCall),
    }
    #[automatically_derived]
    impl EphemeralPoolPositionsCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[194u8, 65u8, 226u8, 122u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for EphemeralPoolPositionsCalls {
        const NAME: &'static str = "EphemeralPoolPositionsCalls";
        const MIN_DATA_LENGTH: usize = 96usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::getPositions(_) => {
                    <getPositionsCall as alloy_sol_types::SolCall>::SELECTOR
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
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<EphemeralPoolPositionsCalls>] = &[
                {
                    fn getPositions(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EphemeralPoolPositionsCalls> {
                        <getPositionsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EphemeralPoolPositionsCalls::getPositions)
                    }
                    getPositions
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
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::getPositions(inner) => {
                    <getPositionsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::getPositions(inner) => {
                    <getPositionsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`EphemeralPoolPositions`](self) contract instance.

See the [wrapper's documentation](`EphemeralPoolPositionsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> EphemeralPoolPositionsInstance<T, P, N> {
        EphemeralPoolPositionsInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
        keys: alloy::sol_types::private::Vec<
            <PoolUtils::PositionKey as alloy::sol_types::SolType>::RustType,
        >,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<EphemeralPoolPositionsInstance<T, P, N>>,
    > {
        EphemeralPoolPositionsInstance::<T, P, N>::deploy(provider, pool, keys)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
        keys: alloy::sol_types::private::Vec<
            <PoolUtils::PositionKey as alloy::sol_types::SolType>::RustType,
        >,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        EphemeralPoolPositionsInstance::<T, P, N>::deploy_builder(provider, pool, keys)
    }
    /**A [`EphemeralPoolPositions`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`EphemeralPoolPositions`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EphemeralPoolPositionsInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for EphemeralPoolPositionsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EphemeralPoolPositionsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EphemeralPoolPositionsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`EphemeralPoolPositions`](self) contract instance.

See the [wrapper's documentation](`EphemeralPoolPositionsInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
            keys: alloy::sol_types::private::Vec<
                <PoolUtils::PositionKey as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::Result<EphemeralPoolPositionsInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, pool, keys);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
            keys: alloy::sol_types::private::Vec<
                <PoolUtils::PositionKey as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { pool, keys },
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
    impl<T, P: ::core::clone::Clone, N> EphemeralPoolPositionsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EphemeralPoolPositionsInstance<T, P, N> {
            EphemeralPoolPositionsInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EphemeralPoolPositionsInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`getPositions`] function.
        pub fn getPositions(
            &self,
            pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
            keys: alloy::sol_types::private::Vec<
                <PoolUtils::PositionKey as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPositionsCall, N> {
            self.call_builder(&getPositionsCall { pool, keys })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EphemeralPoolPositionsInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
