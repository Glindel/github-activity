# 📊 github-activity

`github-activity` is a **Rust CLI tool** that fetches and displays the latest public events from a GitHub user profile.  
It’s a simple way to check a user’s recent activity directly from your terminal.

---

## 🚀 Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/your-username/github-activity.git
cd github-activity
cargo build --release
```

The binary will be available in `target/release/github-activity`.

---

## 📌 Usage

Run the command with a GitHub username as parameter:

```bash
github-activity <username>
```

### Example

```bash
github-activity edwarsthat
```

---

## 📊 Example output

```bash
-----------------------------------
Event Checker for User: {user}
-----------------------------------
- Pushed 1 commits to {repo}
- Pushed 1 commits to {repo}
- Pushed 1 commits to {repo}
- Pushed 1 commits to {repo}
- Pushed 1 commits to {repo}
- Closed pull request 14 on {repo}
- Opened pull request 14 on {repo}
- Pushed 1 commits to {repo}
- Create repository {repo}
- Pushed 1 commits to {repo}
- Closed pull request 13 on {repo}
- Opened pull request 13 on {repo}
- Pushed 1 commits to {repo}
- Create repository {repo}
- Pushed 1 commits to {repo}
- Closed pull request 12 on {repo}
- Opened pull request 12 on {repo}
- Opened pull request 11 on {repo}
- Pushed 1 commits to {repo}
- Delete branch {branch} on {repo}
- Delete branch {branch} on {repo}
- Create repository {repo}
- Create repository {repo}
- Opened issue on {repo}
- Opened issue on {repo}
- Opened issue on {repo}
- Opened issue on {repo}
- Opened issue on {repo}
- Pushed 1 commits to {repo}
- Closed pull request 5 on {repo}
```

---

## 🛠 Future improvements

- Filtering events by type (commits, PRs, issues, repos…)
- Displaying events with colors and icons for better readability
- Exporting activity logs to JSON/Markdown
- Pagination for longer histories
