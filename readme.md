# mtidy

mtidy (Media Tidier) is a command-line tool written in Rust designed to organize and manage CinemaDNG video files captured by the SIGMA fp camera. It addresses the issue of duplicate folder names created by the camera's naming convention, which assigns the same folder name (e.g., A001_001) each time the SSD is formatted, making it difficult to determine the shooting date and preventing the placement of folders in the same hierarchy. mtidy solves this problem by copying or moving the video files to a structured folder hierarchy based on their timestamps.

## Features

- Recursively searches for CinemaDNG video files in the specified input folder
- Organizes video files into a folder structure based on their timestamps (YYYYMMDD_HHMMSS)
- Copies or moves video files and their parent folders to the output folder
- Handles file conflicts by comparing file sizes and timestamps
- Provides an option to move the files instead of copying them
- Outputs logs for copied, moved, and skipped folders

## Usage

```
mtidy <input_folder> <output_folder> [move]
```

- `<input_folder>`: The path to the folder containing the CinemaDNG video files to be organized.
- `<output_folder>`: The path to the folder where the organized video files will be copied or moved to.
- `[move]` (optional): Specify "move" to move the files instead of copying them. If not provided, the files will be copied by default.

## Examples

Copy CinemaDNG video files from the "input" folder to the "output" folder:
```
mtidy /path/to/input /path/to/output
```

Move CinemaDNG video files from the "input" folder to the "output" folder:
```
mtidy /path/to/input /path/to/output move
```

## Installation

1. Make sure you have Rust installed on your system. If not, you can install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2. Clone the repository:
   ```
   git clone https://github.com/yourusername/mtidy.git
   ```

3. Change to the project directory:
   ```
   cd mtidy
   ```

4. Build the project:
   ```
   cargo build --release
   ```

5. The compiled binary will be available in the `target/release` directory. You can move it to a directory in your system's PATH for easier access.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the GitHub repository.

## License

This project is licensed under the [MIT License](LICENSE).