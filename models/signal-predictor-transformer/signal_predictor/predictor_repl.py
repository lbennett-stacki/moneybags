from signal_predictor.predictor import Predictor


class PredictorRepl:
    def __init__(self, predictor: Predictor):
        self.predictor = predictor

    def run(self):
        print("wow")
