import subprocess

def main():
    subprocess.run(
        [
            'clang-format',
            '-i',
            'wrapper/*.c',
        ],
        check=True,
    )
    
if __name__ == '__main__':
    main()