# StudyScraper-rs

## Überblick

StudyScraper-rs ist ein Projekt, das entwickelt wurde, um Informationen aus verschiedenen Quellen zu sammeln und zu verarbeiten. Es ist in Rust geschrieben und nutzt Docker für eine einfache Bereitstellung und Ausführung.

## Voraussetzungen

- Docker
- Docker Compose
- Git

## Installation

### Docker und Docker Compose installieren

Falls Docker und Docker Compose noch nicht installiert sind, können Sie diese von der [Docker-Website](https://www.docker.com/get-started) herunterladen und installieren.

### Git installieren

Besuchen Sie die [offizielle Git-Website](https://git-scm.com/) und installieren Sie Git.

## Projekt klonen

Öffnen Sie ein Terminal und führen Sie die folgenden Kommandos aus, um das Repository zu klonen:

```sh
git clone https://github.com/Gobidev/studyscraper-rs.git
cd studyscraper-rs
```
## Docker Compose verwenden

### Docker-Compose-Datei

Erstellen Sie im Projektverzeichnis eine Datei namens `docker-compose.yml` mit folgendem Inhalt:

```yaml
version: '3.8'

services:
  studyscraper:
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "8080:8080"
    volumes:
      - ./config:/app/config
    environment:
      - RUST_LOG=info
    command: ["studyscraper-rs"]
```
## Container erstellen und starten

Um den Container zu erstellen und zu starten, führen Sie im Terminal folgendes Kommando aus:

```sh
docker-compose up --build
```
Dieses Kommando baut das Docker-Image und startet den Container, wobei der Port 8080 vom Host auf den Container weitergeleitet wird.

## Container stoppen
Um den Container zu stoppen, führen Sie folgendes Kommando aus:
```sh
docker-compose down
```

### Anpassung der Konfigurationsdateien

Falls Ihr Projekt spezifische Konfigurationsdateien oder Umgebungsvariablen benötigt, stellen Sie sicher, dass diese im ./config Verzeichnis vorhanden und korrekt konfiguriert sind.

### Fehlerbehebung

Überprüfen Sie die Log-Ausgaben des Containers oder des Rust-Programms, falls es Fehler gibt.
Stellen Sie sicher, dass alle notwendigen Abhängigkeiten und Umgebungsvariablen korrekt gesetzt sind.
Überprüfen Sie die Issues-Seite des Repositories auf GitHub für mögliche Lösungen.
Beiträge

Beiträge sind willkommen! Bitte eröffnen Sie ein Issue oder einen Pull-Request auf GitHub, um Änderungen oder Verbesserungen vorzuschlagen.

### Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert. Weitere Informationen finden Sie in der LICENSE Datei im Repository.
