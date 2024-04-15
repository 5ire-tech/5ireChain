import Web3 from "web3";
import { BLOCK_TIME, SECONDS, GENESIS_ACCOUNTS } from "../utils/constants";
import {
  customRequest,
  killNodeForTestEVM,
  polkadotApi,
  spawnNodeForTestEVM,
} from "../utils/util";
import { sleep } from "../utils/setup";

import { expect } from "chai";
let web3: Web3;

const TEST_ACCOUNT = "0xdd33Af49c851553841E94066B54Fd28612522901";
const TEST_ACCOUNT_PRIVATE_KEY =
  "0x4ca933bffe83185dda76e7913fc96e5c97cdb7ca1fbfcc085d6376e6f564ef71";

const ERC20_ABI = require("./contracts/MyToken.json").abi;
const ERC20_BYTECODES = require("./contracts/MyToken.json").bytecode;

describe("EVM related Gas using web3js/ethersjs", function () {
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

  it("estimate gas for contract creation", async function () {
    this.timeout(40000);
    let gasEstimation = await web3.eth.estimateGas({
      from: GENESIS_ACCOUNTS[0],
      data: ERC20_BYTECODES,
    });
    expect(gasEstimation).to.eq(894198);
  });
});
