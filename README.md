# RsHtml for Zed

This extension provides official language server support for the [RsHtml](https://github.com/rshtml/rshtml) templating engine in Zed.

## Features

*   **Language Server:** Integrates the RsHtml language server to provide rich language features.
*   **Syntax Highlighting:** Basic syntax highlighting for `.rs.html` files.
*   **Cross-Platform Support:** The language server works seamlessly on Windows, macOS, and Linux.

## Overview

This extension is a lightweight wrapper that automatically starts the RsHtml language server for you, enabling a smoother development experience when working with RsHtml templates.

The extension first checks if `rshtml-analyzer` is available in your system's PATH. If it is, that version will be used. If not, the extension will download and manage its own copy internally. You can override this internal version at any time by installing `rshtml-analyzer` globally, which the extension will then prefer.

If you want to install the `rshtml-analyzer` yourself, you can do so with the following command:
```bash
cargo install --git https://github.com/rshtml/rshtml-analyzer.git --tag v0.1.3
```
*Note: The tag can be updated according to the version.*

## Getting Started

1.  Install the extension from the Zed Marketplace.
2.  Open a project containing `.rs.html` files.
3.  The extension will automatically activate and provide language support.

---

**Enjoy working with RsHtml!**
