# Beautify

**Beautify** is a simple Rust-based command-line tool that prettifies and colorizes tabular output from plain-text input. It aligns columns, limits width, allows column selection, and applies color to help visually distinguish columns.

Each column in the output is assigned a distinct, easily distinguishable color, making it visually simple to separate fields at a glance—especially useful when working in large or complex tables.

By default, Beautify treats the first line of input as a header, aligning it separately and excluding it from coloring to retain clarity. However, if your input doesn’t contain headers, you can suppress this behavior using the `--no-header` flag.

You can also control how wide each column can be with the `--max-width` option. This is useful when working with narrow terminals or very long values, as it wraps or truncates content appropriately.

If your data uses unusual delimiters (e.g. multiple spaces, tabs, or custom symbols), the `--separator` option allows you to define a regular expression to accurately split columns. By default, it uses `\s{2,}`, which means two or more spaces between fields.

Sometimes you only need to see specific fields—Beautify lets you select which columns to display using the `--cols` flag. You provide a comma-separated list of 0-indexed column numbers (e.g., `--cols` 0,2,3), and only those will be printed.

<img width="1555" height="571" alt="Image" src="https://github.com/user-attachments/assets/9e0f626b-004c-4d9f-89f6-6cc7764c700f" />