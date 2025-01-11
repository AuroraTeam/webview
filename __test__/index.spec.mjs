import test from "ava";

import { Window } from "../index.js";

test("Window is a function", (t) => {
  t.is(typeof Window, "function");
});
