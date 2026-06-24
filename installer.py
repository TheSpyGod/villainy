#!/usr/bin/env python3
import subprocess
import sys
import venv
import os
from pathlib import Path

class PythonCLIInstaller:
    """Installs Python CLI tools safely and reproducibly"""
    
    def __init__(self, venv_path="./venv"):
        self.venv_path = Path(venv_path)
        self.python_exe = self.venv_path / "bin" / "python"
        self.pip_exe = self.venv_path / "bin" / "pip"
    
    def setup_venv(self):
        """Creates isolated Python environment — NO SUDO NEEDED"""
        print(f"Creating virtual environment at {self.venv_path}...")
        
        try:
            venv.create(self.venv_path, with_pip=True)
            print("✓ Virtual environment created")
            return True
        except Exception as e:
            print(f"✗ Failed to create venv: {e}")
            return False
    
    def install_package(self, package_name):
        """Installs package via pip inside venv"""
        print(f"Installing {package_name}...")
        
        result = subprocess.run(
            [str(self.pip_exe), "install", package_name],
            capture_output=True,
            text=True,
            timeout=300
        )
        
        if result.returncode != 0:
            print(f"✗ Installation failed:\n{result.stderr}")
            return False
        
        print(f"✓ {package_name} installed")
        return True
    
    def verify_cli(self, cli_name):
        """Checks if CLI command exists"""
        # Look in venv bin directory
        cli_path = self.venv_path / "bin" / cli_name
        if cli_path.exists():
            print(f"✓ CLI found at {cli_path}")
            return True
        
        print(f"✗ CLI {cli_name} not found")
        return False
    
    def get_activation_script(self):
        """Returns the activation command user needs to run"""
        activate = self.venv_path / "bin" / "activate"
        return f"source {activate}"


if __name__ == "__main__":
    installer = PythonCLIInstaller()
    
    # Step 1: Create venv (ONE TIME)
    if not installer.venv_path.exists():
        installer.setup_venv()
    
    # Step 2: Install the CLI tool
    installer.install_package("legendary-gl")  # or whatever CLI you need
    
    # Step 3: Verify
    installer.verify_cli("legendary")
    
    # Step 4: Tell user how to activate
    print(f"\nTo use the CLI, run:\n{installer.get_activation_script()}")
