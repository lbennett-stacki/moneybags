import torch
import pandas as pd


class TrainingData:
    def prepare(self):
        train = pd.read_csv("./data/train.csv")
        test = pd.read_csv("./data/test.csv")

        split_data = self.split(train, test)

        return self.tensorize(*split_data)

    def split(self, train, test):
        features_train = train.iloc[:, :-1].values
        trade_train = train.iloc[:, -1].values

        features_test = test.iloc[:, :-1].values
        trade_test = test.iloc[:, -1].values

        return features_train, trade_train, features_test, trade_test

    def tensorize(self, features_train, trade_train, features_test, trade_test):
        features_train = torch.tensor(features_train, dtype=torch.float32)
        features_test = torch.tensor(features_test, dtype=torch.float32)

        trade_train = torch.tensor(trade_train, dtype=torch.float32).unsqueeze(1)
        trade_test = torch.tensor(trade_test, dtype=torch.float32).unsqueeze(1)

        return features_train, trade_train, features_test, trade_test
