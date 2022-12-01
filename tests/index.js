import assert from "assert";
import { add, task_01_1 } from "../build/debug.js";
assert.strictEqual(add(1, 2), 3);
console.log(task_01_1("test"));
console.log("ok");
