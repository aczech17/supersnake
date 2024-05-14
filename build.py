import os
import shutil
import subprocess
import platform
import zipfile
import toml

def build_game():
    # Wykonaj cargo build --release
    subprocess.run(["cargo", "build", "--release"], check=True)

def get_game_version():
    # Odczytaj wersję gry z pliku Cargo.toml
    with open("Cargo.toml", "r") as f:
        cargo_toml = toml.load(f)
        version = cargo_toml["package"]["version"]
        return version.strip('"')

def create_zip_filename():
    # Wygeneruj nazwę pliku ZIP zgodnie z wymaganiami
    version = get_game_version()
    architecture = platform.machine()
    os_name = platform.system().lower()

    zip_filename = f"supersnake_{version}_{architecture}_{os_name}.zip"
    return zip_filename

def create_zip_archive(zip_filename):
    # Utwórz archiwum ZIP i dodaj do niego pliki
    with zipfile.ZipFile(zip_filename, "w") as zipf:
        # Dodaj plik wykonywalny
        if os.path.exists("target/release/supersnake"):
            zipf.write("target/release/supersnake", "supersnake")
        elif os.path.exists("target/release/supersnake.exe"):
            zipf.write("target/release/supersnake.exe", "supersnake.exe")

        # Dodaj plik konfiguracyjny
        if os.path.exists("config.json"):
            zipf.write("config.json")

        # Dodaj katalog assets i jego zawartość
        if os.path.exists("assets"):
            shutil.make_archive("assets", 'zip', "assets")
            with zipfile.ZipFile("assets.zip") as assets_zip:
                for file in assets_zip.namelist():
                    zipf.writestr(os.path.join("assets", file), assets_zip.read(file))

            # Usuń tymczasowe archiwum katalogu assets
            os.remove("assets.zip")

def main():
    build_game()
    zip_filename = create_zip_filename()
    create_zip_archive(zip_filename)
    print(f"Created {zip_filename}")

if __name__ == "__main__":
    main()
