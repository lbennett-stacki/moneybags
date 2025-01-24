import torch.nn as nn


class MLP(nn.Module):
    def __init__(
        self, feature_count: int, hidden_layer_neuron_count: int, output_size: int
    ):
        super(MLP, self).__init__()
        self.fc1 = nn.Linear(feature_count, hidden_layer_neuron_count)
        self.relu = nn.ReLU()
        self.fc2 = nn.Linear(hidden_layer_neuron_count, hidden_layer_neuron_count)
        self.fc3 = nn.Linear(hidden_layer_neuron_count, output_size)
        self.sigmoid = nn.Sigmoid()

    def forward(self, x: int):
        x = self.relu(self.fc1(x))
        x = self.relu(self.fc2(x))
        x = self.sigmoid(self.fc3(x))
        return x
