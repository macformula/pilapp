import matplotlib.pyplot as plt
import numpy as np

plt.style.use('_mpl-gallery')

chunk_size = -1
sampling_frequency = 1000_000
pil_frequency = 100
with open("default.bin", "rb") as file:
    switches = []
    data = file.read(None if chunk_size == -1 else chunk_size)
    #print(data)
    #for i in range(0, 0 if chunk_size == -1 else chunk_size):
    #   print(data[i])
        
    #print("DONE", len(data))
    for i in range(0, len(data)-2, 2):
        if data[i+2] != data[i]:
            switches.append(i)
    #print(switches)

    # find jitter
    jitter = [0] * len(switches)
    # the first should be assumed to be at zero
    jitter_offset = int(sampling_frequency/pil_frequency)*2 - switches[0]
    jitter[0] = 0
    for i in range(1, len(switches)):
        # times 2 to account for the fact that our values are 2 long. 
        jitter[i] = int(sampling_frequency/pil_frequency)*(i+1)*2 - switches[i] - jitter_offset
    for i in range(10):
        print("Switch ", i, switches[i], jitter[i])

    time = np.linspace(0, len(data)/2, len(switches))
    plt.plot(time, jitter)
    plt.savefig("out.jpg", dpi=300, bbox_inches="tight")
    plt.show()
with open("out.csv", "w") as file:
    file.write("\n".join(str(x) for x in jitter))