# Minecraft Dependency Manager (MDM)

[RU](#что-надо) ||| [EN](#what-you-need)

### Проект переписывается, смотреть в ветку Rewriting!

# Что надо

1. Загрузка из одного файла
2. Декларативно
3. Не нужно запускать сервер чтобы обновить/загрузить плагины и моды!
4. Есть возможность загружать ядра и обновлять их.
5. Загружать конфигурационных файлов с git

# Что реализуется

- [x] Обновление, загрузка плагинов
- [x] Обновление, загрузка ядра
- [x] Чтение конфигурационных файлов
- [x] Загрузка языкового файла
- [ ] Запуск minecraft
- [ ] Бекапы
- [ ] Загрузка конфигурационных файлов через git
- [ ] REST full API для MDC-hub

# Будут ли добавлены датапаки?

Реализовать можно, но по мери надобности.
Так как датапаки - это специфичный инструмент, который лучше устанавливать в ручную.

# Будут ли добавлена поддержка модов?

Начало есть, но нужно реализовать загрузку ядер.

#

# What you need

1. Upload from a single file
2. Declarative
3. No need to start the server to update/download plugins and mods!
4. It is possible to load kernels and update them.
5. Download configuration files from git

# What is implemented

- [x] Update, load plugins
- [x] Updating, loading kernels
- [x] Reading configuration files
- [x] Load language file
- [ ] Running minecraft
- [ ] Backups
- [ ] Uploading configuration files via git
- [ ] REST full API for MDC-hub

# Will datapacks be added?

It's possible to implement it, but only as needed.
Since datapacks are a specific tool that should be installed manually.

# Will mod support be added?

There is a start, but we need to realize the loading of cores.

# Source:

## Plugins:

- Modrinth
- Hangar (not implemented)

## Mods

- Modrinth (soon)
- CurseForge (not implemented)

## Cores

- Vanilla
- PaperMC
- Folia
- PurpurMC
- Fabric (not implemented)
- Forge (not implemented, no api)
- NeoForge (not implemented, no api)
- Velocity
- Waterflow
