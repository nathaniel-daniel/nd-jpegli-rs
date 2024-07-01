import subprocess

def main():
    subprocess.run(
        [
            'clang-format',
            '-i',
            '*.c',
            '*.h',
        ],
        check=True,
    )
    
if __name__ == '__main__':
    main()