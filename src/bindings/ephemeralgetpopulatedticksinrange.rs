///Module containing a contract's types and functions.
/**

```solidity
library PoolUtils {
    struct PopulatedTick { int24 tick; int128 liquidityNet; uint128 liquidityGross; uint256 feeGrowthOutside0X128; uint256 feeGrowthOutside1X128; }
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
struct PopulatedTick { int24 tick; int128 liquidityNet; uint128 liquidityGross; uint256 feeGrowthOutside0X128; uint256 feeGrowthOutside1X128; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PopulatedTick {
        #[allow(missing_docs)]
        pub tick: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub liquidityNet: i128,
        #[allow(missing_docs)]
        pub liquidityGross: u128,
        #[allow(missing_docs)]
        pub feeGrowthOutside0X128: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub feeGrowthOutside1X128: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Int<24>,
            alloy::sol_types::sol_data::Int<128>,
            alloy::sol_types::sol_data::Uint<128>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::I24,
            i128,
            u128,
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
        impl ::core::convert::From<PopulatedTick> for UnderlyingRustTuple<'_> {
            fn from(value: PopulatedTick) -> Self {
                (
                    value.tick,
                    value.liquidityNet,
                    value.liquidityGross,
                    value.feeGrowthOutside0X128,
                    value.feeGrowthOutside1X128,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PopulatedTick {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    tick: tuple.0,
                    liquidityNet: tuple.1,
                    liquidityGross: tuple.2,
                    feeGrowthOutside0X128: tuple.3,
                    feeGrowthOutside1X128: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PopulatedTick {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PopulatedTick {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tick),
                    <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidityNet),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidityGross),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.feeGrowthOutside0X128,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.feeGrowthOutside1X128),
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
        impl alloy_sol_types::SolType for PopulatedTick {
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
        impl alloy_sol_types::SolStruct for PopulatedTick {
            const NAME: &'static str = "PopulatedTick";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PopulatedTick(int24 tick,int128 liquidityNet,uint128 liquidityGross,uint256 feeGrowthOutside0X128,uint256 feeGrowthOutside1X128)",
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
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tick)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.liquidityNet)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.liquidityGross,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.feeGrowthOutside0X128,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.feeGrowthOutside1X128,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PopulatedTick {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.tick)
                    + <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.liquidityNet,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.liquidityGross,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.feeGrowthOutside0X128,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.feeGrowthOutside1X128,
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
                <alloy::sol_types::sol_data::Int<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tick,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.liquidityNet,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.liquidityGross,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.feeGrowthOutside0X128,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.feeGrowthOutside1X128,
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
    struct PopulatedTick {
        int24 tick;
        int128 liquidityNet;
        uint128 liquidityGross;
        uint256 feeGrowthOutside0X128;
        uint256 feeGrowthOutside1X128;
    }
}

interface EphemeralGetPopulatedTicksInRange {
    type V3PoolCallee is address;

    constructor(V3PoolCallee pool, int24 tickLower, int24 tickUpper) payable;

    function getPopulatedTicksInRange(V3PoolCallee pool, int24 tickLower, int24 tickUpper) external payable returns (PoolUtils.PopulatedTick[] memory populatedTicks, int24 tickSpacing);
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
        "name": "populatedTicks",
        "type": "tuple[]",
        "internalType": "struct PoolUtils.PopulatedTick[]",
        "components": [
          {
            "name": "tick",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "liquidityNet",
            "type": "int128",
            "internalType": "int128"
          },
          {
            "name": "liquidityGross",
            "type": "uint128",
            "internalType": "uint128"
          },
          {
            "name": "feeGrowthOutside0X128",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "feeGrowthOutside1X128",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "tickSpacing",
        "type": "int24",
        "internalType": "int24"
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
pub mod EphemeralGetPopulatedTicksInRange {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60806040526040516106ad3803806106ad833981016040819052610022916104ed565b600080610030858585610062565b915091506000828260405160200161004992919061053e565b6040516020818303038152906040529050805160208201fd5b606060008260020b8460020b131561007957600080fd5b846001600160a01b031663d0c93a7c6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100b7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100db91906105d0565b9050808407600090811382860503600290810b600890811d9284870781138588050390920b901d908061010f8985856101e3565b91509150806001600160401b0381111561012b5761012b6105eb565b60405190808252806020026020018201604052801561018457816020015b6040805160a0810182526000808252602080830182905292820181905260608201819052608082015282526000199092019101816101495790505b5095506000845b8460010b8160010b136101d5576101cb8b8289878a860361ffff16815181106101b6576101b6610601565b60200260200101518c8761036660201b60201c565b915060010161018b565b505050505050935093915050565b606060006101f1848461062d565b6101fc906001610656565b61ffff166001600160401b03811115610217576102176105eb565b604051908082528060200260200182016040528015610240578160200160208202803683370190505b509150835b8360010b8160010b1361035d5760006102676001600160a01b038816836103d1565b90508084610275888561062d565b61ffff168151811061028957610289610601565b60209081029190910101527f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f7f5555555555555555555555555555555555555555555555555555555555555555600183901c168203600281901c7f3333333333333333333333333333333333333333333333333333333333333333908116911601600481901c01167f01010101010101010101010101010101010101010101010101010101010101010260f81c600019821460081b176103499084610679565b925050806103569061068c565b9050610245565b50935093915050565b6000805b6101008110156103c5576001811b8516156103bd576000818860081b01870290506103bb89828787806001019850815181106103a8576103a8610601565b60200260200101516103f860201b60201c565b505b60010161036a565b50909695505050505050565b6000600182900b6103ed8463299ce14b60e11b83856020610450565b505060005192915050565b600061040d6001600160a01b03851684610471565b60029390930b825250602080830151600f0b9082015281516001600160801b03166040808301919091528201516060808301919091529091015160809091015250565b8360005282600452808260246000885afa61046a57600080fd5b5050505050565b6040805161010081018252600080825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e0810191909152600282900b816104ce8563f30dba9360e01b8484610100610450565b505092915050565b8051600281900b81146104e857600080fd5b919050565b60008060006060848603121561050257600080fd5b83516001600160a01b038116811461051957600080fd5b9250610527602085016104d6565b9150610535604085016104d6565b90509250925092565b6040808252835190820181905260009060208501906060840190835b818110156105b4578351805160020b8452602080820151600f0b818601526040808301516001600160801b03169086015260608083015190860152608091820151918501919091529093019260a09092019160010161055a565b5050600285900b602085015291506105c99050565b9392505050565b6000602082840312156105e257600080fd5b6105c9826104d6565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b600182810b9082900b03617fff198112617fff8213171561065057610650610617565b92915050565b600181810b9083900b01617fff8113617fff198212171561065057610650610617565b8082018082111561065057610650610617565b60008160010b617fff81036106a3576106a3610617565b6001019291505056fe
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`@Qa\x06\xAD8\x03\x80a\x06\xAD\x839\x81\x01`@\x81\x90Ra\0\"\x91a\x04\xEDV[`\0\x80a\x000\x85\x85\x85a\0bV[\x91P\x91P`\0\x82\x82`@Q` \x01a\0I\x92\x91\x90a\x05>V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80Q` \x82\x01\xFD[```\0\x82`\x02\x0B\x84`\x02\x0B\x13\x15a\0yW`\0\x80\xFD[\x84`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xDB\x91\x90a\x05\xD0V[\x90P\x80\x84\x07`\0\x90\x81\x13\x82\x86\x05\x03`\x02\x90\x81\x0B`\x08\x90\x81\x1D\x92\x84\x87\x07\x81\x13\x85\x88\x05\x03\x90\x92\x0B\x90\x1D\x90\x80a\x01\x0F\x89\x85\x85a\x01\xE3V[\x91P\x91P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01+Wa\x01+a\x05\xEBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\x84W\x81` \x01[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x01IW\x90P[P\x95P`\0\x84[\x84`\x01\x0B\x81`\x01\x0B\x13a\x01\xD5Wa\x01\xCB\x8B\x82\x89\x87\x8A\x86\x03a\xFF\xFF\x16\x81Q\x81\x10a\x01\xB6Wa\x01\xB6a\x06\x01V[` \x02` \x01\x01Q\x8C\x87a\x03f` \x1B` \x1CV[\x91P`\x01\x01a\x01\x8BV[PPPPPP\x93P\x93\x91PPV[```\0a\x01\xF1\x84\x84a\x06-V[a\x01\xFC\x90`\x01a\x06VV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\x17Wa\x02\x17a\x05\xEBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02@W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x83[\x83`\x01\x0B\x81`\x01\x0B\x13a\x03]W`\0a\x02g`\x01`\x01`\xA0\x1B\x03\x88\x16\x83a\x03\xD1V[\x90P\x80\x84a\x02u\x88\x85a\x06-V[a\xFF\xFF\x16\x81Q\x81\x10a\x02\x89Wa\x02\x89a\x06\x01V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x7F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x7FUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU`\x01\x83\x90\x1C\x16\x82\x03`\x02\x81\x90\x1C\x7F33333333333333333333333333333333\x90\x81\x16\x91\x16\x01`\x04\x81\x90\x1C\x01\x16\x7F\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x02`\xF8\x1C`\0\x19\x82\x14`\x08\x1B\x17a\x03I\x90\x84a\x06yV[\x92PP\x80a\x03V\x90a\x06\x8CV[\x90Pa\x02EV[P\x93P\x93\x91PPV[`\0\x80[a\x01\0\x81\x10\x15a\x03\xC5W`\x01\x81\x1B\x85\x16\x15a\x03\xBDW`\0\x81\x88`\x08\x1B\x01\x87\x02\x90Pa\x03\xBB\x89\x82\x87\x87\x80`\x01\x01\x98P\x81Q\x81\x10a\x03\xA8Wa\x03\xA8a\x06\x01V[` \x02` \x01\x01Qa\x03\xF8` \x1B` \x1CV[P[`\x01\x01a\x03jV[P\x90\x96\x95PPPPPPV[`\0`\x01\x82\x90\x0Ba\x03\xED\x84c)\x9C\xE1K`\xE1\x1B\x83\x85` a\x04PV[PP`\0Q\x92\x91PPV[`\0a\x04\r`\x01`\x01`\xA0\x1B\x03\x85\x16\x84a\x04qV[`\x02\x93\x90\x93\x0B\x82RP` \x80\x83\x01Q`\x0F\x0B\x90\x82\x01R\x81Q`\x01`\x01`\x80\x1B\x03\x16`@\x80\x83\x01\x91\x90\x91R\x82\x01Q``\x80\x83\x01\x91\x90\x91R\x90\x91\x01Q`\x80\x90\x91\x01RPV[\x83`\0R\x82`\x04R\x80\x82`$`\0\x88Z\xFAa\x04jW`\0\x80\xFD[PPPPPV[`@\x80Qa\x01\0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R`\x02\x82\x90\x0B\x81a\x04\xCE\x85c\xF3\r\xBA\x93`\xE0\x1B\x84\x84a\x01\0a\x04PV[PP\x92\x91PPV[\x80Q`\x02\x81\x90\x0B\x81\x14a\x04\xE8W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x05\x02W`\0\x80\xFD[\x83Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x19W`\0\x80\xFD[\x92Pa\x05'` \x85\x01a\x04\xD6V[\x91Pa\x055`@\x85\x01a\x04\xD6V[\x90P\x92P\x92P\x92V[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R`\0\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a\x05\xB4W\x83Q\x80Q`\x02\x0B\x84R` \x80\x82\x01Q`\x0F\x0B\x81\x86\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x86\x01R``\x80\x83\x01Q\x90\x86\x01R`\x80\x91\x82\x01Q\x91\x85\x01\x91\x90\x91R\x90\x93\x01\x92`\xA0\x90\x92\x01\x91`\x01\x01a\x05ZV[PP`\x02\x85\x90\x0B` \x85\x01R\x91Pa\x05\xC9\x90PV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x05\xE2W`\0\x80\xFD[a\x05\xC9\x82a\x04\xD6V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01\x82\x81\x0B\x90\x82\x90\x0B\x03a\x7F\xFF\x19\x81\x12a\x7F\xFF\x82\x13\x17\x15a\x06PWa\x06Pa\x06\x17V[\x92\x91PPV[`\x01\x81\x81\x0B\x90\x83\x90\x0B\x01a\x7F\xFF\x81\x13a\x7F\xFF\x19\x82\x12\x17\x15a\x06PWa\x06Pa\x06\x17V[\x80\x82\x01\x80\x82\x11\x15a\x06PWa\x06Pa\x06\x17V[`\0\x81`\x01\x0Ba\x7F\xFF\x81\x03a\x06\xA3Wa\x06\xA3a\x06\x17V[`\x01\x01\x92\x91PPV\xFE",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061001e5760003560e01c8063f2bb008b14610023575b600080fd5b610036610031366004610567565b61004d565b6040516100449291906105cb565b60405180910390f35b606060008260020b8460020b131561006457600080fd5b8473ffffffffffffffffffffffffffffffffffffffff1663d0c93a7c6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100af573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100d39190610669565b9050808407600090811382860503600290810b600890811d9284870781138588050390920b901d90806101078985856101f4565b915091508067ffffffffffffffff81111561012457610124610686565b60405190808252806020026020018201604052801561019b57816020015b6040805160a0810182526000808252602080830182905292820181905260608201819052608082015282527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff9092019101816101425790505b5095506000845b8460010b8160010b136101e6576101dc8b8289878a860361ffff16815181106101cd576101cd6106b5565b60200260200101518c876103a3565b91506001016101a2565b505050505050935093915050565b606060006102028484610713565b61020d906001610759565b61ffff1667ffffffffffffffff81111561022957610229610686565b604051908082528060200260200182016040528015610252578160200160208202803683370190505b509150835b8360010b8160010b1361039a57600061028673ffffffffffffffffffffffffffffffffffffffff881683610408565b905080846102948885610713565b61ffff16815181106102a8576102a86106b5565b60209081029190910101527f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f7f5555555555555555555555555555555555555555555555555555555555555555600183901c168203600281901c7f3333333333333333333333333333333333333333333333333333333333333333908116911601600481901c01167f01010101010101010101010101010101010101010101010101010101010101010260f81c7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff821460081b176103869084610799565b92505080610393906107ac565b9050610257565b50935093915050565b6000805b6101008110156103fc576001811b8516156103f4576000818860081b01870290506103f289828787806001019850815181106103e5576103e56106b5565b6020026020010151610448565b505b6001016103a7565b50909695505050505050565b6000600182900b61043d847f5339c29600000000000000000000000000000000000000000000000000000000838560206104b6565b505060005192915050565b600061046a73ffffffffffffffffffffffffffffffffffffffff8516846104d7565b60029390930b825250602080830151600f0b9082015281516fffffffffffffffffffffffffffffffff166040808301919091528201516060808301919091529091015160809091015250565b8360005282600452808260246000885afa6104d057600080fd5b5050505050565b6040805161010081018252600080825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e0810191909152600282900b8161054d857ff30dba930000000000000000000000000000000000000000000000000000000084846101006104b6565b505092915050565b8060020b811461056457600080fd5b50565b60008060006060848603121561057c57600080fd5b833573ffffffffffffffffffffffffffffffffffffffff811681146105a057600080fd5b925060208401356105b081610555565b915060408401356105c081610555565b809150509250925092565b6040808252835190820181905260009060208501906060840190835b8181101561064d578351805160020b84526020810151600f0b60208501526fffffffffffffffffffffffffffffffff604082015116604085015260608101516060850152608081015160808501525060a0830192506020840193506001810190506105e7565b5050600285900b602085015291506106629050565b9392505050565b60006020828403121561067b57600080fd5b815161066281610555565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b600182810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80008112617fff82131715610753576107536106e4565b92915050565b600181810b9083900b01617fff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800082121715610753576107536106e4565b80820180821115610753576107536106e4565b60008160010b617fff81036107c3576107c36106e4565b6001019291505056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0\x1EW`\x005`\xE0\x1C\x80c\xF2\xBB\0\x8B\x14a\0#W[`\0\x80\xFD[a\x006a\x0016`\x04a\x05gV[a\0MV[`@Qa\0D\x92\x91\x90a\x05\xCBV[`@Q\x80\x91\x03\x90\xF3[```\0\x82`\x02\x0B\x84`\x02\x0B\x13\x15a\0dW`\0\x80\xFD[\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD0\xC9:|`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xAFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xD3\x91\x90a\x06iV[\x90P\x80\x84\x07`\0\x90\x81\x13\x82\x86\x05\x03`\x02\x90\x81\x0B`\x08\x90\x81\x1D\x92\x84\x87\x07\x81\x13\x85\x88\x05\x03\x90\x92\x0B\x90\x1D\x90\x80a\x01\x07\x89\x85\x85a\x01\xF4V[\x91P\x91P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01$Wa\x01$a\x06\x86V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\x9BW\x81` \x01[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x01BW\x90P[P\x95P`\0\x84[\x84`\x01\x0B\x81`\x01\x0B\x13a\x01\xE6Wa\x01\xDC\x8B\x82\x89\x87\x8A\x86\x03a\xFF\xFF\x16\x81Q\x81\x10a\x01\xCDWa\x01\xCDa\x06\xB5V[` \x02` \x01\x01Q\x8C\x87a\x03\xA3V[\x91P`\x01\x01a\x01\xA2V[PPPPPP\x93P\x93\x91PPV[```\0a\x02\x02\x84\x84a\x07\x13V[a\x02\r\x90`\x01a\x07YV[a\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02)Wa\x02)a\x06\x86V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02RW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x83[\x83`\x01\x0B\x81`\x01\x0B\x13a\x03\x9AW`\0a\x02\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x83a\x04\x08V[\x90P\x80\x84a\x02\x94\x88\x85a\x07\x13V[a\xFF\xFF\x16\x81Q\x81\x10a\x02\xA8Wa\x02\xA8a\x06\xB5V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x7F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x7FUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU`\x01\x83\x90\x1C\x16\x82\x03`\x02\x81\x90\x1C\x7F33333333333333333333333333333333\x90\x81\x16\x91\x16\x01`\x04\x81\x90\x1C\x01\x16\x7F\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x02`\xF8\x1C\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x14`\x08\x1B\x17a\x03\x86\x90\x84a\x07\x99V[\x92PP\x80a\x03\x93\x90a\x07\xACV[\x90Pa\x02WV[P\x93P\x93\x91PPV[`\0\x80[a\x01\0\x81\x10\x15a\x03\xFCW`\x01\x81\x1B\x85\x16\x15a\x03\xF4W`\0\x81\x88`\x08\x1B\x01\x87\x02\x90Pa\x03\xF2\x89\x82\x87\x87\x80`\x01\x01\x98P\x81Q\x81\x10a\x03\xE5Wa\x03\xE5a\x06\xB5V[` \x02` \x01\x01Qa\x04HV[P[`\x01\x01a\x03\xA7V[P\x90\x96\x95PPPPPPV[`\0`\x01\x82\x90\x0Ba\x04=\x84\x7FS9\xC2\x96\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x85` a\x04\xB6V[PP`\0Q\x92\x91PPV[`\0a\x04js\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x84a\x04\xD7V[`\x02\x93\x90\x93\x0B\x82RP` \x80\x83\x01Q`\x0F\x0B\x90\x82\x01R\x81Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x80\x83\x01\x91\x90\x91R\x82\x01Q``\x80\x83\x01\x91\x90\x91R\x90\x91\x01Q`\x80\x90\x91\x01RPV[\x83`\0R\x82`\x04R\x80\x82`$`\0\x88Z\xFAa\x04\xD0W`\0\x80\xFD[PPPPPV[`@\x80Qa\x01\0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R`\x02\x82\x90\x0B\x81a\x05M\x85\x7F\xF3\r\xBA\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84a\x01\0a\x04\xB6V[PP\x92\x91PPV[\x80`\x02\x0B\x81\x14a\x05dW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x05|W`\0\x80\xFD[\x835s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\xA0W`\0\x80\xFD[\x92P` \x84\x015a\x05\xB0\x81a\x05UV[\x91P`@\x84\x015a\x05\xC0\x81a\x05UV[\x80\x91PP\x92P\x92P\x92V[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R`\0\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a\x06MW\x83Q\x80Q`\x02\x0B\x84R` \x81\x01Q`\x0F\x0B` \x85\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x85\x01R``\x81\x01Q``\x85\x01R`\x80\x81\x01Q`\x80\x85\x01RP`\xA0\x83\x01\x92P` \x84\x01\x93P`\x01\x81\x01\x90Pa\x05\xE7V[PP`\x02\x85\x90\x0B` \x85\x01R\x91Pa\x06b\x90PV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06{W`\0\x80\xFD[\x81Qa\x06b\x81a\x05UV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\x01\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\x81\x12a\x7F\xFF\x82\x13\x17\x15a\x07SWa\x07Sa\x06\xE4V[\x92\x91PPV[`\x01\x81\x81\x0B\x90\x83\x90\x0B\x01a\x7F\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\x82\x12\x17\x15a\x07SWa\x07Sa\x06\xE4V[\x80\x82\x01\x80\x82\x11\x15a\x07SWa\x07Sa\x06\xE4V[`\0\x81`\x01\x0Ba\x7F\xFF\x81\x03a\x07\xC3Wa\x07\xC3a\x06\xE4V[`\x01\x01\x92\x91PPV",
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
function getPopulatedTicksInRange(V3PoolCallee pool, int24 tickLower, int24 tickUpper) external payable returns (PoolUtils.PopulatedTick[] memory populatedTicks, int24 tickSpacing);
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
        pub populatedTicks: alloy::sol_types::private::Vec<
            <PoolUtils::PopulatedTick as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
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
                alloy::sol_types::sol_data::Array<PoolUtils::PopulatedTick>,
                alloy::sol_types::sol_data::Int<24>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <PoolUtils::PopulatedTick as alloy::sol_types::SolType>::RustType,
                >,
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
            impl ::core::convert::From<getPopulatedTicksInRangeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPopulatedTicksInRangeReturn) -> Self {
                    (value.populatedTicks, value.tickSpacing)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPopulatedTicksInRangeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        populatedTicks: tuple.0,
                        tickSpacing: tuple.1,
                    }
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
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<PoolUtils::PopulatedTick>,
                alloy::sol_types::sol_data::Int<24>,
            );
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
    ///Container for all the [`EphemeralGetPopulatedTicksInRange`](self) function calls.
    pub enum EphemeralGetPopulatedTicksInRangeCalls {
        #[allow(missing_docs)]
        getPopulatedTicksInRange(getPopulatedTicksInRangeCall),
    }
    #[automatically_derived]
    impl EphemeralGetPopulatedTicksInRangeCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[242u8, 187u8, 0u8, 139u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for EphemeralGetPopulatedTicksInRangeCalls {
        const NAME: &'static str = "EphemeralGetPopulatedTicksInRangeCalls";
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
            ) -> alloy_sol_types::Result<EphemeralGetPopulatedTicksInRangeCalls>] = &[
                {
                    fn getPopulatedTicksInRange(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<
                        EphemeralGetPopulatedTicksInRangeCalls,
                    > {
                        <getPopulatedTicksInRangeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                EphemeralGetPopulatedTicksInRangeCalls::getPopulatedTicksInRange,
                            )
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
    /**Creates a new wrapper around an on-chain [`EphemeralGetPopulatedTicksInRange`](self) contract instance.

See the [wrapper's documentation](`EphemeralGetPopulatedTicksInRangeInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> EphemeralGetPopulatedTicksInRangeInstance<T, P, N> {
        EphemeralGetPopulatedTicksInRangeInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<
            EphemeralGetPopulatedTicksInRangeInstance<T, P, N>,
        >,
    > {
        EphemeralGetPopulatedTicksInRangeInstance::<
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
        EphemeralGetPopulatedTicksInRangeInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, pool, tickLower, tickUpper)
    }
    /**A [`EphemeralGetPopulatedTicksInRange`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`EphemeralGetPopulatedTicksInRange`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EphemeralGetPopulatedTicksInRangeInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug
    for EphemeralGetPopulatedTicksInRangeInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EphemeralGetPopulatedTicksInRangeInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EphemeralGetPopulatedTicksInRangeInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`EphemeralGetPopulatedTicksInRange`](self) contract instance.

See the [wrapper's documentation](`EphemeralGetPopulatedTicksInRangeInstance`) for more details.*/
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
        ) -> alloy_contract::Result<EphemeralGetPopulatedTicksInRangeInstance<T, P, N>> {
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
    impl<
        T,
        P: ::core::clone::Clone,
        N,
    > EphemeralGetPopulatedTicksInRangeInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> EphemeralGetPopulatedTicksInRangeInstance<T, P, N> {
            EphemeralGetPopulatedTicksInRangeInstance {
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
    > EphemeralGetPopulatedTicksInRangeInstance<T, P, N> {
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
    > EphemeralGetPopulatedTicksInRangeInstance<T, P, N> {
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
