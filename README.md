# Musical Sound Generator Framework (MSGF) in Rust

This software is released under the MIT License, see LICENSE.txt.

## 外部環境との接続

- cargo build で静的ライブラリが作成されます
- C言語でコールするための msgf.h が用意されています

## IF

- rust_msgf_new() : インスタンスを生成します
- rust_recieve_midi_message() : MIDI受信します
- rust_process() : Audio Buffer を渡す処理です(左右2ch)
- rust_msgf_destroy() : インスタンスを解放します
