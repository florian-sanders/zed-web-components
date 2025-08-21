# Zed Web Components Language Server Extension

## Disclaimer

This extension relies on [the Web Components Language Server](https://github.com/wc-toolkit/wc-language-server) from [Open VSX](https://open-vsx.org/extension/wc-toolkit/web-components-language-server).

## Features

- HTML diagnostics for custom elements and attributes
- Attribute type validation (boolean, number, enum, string)
- Deprecated element and attribute warnings
- Duplicate attribute detection
- Completion and hover support for custom elements
- Works with any Web Components library that provides a Custom Elements Manifest

## How to configure?

### General LSP settings

Settings and configuration tweaks are explained in detail in the [wc-language-server README](https://github.com/wc-toolkit/wc-language-server/blob/main/README.md).

In your global or local settings, enable the language server by adding a `wc-language-server` section in the `lsp` section.

Settings can be passed to the LSP server by adding a `settings` section inside `wc-language-server`.

For instance:
```json
// settings.json
{
  "lsp": {
    "wc-language-server": {
      "settings": {
        "webComponents": {
          "diagnostics": {
            "enabled": true,
            "deprecatedElements": "warning",
            "deprecatedAttributes": "warning",
            "invalidAttributes": "error",
            "duplicateAttributes": "error"
          }
        }
      }
    }
  }
}
```

### Custom Elements Manifest

The language server works with Custom Elements Manifests (CEM) to provide rich diagnostics and completion. You can configure the manifest location in your `wc.config.js` file:

```javascript
// wc.config.js
export default {
  manifests: [
    'custom-elements.json',
    'node_modules/my-components/custom-elements.json'
  ]
}
```

### Supported Languages

The extension provides language server support for:
- **HTML**: Custom element validation and completion
- **JavaScript**: Web Components class definitions
- **TypeScript**: Web Components with type checking

### Framework Support

The language server works with any Web Components framework that generates a Custom Elements Manifest, including:
- Lit
- Stencil
- FAST
- Vanilla Web Components
- And many others

## Configuration Examples

### Basic Configuration

```json
{
  "lsp": {
    "wc-language-server": {
      "settings": {
        "webComponents": {
          "diagnostics": {
            "enabled": true
          }
        }
      }
    }
  }
}
```

### Advanced Configuration

```json
{
  "lsp": {
    "wc-language-server": {
      "settings": {
        "webComponents": {
          "diagnostics": {
            "enabled": true,
            "deprecatedElements": "warning",
            "deprecatedAttributes": "warning",
            "invalidAttributes": "error",
            "duplicateAttributes": "error"
          },
          "completion": {
            "enabled": true,
            "includeDeprecated": false
          }
        }
      }
    }
  }
}
```

### Language-specific Settings

```json
{
  "languages": {
    "HTML": {
      "language_servers": ["wc-language-server"],
      "format_on_save": "on"
    },
    "JavaScript": {
      "language_servers": ["wc-language-server", "typescript-language-server"]
    },
    "TypeScript": {
      "language_servers": ["wc-language-server", "typescript-language-server"]
    }
  }
}
```

## Acknowledgments

This extension code is heavily inspired by the [`stylelint` extension](https://github.com/florian-sanders/zed-stylelint) and the [`html` extension in Zed](https://github.com/zed-industries/zed/tree/main/extensions/html).

The language server functionality comes from the [wc-toolkit/wc-language-server](https://github.com/wc-toolkit/wc-language-server) project, so credit goes to the wc-toolkit team!
