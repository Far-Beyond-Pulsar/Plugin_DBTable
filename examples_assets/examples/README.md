# Database examples

Read-only SQLite databases for the Table Editor. Save a copy into your project
(for example `content/data/`) to edit it.

| File | Extension | Shows |
|---|---|---|
| `items.db` | `.db` | Items and loot tables: defaults, `NOT NULL`, a foreign key |
| `dialogue.sqlite` | `.sqlite` | Dialogue lines per conversation: a `UNIQUE` pair, nullable columns |
| `scores.sqlite3` | `.sqlite3` | A leaderboard: `REAL`, `INTEGER` and `TEXT` columns and an index |

A database cannot hold comments, so each one has an `_about` table with a note
describing it. The three extensions are the same format; the Table Editor opens
all of them.
