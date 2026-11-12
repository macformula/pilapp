import matplotlib.pyplot as plt
import numpy as np

plt.style.use('_mpl-gallery')

chunk_size = -1
sampling_frequency = 500_000
pil_frequency = 100
bytes_per_sample = 2

period_samples = sampling_frequency / pil_frequency
period_bytes = period_samples * bytes_per_sample

with open("default.bin", "rb") as file:
    switches = []
    data = file.read(None if chunk_size == -1 else chunk_size)
    #print(data)
    #for i in range(0, 0 if chunk_size == -1 else chunk_size):
    #   print(data[i])

    #print("DONE", len(data))
    for i in range(0, len(data)-bytes_per_sample, bytes_per_sample):
        if data[i+bytes_per_sample] != data[i]:
            switches.append(i)
    #print(switches)

    jitter = [0.0] * len(switches)
    for i in range(1, len(switches)):
        n = round((switches[i] - switches[0]) / period_bytes)
        jitter[i] = n * period_bytes - (switches[i] - switches[0])

    switch_times = np.array(switches) / bytes_per_sample / sampling_frequency
    jitter_s = np.array(jitter) / bytes_per_sample / sampling_frequency
    jitter_us = jitter_s * 1e6

    slope_us_per_s, intercept_us = np.polyfit(switch_times, jitter_us, 1) 
    trend_us = slope_us_per_s * switch_times + intercept_us
    detrended_us = jitter_us - trend_us
    detrended_s = detrended_us * 1e-6

    print("Drift (linear fit slope): %.3f ppm" % slope_us_per_s)
    print("Detrended jitter: min %.1f us, max %.1f us, std %.2f us" % (
        detrended_us.min(), detrended_us.max(), detrended_us.std()))
    
    plt.plot(switch_times, detrended_us)
    plt.xlabel("time since start of capture (s)")
    plt.ylabel("jitter after linear fit removed (µs)")
    plt.figure(figsize=(10, 5))
    plt.plot(switch_times, detrended_us)
    plt.savefig("out.jpg", dpi=800, bbox_inches="tight")
    plt.show()

with open("out.csv", "w") as file:
    file.write("time_s,jitter_s,detrended_jitter_s\n")
    for t, j, d in zip(switch_times, jitter_s, detrended_s):
        file.write(f"{t},{j},{d}\n")