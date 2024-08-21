import { expect } from "chai";
import { BLOCK_TIME } from "../utils/constants";
import { killNodes, polkadotApi, spawnNodes } from "../utils/util";

describe("Setup for test", function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // This test checks if 5irechain node starts up properly
  // and
  it("should startup node", async () => {
    console.log("Beginning first test");

    // Gets the current ethereum block
    const currentBlockStr = await polkadotApi.query.ethereum.currentBlock();
    const currentBlock = currentBlockStr.toJSON();
    // @ts-ignore
    expect(currentBlock.header.parentHash).not.null;
  });

  // This test is for block production
  it("tests block production", async () => {
    await testBlockProduction();
  });

  // This test is for block finalization
  it("tests block finalization", async () => {
    await testBlockFinalization();
  });

  after(async () => {
    await killNodes();
  });
});

async function testBlockProduction() {
  let previousBlockHash: `0x${string}` | null = null;

  try {
    polkadotApi.rpc.chain.subscribeNewHeads((header) => {
      const blockNumber = header.number.toNumber();
      const blockHash = header.hash.toHex();

      console.log(
        `New block produced - Block Number: ${blockNumber}, Block Hash: ${blockHash}`,
      );

      expect(blockHash).not.null;
      expect(blockHash).not.undefined;

      expect(blockHash !== previousBlockHash).true;

      previousBlockHash = blockHash;
    });

    await new Promise((resolve) => setTimeout(resolve, 60000));
  } catch (error) {
    console.error("Error:", error);
  }
}

async function testBlockFinalization() {
  let previousBlockHash: `0x${string}` | null = null;

  try {
    const finalizedHead = await polkadotApi.rpc.chain.getFinalizedHead();
    const finalizedHeadHash = finalizedHead.toHex();

    console.log(`Finalized Head Hash: ${finalizedHeadHash}`);

    polkadotApi.rpc.chain.subscribeFinalizedHeads((header) => {
      const blockNumber = header.number.toNumber();
      const blockHash = header.hash.toHex();

      console.log(
        `Block finalized - Block Number: ${blockNumber}, Block Hash: ${blockHash}`,
      );

      expect(blockHash).not.null;
      expect(blockHash).not.undefined;

      expect(blockHash !== previousBlockHash).true;

      previousBlockHash = blockHash;
    });

    await new Promise((resolve) => setTimeout(resolve, 60000));
  } catch (error) {
    console.error("Error:", error);
  }
}
