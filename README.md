# Random Picker

## Overview

A simple and efficient web service implemented in Rust using the [Axum](https://github.com/tokio-rs/axum) framework. This allows you to pick a random item from a given list of options provided through the query parameters. The result is displayed as a plain HTML page.

## Features

- **Random Selection**: Picks a random item from a provided list.
- **Error Handling**: Gracefully handles cases where no options are provided.
- **Simple Usage**: Easy-to-use URL query parameter for specifying options.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.50.0 or later)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/yourusername/random-item-picker-api.git
    cd random-item-picker-api
    ```

2. Install dependencies:

    ```bash
    cargo build
    ```

### Running the Server

1. Start the server:

    ```bash
    cargo run
    ```

2. The server will be running on `http://localhost:48012`.

## Usage

To use the Random Item Picker API, open a web browser and navigate to:

```txt
http://localhost:48012?options=foo,bar,baz
```

Replace `foo,bar,baz` with your own comma-separated list of options. The API will pick a random item from the list and display it on the page.

### Example

Request:

```txt
http://localhost:48012?options=apple,banana,orange
```

Response:

```html
<h1>Picked Item: banana</h1>
```
