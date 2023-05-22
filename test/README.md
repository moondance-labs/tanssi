# Moonwall integration tests for Tanssi

Setup: install node and pnpm:

```
sudo npm i -g pnpm
pnpm i
```

Before running tests: compile rust binaries and build ChainSpec files:

```
cargo build --features=fast-runtime --release --all
pnpm run build-spec
```

Run moonwall TUI interface:

```
pnpm moonwall
```

Run tests:

```
# manual-seal tests, only orchestrator chain runs, container chains are mocked
pnpm moonwall test dev_tanssi
# zombienet tests, all the chains run
pnpm moonwall test zombie_tanssi
```

To see the logs of a failing zombienet node:

```
cd /tmp
ls -ltr
# cd into the last zombie folder, that's the most recent zombie network
cd zombie-3aff699b8e6c41a7a0c296f056a750a0_-87975-Ow0nVobAGIPt
# list all the logs
ls *.log
# follow logs
tail -F -n9999 Collator2000-01.log
# nicer interface that allows search
less -R Collator2000-01.log
# or just open it in any other text editor
```

To upgrade moonwall or other dependencies:

```
pnpm up --latest
```


Debugging zombienet

You can enable zombienet debug logs to get more information about the commands that are being run:

```
DEBUG=* pnpm moonwall test zombie_tanssi
```
