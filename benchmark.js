import benchmark from "benchmark";
import * as colorette from "colorette";
import kleur from "kleur";
import * as kleurColors from "kleur/colors";
import chalk from "chalk";
import ansi from "ansi-colors";
import cliColor from "cli-color";
import * as picocolors from "picocolors";
import * as nanocolors from "nanocolors";
import * as yoctocolors from "yoctocolors";
import * as trololors from './trololors/pkg/trololors.js'

function formatNumber(number) {
  return String(number)
    .replace(/\d{3}$/, ",$&")
    .replace(/^(\d|\d\d)(\d{3},)/, "$1,$2");
}

console.log(colorette.green("colorette"));
console.log(kleur.green("kleur"));
console.log(chalk.green("chalk"));
console.log(ansi.green("ansi"));
console.log(cliColor.green("cliColor"));
console.log(picocolors.green("picocolors"));
console.log(nanocolors.green("nanocolors"));
console.log(yoctocolors.green("yoctocolors"));
console.log(trololors.green("trololors"));

const suite = new benchmark.Suite();
let out;

suite
  .add("chalk", () => {
    out = chalk.red("Add plugin to use time limit");
  })
  .add("cli-color", () => {
    out = cliColor.red("Add plugin to use time limit");
  })
  .add("ansi-colors", () => {
    out = ansi.red("Add plugin to use time limit");
  })
  .add("kleur", () => {
    out = kleur.red("Add plugin to use time limit");
  })
  .add("kleur/colors", () => {
    out = kleurColors.red("Add plugin to use time limit");
  })
  .add("colorette", () => {
    out = colorette.red("Add plugin to use time limit");
  })
  .add("nanocolors", () => {
    out = nanocolors.red("Add plugin to use time limit");
  })
  .add("picocolors", () => {
    out = picocolors.red("Add plugin to use time limit");
  })
  .add("yoctocolors", () => {
    out = yoctocolors.red("Add plugin to use time limit");
  })
  .add("trololors", () => {
    out = trololors.red("Add plugin to use time limit");
  })
  .on("cycle", (event) => {
    const name = event.target.name.padEnd("kleur/colors  ".length);
    const hz = formatNumber(event.target.hz.toFixed(0)).padStart(10);
    process.stdout.write(trololors.magenta(`${name}${picocolors.bold(hz)} ops/sec\n`));
  })
  .on("error", (event) => {
    process.stderr.write(trololors.red(event.target.error.toString()) + "\n");
    process.exit(1);
  })
  .run();
 
