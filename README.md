 # Sortfi-CLI 🚀
 
 Hey! If your folders are a total mess and you are tired of cleaning them up manually, this tool is here to help. Sortfi-CLI is a simple and fast terminal utility written in Rust. It automatically looks at your files and quickly moves them into standard folders (like Pictures, Documents, or Videos) based on their extensions.
 
 Also, if you find a useless file while sorting, you don't need to open your file manager - you can throw it into the trash straight from the terminal.
 
 > ⚠️ **Just a Heads Up:** The project is in its early **alpha stage**. To be completely honest, the code is still a bit raw, and there are some workarounds here and there. But it works great, it runs super fast, and I am actively working on it to make it better and cleaner every day.
 
 ## ⚡ Features
 
 - **Smart & Safe:** It completely ignores hidden files (like `.dotfiles`) and files without extensions. Your personal system configurations are completely safe.
 - **You are in Control:** For every single file, the tool will ask you what to do with a simple `[Y/n/t]` prompt. You choose whether to move it, skip it, or bin it.
 - **Really Fast:** Since it is written in Rust, it compiles in less than 2 seconds and runs instantly.
 - **Safe Deletions:** When you choose to delete a file, it goes safely into your native system trash, so you won't lose anything by mistake.
 
 ## 📦 Installation 

 ### Arch Linux
 
 Install the pre-compiled binary package via `pacman`:
 
 ```bash
 curl -LO 'https://github.com/Hiwiku/sortfi-cli/releases/download/v0.1.1-alpha/sortfi-cli-git-0.1.1.alpha-1-x86_64.pkg.tar.zst' 
 sudo pacman -U sortfi-cli-git-0.1.1.alpha-1-x86_64.pkg.tar.zst
 rm sortfi-cli-git-0.1.1.alpha-1-x86_64.pkg.tar.zst
 ```
 
 ### Universal Installer (Any Linux Distribution)
 
 For Ubuntu, Debian, Fedora, and other distributions, you can install the standalone musl binary using our automated script:
 
 ```bash
 cd ~
 curl -LO 'https://github.com/Hiwiku/sortfi-cli/releases/download/v0.1.1-alpha/sortfi-cli-git-0.1.1.alpha-1-x86_64-linux-musl' 
 sudo install -Dm755 sortfi-cli-git-0.1.1.alpha-1-x86_64-linux-musl /usr/local/bin/sortfi
 rm sortfi-cli-git-0.1.1.alpha-1-x86_64-linux-musl
 sortfi
 ```

## 🗑️ Uninstallation

If you ever need to completely remove the utility from your system, just use these commands.

### For Arch Linux (installed via pacman):

```bash
sudo pacman -R sortfi-cli-git
```

### For Universal Installer (installed via sudo install):

```bash
sudo rm /usr/local/bin/sortfi
```

 ## 🕹️ How to Use It
 
 You can run the utility from any directory by simply passing the path to the folder you want to clean up as an argument:
 
 ```bash
 sortfi ~/Downloads
 ```
 
 If you want to sort your home directory, just pass a tilde:
 
 ```bash
 sortfi ~
 ```
 
 ### Quick Controls:
 - `Y` - Yes, move this file to its proper folder!
 - `n` - No, just skip it.
 - `t` - Toss it into the system trash.
 
 ---
 
 ## 🗺️ Roadmap (What I'm Working on Next)
 
- **Custom Settings:** Let users choose their own extensions and target folders.

- **Useful Flags:**
   * `-y` - Automatic mode (sorts everything instantly without asking for confirmation every time).
   * `-r` - Permanent delete (completely wipes the file, bypassing the trash).
   * `-n` - Native mode (ignores your custom settings and runs with basic default rules).
