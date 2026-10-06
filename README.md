# CPH for Zed

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Zed Extension](https://img.shields.io/badge/Zed-Extension-orange)](https://zed.dev/extensions)

**Competitive Programming Helper** for Zed - seamlessly receive problems from [competitive-companion](https://github.com/jmerle/competitive-companion) and generate solution templates with test cases.

## Overview

CPH for Zed integrates with the popular [competitive-companion](https://github.com/jmerle/competitive-companion) browser extension to streamline your competitive programming workflow. Parse problems from **100+ online judges** including Codeforces, AtCoder, LeetCode, and more.

### Key Features

- 📋 **Problem Parsing**: Receive problems directly from competitive-companion
- 📝 **Smart Templates**: Generate solution files with language-specific templates (C++, Python, Rust, Java)
- 🧪 **Test Case Management**: Automatically create input/output test files
- 📁 **Organized Structure**: Problems are neatly organized by contest and problem name
- 🔗 **Metadata Tracking**: Keep track of problem URLs, time limits, and memory constraints

## Supported Online Judges

Works with all websites supported by competitive-companion, including:

- Codeforces
- AtCoder
- LeetCode
- CodeChef
- HackerRank
- SPOJ
- CSES
- And [100+ more](https://github.com/jmerle/competitive-companion#supported-websites)

## Installation

This extension is installed as a **dev extension**. Zed has deprecated slash-command
extensions and no longer accepts them in the public registry — see
[PUBLISHING.md](PUBLISHING.md) for the policy text.

```bash
git clone https://github.com/ashusevim/cph-zed.git
```

Then in Zed:

1. Open the Extensions panel (`Cmd+Shift+X` on macOS, `Ctrl+Shift+X` on Linux)
2. Click **Install Dev Extension**
3. Select the `cph-zed` directory

Zed compiles it with `cargo build --target wasm32-wasip2`. If your Rust toolchain is
not rustup-managed, install that target first:

```bash
rustup target add wasm32-wasip2
```

## Quick Start

### Fetch a problem

Install [competitive-companion][cc] in your browser, navigate to any problem on
Codeforces, AtCoder, etc., and click the extension icon. It shows you the problem JSON.

Copy that JSON and run:

```
/cph-fetch {"name": "A. Two Sum", ...}
```

Zed's extension sandbox only permits writes to the extension's own work directory, so
the files are written there and you get a `cp -r` command to move them into your
project. Run it in your project terminal.

### Why not automatic, like the VS Code original?

The original [agrawal-d/cph][orig] runs an HTTP listener on port 27121 inside the VS
Code extension host and writes files straight into your workspace. A Zed extension can
do neither half:

1. Its WASM sandbox exposes no socket API, so it cannot accept competitive-companion's
   POST.
2. Only the extension's own work directory is writable, so it cannot write into your
   project even if it had the data.

Closing that gap needs a separate native server binary — a different project, not a
Zed extension.

[cc]: https://github.com/jmerle/competitive-companion
[orig]: https://github.com/agrawal-d/cph

## Slash Commands

| Command | Description |
|---------|-------------|
| `/cph-fetch [lang] <json>` | Generate solution template, metadata and test files from pasted JSON. `lang` is `cpp` (default), `python`, `rust`, or `java`. |
| `/cph-run [n]` | Print the command to run test case `n`, or all of them |
| `/cph-test` | Print the command to test against all cases and count passes/failures |
| `/cph-server` | Explain why automatic fetching is unavailable in Zed |

## Generated File Structure

When you parse a problem, CPH creates this structure:

```
problems/
└── Codeforces_-_Educational_Round_123/
    └── A_Example_Problem/
        ├── A_Example_Problem.cpp    # Solution template
        ├── problem.json             # Problem metadata
        └── tests/
            ├── 1.in                 # Sample input 1
            ├── 1.out                # Expected output 1
            ├── 2.in                 # Sample input 2
            └── 2.out                # Expected output 2
```

## Solution Templates

### C++ Template (Default)

```cpp
#include <bits/stdc++.h>
using namespace std;

void solve() {
    // Your solution here
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    
    int t = 1;
    // cin >> t;  // Uncomment for multiple test cases
    while (t--) solve();
    return 0;
}
```

Templates are also available for **Python**, **Rust**, and **Java**.

## Requirements

- **Zed** editor (latest version recommended)
- **competitive-companion** browser extension
- **Rust** with the `wasm32-wasip2` target (Zed compiles the extension for you)

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Related Projects

- [competitive-companion](https://github.com/jmerle/competitive-companion) - Browser extension for parsing problems
- [CPH for VS Code](https://github.com/agrawal-d/cph) - The original this is modelled on. A VS Code extension, so it can listen on a socket and write into your workspace directly; a Zed extension cannot.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [jmerle](https://github.com/jmerle) for creating competitive-companion
- The Zed team for the excellent extension API
- The competitive programming community
