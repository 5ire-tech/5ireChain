import { expect } from 'chai';
import { BLOCK_TIME } from '../utils/constants';
import {killNodes, polkadotApi, spawnNodes} from "../utils/util";

describe('Setup for test', function () {
  this.timeout(300 * BLOCK_TIME);
  // 4 session.
  this.slow(40 * BLOCK_TIME);

  before(async () => {
    await spawnNodes()
  });

  // This test checks if 5irechain node starts up properly
  // and
  it('should startup node', async () => {
    console.log("Beginning first test");

    // Gets the current ethereum block
    const currentBlockStr = await polkadotApi.query.ethereum.currentBlock();
    const currentBlock = currentBlockStr.toJSON();
    // @ts-ignore
    expect(currentBlock.header.parentHash).to.equal(`0x0000000000000000000000000000000000000000000000000000000000000000`);
  });

  after(async () => {
    await killNodes();
  });
});
