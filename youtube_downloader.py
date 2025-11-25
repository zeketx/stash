#!/usr/bin/env python3
"""
YouTube Video Downloader Script
A simple script to download YouTube videos with a nice terminal UI
"""

import sys
import os
import argparse
import time
from pathlib import Path

try:
    import yt_dlp
except ImportError:
    print("yt-dlp not found. Please install it with: pip install yt-dlp")
    sys.exit(1)


def print_logo():
    """Print a simple logo/title"""
    print("\n" + "=" * 50)
    print("🎥  YOUTUBE DOWNLOADER  📥")
    print("=" * 50)


def download_video(url, output_dir="./downloads", quality="best", audio_only=False):
    """
    Download a YouTube video
    
    Args:
        url (str): YouTube video URL
        output_dir (str): Directory to save the downloaded video
        quality (str): Video quality preference
        audio_only (bool): Download audio only
    """
    
    # Create output directory if it doesn't exist
    Path(output_dir).mkdir(parents=True, exist_ok=True)
    
    # Configure yt-dlp options
    ydl_opts = {
        'outtmpl': f'{output_dir}/%(title)s.%(ext)s',
        'format': 'bestaudio/best' if audio_only else quality,
        'http_headers': {
            'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36'
        },
        'extractor_retries': 3,
    }
    
    # Try to use Chrome cookies if available
    try:
        ydl_opts['cookiesfrombrowser'] = ('chrome', None, None, None)
    except Exception:
        # If cookies fail, continue without them
        pass
    
    if audio_only:
        ydl_opts['postprocessors'] = [{
            'key': 'FFmpegExtractAudio',
            'preferredcodec': 'mp3',
            'preferredquality': '192',
        }]
    
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            print(f"Downloading: {url}")
            ydl.download([url])
            print("Download completed successfully!")
            
    except Exception as e:
        print(f"Error downloading video: {str(e)}")
        return False
    
    return True


def download_playlist(url, output_dir="./downloads", quality="best", audio_only=False):
    """
    Download a YouTube playlist
    
    Args:
        url (str): YouTube playlist URL
        output_dir (str): Directory to save the downloaded videos
        quality (str): Video quality preference
        audio_only (bool): Download audio only
    """
    
    # Create output directory if it doesn't exist
    Path(output_dir).mkdir(parents=True, exist_ok=True)
    
    # Configure yt-dlp options for playlist
    ydl_opts = {
        'outtmpl': f'{output_dir}/%(playlist_title)s/%(playlist_index)s - %(title)s.%(ext)s',
        'format': 'bestaudio/best' if audio_only else quality,
        'http_headers': {
            'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36'
        },
        'extractor_retries': 3,
    }
    
    # Try to use Chrome cookies if available
    try:
        ydl_opts['cookiesfrombrowser'] = ('chrome', None, None, None)
    except Exception:
        # If cookies fail, continue without them
        pass
    
    if audio_only:
        ydl_opts['postprocessors'] = [{
            'key': 'FFmpegExtractAudio',
            'preferredcodec': 'mp3',
            'preferredquality': '192',
        }]
    
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            print(f"Downloading playlist: {url}")
            ydl.download([url])
            print("Playlist download completed successfully!")
            
    except Exception as e:
        print(f"Error downloading playlist: {str(e)}")
        return False
    
    return True


def get_video_info(url):
    """
    Get information about a YouTube video
    
    Args:
        url (str): YouTube video URL
    """
    
    ydl_opts = {
        'quiet': True,
        'no_warnings': True,
    }
    
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=False)
            
            print(f"Title: {info.get('title', 'N/A')}")
            print(f"Uploader: {info.get('uploader', 'N/A')}")
            print(f"Duration: {info.get('duration', 'N/A')} seconds")
            print(f"View count: {info.get('view_count', 'N/A')}")
            print(f"Upload date: {info.get('upload_date', 'N/A')}")
            
            # Available formats
            formats = info.get('formats', [])
            print(f"\nAvailable formats:")
            for f in formats[-10:]:  # Show last 10 formats
                print(f"  {f.get('format_id', 'N/A')} - {f.get('format', 'N/A')}")
                
    except Exception as e:
        print(f"Error getting video info: {str(e)}")


def main():
    # Show logo at start
    print_logo()
    
    parser = argparse.ArgumentParser(description='Download YouTube videos')
    parser.add_argument('url', help='YouTube video or playlist URL')
    parser.add_argument('-o', '--output', default='./downloads', 
                       help='Output directory (default: ./downloads)')
    parser.add_argument('-q', '--quality', default='best',
                       help='Video quality (default: best)')
    parser.add_argument('-a', '--audio-only', action='store_true',
                       help='Download audio only (MP3)')
    parser.add_argument('-p', '--playlist', action='store_true',
                       help='Download as playlist')
    parser.add_argument('-i', '--info', action='store_true',
                       help='Show video information only (no download)')
    
    args = parser.parse_args()
    
    if args.info:
        get_video_info(args.url)
    elif args.playlist:
        download_playlist(args.url, args.output, args.quality, args.audio_only)
    else:
        download_video(args.url, args.output, args.quality, args.audio_only)


if __name__ == "__main__":
    main()
