import subprocess
from pathlib import Path
import struct

cwd = Path(r'E:\ClaudeProjects\AI-Native-Student-Execution-OS')
icon_dir = cwd / 'src-tauri' / 'icons'
icon_dir.mkdir(parents=True, exist_ok=True)
icon_path = icon_dir / 'icon.ico'
if not icon_path.exists():
    w, h = 16, 16
    pixels = bytes([255, 0, 0, 255]) * w * h
    info_header = struct.pack('<IIIHHIIIIII', 40, w, h * 2, 1, 32, 0, len(pixels), 0, 0, 0, 0)
    row_bytes = ((w + 31) // 32) * 4
    mask = bytes(row_bytes * h)
    image_data = info_header + pixels + mask
    header = struct.pack('<HHH', 0, 1, 1)
    entry = struct.pack('<BBBBHHII', w, h, 0, 0, 1, 32, len(image_data), 6 + 16)
    with open(icon_path, 'wb') as f:
        f.write(header + entry + image_data)

result = subprocess.run([
    'cargo',
    'build',
    '--manifest-path',
    'src-tauri/Cargo.toml'
], cwd=cwd, capture_output=True, text=True)
output_file = cwd / 'build_result.txt'
output_file.write_text(
    f'WROTE_ICON={icon_path.exists()}\nICON_PATH={icon_path}\nRETURN_CODE={result.returncode}\nSTDOUT=\n{result.stdout}\nSTDERR=\n{result.stderr}\n',
    encoding='utf-8'
)
print('WROTE', output_file)
