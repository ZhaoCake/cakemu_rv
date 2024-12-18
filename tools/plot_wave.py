#!/usr/bin/env python3
import matplotlib.pyplot as plt
import numpy as np

def plot_wave(filename='wave.txt'):
    # 读取数据
    with open(filename, 'r') as f:
        data = [float(line.strip()) for line in f]
    
    # 创建时间轴
    t = np.arange(len(data)) / 1000.0  # 采样率 1kHz
    
    # 绘制波形
    plt.figure(figsize=(12, 6))
    plt.plot(t, data)
    plt.grid(True)
    plt.xlabel('Time (s)')
    plt.ylabel('Amplitude')
    plt.title('Wave Generator Output')
    plt.show()

if __name__ == '__main__':
    plot_wave() 