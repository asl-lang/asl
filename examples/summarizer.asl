---
asl_version: "3.0"
digest: "asl:sha256:15d1a9bc6d4f3698fe0aa2afda2095f1e1d8ef960091db32a5056d2997a2c357"
name: "summarizer"
version: "1.0.0"
description: "Módulo determinístico ASL estruturado para sumarização concisa de texto."
license: "MIT"

interface:
  protocol: "mcp-tool-v1"
  entrypoint: "format_prompt"
  input_schema:
    type: "object"
    additionalProperties: false
    required: ["text", "max_words"]
    properties:
      text:
        type: "string"
        description: "Texto a ser resumido"
      max_words:
        type: "integer"
        description: "Limite de palavras para o resumo"
  output_schema:
    type: "object"
    additionalProperties: false
    required: ["prompt_payload", "constraints_applied"]
    properties:
      prompt_payload:
        type: "string"
      constraints_applied:
        type: "array"
        items:
          type: "string"

capabilities:
  fs:
    confined_read_roots: []
    allow_write: []
  net:
    allow_domains: []
  wasi_components: []

limits:
  max_fuel_opcodes: 500000
  max_heap_kib: 8192
  wall_clock_timeout_ms: 1000
---

# SEÇÃO SEMÂNTICA AI-FIRST (Prefixo Estático Invariante)

## 1. Intent (Diretiva do Módulo)
Formatar e blindar o prompt de sumarização aplicando cotas rígidas de tamanho.

---

```asl
def format_prompt(ctx, input):
    text = input.get("text", "")
    max_words = input.get("max_words", 50)
    payload = "Resuma o seguinte texto em no máximo " + str(max_words) + " palavras:\n\n" + text
    return {
        "prompt_payload": payload,
        "constraints_applied": ["max_words: " + str(max_words)]
    }
```
