 # Sortfi-CLI 🚀
 
 Hey! If your folders are a total mess and you are tired of cleaning them up manually, this tool is here to help. Sortfi-CLI is a simple and fast terminal utility written in Rust. It automatically looks at your files and quickly moves them into standard folders (like Pictures, Documents, or Videos) based on their extensions.
 
 Also, if you find a useless file while sorting, you don't need to open your file manager - you can throw it into the trash straight from the terminal.
 
 > ⚠️ **Just a Heads Up:** The project is in its early **alpha stage**. To be completely honest, the code is still a bit raw, and there are some workarounds here and there. But it works great, it runs super fast, and I am actively working on it to make it better and cleaner every day.
 
 ## ⚡ Features
 
 - **Smart & Safe:** It completely ignores hidden files (like `.dotfiles`) and files without extensions. Your personal system configurations are completely safe.
 - **You are in Control:** For every single file, the tool will ask you what to do with a simple `[Y/n/t]` prompt. You choose whether to move it, skip it, or bin it.
 - **Really Fast:** Since it is written in Rust, it compiles in less than 2 seconds and runs instantly.
 - **Safe Deletions:** When you choose to delete a file, it goes safely into your native system trash, so you won't lose anything by mistake.
 
 ## 🛠️ Installation
 
 You can easily build it from source. Just open your terminal and run these commands:
 
 ```bash
 git clone https://github.com/hiwiku/sortfi-cli.git
 cd sortfi-cli
 cargo build --release
 cp target/release/sortfi-cli ~/sortfi
 cd ~
 rm -rf ~/sortfi-cli
 ~/sortfi
 ```
After running the command, the binary file will be in `~/`, and you can run it with the following command: `~/sortfi`
 
 ## 🕹️ How to Use It
 
 You can run the utility from any directory by simply passing the path to the folder you want to clean up as an argument:
 
 ```bash
 ~/sortfi ~/Downloads
 ```
 
 If you want to sort your home directory, just pass a tilde:
 
 ```bash
 ~/sortfi ~
 ```
 
 ### Quick Controls:
 - `Y` — Yes, move this file to its proper folder!
 - `n` — No, just skip it.
 - `t` — Toss it into the system trash.
 
 ---
 
 ## 🗺️ Roadmap (What I'm Working on Next)
 
 - [ ] **Custom Settings:** Let users choose their own extensions and target folders.
 - [ ] **Useful Flags:**
   * `-y` — Automatic mode (sorts everything instantly without asking for confirmation every time).
   * `-r` — Permanent delete (completely wipes the file, bypassing the trash).
   * `-n` — Native mode (ignores your custom settings and runs with basic default rules).
 - [ ] **Easy Installer:** A quick one-line command using `curl` to install the tool in one second.
