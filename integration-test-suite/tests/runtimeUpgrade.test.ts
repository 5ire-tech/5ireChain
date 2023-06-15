import { expect } from "chai";
import { BLOCK_TIME } from "../utils/constants";
import { killNodes, polkadotApi, spawnNodes } from "../utils/util";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { sudoTx, waitForEvent } from "../utils/setup";

import fs from "fs";

// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: "sr25519" });

describe("Wasm runtime upgrade", function () {
  this.timeout(300 * BLOCK_TIME);
  // 4 session.
  this.slow(40 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  /// Old runtime : Existential Deposit = 1 5ire
  /// New runtime : Existential Deposit = 0 5ire
  it("Existential deposit should be 0 ", async () => {
    const alice = keyring.addFromUri("//Alice");

    const EDBeforeUpgrade = await polkadotApi.consts.balances
      .existentialDeposit;
    // ED should be 1 5ire
    expect(EDBeforeUpgrade.toString()).to.equal("1000000000000000000");
    const code = fs
      .readFileSync('./blobs/node_5ire_runtime.wasm')
      .toString("hex");
    const proposal =
      polkadotApi.tx.system && polkadotApi.tx.system.setCode
        ? polkadotApi.tx.system.setCode(`0x${code}`) // For newer versions of Substrate
        : polkadotApi.tx.consensus.setCode(`0x${code}`); //For previous versions

    await sudoTx(polkadotApi, proposal);

    //await waitForEvent(polkadotApi, 'transactionPayment', 'TransactionFeePaid')
    const EDAfterUpgrade = await polkadotApi.consts.balances.existentialDeposit;
    console.log("Existential Deposit:", EDAfterUpgrade);
  });

  after(async () => {
    await killNodes();
  });
});
