import { expect } from "chai";
import { BLOCK_TIME, alith, baltathar } from "../utils/constants";
import { killNodes, polkadotApi, spawnNodes } from "../utils/util";
import { CodePromise, Abi, ContractPromise } from "@polkadot/api-contract";
import { Keyring } from "@polkadot/api";
import abiFile from "./contracts/psp22_token.json";
import { WeightV2 } from "@polkadot/types/interfaces";
import { sleep, waitForEvent, waitNfinalizedBlocks } from "../utils/setup";
import { BN, BN_ONE } from "@polkadot/util";
import { BigNumber } from "ethers";

let contractAddress: string;
let contractAbi: Abi;
let wasm: string;

describe("Wasm test with psp22 token old ink! version 3", function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();

    let abi: string = JSON.stringify(abiFile);

    wasm = abiFile.source.wasm;
    contractAbi = new Abi(abi, polkadotApi?.registry?.getChainProperties());
  });

  it("Should deploy a psp22 token contract to 5ire chain", async () => {
    // deploy contract
    await deployContract();

    console.log(`Beginning executing wasm contract ${contractAddress}`);
    await executeContract();
  });

  after(async () => {
    await killNodes();
  });
});

const deployContract = async () => {
  console.log("Beginning deploying wasm contract");
  // convert contract json file into usable contract ABI
  const code = new CodePromise(polkadotApi, contractAbi, wasm);

  const tokenSupply = 1000;
  const tokenName = 0;
  const tokenSymbol = 0;
  const tokenDecimal = 1;

  const MAX_CALL_WEIGHT = new BN(10_000_000_000);
  const gasLimit = polkadotApi.registry.createType("WeightV2", {
    refTime: 5908108255,
    proofSize: 131072,
  }) as WeightV2;

  //const storageDepositLimit = new BN( 2_003_435_700_000_000_000);
  const storageDepositLimit = null;

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
      alith,
      { tip: 100, nonce: -1 },
      // @ts-ignore
      ({ contract, status, dispatchError }) => {
        if (status.isInBlock || status.isFinalized) {
          address = contract.address.toString();
          //  console.log(`Block finalized  ${status.asFinalized}`);
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

  contractAddress = address;

  // wait for instantiated event
  await waitForEvent(polkadotApi, "contracts", "Instantiated");
};

const executeContract = async () => {
  const refTime = polkadotApi.registry.createType(
    "Compact<u64>",
    BigInt(10000000000)
  );
  const proofSize = polkadotApi.registry.createType(
    "Compact<u64>",
    BigInt(10000000000)
  );

  const gasLimitForCallAndQuery = polkadotApi.registry.createType(
    "SpWeightsWeightV2Weight",
    {
      refTime: refTime,
      proofSize: proofSize,
    }
  );
  const storageDepositLimitForCallAndQuery = null;

  const contract = new ContractPromise(
    polkadotApi,
    contractAbi,
    contractAddress
  );
  const res1 = await contract.query["psp22::balanceOf"](
    alith.publicKey,
    {
      // @ts-ignore
      gasLimit: gasLimitForCallAndQuery,
      storageDepositLimit: storageDepositLimitForCallAndQuery,
    },
    baltathar.publicKey
  );
  expect(res1.output?.eq(0)).true;

  const TOKEN_AMT = 400;
  // Sign transaction
  const transfer = contract.tx["psp22::transfer"](
    {
      // @ts-ignore
      gasLimit: gasLimitForCallAndQuery,
      storageDepositLimit: storageDepositLimitForCallAndQuery,
    },
    baltathar.publicKey,
    400,
    []
  );

  console.log(`trying to execute transaction`);
  const unsub = await transfer.signAndSend(
    alith,
    { tip: 200, nonce: -1 },
    (result) => {
      console.log(`execute contract transfer transaction is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(
          `execute contract transfer transaction included at blockHash ${result.status.asInBlock}`
        );
        console.log(
          `execute contract transfer transaction waiting for finalization... (can take a minute)`
        );
      } else if (result.status.isFinalized) {
        console.log(
          `execute contract transfer transaction events are ${result.events.length}`
        );
        console.log(
          `execute contract transfer transaction finalized at blockHash ${result.status.asFinalized}`
        );
        result.events.forEach(({ event: { data, method, section }, phase }) => {
          console.log("\t", phase.toString(), `: ${section}.${method}`);
        });
        unsub();
      }
    }
  );
  // await waitForEvent(polkadotApi, "contracts", "ExtrinsicSuccess");
  await waitNfinalizedBlocks(polkadotApi, 4, 60000);

  const res2 = await contract.query["psp22::balanceOf"](
    alith.publicKey,
    {
      // @ts-ignore
      gasLimit: gasLimitForCallAndQuery,
      storageDepositLimit: storageDepositLimitForCallAndQuery,
    },
    baltathar.publicKey
  );

  // Expect Bobs balance to have increased
  // @ts-ignore
  expect(res2.output?.toHuman() > res1.output?.toHuman()).true;
  expect(res2.output?.eq(TOKEN_AMT)).true;
};
