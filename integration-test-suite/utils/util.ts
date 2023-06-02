import { SECONDS } from './constants';
import {ApiPromise, WsProvider} from '@polkadot/api';
import { ChildProcess, execSync } from 'child_process';
import fs from 'fs';
import {
  sleep,
  start5ireChainNode,
} from './setup';

export let polkadotApi: ApiPromise;
export let aliceNode: ChildProcess;
export let bobNode: ChildProcess;
export let charlieNode: ChildProcess;

/**
 * Used for starting up test by spawning the 5irechain node with 3 different accounts
 */
export const spawnNodes = async () => {
  // delete the tmp directory if it exists.
  const gitRoot = execSync('git rev-parse --show-toplevel').toString().trim();
  const tmpDir = `${gitRoot}/tmp`;
  if (fs.existsSync(tmpDir)) {
    // @ts-ignore
    fs.rmSync(tmpDir, { recursive: true });
  }
  aliceNode = start5ireChainNode('alice', { tmp: true, printLogs: true });
  bobNode = start5ireChainNode('bob', { tmp: true, printLogs: false });
  charlieNode = start5ireChainNode('charlie', { tmp: true, printLogs: false });

  console.log('started alice, bob, charlie nodes');

  polkadotApi = await ApiPromise.create();

  /*const wsProvider = new WsProvider('ws://127.0.0.1:9944');
  polkadotApi = await ApiPromise.create({ provider: wsProvider });*/
  return true;
};

/**
 * Kill Nodes started, to be used after test is done
 */
export async function killNodes() {
  await polkadotApi.disconnect();
  aliceNode?.kill('SIGINT');
  bobNode?.kill('SIGINT');
  charlieNode?.kill('SIGINT');
  await sleep(5 * SECONDS);
}
