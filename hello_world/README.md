// パッケージの初期化
npm init -y 

// 開発機能でタイプスクリプトを追加
npm install -D typescript

// 依存パッケージを追加
npm install react react-dom react-scripts
npm install @types/react @types/react-dom

// タイプスクリプトの設定ファイル tsconfig.json を初期化
npx tsc --init

// tsconfig.json に ↓ を追加
// "jsx": "react-jsx"