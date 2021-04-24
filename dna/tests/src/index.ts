import { Orchestrator } from "@holochain/tryorama";

export const appID = "npersonas"

let orchestrator = new Orchestrator();
require("./profiles")(orchestrator);
orchestrator.run();
