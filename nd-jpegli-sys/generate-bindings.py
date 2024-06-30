import subprocess
import tomllib

def main():
    with open('generate-bindings.toml', 'rb') as f:
        config = tomllib.load(f)
    
    command = [
        'bindgen',
        'bindgen-wrapper.h',
        '--allowlist-type', 'jpeg_decompress_struct',
        '--allowlist-var', 'JPEG_(SUSPENDED|HEADER_OK|HEADER_TABLES_ONLY)',
        '--newtype-enum', 'J_COLOR_SPACE',
        '--newtype-enum', 'J_DCT_METHOD',
        
        # Bindgen's layout tests are not cross platform.
        '--no-layout-tests',
        
        '-o', 'src/bindings.rs',
        '--',
        '-nostdinc',
        # '-v',
    ]
    
    includes = [
        'bindgen-wrapper-files',
        'libjxl',
        'libjxl/lib',
        'libjxl/third_party',
        'libjxl/third_party/libjpeg-turbo',
    ]
    for include in includes:
        command.append(f'-I{include}')
    for include in config.get('includes', []):
        command.append(f'-I{include}')
    
    subprocess.run(command, check=True)
    
if __name__ == '__main__':
    main()