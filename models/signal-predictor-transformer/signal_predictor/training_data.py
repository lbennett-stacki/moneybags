import torch
import numpy as np

torch.manual_seed(42)
np.random.seed(42)

input_window = 30
forecast_horizon = 1


def generate_sine_wave_data(seq_length=1000, freq=0.1, amplitude=1.0):
    x = np.arange(seq_length)
    y = amplitude * np.sin(2 * np.pi * freq * x) + np.random.normal(0, 0.1, seq_length)
    return y


def create_sequences(data, input_window, forecast_horizon):
    X, y = [], []
    for i in range(len(data) - input_window - forecast_horizon + 1):
        X.append(data[i : i + input_window])
        y.append(data[i + input_window : i + input_window + forecast_horizon])
    return torch.tensor(X, dtype=torch.float32), torch.tensor(y, dtype=torch.float32)


def get_training_data_sine():
    time_series = generate_sine_wave_data(seq_length=2000, freq=0.01, amplitude=1.0)
    X_all, y_all = create_sequences(time_series, input_window, forecast_horizon)

    return X_all, y_all


def get_training_data():
    return X_all, y_all


def get_split_training_data():
    X_all, y_all = get_training_data()

    train_size = int(len(X_all) * 0.8)
    X_train, y_train = X_all[:train_size], y_all[:train_size]
    X_valid, y_valid = X_all[train_size:], y_all[train_size:]

    print(f"Train set: X={X_train.shape}, y={y_train.shape}")
    print(f"Valid set: X={X_valid.shape}, y={y_valid.shape}")

    return X_train, y_train, X_valid, y_valid
