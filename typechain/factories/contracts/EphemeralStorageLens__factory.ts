/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
import { Signer, utils, Contract, ContractFactory, Overrides } from "ethers";
import type { Provider, TransactionRequest } from "@ethersproject/providers";
import type {
  EphemeralStorageLens,
  EphemeralStorageLensInterface,
} from "../../contracts/EphemeralStorageLens";

const _abi = [
  {
    inputs: [
      {
        internalType: "bytes32[]",
        name: "slots",
        type: "bytes32[]",
      },
    ],
    name: "extsload",
    outputs: [
      {
        internalType: "bytes32[]",
        name: "",
        type: "bytes32[]",
      },
    ],
    stateMutability: "payable",
    type: "function",
  },
] as const;

const _bytecode =
  "0x6080604052348015600f57600080fd5b506101368061001f6000396000f3fe60806040526004361061001e5760003560e01c8063dbd035ff14610023575b600080fd5b61003661003136600461007c565b61004c565b60405161004391906100f3565b60405180910390f35b60606020600052816020528160051b6040016040845b803554825260209182019101828210610062575050806000f35b6000806020838503121561008f57600080fd5b823567ffffffffffffffff8111156100a657600080fd5b8301601f810185136100b757600080fd5b803567ffffffffffffffff8111156100ce57600080fd5b8560208260051b84010111156100e357600080fd5b6020919091019590945092505050565b602080825282518282018190526000918401906040840190835b8181101561012b57835183526020938401939092019160010161010d565b50909594505050505056";

type EphemeralStorageLensConstructorParams =
  | [signer?: Signer]
  | ConstructorParameters<typeof ContractFactory>;

const isSuperArgs = (
  xs: EphemeralStorageLensConstructorParams
): xs is ConstructorParameters<typeof ContractFactory> => xs.length > 1;

export class EphemeralStorageLens__factory extends ContractFactory {
  constructor(...args: EphemeralStorageLensConstructorParams) {
    if (isSuperArgs(args)) {
      super(...args);
    } else {
      super(_abi, _bytecode, args[0]);
    }
  }

  override deploy(
    overrides?: Overrides & { from?: string }
  ): Promise<EphemeralStorageLens> {
    return super.deploy(overrides || {}) as Promise<EphemeralStorageLens>;
  }
  override getDeployTransaction(
    overrides?: Overrides & { from?: string }
  ): TransactionRequest {
    return super.getDeployTransaction(overrides || {});
  }
  override attach(address: string): EphemeralStorageLens {
    return super.attach(address) as EphemeralStorageLens;
  }
  override connect(signer: Signer): EphemeralStorageLens__factory {
    return super.connect(signer) as EphemeralStorageLens__factory;
  }

  static readonly bytecode = _bytecode;
  static readonly abi = _abi;
  static createInterface(): EphemeralStorageLensInterface {
    return new utils.Interface(_abi) as EphemeralStorageLensInterface;
  }
  static connect(
    address: string,
    signerOrProvider: Signer | Provider
  ): EphemeralStorageLens {
    return new Contract(
      address,
      _abi,
      signerOrProvider
    ) as EphemeralStorageLens;
  }
}
