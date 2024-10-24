import { expect } from "chai";
import { step } from "mocha-steps";
import Web3 from "web3";
import {
  ALITH_PRIVATE_KEY,
  BLOCK_TIME,
  SECONDS,
  alith,
} from "../utils/constants";
import {
  customRequest,
  killNodeForTestEVM,
  spawnNodeForTestEVM,
} from "../utils/util";
import { sleep } from "../utils/setup";
import { readFileSync } from "fs";
import { join } from "path";

let web3: Web3;

const ERC20_ABI = require("./contracts/MyToken.json");

const ERC20_BYTECODES = readFileSync(join(__dirname, './contracts/erc20_contract_bytecode.txt'), 'utf8').trim();


async function sendTransaction(web3: Web3) {
  const erc20Contract = new web3.eth.Contract(ERC20_ABI);

  const deployTx = erc20Contract.deploy({
    data: ERC20_BYTECODES,
    arguments: [],
  });

  const gas = await deployTx.estimateGas({ from: alith.address });

  const gasPrice = await web3.eth.getGasPrice();

  const tx = await web3.eth.accounts.signTransaction(
    {
      from: alith.address,
      data: deployTx.encodeABI(),
      gasPrice,
      gas,
    },
    ALITH_PRIVATE_KEY,
  );
  await customRequest(web3, "eth_sendRawTransaction", [tx.rawTransaction]);
  return tx;
}

describe("EVM related Subscription using web3js/ethersjs", function () {
  this.timeout(100 * BLOCK_TIME);
  let subscription: any;
  let logsGenerated = 0;

  before(async () => {
    await spawnNodeForTestEVM();
    // Create instance web3
    web3 = new Web3(
      new Web3.providers.WebsocketProvider("ws://127.0.0.1:9944", {
        reconnect: {
          auto: true,
          delay: 5000, // ms
          maxAttempts: 5,
          onTimeout: false,
        },
      }),
    );
    await sleep(40 * SECONDS);
  });
  after(async () => {
    await killNodeForTestEVM();
  });

  step("should connect", async function () {
    // @ts-ignore
    const connected = web3.currentProvider.connected;
    expect(connected).to.equal(true);
  }).timeout(20000);

  step("should subscribe", async function () {
    subscription = web3.eth.subscribe(
      "newBlockHeaders",
      function (error, result) {},
    );

    let connected = false;
    let subscriptionId = "";
    expect(subscriptionId).is.empty;
    await new Promise<void>((resolve) => {
      subscription.on("connected", function (d: any) {
        connected = true;
        subscriptionId = d;
        resolve();
      });
    });

    subscription.unsubscribe();
    expect(connected).to.equal(true);
    expect(subscriptionId).not.empty;
  }).timeout(20000);
  step("should get newHeads stream", async function (done) {
    subscription = web3.eth.subscribe(
      "newBlockHeaders",
      function (error, result) {},
    );
    let data = null;
    let dataResolve: any = null;
    let dataPromise = new Promise((resolve) => {
      dataResolve = resolve;
    });
    subscription.on("data", function (d: any) {
      data = d;
      subscription.unsubscribe();
      dataResolve();
    });
    await dataPromise;
    done();
  }).timeout(40000);

  step("should get newPendingTransactions stream", async function (done) {
    subscription = web3.eth.subscribe(
      "pendingTransactions",
      function (error, result) {},
    );

    await new Promise<void>((resolve) => {
      subscription.on("connected", function (d: any) {
        resolve();
      });
    });

    const tx = await sendTransaction(web3);
    let data = null;
    await new Promise<void>((resolve) => {
      subscription.on("data", function (d: any) {
        data = d;
        logsGenerated += 1;
        resolve();
      });
    });

    subscription.unsubscribe();
    expect(data).to.be.not.null;
    expect(tx["transactionHash"]).to.be.eq(data);

    done();
  }).timeout(30000);

});
