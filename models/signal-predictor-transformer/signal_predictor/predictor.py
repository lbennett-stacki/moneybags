import torch


class Predictor:
    def __init__(self, model):
        self.model = model

    def predict(self):
        with torch.no_grad():
            print("Predicting trade for input: wow")
            prediction = self.model(4)
            print(f"Predicted trade: {prediction.item():.4f}")
