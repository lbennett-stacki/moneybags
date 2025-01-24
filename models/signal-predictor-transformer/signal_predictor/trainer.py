from signal_predictor.transformer import PriceTransformer
import torch
import torch.nn as nn
import matplotlib.pyplot as plt
from signal_predictor.device import device
from signal_predictor.training_data import (
    input_window,
    forecast_horizon,
    get_split_training_data,
)


class Trainer:
    def train(self):
        X_train, y_train, X_valid, y_valid = get_split_training_data()

        self.model = PriceTransformer(
            d_model=64,
            nhead=4,
            num_layers=2,
            dim_feedforward=128,
            dropout=0.1,
            input_window=input_window,
            forecast_horizon=forecast_horizon,
        )

        criterion = nn.MSELoss()
        optimizer = torch.optim.Adam(self.model.parameters(), lr=1e-3)

        self.model.to(device)
        X_train, y_train = X_train.to(device), y_train.to(device)
        X_valid, y_valid = X_valid.to(device), y_valid.to(device)

        epochs = 20
        batch_size = 32

        for epoch in range(epochs):
            self.model.train()

            permutation = torch.randperm(X_train.size(0))
            X_train = X_train[permutation]
            y_train = y_train[permutation]

            epoch_loss = 0.0

            for i in range(0, X_train.size(0), batch_size):
                x_batch = X_train[i : i + batch_size]
                y_batch = y_train[i : i + batch_size]

                optimizer.zero_grad()

                output = self.model(x_batch)
                loss = criterion(output, y_batch)

                loss.backward()
                optimizer.step()

                epoch_loss += loss.item()

            epoch_loss /= X_train.size(0) / batch_size

            self.model.eval()
            with torch.no_grad():
                val_pred = self.model(X_valid)
                val_loss = criterion(val_pred, y_valid).item()

            print(
                f"Epoch [{epoch+1}/{epochs}], Train Loss: {epoch_loss:.6f}, Valid Loss: {val_loss:.6f}"
            )

        self.model.eval()
        with torch.no_grad():
            predictions = self.model(X_valid).cpu().numpy().flatten()
            ground_truth = y_valid.cpu().numpy().flatten()

        plt.figure(figsize=(10, 6))
        plt.plot(range(len(ground_truth)), ground_truth, label="Actual Price")
        plt.plot(
            range(len(predictions)), predictions, label="Predicted Price", alpha=0.7
        )
        plt.title("Transformer-based Crypto Price Prediction (Validation Set)")
        plt.xlabel("Time Step")
        plt.ylabel("Price")
        plt.legend()
        plt.show()
