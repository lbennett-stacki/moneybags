import torch


class PredictorInput:
    def __init__(self, time: int, momentum: float, mentions: int):
        self.time = time
        self.momentum = momentum
        self.mentions = mentions

    def to_tensor(self):
        return torch.tensor(
            [[self.time, self.momentum, self.mentions]], dtype=torch.float32
        )
