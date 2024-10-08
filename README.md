### 概要
pngの連番画像を読み込んで、フィルターをかけるアプリケーションです。shader toyのように pixel 単位で制御したフィルターを自由に作る事ができます。申しわけ程度のGUIもあります。

![sample_0001](./assets/screenshots/sample_0006.png)

### road map

- [x] GUIを用いて、ソースとなる連番画像をlocal fileから選択できるようにする。
- [x] 複数の連番画像をソースとできるようにする。
- [x] 実行環境を保存できるようにする。
- [ ] モード（乗算・オーバレーイ等）の追加
- [ ] フィルターの追加
- [x] フレームをレンダリングするGUI及び機能の追加
- [x] FPSなどのフレームの情報をGUIで確認できるようにする。
- [ ] 音源をソースとして再生可能にする
- [x] フィルターの入力を動的に変化できるようにする。
- [x] プリコンポーズ機能の追加

### 使用した他人のコード

参照したチュートリアルやサンプルコードのリンクを貼っておきます。

https://sotrh.github.io/learn-wgpu/ wgpuの使い方が分かります。

https://github.com/gfx-rs/wgpu/tree/trunk/examples wgpuのdemoがあります。

https://github.com/googlefonts/compute-shader-101 wgpuでcompute shaderを用いた分かりやすい例（copy.wgslのコードを借用しています）。

https://www.egui.rs/ eguiのデモ。

https://github.com/ejb004/egui-wgpu-demo egui-wgpuのデモ（gui関連のコードを借用しています）。

