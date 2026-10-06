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

### Option A: automatic, via the companion server (recommended)

The server writes files straight into your project over HTTP, so there is no sandbox
in the way.

```bash
# 1. Install the companion server
cargo install cph-server

# 2. Start it
cph-server --port 10045 --dir ./problems
```

Then install [competitive-companion][cc] in your browser, navigate to any problem on
Codeforces, AtCoder, etc., and click the extension icon. Solution files appear
automatically.

### Option B: manual, via `/cph-fetch`

Copy the JSON that competitive-companion shows you and run:

```
/cph-fetch {"name": "A. Two Sum", ...}
```

Zed's extension sandbox only permits writes to the extension's own work directory, so
the files are written there and you get a `cp -r` command to move them into your
project. Run it in your project terminal.

[cc]: https://github.com/jmerle/competitive-companion

## Slash Commands

| Command | Description |
|---------|-------------|
| `/cph-fetch [lang] <json>` | Generate solution template, metadata and test files from pasted JSON. `lang` is `cpp` (default), `python`, `rust`, or `java`. |
| `/cph-run [n]` | Print the command to run test case `n`, or all of them |
| `/cph-test` | Print the command to test against all cases and count passes/failures |
| `/cph-server` | Instructions for the companion server |

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

## Server CLI Options

```
cph-server [OPTIONS]

Options:
  -p, --port <PORT>        Port to listen on [default: 10045]
  -d, --dir <DIR>          Directory for problem files [default: .]
  -l, --language <LANG>    Template language: cpp, python, rust, java [default: cpp]
  -n, --notify             Enable desktop notifications [default: true]
  -h, --help               Print help
  -V, --version            Print version
```

## Requirements

- **Zed** editor (latest version recommended)
- **competitive-companion** browser extension
- **Rust** (for building the companion server)

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Related Projects

- [competitive-companion](https://github.com/jmerle/competitive-companion) - Browser extension for parsing problems
- [cph-server](https://crates.io/crates/cph-server) - Companion HTTP server (published separately)
- [CPH for VS Code](https://marketplace.visualstudio.com/items?itemName=DivyanshuAgrawal.competitive-programming-helper) - Similar extension for VS Code

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [jmerle](https://github.com/jmerle) for creating competitive-companion
- The Zed team for the excellent extension API
- The competitive programming community
