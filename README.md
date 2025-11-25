# YouTube Video Downloader

A simple script to download YouTube videos and playlists using `yt-dlp`.

## Installation

1. **Install Python dependencies:**
   ```bash
   pip install yt-dlp
   ```

2. **Make the scripts executable:**
   ```bash
   chmod +x youtube_downloader.py
   chmod +x ytdownload.sh
   ```

3. **Optional: Install FFmpeg for audio conversion:**
   ```bash
   brew install ffmpeg
   ```

## Usage

### Using the Python script directly:

```bash
# Download a video
python3 youtube_downloader.py "https://www.youtube.com/watch?v=VIDEO_ID"

# Download audio only (MP3)
python3 youtube_downloader.py -a "https://www.youtube.com/watch?v=VIDEO_ID"

# Download to specific directory
python3 youtube_downloader.py -o ~/Downloads/YouTube "https://www.youtube.com/watch?v=VIDEO_ID"

# Download playlist
python3 youtube_downloader.py -p "https://www.youtube.com/playlist?list=PLAYLIST_ID"

# Get video information without downloading
python3 youtube_downloader.py -i "https://www.youtube.com/watch?v=VIDEO_ID"

# Download specific quality
python3 youtube_downloader.py -q "720p" "https://www.youtube.com/watch?v=VIDEO_ID"
```

### Using the bash wrapper:

```bash
# Download a video
./ytdownload.sh "https://www.youtube.com/watch?v=VIDEO_ID"

# Download audio only
./ytdownload.sh -a "https://www.youtube.com/watch?v=VIDEO_ID"

# Download playlist
./ytdownload.sh -p "https://www.youtube.com/playlist?list=PLAYLIST_ID"
```

## Options

- `-o, --output`: Output directory (default: ./downloads)
- `-q, --quality`: Video quality (default: best)
- `-a, --audio-only`: Download audio only (MP3)
- `-p, --playlist`: Download as playlist
- `-i, --info`: Show video information only (no download)

## Examples

```bash
# Download a single video in best quality
./ytdownload.sh "https://www.youtube.com/watch?v=dQw4w9WgXcQ"

# Download audio only to Music folder
./ytdownload.sh -a -o ~/Music "https://www.youtube.com/watch?v=dQw4w9WgXcQ"

# Download entire playlist
./ytdownload.sh -p "https://www.youtube.com/playlist?list=PLrAXtmRdnEQy6numJRVe9YVzdxgPr-2xF"

# Get video info before downloading
./ytdownload.sh -i "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

## Notes

- Videos are downloaded to a `downloads` folder in the current directory by default
- For playlists, videos are organized in subfolders by playlist name
- The script requires `yt-dlp` to be installed via pip
- For audio conversion, `FFmpeg` is required (install with `brew install ffmpeg`)
- Always respect YouTube's Terms of Service and copyright laws
