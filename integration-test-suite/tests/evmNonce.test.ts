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
  polkadotApi,
  spawnNodeForTestEVM,
} from "../utils/util";
import { sleep, waitForEvent } from "../utils/setup";

import { expect } from "chai";
import { step } from "mocha-steps";
import Keyring from "@polkadot/keyring";
import { Wallet, ethers } from "ethers";
let web3: Web3;

const TEST_ACCOUNT = "0xdd33Af49c851553841E94066B54Fd28612522901";
const TEST_ACCOUNT_PRIVATE_KEY =
  "0x4ca933bffe83185dda76e7913fc96e5c97cdb7ca1fbfcc085d6376e6f564ef71";
const TRANFER_VALUE = "1"; // 1 5IRE must be higher than ExistentialDeposit
//const GAS_PRICE = "0x3B9ACA00"; // 1000000000


describe("EVM related Nonce using web3js/ethersjs", function () {
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
    await sleep(3 * SECONDS);
  });
  after(async () => {
    await killNodeForTestEVM();
  });

  step("get nonce", async function () {
    this.timeout(20000);

    expect(
      await web3.eth.getTransactionCount(alith.address, "latest")
    ).to.eq(0);
    const gasPrice = await web3.eth.getGasPrice();
    const tx = await web3.eth.accounts.signTransaction(
      {
        from: alith.address,
        to: TEST_ACCOUNT,
        value: web3.utils.toHex(web3.utils.toWei(TRANFER_VALUE, "ether")),
        gasPrice: web3.utils.toHex(gasPrice),
        gas: "0x100000",
      },
      ALITH_PRIVATE_KEY
    );
    const rep = await customRequest(web3, "eth_sendRawTransaction", [
      tx.rawTransaction,
    ]);
    await sleep(3 * SECONDS);
    expect(
      await web3.eth.getTransactionCount(alith.address, "latest")
    ).to.eq(1);
  });

  step("stalled nonce", async function () {
    let gasPrice = await web3.eth.getGasPrice();
    const tx = await web3.eth.accounts.signTransaction(
      {
        from: alith.address,
        to: TEST_ACCOUNT,
        value: web3.utils.toHex(web3.utils.toWei("2", "ether")),
        gasPrice: web3.utils.toHex(Number(gasPrice)),
        gas: "0x100000",
        nonce: 0,
      },
      ALITH_PRIVATE_KEY
    );
    let result = await customRequest(web3, "eth_sendRawTransaction", [
      tx.rawTransaction,
    ]);
    expect(result?.error?.message).to.be.equal("nonce too low");
  });
});
