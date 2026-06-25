### Villainy
Lekki launcher gier dla systemu Linux, który łączy biblioteki Epic Games i GOG w jeden interfejs. Aplikacja oferuje automatyczne śledzenie czasu gry oraz pełną kompatybilność z Proton/Wine.

#### Funkcje
* Zunifikowana biblioteka: Przeglądaj gry z Epic Games i GOG w jednym miejscu.
* Automatyczne śledzenie czasu: Monitorowanie procesów gry i rejestrowanie czasu grania w tle.
* Wsparcie Proton/Wine: Uruchamianie gier z możliwością konfiguracji ścieżki do wybranego środowiska Proton lub Wine.
* Pulpit statystyk: Śledzenie łącznego czasu gry, ocen oraz historii sesji.
* Trwałe przechowywanie: Baza danych SQLite przechowująca ustawienia i statystyki użytkownika.

#### Wymagania 
* System: Linux (testowane na Ubuntu/Fedora/openSUSE)
* Środowisko: Node.js & pnpm
* Budowanie: Rust & Cargo
* Zależności: Python 3.10+, Wine (lub Steam w przypadku korzystania z Protona)

#### Instalacja i konfiguracja
##### 1. Konfiguracja środowiska
Wymagane narzędzia CLI (legendary oraz gogdl). Upewnij się, że masz zainstalowane python3 oraz pip w swoim systemie, a następnie:

```
``` # Instalacja zależności projektu
pipx install legendary-gl &&

pipx install gogdl &&

pnpm install &&

# Budowanie backendu
cd src-tauri && cargo build --release
```

##### 2. Uwierzytelnianie

Zanim aplikacja będzie mogła pobrać Twoją bibliotekę gier, musisz uwierzytelnić dołączone narzędzia:

**Epic Games (legendary):**

```
source ./venv/bin/activate &&
legendary auth
```

**GOG (gogdl):**

```
source ./venv/bin/activate &&
gogdl login
``` 

#### Jak to działa

##### Biblioteka gier
* Pobieranie: Podczas uruchamiania, Villainy wywołuje polecenia legendary i gogdl, aby pobrać listę Twoich gier.
* Status: Automatycznie wykrywa zainstalowane gry, skanując wskazane katalogi.
* Uruchamianie: Aplikacja tworzy proces gry, korzystając ze skonfigurowanej ścieżki do Proton/Wine oraz odpowiednich zmiennych środowiskowych.

##### Śledzenie czasu gry
* Start: Villainy uruchamia proces gry.
* Monitorowanie: Zadanie w tle śledzi identyfikator procesu (PID).
* Logowanie: Co 10 sekund aktywnej rozgrywki czas sesji jest zapisywany w bazie villainy.db.
* Odświeżanie: Statystyki są aktualizowane w interfejsie w czasie rzeczywistym.

#### Konfiguracja

##### Ścieżka Proton
1. Otwórz Ustawienia w aplikacji.
2. Podaj pełną ścieżkę do pliku wykonywalnego Proton (np. ~/.steam/steam/steamapps/common/Proton 9.0/proton).
3. Jeśli ścieżka nie zostanie podana, launcher automatycznie skorzysta z systemowego /usr/bin/7z

#### Rozwiązywanie problemów 
* Gry nie chcą się uruchomić: Jeśli otrzymasz błąd "No such file or directory", upewnij się, że Wine jest zainstalowane (sudo apt install wine) lub podaj poprawną ścieżkę do Protona w ustawieniach.
* Błąd uwierzytelniania: Jeśli biblioteka jest pusta, uruchom ponownie legendary auth lub gogdl login w terminalu.
* Czas gry nie jest rejestrowany: Upewnij się, że w systemie zainstalowano pakiet procps (dla pgrep) -> sudo apt install procps.
