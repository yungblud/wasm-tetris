# wasm-tetris

```sh
wasm-tetris/
├── src/                # Rust 소스 코드
│   ├── lib.rs          # WebAssembly 모듈 (Rust)
│   └── (추가 코드)
├── pkg/                # wasm-pack이 생성한 바이너리 (자동 생성됨)
├── static/             # 정적 파일 (HTML, JS, CSS 등)
│   ├── index.html      # 브라우저 실행용 HTML
│   ├── index.js        # WebAssembly 로드 및 WebGL 관리
│   └── (추가 파일 가능)
├── Cargo.toml          # Rust 프로젝트 설정
```

## 📢 실행 방법
1. wasm-pack build --target web 실행 (pkg 폴더 생성됨)
2. 로컬 서버 실행 (Python이나 Node.js 사용 가능)
```sh
cd static
python3 -m http.server 8080
```
3. 브라우저에서 http://localhost:8080/ 열기
