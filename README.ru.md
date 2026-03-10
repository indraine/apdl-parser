# apdl-parser

Небольшая библиотека на Rust для парсинга текстовых листингов из **ANSYS APDL** (вывод команд вроде `NLIST`, `ELIST`, `DLIST`, `PRNSOL`).

Проект полезен, когда нужно **автоматически вытащить сетку/результаты из текстового вывода** APDL и дальше обработать это в своём пайплайне (CSV/JSON, постпроцессинг, проверки качества, интеграция с Python и т.д.).

## Что умеет

- Парсит строки данных (whitespace-separated) в типизированные структуры:
  - `Nlist`: координаты узлов (`NLIST`)
  - `Elist`: информация об элементах и их узлах (`ELIST`)
  - `Dlist`: табличные данные вида `node label real imag` (`DLIST`)
  - `Prnsol`: пример результатов по узлам (`PRNSOL`, сейчас `NODE TEMP`)
- Имеет общий хелпер `get_list()` для чтения файла и парсинга в `Vec<T>`.

## Типовые сценарии применения

- **Экспорт сетки** из APDL в свой формат (узлы/элементы) для дальнейшей геометрической обработки.
- **Проверки качества модели** (поиск “дыр” в нумерации, контроль координат/материалов/типов элементов).
- **Постпроцессинг результатов**: собрать результаты из `PRNSOL`/`DLIST` и построить графики/отчёты.
- **Интеграция в CI/пайплайны расчётов**: сравнение результатов между версиями модели, регресс-тесты.

## Установка

Добавьте зависимость в `Cargo.toml`:

```toml
[dependencies]
apdl-parser = { git = "https://github.com/<you>/<repo>" }
```

После публикации в crates.io можно будет заменить на обычную версию.

## Использование

### Пример: прочитать `NLIST` из файла

```rust
use std::path::Path;

use apdl_parser::{Nlist, get_list};

fn main() -> anyhow::Result<()> {
    let nodes: Vec<Nlist> = get_list(Path::new("NLIST.txt"))?;
    println!("nodes: {}", nodes.len());
    println!("{nodes:#?}");
    Ok(())
}
```

### Примеры для файлов из репозитория (`files/*.lis`)

В репозитории есть папка `files/` с примерами листингов. Их можно прогнать через готовые примеры:

```bash
cargo run --example parse_nlist
cargo run --example parse_elist
cargo run --example parse_dlist
cargo run --example parse_prnsol
```

### Пример: парсинг одной строки

```rust
use apdl_parser::Elist;

fn main() -> anyhow::Result<()> {
    let e: Elist = "1  1  1  0  0  0  10  20  30".parse()?;
    println!("{e:?}");
    Ok(())
}
```

## Поддерживаемые типы

- `Nlist` — `src/nlist.rs`
- `Elist` — `src/elist.rs`
- `Dlist` — `src/dlist.rs`
- `Prnsol` — `src/prnsol.rs`

## Roadmap (идеи)

- Больше вариантов `PRNSOL` (не только `TEMP`).
- Болеe строгая валидация формата и сообщения об ошибках.
- Экспорт в CSV/JSON (в отдельном crate, чтобы не тащить зависимости в core).
- Опциональная поддержка `f64`.

## Лицензия

Двойная лицензия: **MIT OR Apache-2.0**. См. `LICENSE-MIT` и `LICENSE-APACHE`.
