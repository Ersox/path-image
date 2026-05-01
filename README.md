# PathImage

A lightweight Rust library that pairs image data with its source path, enabling seamless serialization and deserialization with automatic disk loading.

## Overview

`PathImage<T>` wraps an image value alongside its source path. When serialized, only the path is stored (reducing payload size). When deserialized, the image is automatically loaded from disk.