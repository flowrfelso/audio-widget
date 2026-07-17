export interface PlaybackState {
  playing: boolean;
}

export interface MediaInfo {
  title: string;
  artist: string;
  source: string;
  playing: boolean;
  canNext: boolean;
  canPrevious: boolean;
}
