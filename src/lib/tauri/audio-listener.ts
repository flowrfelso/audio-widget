import { setVolume } from "@/stores/volume";
import { listen } from "@tauri-apps/api/event";

// import { setAudio } from "@/stores/audio";

export async function startAudioListener() {
  return listen<{
    volume: number;

    muted: boolean;
  }>("audio://changed", ({ payload }) => {
    setVolume({
      volume: payload.volume,
      muted: payload.muted,
    });
  });
}
