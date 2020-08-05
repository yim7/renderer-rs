import sys
import os
from PIL import Image


def convert_to_guaimage(path):
    im = Image.open(path)
    w, h = im.size
    lines = []
    for i in range(h):
        pixels = []
        for j in range(w):
            r, g, b, a = im.getpixel((j, h - i - 1))
            pixel = (r << 24) + (g << 16) + (b << 8) + a
            # print(r, g, b, a, pixel)
            # s = ' '.join(str(c) for c in pixel)
            pixels.append(str(pixel))
        row = ' '.join(pixels)
        lines.append(row)
    lines = '\n'.join(lines)

    content = f'guaimage\n1.0\n{w}\n{h}\n{lines}'

    name, _ = os.path.splitext(path)
    new_name = f'{name}.iamge'
    with open(new_name, 'w') as f:
        f.write(content)
        print(f'Convert {name} to {new_name}!')


def main():
    images = sys.argv[1:]
    for path in images:
        convert_to_guaimage(path)


if __name__ == "__main__":
    main()
