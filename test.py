import video_filename_parser
result = video_filename_parser.parse("[TaigaSubs]_Toradora!_(2008)_-_01v2_-_Tiger_and_Dragon_[1920x1080_H.265_FLAC][1234ABCD].mkv")
print(result.videoCodec)
