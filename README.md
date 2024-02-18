# MAA-Z

MAA-Z is a project that aims to rebuild the MaaArknightsAssistant using Tauri and MAA Framework.

This project has another name in Greek, μαα, which will quite resemble the original project's name if you write them in uppercase which is ΜΑΑ(Note the difference though, they are greek letters!).

## Build

First you need to set up the maa framework. We provide a script to do that. The script currently only supports Windows, but you can run the commands manually if you are on another platform. Or feel free to provide a PR to add support for your platform.

```bash
python makedeps.py
```

Then build with pnpm. In fact, any node.js package manager should work, but we recommend pnpm.

```bash
pnpm install
pnpm tauri build
```

## TODO List

This is a project still in early stages of the development and our focus now is listed below.

- [x] Basic UI
- [ ] Tasks ( Help wanted! )
- [ ] Better UI/UX ( This includes a more consistent theme and a better UX )
- [ ] Cross-platform support ( We are currently focusing on Windows, but the ability of Tauri makes it theoretically not too hard to run on other platforms )
