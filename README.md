# blueprint
blueprint


---

## 🧭 5. What To Do Next

### 🛠 Development
- [ ] Add more placeholder variables (`{{author}}`, `{{date}}`, etc.)
- [ ] Support multiple file generation per template
- [ ] Allow nested folders in templates
- [ ] Add flags (e.g., `--template`, `--output`) for automation

### 🚀 Publishing
- [ ] Create CLI entry with `[[bin]]` in `Cargo.toml`
- [ ] Add GitHub Actions for build/test
- [ ] Optionally publish to [crates.io](https://crates.io)

### 📦 Example Usage Idea
```sh
cargo run
# Prompts:
# > Enter project name: blog
# > Choose a template: hello_world
# -> generates `blog/main.txt` from `templates/hello_world.tmpl`