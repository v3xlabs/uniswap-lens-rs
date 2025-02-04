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

interface EphemeralPoolSlots {
    type V3PoolCallee is address;

    constructor(V3PoolCallee pool) payable;

    function getSlots(V3PoolCallee pool) external payable returns (PoolUtils.Slot[] memory slots);
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
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "getSlots",
    "inputs": [
      {
        "name": "pool",
        "type": "address",
        "internalType": "V3PoolCallee"
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
pub mod EphemeralPoolSlots {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405260405161056a38038061056a833981016040819052610022916104c5565b600061002d8261005b565b905060008160405160200161004291906104ee565b6040516020818303038152906040529050805160208201fd5b60606000808080808080806100786001600160a01b038b16610340565b61ffff84169e50959c50939a50919850965094509250905062ffffff60a01b60a087901b1660b886901b60c886901b60d886901b60e886901b60f086901b17171717178717600589016001600160401b038111156100d8576100d861053d565b60405190808252806020026020018201604052801561011d57816020015b60408051808201909152600080825260208201528152602001906001900390816100f65790505b509950604051806040016040528060008152602001828152508a60008151811061014957610149610553565b60200260200101819052505050505050505050604051806040016040528060018152602001610186856001600160a01b03166103a860201b60201c565b8152508260018151811061019c5761019c610553565b60200260200101819052506040518060400160405280600281526020016101d1856001600160a01b03166103c160201b60201c565b815250826002815181106101e7576101e7610553565b60209081029190910101526000806102076001600160a01b0386166103d4565b915091506000828260801b179050604051806040016040528060038152602001828152508560038151811061023e5761023e610553565b6020026020010181905250505050604051806040016040528060048152602001610276856001600160a01b03166103f660201b60201c565b6001600160801b03168152508260048151811061029557610295610553565b602002602001018190525060005b818110156103395760008080806102c36001600160a01b03891686610411565b935093509350935060008160f81b9050808360581b179050808466ffffffffffffff1660201b179050808517905060405180604001604052808760080181526020018281525088876005018151811061031e5761031e610553565b602002602001018190525050505050508060010190506102a3565b5050919050565b600080600080600080600080633850c7bd60e01b90506040518160005260e081600460008d5afa61037057600080fd5b8051602082015160408301516060840151608085015160a086015160c090960151949f939e50919c509a509850919650945092505050565b60006103bb8263f305839960e01b610454565b92915050565b60006103bb82634614131960e01b610454565b60008080806103ea85631ad8b03b60e01b610478565b90969095509350505050565b60008061040a83630d34328160e11b610454565b9392505050565b6040516000908190819081906104328763252c09d760e01b888460806104a4565b8051602082015160408301516060909301519199909850919650945092505050565b6000816000526020600060046000865afa61046e57600080fd5b5050600051919050565b600080826000526040600060046000875afa61049357600080fd5b600051915060205190509250929050565b8360005282600452808260246000885afa6104be57600080fd5b5050505050565b6000602082840312156104d757600080fd5b81516001600160a01b038116811461040a57600080fd5b602080825282518282018190526000918401906040840190835b81811015610532578351805184526020908101518185015290930192604090920191600101610508565b509095945050505050565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052603260045260246000fdfe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`@Qa\x05j8\x03\x80a\x05j\x839\x81\x01`@\x81\x90Ra\0\"\x91a\x04\xC5V[`\0a\0-\x82a\0[V[\x90P`\0\x81`@Q` \x01a\0B\x91\x90a\x04\xEEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80Q` \x82\x01\xFD[```\0\x80\x80\x80\x80\x80\x80\x80a\0x`\x01`\x01`\xA0\x1B\x03\x8B\x16a\x03@V[a\xFF\xFF\x84\x16\x9EP\x95\x9CP\x93\x9AP\x91\x98P\x96P\x94P\x92P\x90Pb\xFF\xFF\xFF`\xA0\x1B`\xA0\x87\x90\x1B\x16`\xB8\x86\x90\x1B`\xC8\x86\x90\x1B`\xD8\x86\x90\x1B`\xE8\x86\x90\x1B`\xF0\x86\x90\x1B\x17\x17\x17\x17\x17\x87\x17`\x05\x89\x01`\x01`\x01`@\x1B\x03\x81\x11\x15a\0\xD8Wa\0\xD8a\x05=V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\x1DW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\0\xF6W\x90P[P\x99P`@Q\x80`@\x01`@R\x80`\0\x81R` \x01\x82\x81RP\x8A`\0\x81Q\x81\x10a\x01IWa\x01Ia\x05SV[` \x02` \x01\x01\x81\x90RPPPPPPPPP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01a\x01\x86\x85`\x01`\x01`\xA0\x1B\x03\x16a\x03\xA8` \x1B` \x1CV[\x81RP\x82`\x01\x81Q\x81\x10a\x01\x9CWa\x01\x9Ca\x05SV[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x01\xD1\x85`\x01`\x01`\xA0\x1B\x03\x16a\x03\xC1` \x1B` \x1CV[\x81RP\x82`\x02\x81Q\x81\x10a\x01\xE7Wa\x01\xE7a\x05SV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\0\x80a\x02\x07`\x01`\x01`\xA0\x1B\x03\x86\x16a\x03\xD4V[\x91P\x91P`\0\x82\x82`\x80\x1B\x17\x90P`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x82\x81RP\x85`\x03\x81Q\x81\x10a\x02>Wa\x02>a\x05SV[` \x02` \x01\x01\x81\x90RPPPP`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01a\x02v\x85`\x01`\x01`\xA0\x1B\x03\x16a\x03\xF6` \x1B` \x1CV[`\x01`\x01`\x80\x1B\x03\x16\x81RP\x82`\x04\x81Q\x81\x10a\x02\x95Wa\x02\x95a\x05SV[` \x02` \x01\x01\x81\x90RP`\0[\x81\x81\x10\x15a\x039W`\0\x80\x80\x80a\x02\xC3`\x01`\x01`\xA0\x1B\x03\x89\x16\x86a\x04\x11V[\x93P\x93P\x93P\x93P`\0\x81`\xF8\x1B\x90P\x80\x83`X\x1B\x17\x90P\x80\x84f\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x1B\x17\x90P\x80\x85\x17\x90P`@Q\x80`@\x01`@R\x80\x87`\x08\x01\x81R` \x01\x82\x81RP\x88\x87`\x05\x01\x81Q\x81\x10a\x03\x1EWa\x03\x1Ea\x05SV[` \x02` \x01\x01\x81\x90RPPPPPP\x80`\x01\x01\x90Pa\x02\xA3V[PP\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80c8P\xC7\xBD`\xE0\x1B\x90P`@Q\x81`\0R`\xE0\x81`\x04`\0\x8DZ\xFAa\x03pW`\0\x80\xFD[\x80Q` \x82\x01Q`@\x83\x01Q``\x84\x01Q`\x80\x85\x01Q`\xA0\x86\x01Q`\xC0\x90\x96\x01Q\x94\x9F\x93\x9EP\x91\x9CP\x9AP\x98P\x91\x96P\x94P\x92PPPV[`\0a\x03\xBB\x82c\xF3\x05\x83\x99`\xE0\x1Ba\x04TV[\x92\x91PPV[`\0a\x03\xBB\x82cF\x14\x13\x19`\xE0\x1Ba\x04TV[`\0\x80\x80\x80a\x03\xEA\x85c\x1A\xD8\xB0;`\xE0\x1Ba\x04xV[\x90\x96\x90\x95P\x93PPPPV[`\0\x80a\x04\n\x83c\r42\x81`\xE1\x1Ba\x04TV[\x93\x92PPPV[`@Q`\0\x90\x81\x90\x81\x90\x81\x90a\x042\x87c%,\t\xD7`\xE0\x1B\x88\x84`\x80a\x04\xA4V[\x80Q` \x82\x01Q`@\x83\x01Q``\x90\x93\x01Q\x91\x99\x90\x98P\x91\x96P\x94P\x92PPPV[`\0\x81`\0R` `\0`\x04`\0\x86Z\xFAa\x04nW`\0\x80\xFD[PP`\0Q\x91\x90PV[`\0\x80\x82`\0R`@`\0`\x04`\0\x87Z\xFAa\x04\x93W`\0\x80\xFD[`\0Q\x91P` Q\x90P\x92P\x92\x90PV[\x83`\0R\x82`\x04R\x80\x82`$`\0\x88Z\xFAa\x04\xBEW`\0\x80\xFD[PPPPPV[`\0` \x82\x84\x03\x12\x15a\x04\xD7W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\nW`\0\x80\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x052W\x83Q\x80Q\x84R` \x90\x81\x01Q\x81\x85\x01R\x90\x93\x01\x92`@\x90\x92\x01\x91`\x01\x01a\x05\x08V[P\x90\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061001e5760003560e01c80631d36f0f014610023575b600080fd5b61003661003136600461058d565b61004c565b60405161004391906105c3565b60405180910390f35b60606000806000806000806000806100798a73ffffffffffffffffffffffffffffffffffffffff1661038b565b61ffff84169e50959c50939a50919850965094509250905076ffffff000000000000000000000000000000000000000060a087901b1660b886901b60c886901b60d886901b60e886901b60f086901b171717171787176005890167ffffffffffffffff8111156100eb576100eb610612565b60405190808252806020026020018201604052801561013057816020015b60408051808201909152600080825260208201528152602001906001900390816101095790505b509950604051806040016040528060008152602001828152508a60008151811061015c5761015c610641565b602002602001018190525050505050505050506040518060400160405280600181526020016101a08573ffffffffffffffffffffffffffffffffffffffff166103f3565b815250826001815181106101b6576101b6610641565b60200260200101819052506040518060400160405280600281526020016101f28573ffffffffffffffffffffffffffffffffffffffff16610425565b8152508260028151811061020857610208610641565b60200260200101819052506000806102358573ffffffffffffffffffffffffffffffffffffffff16610451565b915091506000828260801b179050604051806040016040528060038152602001828152508560038151811061026c5761026c610641565b60200260200101819052505050506040518060400160405280600481526020016102ab8573ffffffffffffffffffffffffffffffffffffffff1661048c565b6fffffffffffffffffffffffffffffffff16815250826004815181106102d3576102d3610641565b602002602001018190525060005b8181101561038457600080808061030e73ffffffffffffffffffffffffffffffffffffffff8916866104c0565b935093509350935060008160f81b9050808360581b179050808466ffffffffffffff1660201b179050808517905060405180604001604052808760080181526020018281525088876005018151811061036957610369610641565b602002602001018190525050505050508060010190506102e1565b5050919050565b600080600080600080600080633850c7bd60e01b90506040518160005260e081600460008d5afa6103bb57600080fd5b8051602082015160408301516060840151608085015160a086015160c090960151949f939e50919c509a509850919650945092505050565b600061041f827ff30583990000000000000000000000000000000000000000000000000000000061051c565b92915050565b600061041f827f461413190000000000000000000000000000000000000000000000000000000061051c565b6000808080610480857f1ad8b03b00000000000000000000000000000000000000000000000000000000610540565b90969095509350505050565b6000806104b9837f1a6865020000000000000000000000000000000000000000000000000000000061051c565b9392505050565b6040516000908190819081906104fa877f252c09d7000000000000000000000000000000000000000000000000000000008884608061056c565b8051602082015160408301516060909301519199909850919650945092505050565b6000816000526020600060046000865afa61053657600080fd5b5050600051919050565b600080826000526040600060046000875afa61055b57600080fd5b600051915060205190509250929050565b8360005282600452808260246000885afa61058657600080fd5b5050505050565b60006020828403121561059f57600080fd5b813573ffffffffffffffffffffffffffffffffffffffff811681146104b957600080fd5b602080825282518282018190526000918401906040840190835b818110156106075783518051845260209081015181850152909301926040909201916001016105dd565b509095945050505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0\x1EW`\x005`\xE0\x1C\x80c\x1D6\xF0\xF0\x14a\0#W[`\0\x80\xFD[a\x006a\x0016`\x04a\x05\x8DV[a\0LV[`@Qa\0C\x91\x90a\x05\xC3V[`@Q\x80\x91\x03\x90\xF3[```\0\x80`\0\x80`\0\x80`\0\x80a\0y\x8As\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\x8BV[a\xFF\xFF\x84\x16\x9EP\x95\x9CP\x93\x9AP\x91\x98P\x96P\x94P\x92P\x90Pv\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xA0\x87\x90\x1B\x16`\xB8\x86\x90\x1B`\xC8\x86\x90\x1B`\xD8\x86\x90\x1B`\xE8\x86\x90\x1B`\xF0\x86\x90\x1B\x17\x17\x17\x17\x17\x87\x17`\x05\x89\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xEBWa\0\xEBa\x06\x12V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x010W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01\tW\x90P[P\x99P`@Q\x80`@\x01`@R\x80`\0\x81R` \x01\x82\x81RP\x8A`\0\x81Q\x81\x10a\x01\\Wa\x01\\a\x06AV[` \x02` \x01\x01\x81\x90RPPPPPPPPP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01a\x01\xA0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xF3V[\x81RP\x82`\x01\x81Q\x81\x10a\x01\xB6Wa\x01\xB6a\x06AV[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x01\xF2\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04%V[\x81RP\x82`\x02\x81Q\x81\x10a\x02\x08Wa\x02\x08a\x06AV[` \x02` \x01\x01\x81\x90RP`\0\x80a\x025\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04QV[\x91P\x91P`\0\x82\x82`\x80\x1B\x17\x90P`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x82\x81RP\x85`\x03\x81Q\x81\x10a\x02lWa\x02la\x06AV[` \x02` \x01\x01\x81\x90RPPPP`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01a\x02\xAB\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04\x8CV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x82`\x04\x81Q\x81\x10a\x02\xD3Wa\x02\xD3a\x06AV[` \x02` \x01\x01\x81\x90RP`\0[\x81\x81\x10\x15a\x03\x84W`\0\x80\x80\x80a\x03\x0Es\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x86a\x04\xC0V[\x93P\x93P\x93P\x93P`\0\x81`\xF8\x1B\x90P\x80\x83`X\x1B\x17\x90P\x80\x84f\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x1B\x17\x90P\x80\x85\x17\x90P`@Q\x80`@\x01`@R\x80\x87`\x08\x01\x81R` \x01\x82\x81RP\x88\x87`\x05\x01\x81Q\x81\x10a\x03iWa\x03ia\x06AV[` \x02` \x01\x01\x81\x90RPPPPPP\x80`\x01\x01\x90Pa\x02\xE1V[PP\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80c8P\xC7\xBD`\xE0\x1B\x90P`@Q\x81`\0R`\xE0\x81`\x04`\0\x8DZ\xFAa\x03\xBBW`\0\x80\xFD[\x80Q` \x82\x01Q`@\x83\x01Q``\x84\x01Q`\x80\x85\x01Q`\xA0\x86\x01Q`\xC0\x90\x96\x01Q\x94\x9F\x93\x9EP\x91\x9CP\x9AP\x98P\x91\x96P\x94P\x92PPPV[`\0a\x04\x1F\x82\x7F\xF3\x05\x83\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05\x1CV[\x92\x91PPV[`\0a\x04\x1F\x82\x7FF\x14\x13\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05\x1CV[`\0\x80\x80\x80a\x04\x80\x85\x7F\x1A\xD8\xB0;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05@V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80a\x04\xB9\x83\x7F\x1Ahe\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05\x1CV[\x93\x92PPPV[`@Q`\0\x90\x81\x90\x81\x90\x81\x90a\x04\xFA\x87\x7F%,\t\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x84`\x80a\x05lV[\x80Q` \x82\x01Q`@\x83\x01Q``\x90\x93\x01Q\x91\x99\x90\x98P\x91\x96P\x94P\x92PPPV[`\0\x81`\0R` `\0`\x04`\0\x86Z\xFAa\x056W`\0\x80\xFD[PP`\0Q\x91\x90PV[`\0\x80\x82`\0R`@`\0`\x04`\0\x87Z\xFAa\x05[W`\0\x80\xFD[`\0Q\x91P` Q\x90P\x92P\x92\x90PV[\x83`\0R\x82`\x04R\x80\x82`$`\0\x88Z\xFAa\x05\x86W`\0\x80\xFD[PPPPPV[`\0` \x82\x84\x03\x12\x15a\x05\x9FW`\0\x80\xFD[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\xB9W`\0\x80\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x06\x07W\x83Q\x80Q\x84R` \x90\x81\x01Q\x81\x85\x01R\x90\x93\x01\x92`@\x90\x92\x01\x91`\x01\x01a\x05\xDDV[P\x90\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD",
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
constructor(V3PoolCallee pool) payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (V3PoolCallee,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <V3PoolCallee as alloy::sol_types::SolType>::RustType,
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
                    (value.pool,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { pool: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (V3PoolCallee,);
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
                (<V3PoolCallee as alloy_sol_types::SolType>::tokenize(&self.pool),)
            }
        }
    };
    /**Function with signature `getSlots(address)` and selector `0x1d36f0f0`.
```solidity
function getSlots(V3PoolCallee pool) external payable returns (PoolUtils.Slot[] memory slots);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlotsCall {
        #[allow(missing_docs)]
        pub pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getSlots(address)`](getSlotsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlotsReturn {
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
            type UnderlyingSolTuple<'a> = (V3PoolCallee,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <V3PoolCallee as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getSlotsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getSlotsCall) -> Self {
                    (value.pool,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlotsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { pool: tuple.0 }
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
            impl ::core::convert::From<getSlotsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getSlotsReturn) -> Self {
                    (value.slots,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlotsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { slots: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSlotsCall {
            type Parameters<'a> = (V3PoolCallee,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSlotsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<PoolUtils::Slot>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSlots(address)";
            const SELECTOR: [u8; 4] = [29u8, 54u8, 240u8, 240u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<V3PoolCallee as alloy_sol_types::SolType>::tokenize(&self.pool),)
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
    ///Container for all the [`EphemeralPoolSlots`](self) function calls.
    pub enum EphemeralPoolSlotsCalls {
        #[allow(missing_docs)]
        getSlots(getSlotsCall),
    }
    #[automatically_derived]
    impl EphemeralPoolSlotsCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[29u8, 54u8, 240u8, 240u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for EphemeralPoolSlotsCalls {
        const NAME: &'static str = "EphemeralPoolSlotsCalls";
        const MIN_DATA_LENGTH: usize = 32usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::getSlots(_) => <getSlotsCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<EphemeralPoolSlotsCalls>] = &[
                {
                    fn getSlots(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EphemeralPoolSlotsCalls> {
                        <getSlotsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EphemeralPoolSlotsCalls::getSlots)
                    }
                    getSlots
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
                Self::getSlots(inner) => {
                    <getSlotsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::getSlots(inner) => {
                    <getSlotsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`EphemeralPoolSlots`](self) contract instance.

See the [wrapper's documentation](`EphemeralPoolSlotsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> EphemeralPoolSlotsInstance<T, P, N> {
        EphemeralPoolSlotsInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<EphemeralPoolSlotsInstance<T, P, N>>,
    > {
        EphemeralPoolSlotsInstance::<T, P, N>::deploy(provider, pool)
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
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        EphemeralPoolSlotsInstance::<T, P, N>::deploy_builder(provider, pool)
    }
    /**A [`EphemeralPoolSlots`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`EphemeralPoolSlots`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EphemeralPoolSlotsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for EphemeralPoolSlotsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EphemeralPoolSlotsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EphemeralPoolSlotsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`EphemeralPoolSlots`](self) contract instance.

See the [wrapper's documentation](`EphemeralPoolSlotsInstance`) for more details.*/
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
        ) -> alloy_contract::Result<EphemeralPoolSlotsInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, pool);
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
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { pool },
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
    impl<T, P: ::core::clone::Clone, N> EphemeralPoolSlotsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EphemeralPoolSlotsInstance<T, P, N> {
            EphemeralPoolSlotsInstance {
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
    > EphemeralPoolSlotsInstance<T, P, N> {
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
        ///Creates a new call builder for the [`getSlots`] function.
        pub fn getSlots(
            &self,
            pool: <V3PoolCallee as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getSlotsCall, N> {
            self.call_builder(&getSlotsCall { pool })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EphemeralPoolSlotsInstance<T, P, N> {
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
