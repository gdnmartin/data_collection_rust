import pandas as pd
import matplotlib.pyplot as plt
from collections import defaultdict

df = pd.read_csv("metrics.csv")

df['cpu_total'] = df[[f'cpu{i}_usage' for i in range(8)]].mean(axis=1)

plt.figure(figsize=(10, 6))
plt.plot(df['cpu_total'], label='CPU Total (%)')
plt.plot(df['cpu_temperature'], label='Temperatura CPU (°C)')
plt.title('Uso de CPU vs Temperatura')
plt.xlabel('Índice temporal')
plt.ylabel('Valor')
plt.legend()
plt.tight_layout()
plt.show()

plt.figure(figsize=(10, 6))
plt.plot(df['memory_used_kb'] / 1024 / 1024, label='RAM usada (GB)')
plt.plot(df['cache_used_kb'] / 1024 / 1024, label='Cache (GB)')
plt.plot(df['swap_used_kb'] / 1024 / 1024, label='Swap (GB)')
plt.title('Uso de Memoria')
plt.xlabel('Índice temporal')
plt.ylabel('GB')
plt.legend()
plt.tight_layout()
plt.show()

plt.figure(figsize=(10, 6))
plt.plot(df['cpu_total'], label='CPU Total (%)')
plt.plot(df['connections'], label='Conexiones')
plt.title('CPU vs Conexiones de Red')
plt.xlabel('Índice temporal')
plt.ylabel('Valor')
plt.legend()
plt.tight_layout()
plt.show()

plt.figure(figsize=(10, 6))
plt.plot(df['disk_reads'], label='Lecturas')
plt.plot(df['disk_writes'], label='Escrituras')
plt.title('Actividad de Disco')
plt.xlabel('Índice temporal')
plt.ylabel('Operaciones')
plt.legend()
plt.tight_layout()
plt.show()

name_cols = [f'top_process{i}_name' for i in range(5)]
cpu_cols = [f'top_process{i}_cpu' for i in range(5)]

process_time_series = defaultdict(lambda: [0] * len(df))

for idx, row in df.iterrows():
    for i in range(5):
        name = row[f'top_process{i}_name']
        cpu = row[f'top_process{i}_cpu']
        if pd.notna(name):
            process_time_series[name][idx] += cpu

process_df = pd.DataFrame(process_time_series)

top_processes = process_df.sum().sort_values(ascending=False).head(10).index

plt.figure(figsize=(12, 6))
for proc in top_processes:
    plt.plot(process_df[proc], label=proc)

plt.title('Evolución del uso de CPU de los procesos principales')
plt.xlabel('Índice temporal')
plt.ylabel('Uso de CPU (%)')
plt.legend()
plt.tight_layout()
plt.show()

process_data = []
for i in range(5):
    names = df[f'top_process{i}_name']
    cpu = df[f'top_process{i}_cpu']
    for name, usage in zip(names, cpu):
        process_data.append((name, usage))

process_df = pd.DataFrame(process_data, columns=['name', 'cpu_usage'])
avg_cpu_by_process = process_df.groupby('name').cpu_usage.mean().sort_values(ascending=False).head(10)

avg_cpu_by_process.plot(kind='bar', figsize=(10, 6))
plt.title('Promedio de uso de CPU por nombre de proceso')
plt.xlabel('Proceso')
plt.ylabel('Uso de CPU (%)')
plt.xticks(rotation=45)
plt.tight_layout()
plt.show()
