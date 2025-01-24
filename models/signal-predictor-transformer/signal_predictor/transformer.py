import torch
import torch.nn as nn


class PriceTransformer(nn.Module):
    def __init__(
        self,
        d_model=64,
        nhead=4,
        num_layers=2,
        dim_feedforward=128,
        dropout=0.1,
        input_window=30,
        forecast_horizon=1,
    ):
        super(PriceTransformer, self).__init__()

        self.d_model = d_model
        self.input_window = input_window
        self.forecast_horizon = forecast_horizon

        self.positional_encoding = nn.Parameter(torch.zeros(1, input_window, d_model))

        self.embedding = nn.Linear(1, d_model)

        encoder_layer = nn.TransformerEncoderLayer(
            d_model=d_model,
            nhead=nhead,
            dim_feedforward=dim_feedforward,
            dropout=dropout,
            batch_first=True,
        )
        self.transformer_encoder = nn.TransformerEncoder(
            encoder_layer, num_layers=num_layers
        )

        self.fc_out = nn.Linear(d_model, forecast_horizon)

    def forward(self, x):
        x = x.unsqueeze(-1)

        x = self.embedding(x)

        x = x + self.positional_encoding[:, : self.input_window, :]

        x = self.transformer_encoder(x)

        x = x[:, -1, :]

        out = self.fc_out(x)

        return out
