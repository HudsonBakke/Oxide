"""Ten-second positional demo; optionally pass a WAV/MP3 path on the command line."""
import math
import sys
import time
from oxide import Audio, AudioListener, AudioSettings, AudioSource, ContentManager, Angle, Vec3


def main():
    audio = ContentManager().load(Audio, sys.argv[1]) if len(sys.argv)>1 else Audio.from_samples(
        [math.sin(math.tau*440*i/48000)*0.15 for i in range(48000)]
    )
    with audio, AudioListener() as listener, AudioSettings(looping=True, max_distance=20) as settings:
        listener.activate()
        with AudioSource(Vec3(0,0,-2), settings, Vec3(0,0,1)) as source:
            source.play(audio)
            print("Headphones: moving source. First 5 seconds omni, then a rotating cone. Live low-pass and modulation follow.")
            for frame in range(200):
                t=frame*0.05
                source.position=Vec3(math.sin(t)*3,0,-2)
                source.direction=Vec3(math.sin(t*2),0,math.cos(t*2))
                if frame==60:
                    settings.low_pass_cutoff=400
                    settings.pitch=0.75
                    print("Low-pass 400 Hz, pitch 0.75")
                if frame==100:
                    source.play_directional(audio,Angle.degrees(15),Angle.degrees(60),0.05)
                if frame==130:
                    settings.low_pass_cutoff=24000
                    settings.modulation_frequency=6
                    settings.modulation_depth=0.8
                    print("Modulation 6 Hz")
                _=source.state
                time.sleep(0.05)
            source.pause()
            time.sleep(0.25)
            source.resume()
            time.sleep(0.25)
            source.stop()


if __name__ == "__main__":
    main()
