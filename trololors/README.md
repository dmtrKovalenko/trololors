<p align="center">
  <img src="./logo.jpeg"/>
</p>

<h1 align="center"> TROLOLORS </h1>
<h3 align="center"> Not the fastest terminal colors library. Don't even ask about size. </h3>

<div align="center">
    <img src="https://forthebadge.com/images/badges/uses-badges.svg">
    <img src="https://forthebadge.com/images/badges/0-percent-optimized.svg">
    <img src="https://forthebadge.com/images/badges/made-with-rust.svg">
    <img src="https://forthebadge.com/images/badges/makes-people-smile.svg">    
</div>

# Why? 

Don't even try to use it. But maybe you wan't say to your boss that you used RUST for your next projects. 

## Features

- **142x slower and 52x bigger than [nanocolors](https://github.com/ai/nanocolors)**
- Cat unfriendly
- NO additional colors
- Non tree-shakable, uses wasm-pack each time you import it
- Not maintained (at least for another week)
- Have 2 dependencies, build-time dependencies 


# Benchmark 

At least we tried 

```
chalk         24,616,741 ops/sec
cli-color      1,285,172 ops/sec
ansi-colors    5,777,891 ops/sec
kleur         25,739,702 ops/sec
kleur/colors  40,477,790 ops/sec
colorette     44,079,926 ops/sec
nanocolors    44,129,214 ops/sec
picocolors    43,642,815 ops/sec
trololors        292,323 ops/sec
``` 

## Install

```sh
npm install trololors
```

## Usage

![no god pleas no](./no-god.gif)
```js
import * as colors from 'trololors';

console.log(colors.red('trololo'));
```


### Colors

- `black`
- `red`
- `green`
- `yellow`
- `blue`
- `magenta`
- `cyan`
- `white`
- `gray`

### Background colors

- `bgBlack`
- `bgRed`
- `bgGreen`
- `bgYellow`
- `bgBlue`
- `bgMagenta`
- `bgCyan`
- `bgWhite`
- `bgGray`