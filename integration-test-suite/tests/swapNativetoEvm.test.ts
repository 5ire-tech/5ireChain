import { expect } from "chai";
import { BLOCK_TIME } from "../utils/constants";
import { killNodes, polkadotApi, spawnNodes } from "../utils/util";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { sudoTx, waitForEvent } from "../utils/setup";
import { addressToEvm } from "@polkadot/util-crypto";
import { Web3 } from "web3";

// Setup the API and Alice Account
async function init() {
  const charlie = keyring.addFromUri("//Charlie", { name: "Charlie default" });

  const charlieEthAccount = addressToEvm(charlie.addressRaw);
  return { charlie, charlieEthAccount };
}

// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: "sr25519" });

describe("Swap token tests", function () {
  this.timeout(300 * BLOCK_TIME);
  // 4 session.
  this.slow(40 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Should swap native token to evm token
  it.only("Swap native tokens to evm tokens ", async () => {
    const { charlie, charlieEthAccount } = await init();
    const web3 = new Web3(
      new Web3.providers.HttpProvider("http://127.0.0.1:9933")
    );
    const addressString = web3.utils.bytesToHex(charlieEthAccount);
    let charlieBalance = await web3.eth.getBalance(addressString);
    console.log("Balance:", charlieBalance);
    let expectationBalance = web3.utils.toBigInt(0);
    //assert that bob initial evm balance is 0
    expect(charlieBalance).to.equal(expectationBalance);

    //Create a extrinsic, transferring 10 5ire coin to Bob
    const amount = polkadotApi.createType("Balance", "10000000000000000000");
    const transaction = polkadotApi.tx.evm.deposit(charlieEthAccount, amount);

    const unsub = await transaction.signAndSend(charlie, (result) => {
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
    let charlieBalanceAfter = await web3.eth.getBalance(addressString);
    let expectationBalanceAfter = web3.utils.toBigInt("10000000000000000000");
    expect(charlieBalanceAfter).to.equal(expectationBalanceAfter);
  });

  after(async () => {
    await killNodes();
  });
});
