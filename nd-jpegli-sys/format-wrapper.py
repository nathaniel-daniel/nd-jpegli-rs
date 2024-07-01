import subprocess

def main():
    subprocess.run(
        [
            'clang-format',
            '-i',
            'wrapper/*.c',
            'wrapper/*.h',
        ],
        check=True,
    )
    
if __name__ == '__main__':
    main()