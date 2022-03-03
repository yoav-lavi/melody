import { Melody } from "../mod.ts";

const melody = new Melody();
await melody.init();
melody.compile(`
/* matches the batman theme tune */
16 of "na";

2 of match {
  <space>;
  "batman";
}
`, { debug: true });