import matplotlib.pyplot as plt
import numpy as np
import sys

sampling_frequency = 24_000_000
pil_frequency = 100
center_mean = True
expected = sampling_frequency / pil_frequency
path = sys.argv[1]
switches = np.loadtxt(path, skiprows=1)

mean = switches.mean()
print(f"mean:  {mean:.2f} ticks ({mean / sampling_frequency * 1e3:.4f} ms)")
print(f"std:   {switches.std():.2f} ticks ({switches.std() / sampling_frequency * 1e6:.2f} us)")
print(f"expected period: {expected:.0f} ticks ({1e3 / pil_frequency:.2f} ms)")

ref = mean if center_mean else expected
jitter_ticks = switches - ref
jitter_us = jitter_ticks / sampling_frequency * 1e6

abs_dev_ticks = np.abs(jitter_ticks)
p99_ticks = np.percentile(abs_dev_ticks, 99)
p99_us = p99_ticks / sampling_frequency * 1e6

print("Max deviation:", abs_dev_ticks.max())
print(f"p99 deviation: {p99_ticks:.2f} ticks ({p99_us:.2f} us)")

time_s = np.arange(len(switches)) / pil_frequency

fig, ax = plt.subplots()
ax.plot(time_s, jitter_us)
ax.axhline(p99_us, linestyle="--", linewidth=1, label=f"p99 |dev| = {p99_us:.2f} µs")
ax.axhline(-p99_us, linestyle="--", linewidth=1)
ax.set_xlabel("Time (s)")
ax.set_ylabel("Deviation from mean period (µs)" if center_mean else "Jitter µs")
ax.legend()
fig.savefig("outarchive.jpg", dpi=800, bbox_inches="tight")