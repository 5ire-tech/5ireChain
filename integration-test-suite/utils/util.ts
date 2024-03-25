import { SECONDS } from "./constants";
import { ApiPromise } from "@polkadot/api";
import { ChildProcess, execSync } from "child_process";
import fs from "fs";
import { sleep, start5ireChainNode } from "./setup";
import Web3 from "web3";

export let polkadotApi: ApiPromise;

export let aliceNode: ChildProcess;
export let bobNode: ChildProcess;
export let charlieNode: ChildProcess;
export let web3: Web3;

/**
 * Used for starting up test by spawning the 5irechain node with 3 different accounts
 */
export const spawnNodes = async () => {
  await removeTmp();
  aliceNode = start5ireChainNode("alice", { tmp: true, printLogs: false });
  bobNode = start5ireChainNode("bob", { tmp: true, printLogs: false });
  charlieNode = start5ireChainNode("charlie", { tmp: true, printLogs: false });

  console.log("started alice, bob, charlie nodes");

  polkadotApi = await ApiPromise.create();

  web3 = new Web3("ws://127.0.0.1:9944");
  return true;
};

/**
 * Kill Nodes started, to be used after test is done
 */
export async function killNodes() {
  await polkadotApi.disconnect();
  aliceNode?.kill("SIGINT");
  bobNode?.kill("SIGINT");
  charlieNode?.kill("SIGINT");
  await sleep(2 * SECONDS);
}


export const spawnNodeForTestEVM = async () => {
  await removeTmp();
  aliceNode = start5ireChainNode("alice", { tmp: true, printLogs: true });

  console.log("started alice node");
  
  
  return true;
};

export async function killNodeForTestEVM() {
  aliceNode?.kill("SIGINT");
  await sleep(2 * SECONDS);
}


export async function removeTmp() {
  // delete the tmp directory if it exists.
  const gitRoot = execSync("git rev-parse --show-toplevel").toString().trim();
  const tmpDir = `${gitRoot}/tmp/fire`;
  console.log(`tmp directory is ${tmpDir}`);
  if (fs.existsSync(tmpDir)) {
    console.log(`tmp directory exists ${tmpDir}`);
    // @ts-ignore
    fs.rmSync(tmpDir, { recursive: true });
  }

  if (fs.existsSync(tmpDir)) {
    console.log(`tmp directory still exists ${tmpDir}`);
    // @ts-ignore
    fs.rmSync(tmpDir, { recursive: true });
  } else {
    console.log(`tmp directory doesn't exists anymore ${tmpDir}`);
  }

}