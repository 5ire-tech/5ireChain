import { expect } from "chai";
import { BLOCK_TIME } from "../utils/constants";
import { killNodes, polkadotApi, spawnNodes } from "../utils/util";
import { CodePromise, Abi } from "@polkadot/api-contract";
import { ApiPromise, Keyring } from "@polkadot/api";
import abiFile from "./contracts/counter.json";

import { sleep, waitForEvent } from "../utils/setup";

describe("Wasm test with new ink! version 4", function () {
  this.timeout(300 * BLOCK_TIME);
  // 4 session.
  this.slow(40 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  it("Should deploy a counter wasm contract to 5ire chain", async () => {
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

  const contract = new CodePromise(api, contractAbi, contractWasm);

  const gasLimit = 100000n * 1000000n;
  const storageDepositLimit = null;

  let initValue = 0;

  const keyring = new Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice");
  const tx = contract.tx.new(
    { gasLimit: gasLimit, storageDepositLimit: storageDepositLimit },
    initValue
  );

  let address: string;

  const unsub = await tx.signAndSend(
    alice,
    // @ts-ignore
    ({ contract, status, dispatchError }) => {
      if (status.isInBlock || status.isFinalized) {
        address = contract.address.toString();
        console.log("Contract address:",address);
        unsub();
      }

      if (dispatchError) {
        console.log(`error occurred ${dispatchError}`);
      }

      expect(address).not.null;
    }
  );
};
