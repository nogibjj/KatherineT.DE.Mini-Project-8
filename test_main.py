import pandas as pd

dataset = "cereal.csv"

def test_max():
    data = pd.read_csv("cereal.csv", sep=";")
    assert data['calories'].max() == 160