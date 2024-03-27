import Web3 from "web3";
import {
  BLOCK_TIME,
  EXISTENTIAL_DEPOSIT,
  GENESIS_ACCOUNT,
  GENESIS_ACCOUNT_BALANCE,
  SECONDS,
} from "../utils/constants";
import {
  customRequest,
  killNodeForTestEVM,
  polkadotApi,
  spawnNodeForTestEVM,
} from "../utils/util";
import { sleep, waitForEvent } from "../utils/setup";

import { expect } from "chai";
import { step } from "mocha-steps";
import Keyring from "@polkadot/keyring";
import { Wallet, ethers } from "ethers";
let web3: Web3;
let aliceEthAccount: Wallet;

const TEST_ACCOUNT = "0xdd33Af49c851553841E94066B54Fd28612522901";
const TEST_ACCOUNT_PRIVATE_KEY =
  "0x4ca933bffe83185dda76e7913fc96e5c97cdb7ca1fbfcc085d6376e6f564ef71";
const TRANFER_VALUE = "1"; // 1 5IRE must be higher than ExistentialDeposit
//const GAS_PRICE = "0x3B9ACA00"; // 1000000000

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

describe("EVM related Balance", function () {
  this.timeout(100 * BLOCK_TIME);
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
  step("genesis balance is setup correctly", async function () {
    expect(await web3.eth.getBalance(GENESIS_ACCOUNT)).to.equal(
      GENESIS_ACCOUNT_BALANCE
    );
  });

  step("balance to be updated after transfer", async function () {
    this.timeout(40000);
    const nonce = await web3.eth.getTransactionCount(
      aliceEthAccount.address,
      "latest"
    );
    const gasPrice = await web3.eth.getGasPrice();
    const tx = await web3.eth.accounts.signTransaction(
      {
        from: aliceEthAccount.address,
        to: TEST_ACCOUNT,
        value: web3.utils.toHex(web3.utils.toWei(TRANFER_VALUE, "ether")),
        gasPrice: web3.utils.toHex(gasPrice),
        gas: "0x100000",
        nonce,
      },
      aliceEthAccount.privateKey
    );
    const rep = await customRequest(web3, "eth_sendRawTransaction", [
      tx.rawTransaction,
    ]);
    await sleep(2 * SECONDS);
    // ALICE_ACCOUNT_BALANCE - (21000 * gasPrice) - value;
    const INITIAL_ALICE_BALANCE = web3.utils.toWei("10", "ether");

    const expectedAliceBalance = (
      BigInt(INITIAL_ALICE_BALANCE) -
      BigInt(21000) * BigInt(gasPrice) -
      BigInt(web3.utils.toWei(TRANFER_VALUE, "ether"))
    ).toString();
    const expectedTestBalance = (
      Number(web3.utils.toWei(TRANFER_VALUE, "ether")) - EXISTENTIAL_DEPOSIT
    ).toString();

    expect(await web3.eth.getBalance(aliceEthAccount.address)).to.equal(
      expectedAliceBalance
    );
    expect(await web3.eth.getBalance(TEST_ACCOUNT)).to.equal(
      expectedTestBalance
    );
  });

  step("gas price too low", async function () {
    const nonce = await web3.eth.getTransactionCount(
      aliceEthAccount.address,
      "latest"
    );

    let gasPrice = await web3.eth.getGasPrice();
    const tx = await web3.eth.accounts.signTransaction(
      {
        from: aliceEthAccount.address,
        to: TEST_ACCOUNT,
        value: web3.utils.toHex(web3.utils.toWei(TRANFER_VALUE, "ether")),
        gasPrice: web3.utils.toHex(Number(gasPrice) - 1),
        gas: "0x100000",
        nonce: nonce,
      },
      aliceEthAccount.privateKey
    );
    let result = await customRequest(web3, "eth_sendRawTransaction", [
      tx.rawTransaction,
    ]);
    expect(result?.error?.message).to.be.equal(
      "gas price less than block base fee"
    );
  });

  step("balance insufficient", async function () {
    const nonce = await web3.eth.getTransactionCount(
      aliceEthAccount.address,
      "latest"
    );
    let gasPrice = await web3.eth.getGasPrice();
    let testAccountBalance = await web3.eth.getBalance(TEST_ACCOUNT);
    const tx = await web3.eth.accounts.signTransaction(
      {
        from: TEST_ACCOUNT,
        to: GENESIS_ACCOUNT,
        value: Number(testAccountBalance) + 1,
        gasPrice: gasPrice,
        gas: "0x100000",
        nonce: nonce,
      },
      TEST_ACCOUNT_PRIVATE_KEY
    );
    let result = await customRequest(web3, "eth_sendRawTransaction", [
      tx.rawTransaction,
    ]);
    expect(result?.error?.message).to.be.equal(
      "insufficient funds for gas * price + value"
    );
  });
});
