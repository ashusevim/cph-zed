# Publishing CPH for Zed

Follow these steps to publish your extension so everyone can use it!

## Step 1: Create GitHub Repository

```bash
# Go to https://github.com/new and create "cph-zed" repository

# Then push your code:
cd /home/wanony/projects/AI/cph-zed
git remote add origin https://github.com/YOUR_USERNAME/cph-zed.git
git branch -M main
git push -u origin main
```

## Step 2: Fork zed-industries/extensions

1. Go to https://github.com/zed-industries/extensions
2. Click **Fork** (to your personal account, NOT an organization)
3. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/extensions.git
   cd extensions
   git submodule init
   git submodule update
   ```

## Step 3: Add Your Extension as Submodule

```bash
# From the extensions repo root:
git submodule add https://github.com/YOUR_USERNAME/cph-zed.git extensions/cph
git add extensions/cph
```

## Step 4: Update extensions.toml

Add this to the top-level `extensions.toml`:

```toml
[cph]
submodule = "extensions/cph"
version = "0.1.0"
```

## Step 5: Sort and Submit PR

```bash
pnpm sort-extensions
git add .
git commit -m "Add CPH (Competitive Programming Helper) extension"
git push origin main
```

Then go to GitHub and create a Pull Request to `zed-industries/extensions`.

## Optional: Publish Server to crates.io

```bash
cd /home/wanony/projects/AI/cph-server

# Login to crates.io
cargo login

# Publish
cargo publish
```

## Requirements Checklist

- [x] MIT License included
- [x] extension.toml with required fields
- [x] Cargo.toml properly configured
- [x] README.md with documentation
