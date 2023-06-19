import { expect } from "chai";
import { BLOCK_TIME } from "../utils/constants";
import { killNodes, polkadotApi, spawnNodes } from "../utils/util";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { sudoTx, waitForEvent } from "../utils/setup";
import { addressToEvm } from "@polkadot/util-crypto";
import { Web3 } from "web3";

// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: "sr25519" });

describe("EVM token tests", function () {
  this.timeout(300 * BLOCK_TIME);
  // 4 session.
  this.slow(40 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Should transfer EVM token
  it("Should transfer EVM tokens", async () => {
    const {
      alice,
      aliceEthAccount,
      bob,
      bobEthAccount,
      testAccount,
      testEthAccount,
    } = await init();

    const web3 = new Web3(
      new Web3.providers.HttpProvider("http://127.0.0.1:9933")
    );
    const testAddressString = web3.utils.bytesToHex(testEthAccount);
    const bobAddressString = web3.utils.bytesToHex(bobEthAccount);
    //Create a extrinsic, transferring 10 5ire coin to test Account
    // Because we can get privateKey through test Account
    // subkey inspect "movie shock injury cliff envelope armed van lunar mail disease balance cigar"
    const amount = polkadotApi.createType("Balance", "10000000000000000000");
    const transaction = polkadotApi.tx.evm.deposit(testEthAccount, amount);

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
    let testBalanceAfter = await web3.eth.getBalance(testAddressString);
    let expectationBalanceAfter = web3.utils.toBigInt("10000000000000000000");
    expect(testBalanceAfter).to.equal(expectationBalanceAfter);
    const privateKeyTestAccount =
      "2da33fc82c9b1e22a2a7d33ff963c0a8a528f6d88aec1ce181b398947ebfafdb";

      const gasPrice = await web3.eth.getGasPrice();
    const transferEVM = {
      from: testAddressString,
      to: bobAddressString,
      value:"1000000000000000000",
      gas: BigInt(21000),
      gasPrice: gasPrice, //wei
      nonce: await web3.eth.getTransactionCount(testAddressString, "pending"),
    };
    const gasAmount = await web3.eth.estimateGas(transferEVM);
    let fee = Number((gasPrice * gasAmount) / BigInt(10 ** 18));
    transferEVM.gas = gasAmount;
    
    const signedTx = await web3.eth.accounts.signTransaction(
      transferEVM,
      "0x2da33fc82c9b1e22a2a7d33ff963c0a8a528f6d88aec1ce181b398947ebfafdb"
    );
    const txInfo = await web3.eth.sendSignedTransaction(signedTx.rawTransaction);
    console.log("Here TxInfo", txInfo);
  });
  // Setup the API and Alice Account
  async function init() {
    const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
    const bob = keyring.addFromUri("//Bob", { name: "Bob default" });
    const testAccount = keyring.addFromMnemonic(
      "movie shock injury cliff envelope armed van lunar mail disease balance cigar"
    );

    const aliceEthAccount = addressToEvm(alice.addressRaw);
    const bobEthAccount = addressToEvm(bob.addressRaw);
    const testEthAccount = addressToEvm(testAccount.addressRaw);
    return {
      alice,
      aliceEthAccount,
      bob,
      bobEthAccount,
      testAccount,
      testEthAccount,
    };
  }

  after(async () => {
    await killNodes();
  });
});
