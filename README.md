# Idle Battle

## Getting started

Install packages

```sh
yarn
```

## Test

Single run testing:

```sh
anchor test
```

With watcher:

```sh
fswatch -o $PWD/programs/idle-battle \
    | xargs -n1 -I{} anchor test
```

Use `anchor test --skip-local-validator` if you already have a local validator running.

## Build

```sh
anchor build --idl-ts $PWD/app-ts/src/idl
```

## Deploy

```sh
anchor deploy
```
