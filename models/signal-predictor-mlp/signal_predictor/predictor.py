from signal_predictor.predictor_input import PredictorInput
import torch


class Predictor:
    def __init__(self, model):
        self.model = model

    def predict(self, input: PredictorInput):
        with torch.no_grad():
            print(f"Predicting trade for input: {input.__dict__}")
            prediction = self.model(input.to_tensor())
            print(f"Predicted trade: {prediction.item():.4f}")
