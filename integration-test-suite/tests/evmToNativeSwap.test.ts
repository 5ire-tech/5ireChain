import { expect } from 'chai';
import { BLOCK_TIME } from '../utils/constants';
import {killNodes, polkadotApi, spawnNodes} from "../utils/util";
import {Keyring} from "@polkadot/api";
import {addressToEvm} from "@polkadot/util-crypto";
import {waitForEvent} from "../utils/setup";
import Web3  from "web3";

// Setup the API and Alice Account
async function init() {
  const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

  const aliceEthAccount = addressToEvm(alice.addressRaw);
  return { alice, aliceEthAccount };
}

// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: "sr25519" });

describe("Swap EVM tokens to Native tokens test", function () {
  this.timeout(300 * BLOCK_TIME);
  this.slow(40 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Should swap evm token to native token
  it("Swap evm tokens to native tokens ", async () => {
    const { alice, aliceEthAccount } = await init();
    const web3 = new Web3(
      new Web3.providers.HttpProvider("http://127.0.0.1:9933")
    );
    const addressString = web3.utils.bytesToHex(Array.from(aliceEthAccount));
    // @ts-ignore
    let {data: aliceBalanceBefore} =  await polkadotApi.query.system.account(alice.address);

    //Create a extrinsic, withdraw 10 5ire coin from Alice
    const amount = polkadotApi.createType("Balance", "10000000000000000000");
    const transaction = await polkadotApi.tx.evm.withdraw(aliceEthAccount, amount);

    const unsub = await transaction.signAndSend(alice,  {tip: 200000000, nonce: -1}, (result) => {
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
   // @ts-ignore
    const { data: aliceBalanceAfter} = await polkadotApi.query.system.account(alice.address);
    expect(  aliceBalanceAfter.free.toBigInt() > aliceBalanceBefore.free.toBigInt()).true;

  });

  after(async () => {
    await killNodes();
  });
});

