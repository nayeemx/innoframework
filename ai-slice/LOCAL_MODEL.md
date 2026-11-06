# Local Model Setup (llama.cpp)

## Available models (Ollama manifests + GGUF blobs)

| Manifest | Model | Blob size | Best for |
|---|---|---|---|
| gemma4/e4b | Gemma 4 4B | ~9.6GB | General purpose, larger context |
| llama3.2/3b | Llama 3.2 3B | ~2GB | Fast, lightweight demos (default) |
| hermes3/latest | Hermes 3 | ~4.66GB | Instruction following, tool use |

## Server binary
`D:\models\llama.cpp\b9142\llama-server.exe`

## Start the server
```powershell
# Default: llama3.2-3b on port 11434
node scripts/start-llama-server.js

# Custom model
LLAMA_MODEL="D:\models\blobs\sha256-4c27e0f5b5adf02ac956c7322bd2ee7636fe3f45a8512c9aba5385242cb6e09a" \
node scripts/start-llama-server.js

# Stop
node scripts/stop-llama-server.js
```

## Verify
```powershell
curl http://127.0.0.1:11434/v1/models
```

## Provider
`LlamaCppProvider` in `src/llama.ts` connects to `http://127.0.0.1:11434/v1/chat/completions`
and implements the `ModelProvider` interface used by `Agent`.

## Notes
- The model must be loaded into RAM (~2GB for llama3.2-3b, ~9.6GB for gemma4-e4b).
- First request may take several seconds while the model initializes.
- Tool calling support depends on the llama-server build; if native tool calling
  is not available, the agent falls back to parsing JSON tool calls from text.
