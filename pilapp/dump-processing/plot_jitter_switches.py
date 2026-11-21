import matplotlib.pyplot as plt
import numpy as np

plt.style.use('_mpl-gallery')

sampling_frequency = 24_000_000 
pil_frequency = 100
expected = sampling_frequency/pil_frequency
switches = np.loadtxt("archive.txt", skiprows=1)

mean = switches.mean()
print(f"mean:  {mean:.2f} ticks ({mean / sampling_frequency * 1e3:.4f} ms)")
print(f"std:   {switches.std():.2f} ticks ({switches.std() / sampling_frequency * 1e6:.2f} us)")
print(f"expected period: {sampling_frequency / pil_frequency:.0f} ticks ({1e3 / pil_frequency:.2f} ms)")

jitter_us = (switches - expected) / sampling_frequency * 1e6

time_s = np.arange(len(switches)) / pil_frequency

fig, ax = plt.subplots()
ax.plot(time_s, jitter_us)
ax.set_xlabel("Time (s)")
ax.set_ylabel("Deviation from mean period (µs)")
fig.savefig("outarchive.jpg", dpi=800, bbox_inches="tight")
