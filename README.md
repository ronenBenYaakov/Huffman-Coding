# Huffman Coding File Compressor and Decompressor

This project implements a file compression and decompression tool using Huffman coding in Rust. The tool reduces the size of a file by encoding it into a more efficient binary format based on the frequency of characters in the file. The decompressor can later reconstruct the original file using the encoded data.

## Features

- **Compression**: Efficiently compresses files by converting characters to variable-length binary codes based on their frequencies.
- **Decompression**: Restores the original file from the compressed format using the stored Huffman tree.
- **Rust Implementation**: Written in Rust for fast performance and memory efficiency.
- **Command Line Interface**: A simple CLI for both compression and decompression operations.

## Prerequisites

- Rust 1.XX or higher
- Cargo (Rust’s package manager)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/ronenBenYaakov/huffman-coding.git
   cd huffman-coding
