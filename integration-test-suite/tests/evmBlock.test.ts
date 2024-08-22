import { spawnNodeForTestEVM, killNodeForTestEVM } from "../utils/util";
import { BLOCK_TIME, ETH_BLOCK_GAS_LIMIT } from "../utils/constants";
import { expect } from "chai";
import { step } from "mocha-steps";
import Web3 from "web3";
let web3: Web3;
describe("EVM related Block using web3js/ethersjs", function () {
  this.timeout(100 * BLOCK_TIME);

  before(async () => {
    await spawnNodeForTestEVM();
    // Create instance web3
    web3 = new Web3(
      new Web3.providers.WebsocketProvider("ws://127.0.0.1:9944", {
        reconnect: {
          auto: true,
          delay: 3000, // ms
          maxAttempts: 5,
          onTimeout: false,
        },
      }),
    );
  });

  after(async () => {
    await killNodeForTestEVM();
  });
  it("should return genesis block by number", async function () {
    const block = await web3.eth.getBlock(0);
    expect(block).to.deep.include({
      author: "0x0000000000000000000000000000000000000000",
      difficulty: "0",
      extraData: "0x",
      gasLimit: ETH_BLOCK_GAS_LIMIT,
      gasUsed: 0,
      logsBloom:
        "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
      miner: "0x0000000000000000000000000000000000000000",
      number: 0,
      receiptsRoot:
        "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
      size: 505,
      timestamp: 0,
      totalDifficulty: "0",
    });

    expect(block.nonce).to.eql("0x0000000000000000");
    expect(block.hash).to.be.a("string").lengthOf(66);
    expect(block.parentHash).to.be.a("string").lengthOf(66);
    expect(block.timestamp).to.be.a("number");
  });
  step("should have empty uncles and correct sha3Uncles", async function () {
    const block = await web3.eth.getBlock(0);
    expect(block.uncles).to.be.a("array").empty;
    expect(block.sha3Uncles).to.equal(
      "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
    );
  });

  step(
    "should have empty transactions and correct transactionRoot",
    async function () {
      const block = await web3.eth.getBlock(0);
      expect(block.transactions).to.be.a("array").empty;
      expect(block).to.include({
        transactionsRoot:
          "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
      });
    },
  );
  step("get block by hash", async function () {
    const latest_block = await web3.eth.getBlock("latest");
    const block = await web3.eth.getBlock(latest_block.hash);
    expect(block.hash).to.be.eq(latest_block.hash);
  });
});
