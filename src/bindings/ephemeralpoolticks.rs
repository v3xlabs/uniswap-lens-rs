///Module containing a contract's types and functions.
/**

```solidity
library PoolUtils {
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
    struct Slot {
        uint256 slot;
        uint256 data;
    }
}

interface EphemeralPoolTicks {
    type V3PoolCallee is address;

    constructor(V3PoolCallee pool, int24 tickLower, int24 tickUpper) payable;

    function getPopulatedTicksInRange(V3PoolCallee pool, int24 tickLower, int24 tickUpper) external payable returns (PoolUtils.Slot[] memory slots);
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
        "name": "tickLower",
        "type": "int24",
        "internalType": "int24"
      },
      {
        "name": "tickUpper",
        "type": "int24",
        "internalType": "int24"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "getPopulatedTicksInRange",
    "inputs": [
      {
        "name": "pool",
        "type": "address",
        "internalType": "V3PoolCallee"
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
pub mod EphemeralPoolTicks {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405260405161073f38038061073f833981016040819052610022916105bb565b600061002f84848461005d565b9050600081604051602001610044919061060c565b6040516020818303038152906040529050805160208201fd5b60608160020b8360020b131561007257600080fd5b6000846001600160a01b031663d0c93a7c6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100b2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100d6919061065b565b9050808407600090811382860503600290810b600890811d9284870781138588050390920b901d908061010a8985856101ce565b91509150600281901b6001600160401b0381111561012a5761012a61067d565b60405190808252806020026020018201604052801561016f57816020015b60408051808201909152600080825260208201528152602001906001900390816101485790505b5095506000845b8460010b8160010b136101c0576101b68b8289878a860361ffff16815181106101a1576101a1610693565b60200260200101518c8761035160201b60201c565b9150600101610176565b505050505050509392505050565b606060006101dc84846106bf565b6101e79060016106e8565b61ffff166001600160401b038111156102025761020261067d565b60405190808252806020026020018201604052801561022b578160200160208202803683370190505b509150835b8360010b8160010b136103485760006102526001600160a01b038816836104f7565b9050808461026088856106bf565b61ffff168151811061027457610274610693565b60209081029190910101527f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f7f5555555555555555555555555555555555555555555555555555555555555555600183901c168203600281901c7f3333333333333333333333333333333333333333333333333333333333333333908116911601600481901c01167f01010101010101010101010101010101010101010101010101010101010101010260f81c600019821460081b17610334908461070b565b925050806103419061071e565b9050610230565b50935093915050565b60006005815b6101008110156104ea576001811b8616156104e257600888901b81018702600061038a6001600160a01b038c168361051e565b60008381526020868152604091829020835182850151845180860190955282855260801b179183018290528a5160018b81019b959650909101939192918b9181106103d7576103d7610693565b6020026020010181905250506040518060400160405280828060010193508152602001836040015181525088888060010199508151811061041a5761041a610693565b60200260200101819052506040518060400160405280828060010193508152602001836060015181525088888060010199508151811061045c5761045c610693565b6020026020010181905250600060e08301518060f81b91505060c0830151818160d81b1791505060a0830151818160381b17915050608083015166ffffffffffffff168181179150506040518060400160405280838152602001828152508989806001019a50815181106104d2576104d2610693565b6020026020010181905250505050505b600101610357565b5091979650505050505050565b6000600182900b6105138463299ce14b60e11b83856020610583565b505060005192915050565b6040805161010081018252600080825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e0810191909152600282900b8161057b8563f30dba9360e01b8484610100610583565b505092915050565b8360005282600452808260246000885afa61059d57600080fd5b5050505050565b8051600281900b81146105b657600080fd5b919050565b6000806000606084860312156105d057600080fd5b83516001600160a01b03811681146105e757600080fd5b92506105f5602085016105a4565b9150610603604085016105a4565b90509250925092565b602080825282518282018190526000918401906040840190835b81811015610650578351805184526020908101518185015290930192604090920191600101610626565b509095945050505050565b60006020828403121561066d57600080fd5b610676826105a4565b9392505050565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b600182810b9082900b03617fff198112617fff821317156106e2576106e26106a9565b92915050565b600181810b9083900b01617fff8113617fff19821217156106e2576106e26106a9565b808201808211156106e2576106e26106a9565b60008160010b617fff8103610735576107356106a9565b6001019291505056fe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`@Qa\x07?8\x03\x80a\x07?\x839\x81\x01`@\x81\x90Ra\0\"\x91a\x05\xBBV[`\0a\0/\x84\x84\x84a\0]V[\x90P`\0\x81`@Q` \x01a\0D\x91\x90a\x06\x0CV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80Q` \x82\x01\xFD[``\x81`\x02\x0B\x83`\x02\x0B\x13\x15a\0rW`\0\x80\xFD[`\0\x84`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xB2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xD6\x91\x90a\x06[V[\x90P\x80\x84\x07`\0\x90\x81\x13\x82\x86\x05\x03`\x02\x90\x81\x0B`\x08\x90\x81\x1D\x92\x84\x87\x07\x81\x13\x85\x88\x05\x03\x90\x92\x0B\x90\x1D\x90\x80a\x01\n\x89\x85\x85a\x01\xCEV[\x91P\x91P`\x02\x81\x90\x1B`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01*Wa\x01*a\x06}V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01oW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01HW\x90P[P\x95P`\0\x84[\x84`\x01\x0B\x81`\x01\x0B\x13a\x01\xC0Wa\x01\xB6\x8B\x82\x89\x87\x8A\x86\x03a\xFF\xFF\x16\x81Q\x81\x10a\x01\xA1Wa\x01\xA1a\x06\x93V[` \x02` \x01\x01Q\x8C\x87a\x03Q` \x1B` \x1CV[\x91P`\x01\x01a\x01vV[PPPPPPP\x93\x92PPPV[```\0a\x01\xDC\x84\x84a\x06\xBFV[a\x01\xE7\x90`\x01a\x06\xE8V[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\x02Wa\x02\x02a\x06}V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02+W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x83[\x83`\x01\x0B\x81`\x01\x0B\x13a\x03HW`\0a\x02R`\x01`\x01`\xA0\x1B\x03\x88\x16\x83a\x04\xF7V[\x90P\x80\x84a\x02`\x88\x85a\x06\xBFV[a\xFF\xFF\x16\x81Q\x81\x10a\x02tWa\x02ta\x06\x93V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x7F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x7FUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU`\x01\x83\x90\x1C\x16\x82\x03`\x02\x81\x90\x1C\x7F33333333333333333333333333333333\x90\x81\x16\x91\x16\x01`\x04\x81\x90\x1C\x01\x16\x7F\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x02`\xF8\x1C`\0\x19\x82\x14`\x08\x1B\x17a\x034\x90\x84a\x07\x0BV[\x92PP\x80a\x03A\x90a\x07\x1EV[\x90Pa\x020V[P\x93P\x93\x91PPV[`\0`\x05\x81[a\x01\0\x81\x10\x15a\x04\xEAW`\x01\x81\x1B\x86\x16\x15a\x04\xE2W`\x08\x88\x90\x1B\x81\x01\x87\x02`\0a\x03\x8A`\x01`\x01`\xA0\x1B\x03\x8C\x16\x83a\x05\x1EV[`\0\x83\x81R` \x86\x81R`@\x91\x82\x90 \x83Q\x82\x85\x01Q\x84Q\x80\x86\x01\x90\x95R\x82\x85R`\x80\x1B\x17\x91\x83\x01\x82\x90R\x8AQ`\x01\x8B\x81\x01\x9B\x95\x96P\x90\x91\x01\x93\x91\x92\x91\x8B\x91\x81\x10a\x03\xD7Wa\x03\xD7a\x06\x93V[` \x02` \x01\x01\x81\x90RPP`@Q\x80`@\x01`@R\x80\x82\x80`\x01\x01\x93P\x81R` \x01\x83`@\x01Q\x81RP\x88\x88\x80`\x01\x01\x99P\x81Q\x81\x10a\x04\x1AWa\x04\x1Aa\x06\x93V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80\x82\x80`\x01\x01\x93P\x81R` \x01\x83``\x01Q\x81RP\x88\x88\x80`\x01\x01\x99P\x81Q\x81\x10a\x04\\Wa\x04\\a\x06\x93V[` \x02` \x01\x01\x81\x90RP`\0`\xE0\x83\x01Q\x80`\xF8\x1B\x91PP`\xC0\x83\x01Q\x81\x81`\xD8\x1B\x17\x91PP`\xA0\x83\x01Q\x81\x81`8\x1B\x17\x91PP`\x80\x83\x01Qf\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x81\x17\x91PP`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x82\x81RP\x89\x89\x80`\x01\x01\x9AP\x81Q\x81\x10a\x04\xD2Wa\x04\xD2a\x06\x93V[` \x02` \x01\x01\x81\x90RPPPPP[`\x01\x01a\x03WV[P\x91\x97\x96PPPPPPPV[`\0`\x01\x82\x90\x0Ba\x05\x13\x84c)\x9C\xE1K`\xE1\x1B\x83\x85` a\x05\x83V[PP`\0Q\x92\x91PPV[`@\x80Qa\x01\0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R`\x02\x82\x90\x0B\x81a\x05{\x85c\xF3\r\xBA\x93`\xE0\x1B\x84\x84a\x01\0a\x05\x83V[PP\x92\x91PPV[\x83`\0R\x82`\x04R\x80\x82`$`\0\x88Z\xFAa\x05\x9DW`\0\x80\xFD[PPPPPV[\x80Q`\x02\x81\x90\x0B\x81\x14a\x05\xB6W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x05\xD0W`\0\x80\xFD[\x83Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xE7W`\0\x80\xFD[\x92Pa\x05\xF5` \x85\x01a\x05\xA4V[\x91Pa\x06\x03`@\x85\x01a\x05\xA4V[\x90P\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x06PW\x83Q\x80Q\x84R` \x90\x81\x01Q\x81\x85\x01R\x90\x93\x01\x92`@\x90\x92\x01\x91`\x01\x01a\x06&V[P\x90\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x06mW`\0\x80\xFD[a\x06v\x82a\x05\xA4V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01\x82\x81\x0B\x90\x82\x90\x0B\x03a\x7F\xFF\x19\x81\x12a\x7F\xFF\x82\x13\x17\x15a\x06\xE2Wa\x06\xE2a\x06\xA9V[\x92\x91PPV[`\x01\x81\x81\x0B\x90\x83\x90\x0B\x01a\x7F\xFF\x81\x13a\x7F\xFF\x19\x82\x12\x17\x15a\x06\xE2Wa\x06\xE2a\x06\xA9V[\x80\x82\x01\x80\x82\x11\x15a\x06\xE2Wa\x06\xE2a\x06\xA9V[`\0\x81`\x01\x0Ba\x7F\xFF\x81\x03a\x075Wa\x075a\x06\xA9V[`\x01\x01\x92\x91PPV\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061001e5760003560e01c8063f2bb008b14610023575b600080fd5b610036610031366004610618565b61004c565b604051610043919061067c565b60405180910390f35b60608160020b8360020b131561006157600080fd5b60008473ffffffffffffffffffffffffffffffffffffffff1663d0c93a7c6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100ae573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100d291906106cb565b9050808407600090811382860503600290810b600890811d9284870781138588050390920b901d90806101068985856101c5565b91509150600281901b67ffffffffffffffff811115610127576101276106ef565b60405190808252806020026020018201604052801561016c57816020015b60408051808201909152600080825260208201528152602001906001900390816101455790505b5095506000845b8460010b8160010b136101b7576101ad8b8289878a860361ffff168151811061019e5761019e61071e565b60200260200101518c87610374565b9150600101610173565b505050505050509392505050565b606060006101d3848461077c565b6101de9060016107c2565b61ffff1667ffffffffffffffff8111156101fa576101fa6106ef565b604051908082528060200260200182016040528015610223578160200160208202803683370190505b509150835b8360010b8160010b1361036b57600061025773ffffffffffffffffffffffffffffffffffffffff881683610527565b90508084610265888561077c565b61ffff16815181106102795761027961071e565b60209081029190910101527f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f7f5555555555555555555555555555555555555555555555555555555555555555600183901c168203600281901c7f3333333333333333333333333333333333333333333333333333333333333333908116911601600481901c01167f01010101010101010101010101010101010101010101010101010101010101010260f81c7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff821460081b176103579084610802565b9250508061036490610815565b9050610228565b50935093915050565b60006005815b61010081101561051a576001811b86161561051257600888901b8101870260006103ba73ffffffffffffffffffffffffffffffffffffffff8c1683610567565b60008381526020868152604091829020835182850151845180860190955282855260801b179183018290528a5160018b81019b959650909101939192918b9181106104075761040761071e565b6020026020010181905250506040518060400160405280828060010193508152602001836040015181525088888060010199508151811061044a5761044a61071e565b60200260200101819052506040518060400160405280828060010193508152602001836060015181525088888060010199508151811061048c5761048c61071e565b6020026020010181905250600060e08301518060f81b91505060c0830151818160d81b1791505060a0830151818160381b17915050608083015166ffffffffffffff168181179150506040518060400160405280838152602001828152508989806001019a50815181106105025761050261071e565b6020026020010181905250505050505b60010161037a565b5091979650505050505050565b6000600182900b61055c847f5339c29600000000000000000000000000000000000000000000000000000000838560206105e5565b505060005192915050565b6040805161010081018252600080825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e0810191909152600282900b816105dd857ff30dba930000000000000000000000000000000000000000000000000000000084846101006105e5565b505092915050565b8360005282600452808260246000885afa6105ff57600080fd5b5050505050565b8060020b811461061557600080fd5b50565b60008060006060848603121561062d57600080fd5b833573ffffffffffffffffffffffffffffffffffffffff8116811461065157600080fd5b9250602084013561066181610606565b9150604084013561067181610606565b809150509250925092565b602080825282518282018190526000918401906040840190835b818110156106c0578351805184526020908101518185015290930192604090920191600101610696565b509095945050505050565b6000602082840312156106dd57600080fd5b81516106e881610606565b9392505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b600182810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80008112617fff821317156107bc576107bc61074d565b92915050565b600181810b9083900b01617fff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000821217156107bc576107bc61074d565b808201808211156107bc576107bc61074d565b60008160010b617fff810361082c5761082c61074d565b6001019291505056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0\x1EW`\x005`\xE0\x1C\x80c\xF2\xBB\0\x8B\x14a\0#W[`\0\x80\xFD[a\x006a\x0016`\x04a\x06\x18V[a\0LV[`@Qa\0C\x91\x90a\x06|V[`@Q\x80\x91\x03\x90\xF3[``\x81`\x02\x0B\x83`\x02\x0B\x13\x15a\0aW`\0\x80\xFD[`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xAEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xD2\x91\x90a\x06\xCBV[\x90P\x80\x84\x07`\0\x90\x81\x13\x82\x86\x05\x03`\x02\x90\x81\x0B`\x08\x90\x81\x1D\x92\x84\x87\x07\x81\x13\x85\x88\x05\x03\x90\x92\x0B\x90\x1D\x90\x80a\x01\x06\x89\x85\x85a\x01\xC5V[\x91P\x91P`\x02\x81\x90\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01'Wa\x01'a\x06\xEFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01lW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01EW\x90P[P\x95P`\0\x84[\x84`\x01\x0B\x81`\x01\x0B\x13a\x01\xB7Wa\x01\xAD\x8B\x82\x89\x87\x8A\x86\x03a\xFF\xFF\x16\x81Q\x81\x10a\x01\x9EWa\x01\x9Ea\x07\x1EV[` \x02` \x01\x01Q\x8C\x87a\x03tV[\x91P`\x01\x01a\x01sV[PPPPPPP\x93\x92PPPV[```\0a\x01\xD3\x84\x84a\x07|V[a\x01\xDE\x90`\x01a\x07\xC2V[a\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xFAWa\x01\xFAa\x06\xEFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02#W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x83[\x83`\x01\x0B\x81`\x01\x0B\x13a\x03kW`\0a\x02Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x83a\x05'V[\x90P\x80\x84a\x02e\x88\x85a\x07|V[a\xFF\xFF\x16\x81Q\x81\x10a\x02yWa\x02ya\x07\x1EV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x7F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x7FUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU`\x01\x83\x90\x1C\x16\x82\x03`\x02\x81\x90\x1C\x7F33333333333333333333333333333333\x90\x81\x16\x91\x16\x01`\x04\x81\x90\x1C\x01\x16\x7F\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x02`\xF8\x1C\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x14`\x08\x1B\x17a\x03W\x90\x84a\x08\x02V[\x92PP\x80a\x03d\x90a\x08\x15V[\x90Pa\x02(V[P\x93P\x93\x91PPV[`\0`\x05\x81[a\x01\0\x81\x10\x15a\x05\x1AW`\x01\x81\x1B\x86\x16\x15a\x05\x12W`\x08\x88\x90\x1B\x81\x01\x87\x02`\0a\x03\xBAs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x16\x83a\x05gV[`\0\x83\x81R` \x86\x81R`@\x91\x82\x90 \x83Q\x82\x85\x01Q\x84Q\x80\x86\x01\x90\x95R\x82\x85R`\x80\x1B\x17\x91\x83\x01\x82\x90R\x8AQ`\x01\x8B\x81\x01\x9B\x95\x96P\x90\x91\x01\x93\x91\x92\x91\x8B\x91\x81\x10a\x04\x07Wa\x04\x07a\x07\x1EV[` \x02` \x01\x01\x81\x90RPP`@Q\x80`@\x01`@R\x80\x82\x80`\x01\x01\x93P\x81R` \x01\x83`@\x01Q\x81RP\x88\x88\x80`\x01\x01\x99P\x81Q\x81\x10a\x04JWa\x04Ja\x07\x1EV[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80\x82\x80`\x01\x01\x93P\x81R` \x01\x83``\x01Q\x81RP\x88\x88\x80`\x01\x01\x99P\x81Q\x81\x10a\x04\x8CWa\x04\x8Ca\x07\x1EV[` \x02` \x01\x01\x81\x90RP`\0`\xE0\x83\x01Q\x80`\xF8\x1B\x91PP`\xC0\x83\x01Q\x81\x81`\xD8\x1B\x17\x91PP`\xA0\x83\x01Q\x81\x81`8\x1B\x17\x91PP`\x80\x83\x01Qf\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x81\x17\x91PP`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x82\x81RP\x89\x89\x80`\x01\x01\x9AP\x81Q\x81\x10a\x05\x02Wa\x05\x02a\x07\x1EV[` \x02` \x01\x01\x81\x90RPPPPP[`\x01\x01a\x03zV[P\x91\x97\x96PPPPPPPV[`\0`\x01\x82\x90\x0Ba\x05\\\x84\x7FS9\xC2\x96\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x85` a\x05\xE5V[PP`\0Q\x92\x91PPV[`@\x80Qa\x01\0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R`\x02\x82\x90\x0B\x81a\x05\xDD\x85\x7F\xF3\r\xBA\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84a\x01\0a\x05\xE5V[PP\x92\x91PPV[\x83`\0R\x82`\x04R\x80\x82`$`\0\x88Z\xFAa\x05\xFFW`\0\x80\xFD[PPPPPV[\x80`\x02\x0B\x81\x14a\x06\x15W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x06-W`\0\x80\xFD[\x835s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06QW`\0\x80\xFD[\x92P` \x84\x015a\x06a\x81a\x06\x06V[\x91P`@\x84\x015a\x06q\x81a\x06\x06V[\x80\x91PP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x06\xC0W\x83Q\x80Q\x84R` \x90\x81\x01Q\x81\x85\x01R\x90\x93\x01\x92`@\x90\x92\x01\x91`\x01\x01a\x06\x96V[P\x90\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x06\xDDW`\0\x80\xFD[\x81Qa\x06\xE8\x81a\x06\x06V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\x01\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\x81\x12a\x7F\xFF\x82\x13\x17\x15a\x07\xBCWa\x07\xBCa\x07MV[\x92\x91PPV[`\x01\x81\x81\x0B\x90\x83\x90\x0B\x01a\x7F\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\x82\x12\x17\x15a\x07\xBCWa\x07\xBCa\x07MV[\x80\x82\x01\x80\x82\x11\x15a\x07\xBCWa\x07\xBCa\x07MV[`\0\x81`\x01\x0Ba\x7F\xFF\x81\x03a\x08,Wa\x08,a\x07MV[`\x01\x01\x92\x91PPV",
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
constructor(V3PoolCallee pool, int24 tickLower, int24 tickUpper) payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub tickLower: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub tickUpper: alloy::sol_types::private::primitives::aliases::I24,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                V3PoolCallee,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <V3PoolCallee as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.pool, value.tickLower, value.tickUpper)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pool: tuple.0,
                        tickLower: tuple.1,
                        tickUpper: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                V3PoolCallee,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
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
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickLower),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickUpper),
                )
            }
        }
    };
    /**Function with signature `getPopulatedTicksInRange(address,int24,int24)` and selector `0xf2bb008b`.
```solidity
function getPopulatedTicksInRange(V3PoolCallee pool, int24 tickLower, int24 tickUpper) external payable returns (PoolUtils.Slot[] memory slots);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPopulatedTicksInRangeCall {
        #[allow(missing_docs)]
        pub pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub tickLower: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub tickUpper: alloy::sol_types::private::primitives::aliases::I24,
    }
    ///Container type for the return parameters of the [`getPopulatedTicksInRange(address,int24,int24)`](getPopulatedTicksInRangeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPopulatedTicksInRangeReturn {
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
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <V3PoolCallee as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getPopulatedTicksInRangeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPopulatedTicksInRangeCall) -> Self {
                    (value.pool, value.tickLower, value.tickUpper)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPopulatedTicksInRangeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pool: tuple.0,
                        tickLower: tuple.1,
                        tickUpper: tuple.2,
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
            impl ::core::convert::From<getPopulatedTicksInRangeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPopulatedTicksInRangeReturn) -> Self {
                    (value.slots,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPopulatedTicksInRangeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { slots: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPopulatedTicksInRangeCall {
            type Parameters<'a> = (
                V3PoolCallee,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPopulatedTicksInRangeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<PoolUtils::Slot>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPopulatedTicksInRange(address,int24,int24)";
            const SELECTOR: [u8; 4] = [242u8, 187u8, 0u8, 139u8];
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
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickLower),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickUpper),
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
    ///Container for all the [`EphemeralPoolTicks`](self) function calls.
    pub enum EphemeralPoolTicksCalls {
        #[allow(missing_docs)]
        getPopulatedTicksInRange(getPopulatedTicksInRangeCall),
    }
    #[automatically_derived]
    impl EphemeralPoolTicksCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[242u8, 187u8, 0u8, 139u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for EphemeralPoolTicksCalls {
        const NAME: &'static str = "EphemeralPoolTicksCalls";
        const MIN_DATA_LENGTH: usize = 96usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::getPopulatedTicksInRange(_) => {
                    <getPopulatedTicksInRangeCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<EphemeralPoolTicksCalls>] = &[
                {
                    fn getPopulatedTicksInRange(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EphemeralPoolTicksCalls> {
                        <getPopulatedTicksInRangeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EphemeralPoolTicksCalls::getPopulatedTicksInRange)
                    }
                    getPopulatedTicksInRange
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
                Self::getPopulatedTicksInRange(inner) => {
                    <getPopulatedTicksInRangeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::getPopulatedTicksInRange(inner) => {
                    <getPopulatedTicksInRangeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`EphemeralPoolTicks`](self) contract instance.

See the [wrapper's documentation](`EphemeralPoolTicksInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> EphemeralPoolTicksInstance<T, P, N> {
        EphemeralPoolTicksInstance::<T, P, N>::new(address, provider)
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
        tickLower: alloy::sol_types::private::primitives::aliases::I24,
        tickUpper: alloy::sol_types::private::primitives::aliases::I24,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<EphemeralPoolTicksInstance<T, P, N>>,
    > {
        EphemeralPoolTicksInstance::<
            T,
            P,
            N,
        >::deploy(provider, pool, tickLower, tickUpper)
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
        tickLower: alloy::sol_types::private::primitives::aliases::I24,
        tickUpper: alloy::sol_types::private::primitives::aliases::I24,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        EphemeralPoolTicksInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, pool, tickLower, tickUpper)
    }
    /**A [`EphemeralPoolTicks`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`EphemeralPoolTicks`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EphemeralPoolTicksInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for EphemeralPoolTicksInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EphemeralPoolTicksInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EphemeralPoolTicksInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`EphemeralPoolTicks`](self) contract instance.

See the [wrapper's documentation](`EphemeralPoolTicksInstance`) for more details.*/
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
            tickLower: alloy::sol_types::private::primitives::aliases::I24,
            tickUpper: alloy::sol_types::private::primitives::aliases::I24,
        ) -> alloy_contract::Result<EphemeralPoolTicksInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                pool,
                tickLower,
                tickUpper,
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
            provider: P,
            pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
            tickLower: alloy::sol_types::private::primitives::aliases::I24,
            tickUpper: alloy::sol_types::private::primitives::aliases::I24,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            pool,
                            tickLower,
                            tickUpper,
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
    impl<T, P: ::core::clone::Clone, N> EphemeralPoolTicksInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EphemeralPoolTicksInstance<T, P, N> {
            EphemeralPoolTicksInstance {
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
    > EphemeralPoolTicksInstance<T, P, N> {
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
        ///Creates a new call builder for the [`getPopulatedTicksInRange`] function.
        pub fn getPopulatedTicksInRange(
            &self,
            pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
            tickLower: alloy::sol_types::private::primitives::aliases::I24,
            tickUpper: alloy::sol_types::private::primitives::aliases::I24,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPopulatedTicksInRangeCall, N> {
            self.call_builder(
                &getPopulatedTicksInRangeCall {
                    pool,
                    tickLower,
                    tickUpper,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EphemeralPoolTicksInstance<T, P, N> {
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
