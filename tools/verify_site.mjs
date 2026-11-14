// Validate the static browser's inline JavaScript without a browser or network.
import { readFileSync } from "node:fs";
import { Script } from "node:vm";
const html = readFileSync(new URL("../index.html", import.meta.url), "utf8");
const result = html.match(/<script>([\s\S]*?)<\/script>/);
if (!result) throw new Error("No inline browser script found");
new Script(result[1], { filename: "index.html" });
console.log("Browser JavaScript parsed successfully.");
