import { componentize } from "@bytecodealliance/componentize-js";
import { readFile, writeFile } from "node:fs/promises";

const jsSource = await readFile("exports.js", "utf8");
const worldSource = await readFile("../wit/world.wit", "utf8");

const { component, imports } = await componentize(jsSource, worldSource, {
  worldName: "gpio-app",
  debug: false,
  enableStdout: false,
});

console.log(imports);
await writeFile("guest.component.wasm", component);
