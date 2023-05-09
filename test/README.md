Setup: install node and pnpm:

```
sudo npm i -g pnpm
pnpm i
pnpm moonwall
```

Run tests:

```
pnpm moonwall test dev_tanssi
pnpm moonwall test zombie_tanssi
```

To see the logs of a failing zombienet node:

```
cd /tmp
ls -ltr zombie*
# cd into the last folder, that's the most recent zombie network
cd zombie-3aff699b8e6c41a7a0c296f056a750a0_-87975-Ow0nVobAGIPt
# list all the logs
ls *.log
# follow logs
tail -F -n9999 Collator2000-01.log
# nicer interface that allows search
less -R Collator2000-01.log
# or just open it in any other text editor
```

To upgrade moonwall:

```
pnpm up --latest
```
