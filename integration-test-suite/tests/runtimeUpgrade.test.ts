import { expect } from "chai";
import { BLOCK_TIME } from "../utils/constants";
import { killNodes, polkadotApi, spawnNodes } from "../utils/util";
import {  Keyring } from "@polkadot/api";
import {
  uncheckedSudoTx,
  waitForEvent,
} from "../utils/setup";
import type { WeightV2 } from "@polkadot/types/interfaces";
import { BN } from "@polkadot/util";

import fs from "fs";
import path from "path";
import {DetectCodec} from "@polkadot/types/types/detect";
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
    //expect(EDBeforeUpgrade.toString()).to.equal("1000000000000000000");
    console.log("Existential Deposit:", EDBeforeUpgrade.toHuman());
    const proposal = polkadotApi.tx.system.setCode(`0x${code}`)

    const weight: DetectCodec<any, any> = polkadotApi.registry.createType('WeightV2', {
        refTime: new BN(0),
        proofSize: new BN(0),
      }) as WeightV2;

    await uncheckedSudoTx(weight, polkadotApi, proposal);
    await waitForEvent(polkadotApi, 'system', 'ExtrinsicSuccess');
    const EDAfterUpgrade = polkadotApi.consts.balances.existentialDeposit;

    console.log("Existential Deposit:", EDAfterUpgrade.toHuman());

    expect(EDAfterUpgrade.toHuman() == 0).true;
    expect(EDBeforeUpgrade.toHuman() != EDAfterUpgrade.toHuman()).true;
  });

  after(async () => {
    await killNodes();
  });
});
