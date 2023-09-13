import argparse
from io import StringIO
import os

import numpy as np
import matplotlib.pyplot as plt


def read_pipe_as_string_io() -> StringIO:
    content = list()

    while True:
        try:
            content.append(input())
        except:
            break;

    return StringIO('\n'.join(content))

def load_simulation(file_path: str = '') -> tuple[np.ndarray, np.ndarray]:
    file = file_path if file_path else read_pipe_as_string_io()

    data = np.loadtxt(file, delimiter=',').T

    steps = data[0]
    simulation = data[[ i for i in range(1, data.shape[0]) ]]

    return steps, simulation

def b_plot(simulation_data: tuple[np.ndarray, np.ndarray], output_file: str):
    steps, simulation = simulation_data

    fig, ax = plt.subplots()

    fig.set_size_inches(8, 6)

    ax.grid()

    ax.set(
        title="Modelo de Competição Inibidora",
        xlabel="Tempo",
        ylabel="Quantidades",
    )

    labels = [ 'S', 'E', 'I', "ES", "EI", 'P' ]

    for i, y in enumerate(simulation):
        ax.plot(steps, y, label=labels[i])

    fig.legend()

    fig.savefig(output_file)

def plot(simulation_data: tuple[np.ndarray, np.ndarray], output_file: str):
    steps, simulation = simulation_data

    components = [ 'S', 'E', 'I', "ES", "EI", 'P' ]
    colors = [ 'b', 'g', 'r', 'c', 'm', 'y' ]

    for i, component in enumerate(components):
        fig, ax = plt.subplots()

        fig.set_size_inches(4, 6)

        ax.grid()

        ax.set(
            title="Inhibitive Competition",
            xlabel="Time (Days)",
            ylabel=f"[ {component} ]",
        )

        ax.plot(steps, simulation[i], color=colors[i])

        dir = os.path.dirname(output_file)
        file = component + "_" + os.path.basename(output_file)

        plot_file = os.path.join(dir, file)

        fig.savefig(plot_file)


def main(output_file, csv_file: str = ''):
    plot(
        simulation_data=load_simulation(csv_file),
        output_file=output_file,
    )

if __name__ == "__main__":
    parser = argparse.ArgumentParser()

    parser.add_argument("--output-file", type=str, required=True)
    parser.add_argument("--input-file", type=str, default='')

    args = parser.parse_args()

    dirs = os.path.dirname(args.output_file)
    if dirs:
        os.makedirs(dirs, exist_ok=True)

    main(
        csv_file=args.input_file,
        output_file=args.output_file
    )

