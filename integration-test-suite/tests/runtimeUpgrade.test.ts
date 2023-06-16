import { expect } from "chai";
import { BLOCK_TIME } from "../utils/constants";
import { killNodes, polkadotApi, spawnNodes } from "../utils/util";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { sudoUncheckTx, waitForEvent, waitForTheNextSession, waitNfinalizedBlocks } from "../utils/setup";
import type { WeightV2 } from "@polkadot/types/interfaces";
import { BN } from "@polkadot/util";

import fs from "fs";
import path from "path";
// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: "sr25519" });

describe("Wasm runtime upgrade", function () {
  this.timeout(300 * BLOCK_TIME);
  // 4 session.
  this.slow(40 * BLOCK_TIME);
  let code: any;
  before(async () => {
    await spawnNodes();

    const wasm_dir = path.join(__dirname, "/blobs/node_5ire_runtime.compact.compressed.wasm");
    code = fs.readFileSync(wasm_dir).toString("hex");
  });

  /// Old runtime : Existential Deposit = 1 5ire
  /// New runtime : Existential Deposit = 0 5ire
  it("Existential deposit should be 0 ", async () => {
    const alice = keyring.addFromUri("//Alice");

    const EDBeforeUpgrade = polkadotApi.consts.balances
      .existentialDeposit;
    // ED should be 1 5ire
    expect(EDBeforeUpgrade.toString()).to.equal("1000000000000000000");
    const proposal = polkadotApi.tx.system.setCode(`0x${code}`)

    //await sudoUncheckTx(polkadotApi, proposal);
    const weight = polkadotApi.registry.createType('WeightV2', {
        refTime: new BN(0),
        proofSize: new BN(0),
      }) as WeightV2;

    
    let unsub = await polkadotApi.tx.sudo.sudoUncheckedWeight(proposal, weight)
        .signAndSend(alice, (result) => {
                console.log(`Current status is ${result.status}`);
                if (result.status.isInBlock) {
                    console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
                } else if (result.status.isFinalized) {
                    console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
                    unsub();
                    
                }
            }
        )
    await waitForEvent(polkadotApi, 'sudo', 'Sudid');
    const EDAfterUpgrade = polkadotApi.consts.balances.existentialDeposit;
    console.log("Existential Deposit:", EDAfterUpgrade);
  });

  after(async () => {
    await killNodes();
  });
});
