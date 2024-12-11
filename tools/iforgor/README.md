# iforgor

`iforgor` is an interactive tool to manage and run scripts. See the full documentation
[here](https://github.com/nanocryk/iforgor/blob/main/iforgor/README.md).

After installing it, add the tanssi config file with
`iforgor source add TANSSI_PATH/tools/iforgor/tanssi.toml`. You can then run `iforgor` to search
and run scripts listed in this file. After changes to `tanssi.toml`, run `iforgor source reload` to
reload all registered config files.

Some commands may use the additional binary `ichoose`, which provides an interactive selection tool
for CLI scripts. Scripts should use this command if the list of choices is easy to fetch and the
choices are hard to remember.