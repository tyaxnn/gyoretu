### 概要
pngの連番画像を読み込んで、フィルターをかけるアプリケーションです。shader toyのように pixel 単位で制御したフィルターを自由に作る事ができます。申しわけ程度のGUIもあります。

![sample_0001](./assets/screenshots/sample_0001.png)

### 追加したい機能

- [ ] フィルターの追加
- [ ] GUIの強化（FPSの表示、連番画像をGUIから開く、など）
- [ ] ファイルの書き出し
- [ ] wavやキーボードをフィルターの入力とする
- [ ] 複数の連番画像への対応

### 使用した他人のコード

ライセンスの書き方が分からないので、参照したチュートリアルやサンプルコードのリンクを貼っておきます。

https://sotrh.github.io/learn-wgpu/ wgpuの使い方が分かります。

https://github.com/gfx-rs/wgpu/tree/trunk/examples wgpuのdemoがあります。

https://github.com/googlefonts/compute-shader-101 wgpuでcompute shaderを用いた分かりやすい例（copy.wgslのコードを借用しています）。

https://www.egui.rs/ eguiのデモ。

https://github.com/ejb004/egui-wgpu-demo egui-wgpuのデモ（gui関連のコードを借用しています）。

