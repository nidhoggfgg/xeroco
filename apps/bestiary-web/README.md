# Bestiary Web

静态宠物图鉴页面，直接读取 `generated/` 目录下的导出资源。

## 生成数据

```bash
cargo run -p bestiary-export
```

## 本地预览

需要通过静态文件服务器打开，不能直接双击 `index.html`，例如：

```bash
cd apps/bestiary-web
python3 -m http.server 4173
```

然后访问 [http://localhost:4173](http://localhost:4173)。
