## Schedule と Priority

このドキュメントでは、エンジンで使用している `Schedule`、`Stage`、`Priority` のモデルを説明します。

### 概要

- `Stage` は実行の粗い位相（フェーズ）を表します。現在のステージは次のとおりです: `Startup`, `ProcessInput`, `Update`, `FixedUpdate`, `PreRender`, `Render`, `LateUpdate`。

- `Schedule` は各 `Stage` ごとにシステムを保持し、さらに各ステージ内で優先度バケット（小さな配列）でグルーピングします。優先度インデックスが小さいほど先に実行されます。

- `Priority` は呼び出し箇所にマジックナンバーを置かないための小さな列挙型です。内部でバケットインデックスにマップされます。可読性向上のために enum（例: `Priority::High`）の使用を推奨します。

- `MAX_PRIORITY` は安全のための上限で、これを超える優先度はクランプされ警告がログに出ます（`log::warn!`）。

### API

- `App::add_system(stage, priority, system)` — 指定した `stage` に対して、与えられた優先度でシステムを登録します。`priority` は `usize` または `Priority` を受け付けます。

- `App::add_event(event, update_stage, priority)` — `Events<T>` リソースを登録し、その `update()` を `update_stage` の指定した優先度で実行するようにスケジュールします。

注: API は優先度を明示的に渡す設計です。暗黙の実行順やマジックナンバーを避けるために、`Priority::{Highest, High, Normal, Low, Lowest}` を使ってください。

### 例

```rust
app.add_system(Stage::ProcessInput, Priority::High, input_system);
app.add_event(Events::<KeyboardInputEvent>::new(), Stage::LateUpdate, Priority::Normal);
```
