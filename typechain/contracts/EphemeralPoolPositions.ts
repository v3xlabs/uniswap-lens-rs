/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
import type {
  BaseContract,
  BigNumber,
  BigNumberish,
  BytesLike,
  CallOverrides,
  ContractTransaction,
  PayableOverrides,
  PopulatedTransaction,
  Signer,
  utils,
} from "ethers";
import type { FunctionFragment, Result } from "@ethersproject/abi";
import type { Listener, Provider } from "@ethersproject/providers";
import type {
  TypedEventFilter,
  TypedEvent,
  TypedListener,
  OnEvent,
} from "../common";

export declare namespace PoolUtils {
  export type PositionKeyStruct = {
    owner: string;
    tickLower: BigNumberish;
    tickUpper: BigNumberish;
  };

  export type PositionKeyStructOutput = [string, number, number] & {
    owner: string;
    tickLower: number;
    tickUpper: number;
  };

  export type SlotStruct = { slot: BigNumberish; data: BigNumberish };

  export type SlotStructOutput = [BigNumber, BigNumber] & {
    slot: BigNumber;
    data: BigNumber;
  };
}

export interface EphemeralPoolPositionsInterface extends utils.Interface {
  functions: {
    "getPositions(address,(address,int24,int24)[])": FunctionFragment;
  };

  getFunction(nameOrSignatureOrTopic: "getPositions"): FunctionFragment;

  encodeFunctionData(
    functionFragment: "getPositions",
    values: [string, PoolUtils.PositionKeyStruct[]]
  ): string;

  decodeFunctionResult(
    functionFragment: "getPositions",
    data: BytesLike
  ): Result;

  events: {};
}

export interface EphemeralPoolPositions extends BaseContract {
  connect(signerOrProvider: Signer | Provider | string): this;
  attach(addressOrName: string): this;
  deployed(): Promise<this>;

  interface: EphemeralPoolPositionsInterface;

  queryFilter<TEvent extends TypedEvent>(
    event: TypedEventFilter<TEvent>,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined
  ): Promise<Array<TEvent>>;

  listeners<TEvent extends TypedEvent>(
    eventFilter?: TypedEventFilter<TEvent>
  ): Array<TypedListener<TEvent>>;
  listeners(eventName?: string): Array<Listener>;
  removeAllListeners<TEvent extends TypedEvent>(
    eventFilter: TypedEventFilter<TEvent>
  ): this;
  removeAllListeners(eventName?: string): this;
  off: OnEvent<this>;
  on: OnEvent<this>;
  once: OnEvent<this>;
  removeListener: OnEvent<this>;

  functions: {
    getPositions(
      pool: string,
      keys: PoolUtils.PositionKeyStruct[],
      overrides?: PayableOverrides & { from?: string }
    ): Promise<ContractTransaction>;
  };

  getPositions(
    pool: string,
    keys: PoolUtils.PositionKeyStruct[],
    overrides?: PayableOverrides & { from?: string }
  ): Promise<ContractTransaction>;

  callStatic: {
    getPositions(
      pool: string,
      keys: PoolUtils.PositionKeyStruct[],
      overrides?: CallOverrides
    ): Promise<PoolUtils.SlotStructOutput[]>;
  };

  filters: {};

  estimateGas: {
    getPositions(
      pool: string,
      keys: PoolUtils.PositionKeyStruct[],
      overrides?: PayableOverrides & { from?: string }
    ): Promise<BigNumber>;
  };

  populateTransaction: {
    getPositions(
      pool: string,
      keys: PoolUtils.PositionKeyStruct[],
      overrides?: PayableOverrides & { from?: string }
    ): Promise<PopulatedTransaction>;
  };
}
