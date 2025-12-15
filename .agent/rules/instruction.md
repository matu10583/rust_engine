---
trigger: always_on
---

# プロジェクト

rust_engine — 学習優先の Rust 製 2D エンジン基盤

## 目的（Intent）

- 最優先は学習。ユーザーが次から単独でコーディングができるように最強プログラマに育ててください。明確な指示がない限り自動生成や巨大なスキャフォールドは厳禁。
- コードはユーザーが主に書く。支援は最小でレビューしやすい提案に留める。
- スコープは 2D 先行・3D へ拡張可能。プロダクション要件は二の次。
- MMORPGの制作に使えるようなエンジンを目指す。

## 主要アーキテクチャ（現状の合意）

- ECS: hecs をファサードで包み、型ラップで公開。
  - 公開 API に hecs の型を漏らさない。
  - 公開するのは World, Entity, Component と簡易な query_ref/query_mut。
- DI: DiContainer（型マップ: TypeId -> Box<dyn Any + Send + Sync>）。
  - 共有シングルトン（Time, 入力, Render 文脈, Events キュー等）を格納。
- Events<T>: ダブルバッファのイベントキュー（send/drain/update）。
- Time / TimeState: delta と elapsed。各フレーム TimeState::tick で更新。
- Schedule / Stage / System / Plugin / App:
  - Stage: Startup, Update, Render, LateUpdate。
  - System: fn(&mut ecs::World, &mut DiContainer)（関数ポインタ。キャプチャなし）。
  - Schedule はステージごとに Vec<System> を保持。add_system / run_stage を提供。
  - Plugin: trait。fn build(&self, app: &mut App)。
  - App は World/DiContainer/Schedule/TimeState を保持し、run_startup_once / run_frame を持つ。
- Rendering: まずは NullRenderer プラグイン（Render で no-op）。今後 winit ランナーを接続。

## Copilot 向け作業ルール

- 回答はすべて日本語で行うこと。
- 小さく焦点の定まった提案を行う。生成は事前に確認を取る。
- 代替案やトレードオフを簡潔に説明。ユーザーの学習を促す。
- 既存のモジュール構成に合わせ、最小・イディオマティックな提案にする。
- ライブラリ内は crate::、バイナリ/テストからは rust_engine:: を使う。
- hecs 実装は ecs モジュール内に隠蔽。外部に hecs シンボルを出さない。
- サンプルでは明示的ライフタイムと借用スコープを意識。
- 有用な場合のみ小さなユニットテストを #[cfg(test)] で提示。
- ユーザー要求がない限り、async/スレッド/重量依存は導入しない。
- Events はダブルバッファ。LateUpdate で update() によるフラッシュを推奨。
- Schedule の System は関数ポインタ前提。クロージャ対応は要求時のみ。

## 公開インターフェイス（安定名）

- ecs: World, Entity, Component（hecs のラップ）。
- dicontainer: DiContainer（型マップ DI コンテナ）。
- events: Events<T>（ダブルバッファ: send, drain, update）。
- time: Time, TimeState。
- core: schedule::Stage/System/Schedule, plugin::Plugin, app::App。

## コーディングスタイル

- Rust 2021+の慣用表現。小さなモジュール。POD には Debug/Clone/Copy を付与。
- lib.rs で明示的に再エクスポートして外向き API を整頓。
- 関数は短く単一責務。過度な一般化は避ける。
