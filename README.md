# Musical Sound Generator Framework (MSGF) in Rust

This software is released under the MIT License, see LICENSE.txt.

## 外部環境との接続

- cargo build で静的ライブラリが作成されます
- C言語でコールするための msgf.h が用意されています
- 私自身は Xcode で、Swift+ObjectiveC によるMacのコンソールアプリを作成し、そこからこのRustライブラリをコールしています。
    - ご連絡いただければ、アプリ環境についてお知らせいたします。mailto: JCA03205@gmail.com

## IF

- rust_msgf_new() : インスタンスを生成します generate an instance.
- rust_recieve_midi_message() : MIDI受信します receive a midi message.
- rust_process() : Audio 信号を生成し、Audio Buffer を渡す処理です(左右2ch) generate audio signal and send audio buffer to system. 
- rust_msgf_destroy() : インスタンスを解放します release an instance.

## 各フォルダの内容

- /src : IF, configuration etc.
- /src/general: Framework
- /src/engine: Singnal Processing & Cotrol Engine
- /src/app: Soft Synth. Application
    - va: Virtual Analog Tone Generator