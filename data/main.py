import pandas as pd
import matplotlib.pyplot as plt

# Cargar CSV
df = pd.read_csv("metrics.csv")

# Extraer nombres y CPU en forma larga
process_data = []
for i in range(5):
    names = df[f'top_process{i}_name']
    cpu = df[f'top_process{i}_cpu']
    for name, usage in zip(names, cpu):
        process_data.append((name, usage))

# Crear DataFrame y agrupar por nombre
process_df = pd.DataFrame(process_data, columns=['name', 'cpu_usage'])
avg_cpu_by_process = process_df.groupby('name').cpu_usage.mean().sort_values(ascending=False).head(5)

# Graficar
avg_cpu_by_process.plot(kind='bar', figsize=(10, 6))
plt.title('Promedio de uso de CPU por nombre de proceso')
plt.xlabel('Proceso')
plt.ylabel('Uso de CPU (%)')
plt.xticks(rotation=45)
plt.tight_layout()
plt.show()
