import { expect } from "chai";
import { BLOCK_TIME } from "../utils/constants";
import { killNodes, polkadotApi, spawnNodes } from "../utils/util";
import { CodePromise, ContractPromise } from "@polkadot/api-contract";
import { ApiPromise, Keyring } from "@polkadot/api";
import contractFile from "./contracts/counter.json";
import type { WeightV2 } from "@polkadot/types/interfaces";
import { BN } from "@polkadot/util";
import { waitForEvent } from "../utils/setup";

describe.skip("Wasm test with new ink! version 4", function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  it("Should deploy/interact/query a counter wasm contract to 5ire chain", async () => {
    console.log("Beginning deploying counter wasm contract");

    const keyring = new Keyring({ type: "sr25519" });
    const alice = keyring.addFromUri("//Alice");

    const gasLimit = polkadotApi.registry.createType("WeightV2", {
      refTime: 5908108255,
      proofSize: new BN(131072),
    }) as WeightV2;

    const storageDepositLimit = null;

    let contractFileString: string = JSON.stringify(contractFile);

    let wasm = contractFile.source.wasm;
    // deploy contract
    let contractAddress = await deployContract(
      alice,
      polkadotApi,
      contractFileString,
      wasm,
      gasLimit,
      storageDepositLimit
    );
    console.log("Address:", contractAddress);
    await waitForEvent(polkadotApi, "contracts", "Instantiated");

    // query transaction
    let countBefore = await queryTransaction(
      alice,
      polkadotApi,
      contractFileString,
      contractAddress,
      gasLimit,
      storageDepositLimit
    );
    // Before trigger inc function
    // count should be 0

    const actualBefore = { Ok: "0".toString() };

    // @ts-ignore
    expect(countBefore).to.deep.equal(actualBefore);

    // interact contract with inc transaction
    await incTransaction(
      alice,
      polkadotApi,
      contractFileString,
      contractAddress,
      gasLimit,
      storageDepositLimit
    );

    // wait for contract called event
    await waitForEvent(polkadotApi, "contracts", "Called");

    // query transaction
    let countAfter = await queryTransaction(
      alice,
      polkadotApi,
      contractFileString,
      contractAddress,
      gasLimit,
      storageDepositLimit
    );
    // After trigger inc function
    // count should be 1
    const actualAfter = { Ok: "1".toString() };
    // @ts-ignore
    expect(countAfter).to.deep.equal(actualAfter);
  });

  after(async () => {
    await killNodes();
  });
});

const deployContract = async (
  alice: any,
  api: ApiPromise,
  contractFile: string,
  contractWasm: string,
  gasLimit: WeightV2,
  storageDepositLimit: any
) => {
  // convert contract json file into usable contract ABI
  //let contractAbi = new Abi(contractFile, api?.registry?.getChainProperties());

  const code = new CodePromise(api, contractFile, contractWasm);

  let initValue = 0;

  const tx = code.tx.new(
    { gasLimit: gasLimit, storageDepositLimit: storageDepositLimit },
    initValue
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

        expect(address).not.null;
      }
    );
  });

  return address;
};

const incTransaction = async (
  alice: any,
  api: ApiPromise,
  contractFile: string,
  contractAddress: string,
  gasLimit: WeightV2,
  storageDepositLimit: any
) => {
  console.log("Begin triggering inc transaction smart contract");
  // convert contract json file into usable contract ABI
  //Define deployed contract with metadata + contract address
  const contract = new ContractPromise(api, contractFile, contractAddress);

  //Sign transaction
  const tx = contract.tx.inc({
    gasLimit: gasLimit,
    storageDepositLimit: storageDepositLimit,
  });

  await tx.signAndSend(
    alice,
    // @ts-ignore
    (result) => {
      if (result.status.isInBlock || result.status.isFinalized) {
        console.log("Block finalized");
      }
    }
  );
};

const queryTransaction = async (
  alice: any,
  api: ApiPromise,
  contractFile: string,
  contractAddress: string,
  gasLimit: WeightV2,
  storageDepositLimit: any
) => {
  console.log("Begin querying smart contract");

  //Define deployed contract with metadata + contract address
  const contract = new ContractPromise(api, contractFile, contractAddress);

  // Query value from contract
  const { result, output } = await contract.query.get(alice.address, {
    gasLimit,
    storageDepositLimit,
  });
  // check if the call was successful
  expect(result.isOk).to.equal(true);

  let value = output?.toHuman();

  return value;
};
