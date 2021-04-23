const sleep = (ms) =>
  new Promise((resolve) => setTimeout(() => resolve(null), ms));

import { InstallAgentApp, _log } from "./common";

module.exports = async (orchestrator) => {
  orchestrator.registerScenario("Who am I", async (s, t) => {
    const alice_cell = await InstallAgentApp(s, "alice-cell");

    let hello_word_result = await alice_cell.call("peershare", "hello_word", {
      content: "Hello to Holochain",
    });

    _log("Hellow Word Result", hello_word_result);

    t.deepEqual(hello_word_result.data, "Hello to Holochain");

    await sleep(10);
  });
};
