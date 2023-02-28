import pandas as pd

df = pd.read_csv('top_songs.csv')

print(df.head()) 

df.to_csv("top_songs_export.csv")