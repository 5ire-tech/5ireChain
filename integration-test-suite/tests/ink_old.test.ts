import { assert, expect } from "chai";
import { BLOCK_TIME } from "../utils/constants";
import { killNodes, polkadotApi, spawnNodes } from "../utils/util";
import { CodePromise, Abi, ContractPromise } from "@polkadot/api-contract";
import { ApiPromise, Keyring } from "@polkadot/api";
import abiFile from "./contracts/psp22_token.json";
import {WeightV2} from "@polkadot/types/interfaces";
import { sleep, waitForEvent } from "../utils/setup";
import {BN, BN_ONE} from "@polkadot/util";
describe("Wasm test with erc20 token old ink! version 3", function () {
  this.timeout(300 * BLOCK_TIME);
  // 4 session.
  this.slow(40 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  it("Should deploy a erc20 token contract to 5ire chain", async () => {
    console.log("Beginning deploying wasm contract");

    let abi: string = JSON.stringify(abiFile);

    let wasm = abiFile.source.wasm;
    // deploy contract
    await deployContract(polkadotApi, abi, wasm);

    // wait for instantiated event
    await waitForEvent(polkadotApi, "contracts", "Instantiated");
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
    tokenName,
    tokenSupply,
    tokenSymbol,
    tokenDecimal
  );

  let address: string;

  address = await new Promise(async (resolve, reject) => {
    await tx.signAndSend(
      alice,
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

  expect(address).not.null;

  // Query balanceOf  with Bob account
  const bob = keyring.addFromUri("//Bob");
  const contract = new ContractPromise(api, contractAbi, address);

  // Sign transaction

  const transfer = contract.tx["psp22::transfer"]({
    gasLimit: gasLimit,
    storageDepositLimit: storageDepositLimit,
  },bob.address, '100',[]);
  
  await transfer.signAndSend(
    alice,
    // @ts-ignore
    result => {
      if (result.status.isInBlock || result.status.isFinalized) {
        console.log("Block finalized");
        
      }

    }
  );

  const { result, output } = await contract.query["psp22::balanceOf"](
    alice.address,
    {
      gasLimit: gasLimit,
      storageDepositLimit: null,
    },
    bob.address
  );
  // check if the call was successful
  if (result.isOk) {
    // output the return value
    console.log("Success in old smart contract -> Value:", output?.toHuman());
  } else {
    console.error("Error", result.asErr);
  }


};
