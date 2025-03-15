import json
import numpy as np
import matplotlib.pyplot as plt
import os

DATA_DIR = "data/"

# Load JSON data
def load_json(filename):
    with open(os.path.join(DATA_DIR, filename), "r") as file:
        return json.load(file)

# Plot bar chart for premade boards (Exhaustive vs Optimized)
def plot_premade(exhaustive_data, optimized_data):
    indices = [entry["board_index"] for entry in exhaustive_data]
    times_ex = [entry["execution_time_ms"] for entry in exhaustive_data]
    times_opt = [entry["execution_time_ms"] for entry in optimized_data]

    plt.figure(figsize=(10, 5))
    width = 0.4  # Bar width

    ex_bars = plt.bar(np.array(indices) - width/2, times_ex, width=width, color="blue", alpha=0.7, label="Exhaustive")
    opt_bars = plt.bar(np.array(indices) + width/2, times_opt, width=width, color="green", alpha=0.7, label="Optimized")

    plt.title("Execution Time on Premade Boards")
    plt.xlabel("Board Index")
    plt.ylabel("Execution Time (ms)")
    plt.xticks(indices)  # Show all indices on the x-axis
    plt.legend()
    plt.grid(axis="y", linestyle="--", alpha=0.7)

    for bar in ex_bars:
        plt.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 7, f"{bar.get_height():.1f} ms", ha='center', fontsize=9, color="blue")
    for bar in opt_bars:
        plt.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 7, f"{bar.get_height():.1f} ms", ha='center', fontsize=9, color="green")

    plt.savefig(os.path.join(DATA_DIR, "ex_premade_comparison.png"))
    plt.close()

# Plot line graph for random boards (Exhaustive vs Optimized)
def plot_random(exhaustive_data, optimized_data):
    indices = [entry["board_index"] for entry in exhaustive_data]
    times_ex = [entry["execution_time_ms"] for entry in exhaustive_data]
    times_opt = [entry["execution_time_ms"] for entry in optimized_data]

    # Compute cumulative average runtime
    avg_times_ex = np.cumsum(times_ex) / np.arange(1, len(times_ex) + 1)
    avg_times_opt = np.cumsum(times_opt) / np.arange(1, len(times_opt) + 1)

    plt.figure(figsize=(10, 5))
    plt.plot(indices, avg_times_ex, linestyle="-", alpha=0.7, label=f"Exhaustive (Final: {avg_times_ex[-1]:.1f} ms)", color="blue")
    plt.plot(indices, avg_times_opt, linestyle="-", alpha=0.7, label=f"Optimized (Final: {avg_times_opt[-1]:.1f} ms)", color="green")

    plt.title("Average Execution Time Over Simulated Boards")
    plt.xlabel("Number of Boards Simulated")
    plt.ylabel("Average Execution Time (ms)")
    plt.legend()
    plt.grid(True, linestyle="--", alpha=0.7)

    # Ensure axes start at 0
    plt.xlim(0, max(indices) if indices else 1)
    plt.ylim(0, max(max(avg_times_ex), max(avg_times_opt)) if indices else 1)

    plt.savefig(os.path.join(DATA_DIR, "ex_random_comparison.png"))
    plt.close()

# Load JSON data
premade_exhaustive = load_json("ex_premade.json")
premade_optimized = load_json("opt_premade.json")
random_exhaustive = load_json("ex_random.json")
random_optimized = load_json("opt_random.json")

# Plot comparisons
plot_premade(premade_exhaustive, premade_optimized)
plot_random(random_exhaustive, random_optimized)
