# CPH for Zed - Competitive Programming Helper

A Zed editor extension that integrates with [competitive-companion](https://github.com/jmerle/competitive-companion) browser extension to streamline competitive programming workflows.

## Features

- 📋 **Problem Fetching**: Receive problems via slash commands or HTTP server
- 📝 **Template Generation**: Auto-create solution files with customizable templates
- 🧪 **Test Cases**: Automatically create input/output test files
- 🏃 **Quick Testing**: Commands to run and test solutions

## Installation

### From Zed Extensions
1. Open Zed
2. Press `Cmd+Shift+X` (macOS) or `Ctrl+Shift+X` (Linux)
3. Search for "Competitive Programming Helper"
4. Click Install

### As Dev Extension
```bash
git clone https://github.com/wanony/cph-zed.git
cd cph-zed
# In Zed: Extensions → Install Dev Extension → Select this directory
```

## Usage

### Slash Commands

| Command | Description |
|---------|-------------|
| `/cph-fetch <json>` | Create problem files from pasted JSON |
| `/cph-run [n]` | Run solution with test case n (or all) |
| `/cph-test` | Test solution against all cases |
| `/cph-server` | Info about the companion server |

### With Competitive Companion

1. Install [competitive-companion](https://github.com/jmerle/competitive-companion) browser extension
2. Install and run the companion server:
   ```bash
   cargo install cph-server
   cph-server --port 10045 --dir ./problems
   ```
3. Open a problem on Codeforces, AtCoder, etc.
4. Click the competitive-companion icon
5. Solution files are created automatically!

## Generated Files

```
problems/
└── Codeforces_Round_123/
    └── A_Problem_Name/
        ├── A_Problem_Name.cpp    # Solution template
        ├── problem.json          # Problem metadata
        └── tests/
            ├── 1.in
            ├── 1.out
            ├── 2.in
            └── 2.out
```

## Configuration

Configure via Zed settings (`settings.json`):

```json
{
  "cph": {
    "language": "cpp",
    "template_path": "~/.config/cph/template.cpp"
  }
}
```

## License

MIT License - see [LICENSE](LICENSE)
