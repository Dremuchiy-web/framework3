# Cassiopeia ISS Data Collection System

Распределенный монолит для сбора и визуализации космических данных.

## Структура проекта

- `rust_iss` - Rust бэкенд (Axum + SQLx)
- `php_web` - Laravel фронтенд
- `pascal_legacy` - Legacy модуль для генерации CSV/XLSX
- `nginx` - Reverse proxy
- `iss_db` - PostgreSQL база данных
- `redis_cache` - Redis кэш

## Запуск проекта

```bash
docker-compose up -d
```

## API Endpoints

### ISS
- `GET /api/iss` - Получить все данные ISS
- `GET /api/iss/latest` - Получить последние данные ISS
- `POST /api/iss/fetch` - Загрузить данные из API

### OSDR
- `GET /api/osdr` - Получить все данные OSDR
- `POST /api/osdr/fetch` - Загрузить данные из API

### JWST
- `GET /api/jwst` - Получить все данные JWST
- `POST /api/jwst/fetch` - Загрузить данные из API

### Astronomy
- `GET /api/astronomy` - Получить все данные Astronomy
- `POST /api/astronomy/fetch` - Загрузить данные из API

### Cache
- `GET /api/cache/:key` - Получить значение из кэша
- `POST /api/cache/:key` - Установить значение в кэш

## Фронтенд

Доступен по адресу: http://localhost

- `/` - Главная страница (Dashboard)
- `/iss` - Страница ISS данных
- `/osdr` - Страница OSDR данных
- `/jwst` - Страница JWST данных
- `/astronomy` - Страница Astronomy данных

## Особенности

- Rate-Limit на основе tower-governor
- Redis кэширование
- Валидация данных отдельными классами
- Фильтрация и поиск на фронтенде
- Анимации и CSS визуализация
- Генерация CSV и XLSX файлов

## Переменные окружения

Создайте файл `.env` в корне проекта:

```
DATABASE_URL=postgresql://cassiopeia:cassiopeia_pass@iss_db:5432/iss_data
REDIS_URL=redis://redis_cache:6379
ISS_API_KEY=your_key
NASA_API_KEY=your_key
OSDR_API_KEY=your_key
JWST_API_KEY=your_key
ASTRONOMY_API_KEY=your_key
```

