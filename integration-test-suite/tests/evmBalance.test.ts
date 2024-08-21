import Web3 from "web3";
import {
  BLOCK_TIME,
  EXISTENTIAL_DEPOSIT,
  GENESIS_ACCOUNT_BALANCE,
  SECONDS,
  GENESIS_ACCOUNTS,
  alith,
  ALITH_PRIVATE_KEY,
} from "../utils/constants";
import {
  customRequest,
  killNodeForTestEVM,
  spawnNodeForTestEVM,
} from "../utils/util";
import { sleep } from "../utils/setup";

import { expect } from "chai";
import { step } from "mocha-steps";
let web3: Web3;

const TEST_ACCOUNT = "0xdd33Af49c851553841E94066B54Fd28612522901";
const TEST_ACCOUNT_PRIVATE_KEY =
  "0x4ca933bffe83185dda76e7913fc96e5c97cdb7ca1fbfcc085d6376e6f564ef71";
const TRANFER_VALUE = "1"; // 1 5IRE must be higher than ExistentialDeposit
//const GAS_PRICE = "0x3B9ACA00"; // 1000000000

describe("EVM related Balance using web3js/ethersjs", function () {
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
      }),
    );
    await sleep(20 * SECONDS);
  });
  after(async () => {
    await killNodeForTestEVM();
  });
  step("genesis balance is setup correctly", async function () {
    for (let address of GENESIS_ACCOUNTS) {
      expect(await web3.eth.getBalance(address)).to.equal(
        GENESIS_ACCOUNT_BALANCE,
      );
      console.log(address + " has expected balance");
    }
  });

  step("balance to be updated after transfer", async function () {
    this.timeout(40000);
    const nonce = await web3.eth.getTransactionCount(alith.address, "latest");
    const INITIAL_ALICE_BALANCE = await web3.eth.getBalance(alith.address);
    console.log(INITIAL_ALICE_BALANCE);
    const gasPrice = await web3.eth.getGasPrice();
    const tx = await web3.eth.accounts.signTransaction(
      {
        from: alith.address,
        to: TEST_ACCOUNT,
        value: web3.utils.toHex(web3.utils.toWei(TRANFER_VALUE, "ether")),
        gasPrice: web3.utils.toHex(gasPrice),
        gas: 21000,
        nonce,
      },
      ALITH_PRIVATE_KEY,
    );
    const rep = await customRequest(web3, "eth_sendRawTransaction", [
      tx.rawTransaction,
    ]);
    await sleep(3 * SECONDS);
    // ALICE_ACCOUNT_BALANCE - (21000 * gasPrice) - value;

    // const expectedAliceBalance = (
    //   BigInt(INITIAL_ALICE_BALANCE) -
    //   BigInt(21000) * BigInt(gasPrice) -
    //   BigInt(web3.utils.toWei(TRANFER_VALUE, "ether"))
    // ).toString();
    const expectedTestBalance = (
      Number(web3.utils.toWei(TRANFER_VALUE, "ether")) - EXISTENTIAL_DEPOSIT
    ).toString();

    // expect(await web3.eth.getBalance(alith.address)).to.equal(
    //   expectedAliceBalance
    // );
    expect(await web3.eth.getBalance(TEST_ACCOUNT)).to.equal(
      expectedTestBalance,
    );
  });

  step("gas price too low", async function () {
    const nonce = await web3.eth.getTransactionCount(alith.address, "latest");

    let gasPrice = await web3.eth.getGasPrice();
    const tx = await web3.eth.accounts.signTransaction(
      {
        from: alith.address,
        to: TEST_ACCOUNT,
        value: web3.utils.toHex(web3.utils.toWei(TRANFER_VALUE, "ether")),
        gasPrice: web3.utils.toHex(Number(gasPrice) - 1),
        gas: "0x100000",
        nonce: nonce,
      },
      ALITH_PRIVATE_KEY,
    );
    let result = await customRequest(web3, "eth_sendRawTransaction", [
      tx.rawTransaction,
    ]);
    expect(result?.error?.message).to.be.equal(
      "gas price less than block base fee",
    );
  });

  step("balance insufficient", async function () {
    const nonce = await web3.eth.getTransactionCount(alith.address, "latest");
    let gasPrice = await web3.eth.getGasPrice();
    let testAccountBalance = await web3.eth.getBalance(TEST_ACCOUNT);
    const tx = await web3.eth.accounts.signTransaction(
      {
        from: TEST_ACCOUNT,
        to: GENESIS_ACCOUNTS[0],
        value: Number(testAccountBalance) + 1,
        gasPrice: gasPrice,
        gas: "0x100000",
        nonce: nonce,
      },
      TEST_ACCOUNT_PRIVATE_KEY,
    );
    let result = await customRequest(web3, "eth_sendRawTransaction", [
      tx.rawTransaction,
    ]);
    expect(result?.error?.message).to.be.equal(
      "insufficient funds for gas * price + value",
    );
  });
});
