import { expect } from "chai";
import { step } from "mocha-steps";
import Web3 from "web3";
import { BLOCK_TIME, SECONDS } from "../utils/constants";
import {
  customRequest,
  killNodeForTestEVM,
  polkadotApi,
  spawnNodeForTestEVM,
} from "../utils/util";
import { sleep, waitForEvent } from "../utils/setup";
import Keyring from "@polkadot/keyring";
import { Wallet, ethers } from "ethers";

let web3: Web3;
let aliceEthAccount: Wallet;

const ERC20_ABI = require("./contracts/MyToken.json").abi;
const ERC20_BYTECODES = require("./contracts/MyToken.json").bytecode;

// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: "sr25519" });

async function init() {
  const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
  // create eth accout from ethers
  const aliceEthAccount = ethers.Wallet.fromMnemonic(
    "bottom drive obey lake curtain smoke basket hold race lonely fit walk"
  );

  //swap native to evm balance 10 coin
  const amount = polkadotApi.createType("Balance", "10000000000000000000");
  const transaction = polkadotApi.tx.evm.deposit(
    aliceEthAccount.address,
    amount
  );

  const unsub = await transaction.signAndSend(alice, (result) => {
    console.log(`Swap is ${result.status}`);
    if (result.status.isInBlock) {
      console.log(`Swap included at blockHash ${result.status.asInBlock}`);
      console.log(`Waiting for finalization... (can take a minute)`);
    } else if (result.status.isFinalized) {
      console.log(`events are ${result.events}`);
      console.log(`Swap finalized at blockHash ${result.status.asFinalized}`);
      unsub();
    }
  });

  await waitForEvent(polkadotApi, "balances", "Transfer");

  return aliceEthAccount;
}

async function sendTransaction(web3: Web3) {
  const erc20Contract = new web3.eth.Contract(ERC20_ABI);

  const deployTx = erc20Contract.deploy({
    data: ERC20_BYTECODES,
    arguments: [],
  });

  const gas = await deployTx.estimateGas({ from: aliceEthAccount.address });

  const gasPrice = await web3.eth.getGasPrice();

  const tx = await web3.eth.accounts.signTransaction(
    {
      from: aliceEthAccount.address,
      data: deployTx.encodeABI(),
      gasPrice,
      gas,
    },
    aliceEthAccount.privateKey
  );
  await customRequest(web3, "eth_sendRawTransaction", [tx.rawTransaction]);
  return tx;
}

describe("EVM related Subscription", function () {
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
      })
    );
    aliceEthAccount = await init();
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
      function (error, result) {}
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
      function (error, result) {}
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
      function (error, result) {}
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

  step("should subscribe to all logs", async function (done) {
    subscription = web3.eth.subscribe("logs", {}, function (error, result) {});

    await new Promise<void>((resolve) => {
      subscription.on("connected", function (d: any) {
        resolve();
      });
    });

    const tx = await sendTransaction(web3);
    let data = null;
    let dataResolve: any = null;
    let dataPromise = new Promise((resolve) => {
      dataResolve = resolve;
    });
    subscription.on("data", function (d: any) {
      data = d;
      logsGenerated += 1;
      dataResolve();
    });

    await dataPromise;

    subscription.unsubscribe();
    const block = await web3.eth.getBlock("latest");
    expect(data).to.include({
      blockHash: block.hash,
      blockNumber: block.number,
      data: "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
      logIndex: 0,
      removed: false,
      transactionHash: block.transactions[0],
      transactionIndex: 0,
      transactionLogIndex: "0x0",
    });
    done();
  }).timeout(20000);
});
