import subprocess
import sys

required = ["rich", "questionary", "requests"]

for package in required:
    try:
        __import__(package)
    except ImportError:
        print(f"Installing missing package: {package}")
        subprocess.check_call(
            [sys.executable, "-m", "pip", "install", package]
        )
        
import os
import time
import requests
import questionary
from rich.console import Console
from rich.panel import Panel
from rich.progress import Progress, SpinnerColumn, BarColumn, TextColumn
from rich.table import Table

console = Console()

REGISTRY_URL = "https://raw.githubusercontent.com/skiLLM-Labs/skiLLM/refs/heads/main/registry.json"
BASE_RAW = "https://raw.githubusercontent.com/skiLLM-Labs/skiLLM/refs/heads/main/"

PLATFORMS = {
    "Claude Code": os.path.expanduser("~/.claude/skills"),
    "Cursor": os.path.expanduser("~/.cursor/skills"),
    "Codex": os.path.expanduser("~/.codex/skills"),
    "Continue": os.path.expanduser("~/.continue/skills"),
}


def show_header():
    console.clear()

    header = """
           /$$       /$$ /$$       /$$       /$$      /$$
          | $$      |__/| $$      | $$      | $$$    /$$$
  /$$$$$$$| $$   /$$ /$$| $$      | $$      | $$$$  /$$$$
 /$$_____/| $$  /$$/| $$| $$      | $$      | $$ $$/$$ $$
|  $$$$$$ | $$$$$$/ | $$| $$      | $$      | $$  $$$| $$
 \____  $$| $$_  $$ | $$| $$      | $$      | $$\  $ | $$
 /$$$$$$$/| $$ \  $$| $$| $$$$$$$$| $$$$$$$$| $$ \/  | $$
|_______/ |__/  \__/|__/|________/|________/|__/     |__/

Installer for skiLLM Skills
    """

    console.print(
        Panel.fit(
            header,
            border_style="white",
        )
    )


def load_registry():
    with console.status("[bold white]Fetching registry...[/bold white]"):
        response = requests.get(REGISTRY_URL)
        time.sleep(0.5)
        return response.json()


def show_platforms():
    table = Table(title="Supported Platforms")

    table.add_column("Platform")
    table.add_column("Install Path")

    for name, path in PLATFORMS.items():
        table.add_row(name, path)

    console.print(table)


def install_skill(skill, install_path):
    url = BASE_RAW + skill["path"]

    os.makedirs(install_path, exist_ok=True)

    filename = skill["name"] + ".md"

    full_path = os.path.join(install_path, filename)

    with Progress(
        SpinnerColumn(),
        TextColumn("[bold white]Downloading[/bold white] {task.description}"),
        BarColumn(),
        TextColumn("{task.percentage:>3.0f}%"),
        console=console,
    ) as progress:

        task = progress.add_task(skill["name"], total=100)

        response = requests.get(url)

        for i in range(100):
            time.sleep(0.01)
            progress.update(task, advance=1)

    with open(full_path, "w", encoding="utf-8") as f:
        f.write(response.text)

    console.print(
        f"[bold green]✓ Installed[/bold green] [white]{skill['name']}[/white]"
    )
    console.print(f"[dim]{full_path}[/dim]\n")


def main():
    show_header()

    registry = load_registry()

    show_platforms()

    platform = questionary.select(
        "Select platform:",
        choices=list(PLATFORMS.keys())
    ).ask()

    install_mode = questionary.select(
        "Select install mode:",
        choices=[
            "Install One Skill",
            "Install All Skills"
        ]
    ).ask()

    install_path = PLATFORMS[platform]

    console.print()

    if install_mode == "Install One Skill":

        skill_names = [s["name"] for s in registry["skills"]]

        selected_skill = questionary.select(
            "Select skill:",
            choices=skill_names
        ).ask()

        skill = next(
            s for s in registry["skills"]
            if s["name"] == selected_skill
        )

        install_skill(skill, install_path)

    else:

        for skill in registry["skills"]:
            install_skill(skill, install_path)

    console.print(
        Panel.fit(
            "[bold green]Installation Complete[/bold green]",
            border_style="green"
        )
    )


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        console.print("\n[bold red]Installation cancelled.[/bold red]")
