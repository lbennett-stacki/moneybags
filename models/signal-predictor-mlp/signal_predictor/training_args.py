from signal_predictor.hyperparameters import Hyperparameters


class TrainingArgs:
    def __init__(self, features_train, hyperparameters: Hyperparameters):
        self.input_size = features_train.shape[1]
        self.output_size = 1

        self.hidden_layer_neuron_count = hyperparameters.hidden_layer_neuron_count
        self.epochs = hyperparameters.epochs
