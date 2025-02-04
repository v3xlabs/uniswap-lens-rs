/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
import {
  Signer,
  utils,
  Contract,
  ContractFactory,
  PayableOverrides,
  BigNumberish,
} from "ethers";
import type { Provider, TransactionRequest } from "@ethersproject/providers";
import type {
  EphemeralGetPosition,
  EphemeralGetPositionInterface,
} from "../../contracts/EphemeralGetPosition";

const _abi = [
  {
    inputs: [
      {
        internalType: "contract IUniswapV3NonfungiblePositionManager",
        name: "npm",
        type: "address",
      },
      {
        internalType: "uint256",
        name: "tokenId",
        type: "uint256",
      },
    ],
    stateMutability: "payable",
    type: "constructor",
  },
  {
    inputs: [
      {
        internalType: "contract IUniswapV3NonfungiblePositionManager",
        name: "npm",
        type: "address",
      },
      {
        internalType: "uint256",
        name: "tokenId",
        type: "uint256",
      },
    ],
    name: "getPosition",
    outputs: [
      {
        components: [
          {
            internalType: "uint256",
            name: "tokenId",
            type: "uint256",
          },
          {
            internalType: "address",
            name: "owner",
            type: "address",
          },
          {
            components: [
              {
                internalType: "uint96",
                name: "nonce",
                type: "uint96",
              },
              {
                internalType: "address",
                name: "operator",
                type: "address",
              },
              {
                internalType: "address",
                name: "token0",
                type: "address",
              },
              {
                internalType: "address",
                name: "token1",
                type: "address",
              },
              {
                internalType: "uint24",
                name: "fee",
                type: "uint24",
              },
              {
                internalType: "int24",
                name: "tickLower",
                type: "int24",
              },
              {
                internalType: "int24",
                name: "tickUpper",
                type: "int24",
              },
              {
                internalType: "uint128",
                name: "liquidity",
                type: "uint128",
              },
              {
                internalType: "uint256",
                name: "feeGrowthInside0LastX128",
                type: "uint256",
              },
              {
                internalType: "uint256",
                name: "feeGrowthInside1LastX128",
                type: "uint256",
              },
              {
                internalType: "uint128",
                name: "tokensOwed0",
                type: "uint128",
              },
              {
                internalType: "uint128",
                name: "tokensOwed1",
                type: "uint128",
              },
            ],
            internalType: "struct PositionFull",
            name: "position",
            type: "tuple",
          },
          {
            components: [
              {
                internalType: "uint160",
                name: "sqrtPriceX96",
                type: "uint160",
              },
              {
                internalType: "int24",
                name: "tick",
                type: "int24",
              },
              {
                internalType: "uint16",
                name: "observationIndex",
                type: "uint16",
              },
              {
                internalType: "uint16",
                name: "observationCardinality",
                type: "uint16",
              },
              {
                internalType: "uint16",
                name: "observationCardinalityNext",
                type: "uint16",
              },
              {
                internalType: "uint32",
                name: "feeProtocol",
                type: "uint32",
              },
              {
                internalType: "bool",
                name: "unlocked",
                type: "bool",
              },
            ],
            internalType: "struct Slot0",
            name: "slot0",
            type: "tuple",
          },
          {
            internalType: "uint128",
            name: "activeLiquidity",
            type: "uint128",
          },
          {
            internalType: "uint8",
            name: "decimals0",
            type: "uint8",
          },
          {
            internalType: "uint8",
            name: "decimals1",
            type: "uint8",
          },
        ],
        internalType: "struct PositionState",
        name: "state",
        type: "tuple",
      },
    ],
    stateMutability: "payable",
    type: "function",
  },
] as const;

const _bytecode =
  "0x6080604052604051610913380380610913833981016040819052610022916106cd565b600061002e838361005c565b90506000816040516020016100439190610769565b6040516020818303038152906040529050805160208201fd5b61012b6040805160e0808201835260008083526020808401829052845161018081018652828152908101829052808501829052606081018290526080810182905260a0810182905260c08101829052918201819052610100820181905261012082018190526101408201819052610160820152909182019081526040805160e08101825260008082526020828101829052928201819052606082018190526080820181905260a0820181905260c082015291019081526000602082018190526040820181905260609091015290565b6101358383610167565b6001600160a01b0316602082015260408101516101559084908490610184565b506101618383836101aa565b92915050565b60008061017c846331a9108f60e11b8561036f565b949350505050565b63133f757160e31b600081815260048490529061018083602484885afa95945050505050565b818152604081015160006101bd8561039d565b604080840151606085015160808601519251630b4c774160e11b81526001600160a01b039283166004820152908216602482015262ffffff90921660448301529190911690631698ee8290606401602060405180830381865afa158015610228573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061024c91906108c8565b90506102606001600160a01b0382166103b8565b6001600160801b03166080840152606083015161027e9082906103cc565b60e08201516001600160801b031615610327576000806102b6838560a001518660c001518860600151602001516103f260201b60201c565b915091506000806102de8660e0015185858961010001518a61012001516104cd60201b60201c565b915091508186610140018181516102f591906108e5565b6001600160801b0316905250610160860180518291906103169083906108e5565b6001600160801b0316905250505050505b604082015161033e906001600160a01b0316610512565b60ff1660a0840152606082015161035d906001600160a01b0316610512565b60ff1660c09093019290925250505050565b600082600052816004526020600060246000875afa610392573d6000803e3d6000fd5b505060005192915050565b6000806103b18363c45a015560e01b610538565b9392505050565b6000806103b183630d34328160e11b610538565b633850c7bd60e01b600081815260e0908390600490865afa6103ed57600080fd5b505050565b600080806104096001600160a01b0388168761055c565b905060006104206001600160a01b0389168761055c565b90508660020b8560020b121561044f5780604001518260400151039350806060015182606001510392506104c2565b8560020b8560020b1261047b5781604001518160400151039350816060015181606001510392506104c2565b604080820151908301516104976001600160a01b038b166105c1565b03039350806060015182606001516104bd8a6001600160a01b03166105d460201b60201c565b030392505b505094509492505050565b6000806104ea6001600160801b038816858803600160801b6105e7565b91506105066001600160801b038816848703600160801b6105e7565b90509550959350505050565b6012600090815263313ce56760e01b602081815280600481865afa60051b519392505050565b6000816000526020600060046000865afa61055257600080fd5b5050600051919050565b6040805161010081018252600080825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e0810191909152600282900b816105b98563f30dba9360e01b84846101006105f4565b505092915050565b60006101618263f305839960e01b610538565b600061016182634614131960e01b610538565b600061017c848484610615565b8360005282600452808260246000885afa61060e57600080fd5b5050505050565b8282026000198385098181108201900380610646578261063d5763ae47f7026000526004601cfd5b508190046103b1565b80831161065b5763ae47f7026000526004601cfd5b82848609600084810385169485900494848311909303908390038390046001010292030417600260038302811880840282030280840282030280840282030280840282030280840282030280840290910302029392505050565b6001600160a01b03811681146106ca57600080fd5b50565b600080604083850312156106e057600080fd5b82516106eb816106b5565b6020939093015192949293505050565b60018060a01b038151168252602081015160020b602083015261ffff604082015116604083015261ffff606082015116606083015261ffff608082015116608083015260a081015161075560a084018263ffffffff169052565b5060c08101516103ed60c084018215159052565b815181526020808301516001600160a01b03169082015260408083015180516103008401926107a391908501906001600160601b03169052565b60208101516001600160a01b03811660608501525060408101516001600160a01b03811660808501525060608101516001600160a01b03811660a085015250608081015162ffffff811660c08501525060a081015161080760e085018260020b9052565b5060c081015161081d61010085018260020b9052565b5060e08101516001600160801b038116610120850152506101008101516101408401526101208101516101608401526101408101516108686101808501826001600160801b03169052565b5061016001516001600160801b03166101a083015260608301516108906101c08401826106fb565b5060808301516001600160801b03166102a083015260a083015160ff9081166102c084015260c0909301519092166102e09091015290565b6000602082840312156108da57600080fd5b81516103b1816106b5565b6001600160801b03818116838216019081111561016157634e487b7160e01b600052601160045260246000fdfe";

type EphemeralGetPositionConstructorParams =
  | [signer?: Signer]
  | ConstructorParameters<typeof ContractFactory>;

const isSuperArgs = (
  xs: EphemeralGetPositionConstructorParams
): xs is ConstructorParameters<typeof ContractFactory> => xs.length > 1;

export class EphemeralGetPosition__factory extends ContractFactory {
  constructor(...args: EphemeralGetPositionConstructorParams) {
    if (isSuperArgs(args)) {
      super(...args);
    } else {
      super(_abi, _bytecode, args[0]);
    }
  }

  override deploy(
    npm: string,
    tokenId: BigNumberish,
    overrides?: PayableOverrides & { from?: string }
  ): Promise<EphemeralGetPosition> {
    return super.deploy(
      npm,
      tokenId,
      overrides || {}
    ) as Promise<EphemeralGetPosition>;
  }
  override getDeployTransaction(
    npm: string,
    tokenId: BigNumberish,
    overrides?: PayableOverrides & { from?: string }
  ): TransactionRequest {
    return super.getDeployTransaction(npm, tokenId, overrides || {});
  }
  override attach(address: string): EphemeralGetPosition {
    return super.attach(address) as EphemeralGetPosition;
  }
  override connect(signer: Signer): EphemeralGetPosition__factory {
    return super.connect(signer) as EphemeralGetPosition__factory;
  }

  static readonly bytecode = _bytecode;
  static readonly abi = _abi;
  static createInterface(): EphemeralGetPositionInterface {
    return new utils.Interface(_abi) as EphemeralGetPositionInterface;
  }
  static connect(
    address: string,
    signerOrProvider: Signer | Provider
  ): EphemeralGetPosition {
    return new Contract(
      address,
      _abi,
      signerOrProvider
    ) as EphemeralGetPosition;
  }
}
