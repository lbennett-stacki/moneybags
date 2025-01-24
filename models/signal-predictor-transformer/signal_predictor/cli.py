from signal_predictor.predictor import Predictor
from signal_predictor.predictor_repl import PredictorRepl
from signal_predictor.trainer import Trainer
import argparse


class Cli:
    def parse_args(self):
        parser = argparse.ArgumentParser(
            description="Train and use signal predictor transformer models"
        )
        parser.add_argument(
            "--train",
            action="store_true",
            help="Train models",
        )
        parser.add_argument(
            "--repl",
            action="store_true",
            help="Start a CLI inference REPL",
        )
        args = parser.parse_args()

        return args

    def run(self):
        args = self.parse_args()

        trainer = Trainer()

        if args.train:
            trainer.train()

        if args.repl:
            predictor = Predictor(trainer.model)
            repl = PredictorRepl(predictor)
            repl.run()
