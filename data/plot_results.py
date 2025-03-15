import json
import numpy as np
import matplotlib.pyplot as plt
import os

DATA_DIR = "data/"

# Load JSON data
def load_json(filename):
    with open(os.path.join(DATA_DIR, filename), "r") as file:
        return json.load(file)

# Plot bar chart for premade boards
def plot_premade(data):
    indices = [entry["board_index"] for entry in data]
    times = [entry["execution_time_ms"] for entry in data]

    plt.figure(figsize=(10, 5))
    plt.bar(indices, times, color="blue", alpha=0.7)
    plt.title("Execution Time on Premade Boards")
    plt.xlabel("Board Index")
    plt.ylabel("Execution Time (ms)")
    plt.grid(axis="y", linestyle="--", alpha=0.7)

    plt.savefig(os.path.join(DATA_DIR, "ex_premade_plot.png"))
    plt.close()

# Plot scatter plot for random boards
def plot_random(data):
    indices = [entry["board_index"] for entry in data]
    times = [entry["execution_time_ms"] for entry in data]

    # Compute cumulative average runtime
    avg_times = np.cumsum(times) / np.arange(1, len(times) + 1)

    plt.figure(figsize=(10, 5))
    plt.plot(indices, avg_times, linestyle="-", alpha=0.7)

    plt.title("Average Execution Time Over Simulated Boards")
    plt.xlabel("Number of Boards Simulated")
    plt.ylabel("Average Execution Time (ms)")
    plt.grid(True, linestyle="--", alpha=0.7)

    # Ensure axes start at 0
    plt.xlim(0, max(indices) if indices else 1)
    plt.ylim(0, max(avg_times) if avg_times.any() else 1)

    plt.savefig(os.path.join(DATA_DIR, "ex_random_plot.png"))
    plt.close()

# Run plotting functions
premade_data = load_json("ex_premade.json")
random_data = load_json("ex_random.json")

plot_premade(premade_data)
plot_random(random_data)
