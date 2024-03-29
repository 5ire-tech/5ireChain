import Web3 from "web3";
import { BLOCK_TIME, CHAIN_ID, SECONDS } from "../utils/constants";
import {
  killNodeForTestEVM,
  spawnNodeForTestEVM,
} from "../utils/util";
import { sleep } from "../utils/setup";

import { expect } from "chai";

let web3: Web3;

describe("EVM related RPC Constants", function () {
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
    await sleep(40 * SECONDS);
  });
  after(async () => {
    await killNodeForTestEVM();
  });
  it("should have 0 hashrate", async function () {
    expect(await web3.eth.getHashrate()).to.equal(0);
  });

  it("should have chainId", async function () {
    // The chainId is defined by the Substrate Chain Id, default to 42
    expect(await web3.eth.getChainId()).to.equal(CHAIN_ID);
  });

  it("should have no account", async function () {
    expect(await web3.eth.getAccounts()).to.eql([]);
  });

});
