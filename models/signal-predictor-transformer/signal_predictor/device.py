import torch

is_gpu_available = torch.cuda.is_available()
device = torch.device("cuda" if is_gpu_available else "cpu")
