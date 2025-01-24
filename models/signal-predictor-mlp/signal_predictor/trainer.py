from signal_predictor.hyperparameters import Hyperparameters
from signal_predictor.mlp import MLP
from signal_predictor.predictor import Predictor
from signal_predictor.predictor_input import PredictorInput
from signal_predictor.training_args import TrainingArgs
from signal_predictor.training_data import TrainingData
import torch
import torch.nn as nn
import torch.optim as optim


class Trainer:
    def __init__(self):
        self.training_data = TrainingData()
        self.hyperparameters = Hyperparameters()

    def prepare_data(self):
        features_train, trade_train, features_test, trade_test = (
            self.training_data.prepare()
        )
        self.features_train = features_train
        self.training_args = TrainingArgs(features_train, self.hyperparameters)
        self.trade_train = trade_train
        self.features_test = features_test
        self.trade_test = trade_test

    def prepare(self):
        self.prepare_data()

        self.model = MLP(
            self.training_args.input_size,
            self.training_args.hidden_layer_neuron_count,
            self.training_args.output_size,
        )

        self.criterion = nn.BCELoss()
        self.optimizer = optim.Adam(self.model.parameters(), lr=0.01)

        self.epochs = self.training_args.epochs

    def train(self):
        self.prepare()

        print(f"Training with hyperparameters: {self.hyperparameters.__dict__}")

        for epoch in range(self.epochs):
            outputs = self.model(self.features_train)
            loss = self.criterion(outputs, self.trade_train)

            self.optimizer.zero_grad()
            loss.backward()
            self.optimizer.step()

            if (epoch + 1) % 10 == 0:
                print(f"Epoch [{epoch + 1}/{self.epochs}] >> Loss: {loss.item():.4f}")
        print("\n")

        self.evaluate()
        self.example_inferrence()

    def evaluate(self):
        with torch.no_grad():
            trade_prediction = self.model(self.features_test)
            trade_prediction_class = (trade_prediction > 0.5).float()
            accuracy = (trade_prediction_class == self.trade_test).float().mean()
            print(f"Test Accuracy: {accuracy:.4f}")

    def example_inferrence(self):
        predictor = Predictor(self.model)

        input = PredictorInput(3, 2.5, 35)
        predictor.predict(input)
