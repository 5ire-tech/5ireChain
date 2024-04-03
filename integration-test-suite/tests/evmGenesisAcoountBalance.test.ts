import Web3 from "web3";
import {
  BLOCK_TIME,
  SECONDS,
  GENESIS_ACCOUNTS,
  GENESIS_ACCOUNT_BALANCE
} from "../utils/constants";
import {
  killNodeForTestEVM,
  spawnNodeForTestEVM,
} from "../utils/util";
import { sleep } from "../utils/setup";

import { expect } from "chai";
import { step } from "mocha-steps";
let web3: Web3;


describe.only("EVM related Balance", function () {
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
    step("genesis balance is setup correctly", async function () {
        for(let address of GENESIS_ACCOUNTS){
            expect(await web3.eth.getBalance(address)).to.equal(GENESIS_ACCOUNT_BALANCE);
            console.log(address + " has expected balance");
        };
    });
});