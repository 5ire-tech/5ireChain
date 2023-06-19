import { expect } from "chai";
import { BLOCK_TIME } from "../utils/constants";
import { killNodes, polkadotApi, spawnNodes } from "../utils/util";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { sudoTx, waitForEvent } from "../utils/setup";
import { addressToEvm } from "@polkadot/util-crypto";
import { Web3 } from "web3";
import { ethers } from "ethers";

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
    const { alice, aliceEthAccount, bob, bobEthAccount } = await init();
    const privateKeyAlice =
      "0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a";


    const provider = new ethers.providers.JsonRpcProvider('http://127.0.0.1:9933');
    const signer = new ethers.Wallet(privateKeyAlice, provider);
    const tx = await signer.sendTransaction({
        to: "0x8eaf04151687736326c9fea17e25fc5287613693",
        value: ethers.utils.parseEther(String(1)),
        
      });

     await tx.wait();

  });
  // Setup the API and Alice Account
  async function init() {
    const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
    const bob = keyring.addFromUri("//Bob", { name: "Bob default" });

    const aliceEthAccount = addressToEvm(alice.addressRaw);
    const bobEthAccount = addressToEvm(bob.addressRaw);
    return { alice, aliceEthAccount, bob, bobEthAccount };
  }

  after(async () => {
    await killNodes();
  });
});
