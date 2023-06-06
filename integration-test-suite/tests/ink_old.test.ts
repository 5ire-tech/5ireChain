import { assert, expect } from "chai";
import { BLOCK_TIME } from "../utils/constants";
import { killNodes, polkadotApi, spawnNodes } from "../utils/util";
import { CodePromise, Abi, ContractPromise } from "@polkadot/api-contract";
import {ApiPromise, Keyring, WsProvider} from "@polkadot/api";
import abiFile from "./contracts/psp22_token.json";
import {WeightV2} from "@polkadot/types/interfaces";
import { sleep, waitForEvent } from "../utils/setup";
import {BN, BN_ONE} from "@polkadot/util";


describe("Wasm test with psp22 token old ink! version 3", function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  it("Should deploy a psp22 token contract to 5ire chain", async () => {
    console.log("Beginning deploying wasm contract");

    let abi: string = JSON.stringify(abiFile);

    let wasm = abiFile.source.wasm;
    // deploy contract
    await deployContract(polkadotApi, abi, wasm);
  });

  after(async () => {
    await killNodes();
  });
});

const deployContract = async (
  api: ApiPromise,
  contractFile: string,
  contractWasm: string
) => {
  // convert contract json file into usable contract ABI
  let contractAbi = new Abi(contractFile, api?.registry?.getChainProperties());

  const code = new CodePromise(api, contractAbi, contractWasm);

  //const gasLimit = 100000n * 1000000n;
  const MAX_CALL_WEIGHT = new BN(5_000_000_000).isub(BN_ONE);
  const gasLimit = polkadotApi.registry.createType('WeightV2', {
    refTime: MAX_CALL_WEIGHT,
    proofSize: new BN(1_000_000),
  }) as WeightV2;

  const storageDepositLimit = null;

  const tokenSupply = 1000;
  const tokenName = 0;
  const tokenSymbol = 0;
  const tokenDecimal = 1;

  const keyring = new Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice");
  const tx = code.tx.new(
    { gasLimit: gasLimit, storageDepositLimit: storageDepositLimit },
    tokenSupply,
    tokenName,
    tokenSymbol,
    tokenDecimal
  );

  let address: string;

  address = await new Promise(async (resolve, reject) => {
    await tx.signAndSend(
      alice,
      {tip: 100, nonce: -1},
      // @ts-ignore
      ({ contract, status, dispatchError }) => {
        if (status.isInBlock || status.isFinalized) {
          address = contract.address.toString();
          resolve(address);
        }

        if (dispatchError) {
          console.log(`error occurred ${dispatchError}`);
          reject(dispatchError);
        }
      }
    );
  });

  console.log(`address is ${address}`);
  expect(address).not.null;

  // wait for instantiated event
  await waitForEvent(polkadotApi, "contracts", "Instantiated");

  const bob = keyring.addFromUri("//Bob");
  const contract = new ContractPromise(api, contractAbi, address);

  const gasLimitForCallAndQuery = polkadotApi.registry.createType('WeightV2', {
    refTime: 5908108255,
    proofSize: new BN(131072),
  }) as WeightV2;
  const storageDepositLimitForCallAndQuery = 17000000000;

  const { output:initialBobBalance } = await contract.query["psp22::balanceOf"](
    alice.address,
    {
      gasLimit: gasLimitForCallAndQuery,
      storageDepositLimit: storageDepositLimitForCallAndQuery,
    },
    bob.address,
  );

  // Sign transaction
  const transfer = contract.tx["psp22::transfer"]({
    gasLimit: gasLimitForCallAndQuery,
    storageDepositLimit: storageDepositLimitForCallAndQuery,
  },bob.address, '400',[]);

  await transfer.signAndSend(
    alice,
    // @ts-ignore
    result => {
      if (result.status.isInBlock || result.status.isFinalized) {
        console.log("Block finalized");
      }

    }
  );

  // wait for contract called event
  await waitForEvent(polkadotApi, 'contracts', 'Called')

  const { output:finalBobBalance } = await contract.query["psp22::balanceOf"](
    alice.address,
    {
      gasLimit: gasLimitForCallAndQuery,
      storageDepositLimit: storageDepositLimitForCallAndQuery,
    },
    bob.address,
  );

  // Expect Bobs balance to have increased
  // @ts-ignore
  expect(finalBobBalance?.toHuman() > initialBobBalance?.toHuman()).true
};
