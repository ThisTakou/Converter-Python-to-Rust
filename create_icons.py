#!/usr/bin/env python3
"""Generate minimal valid ICO file without PIL dependency"""
import struct
import os

def create_minimal_ico(output_path, size=32):
    """Create a minimal valid .ico file (monochrome bitmap)"""

    # ICO header
    ico_header = struct.pack('<HHH', 0, 1, 1)  # Reserved, Type (1=icon), Count

    # Icon directory entry
    bmp_size = size * size // 8 + size * size // 8  # XOR mask + AND mask
    header_size = 40  # BITMAPINFOHEADER size
    total_size = header_size + bmp_size

    dir_entry = struct.pack('<BBBBHHII',
        size,           # Width
        size,           # Height
        0,              # Color count (0 = more than 256)
        0,              # Reserved
        1,              # Color planes
        1,              # Bits per pixel
        total_size,     # Size of image data
        22              # Offset to image data (6 + 16)
    )

    # BITMAPINFOHEADER
    bmp_header = struct.pack('<IIIHHIIIIII',
        40,             # Header size
        size,           # Width
        size * 2,       # Height (doubled for icon)
        1,              # Planes
        1,              # Bits per pixel
        0,              # Compression
        bmp_size,       # Image size
        0,              # X pixels per meter
        0,              # Y pixels per meter
        0,              # Colors used
        0               # Important colors
    )

    # Create simple pattern (diagonal lines)
    xor_mask = bytearray(size * size // 8)
    and_mask = bytearray(size * size // 8)

    for y in range(size):
        for x in range(size):
            byte_pos = y * (size // 8) + x // 8
            bit_pos = 7 - (x % 8)

            # Create a simple pattern
            if (x + y) % 4 == 0:
                xor_mask[byte_pos] |= (1 << bit_pos)

    # Write file
    with open(output_path, 'wb') as f:
        f.write(ico_header)
        f.write(dir_entry)
        f.write(bmp_header)
        f.write(xor_mask)
        f.write(and_mask)

    print(f"Created minimal valid ICO: {output_path}")

def create_png_fallback(output_path, size=256):
    """Create a minimal PNG file (single color)"""
    # Minimal PNG: 1x1 pixel, blue
    png_data = bytes([
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,  # PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,  # IHDR chunk
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,  # 1x1 size
        0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53,
        0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41,  # IDAT chunk
        0x54, 0x08, 0xD7, 0x63, 0x60, 0xA0, 0x0E, 0x00,  # Blue pixel data
        0x00, 0x02, 0x00, 0x01, 0xE2, 0x21, 0xBC, 0x33,
        0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44,  # IEND chunk
        0xAE, 0x42, 0x60, 0x82
    ])

    with open(output_path, 'wb') as f:
        f.write(png_data)

    print(f"Created minimal PNG: {output_path}")

# Create icons directory
icons_dir = "py2rs/src-tauri/icons"
os.makedirs(icons_dir, exist_ok=True)

# Generate valid icons
print("Creating valid icon files...")
create_minimal_ico(f"{icons_dir}/icon.ico", 32)
create_png_fallback(f"{icons_dir}/32x32.png")
create_png_fallback(f"{icons_dir}/128x128.png")
create_png_fallback(f"{icons_dir}/icon.png")
create_png_fallback(f"{icons_dir}/icon@2x.png")

print("\nValid icon files created successfully!")
print("Note: These are minimal placeholders. For production, use proper icon design tools.")
