import regex
import os
import shutil
import subprocess

JPEGLI_SOURCES_PATTERN = regex.compile("libjxl_jpegli_sources.*?=.*?\\[(.*?)\\]", regex.MULTILINE | regex.DOTALL)
LIBJXL_BASE_SOURCES_PATTERN = regex.compile("libjxl_base_sources.*?=.*?\\[(.*?)\\]", regex.MULTILINE | regex.DOTALL)
LIBJXL_PUBLIC_HEADERS_PATTERN = regex.compile("libjxl_public_headers.*?=.*?\\[(.*?)\\]", regex.MULTILINE | regex.DOTALL)
INCLUDE_PATTERN = regex.compile('^#include ["<](.*)[">]')
DEFINE_HWY_TARGET_INCLUDE_PATTERN = regex.compile('^#define HWY_TARGET_INCLUDE "(.*)"')

def parse_lib_gni_string_array(array):
    array = ''.join(line for line in array.split('\n') if not line.strip().startswith('#'))
    entries = []
    for entry in array.split(','):
        entry = entry.strip()
        if len(entry) == 0:
            continue
        assert entry[0] == '"' and entry[-1] == '"'
        
        entries.append(entry[1:-1])
    return entries
    
def extract_gni_string_table(text, name):
    PATTERN = regex.compile(f'{name}.*?=.*?\\[(.*?)\\]', regex.MULTILINE | regex.DOTALL)
    match = PATTERN.search(text)
    group = match.group(1)
    return parse_lib_gni_string_array(group)

def main():
    out_dir = 'fake-jpegli'
    out_include_dir = os.path.join(out_dir, 'include')
    out_include_jpegli_dir = os.path.join(out_include_dir, 'jpegli')
    out_include_libjxl_dir = os.path.join(out_include_dir, 'jxl')
    out_include_libjpeg_turbo_dir = os.path.join(out_include_dir, 'libjpeg-turbo')
    out_include_hwy_dir = os.path.join(out_include_dir, 'hwy')
    out_src_jpegli_dir = os.path.join(out_dir, 'jpegli')
    out_src_hwy_dir = os.path.join(out_dir, 'hwy')
    libjxl_libdir = os.path.join('libjxl', 'lib')
    highway_dir = os.path.join('libjxl', 'third_party', 'highway')
    
    if os.path.exists(out_dir):
        shutil.rmtree(out_dir) 
    os.makedirs(out_dir)
    os.makedirs(out_include_dir)
    os.makedirs(out_include_jpegli_dir)
    os.makedirs(out_include_libjxl_dir)
    os.makedirs(out_include_libjpeg_turbo_dir)
    os.makedirs(out_include_hwy_dir)
    os.makedirs(out_src_jpegli_dir)
    os.makedirs(out_src_hwy_dir)
    
    with open(os.path.join(libjxl_libdir, 'lib.gni'), 'r', encoding = 'utf-8') as f:
        lib_gni = f.read()
    
    match = JPEGLI_SOURCES_PATTERN.search(lib_gni)
    group = match.group(1)
    libjxl_jpegli_sources = parse_lib_gni_string_array(group)
    
    match = LIBJXL_BASE_SOURCES_PATTERN.search(lib_gni)
    group = match.group(1)
    libjxl_base_sources = parse_lib_gni_string_array(group)
    
    match = LIBJXL_PUBLIC_HEADERS_PATTERN.search(lib_gni)
    group = match.group(1)
    libjxl_public_headers = parse_lib_gni_string_array(group)
    
    with open(os.path.join(highway_dir, 'hwy.gni'), 'r', encoding='utf-8') as f:
        hwy_gni = f.read()
        
    hwy_public = extract_gni_string_table(hwy_gni, 'hwy_public')
    for file in hwy_public:
        file = file.replace('$_hwy', 'hwy')
        input_path = os.path.join(highway_dir, file)
        output_path = os.path.join(out_include_dir, file)
        
        output_dir_name = os.path.dirname(output_path)
        if not os.path.exists(output_dir_name):
            os.makedirs(output_dir_name)
        
        shutil.copy(input_path, output_path)
        
    hwy_sources = extract_gni_string_table(hwy_gni, 'hwy_sources')
    
    libjpeg_turbo_copy = [
        ('libjxl/third_party/libjpeg-turbo/jpeglib.h', 'include/libjpeg-turbo/jpeglib.h'),
        ('libjxl/third_party/libjpeg-turbo/jmorecfg.h', 'include/libjpeg-turbo/jmorecfg.h'),
    ]
    for input_path, output_path in libjpeg_turbo_copy:
        output_path = os.path.join(out_dir, output_path)
        shutil.copy(input_path, output_path)
        
    hwy_copy = [
        ('libjxl/third_party/highway/hwy/ops/tuple-inl.h', 'include/hwy/ops/tuple-inl.h'),
    ]
    for input_path, output_path in hwy_copy:
        output_path = os.path.join(out_dir, output_path)
        shutil.copy(input_path, output_path)
       
    undef_keys = {
        'C_ARITH_CODING_SUPPORTED',
        'D_ARITH_CODING_SUPPORTED',
        'WITH_SIMD',
        'RIGHT_SHIFT_IS_UNSIGNED',
    }
    def_keys = {
        'MEM_SRCDST_SUPPORTED'
    }
    with open('libjxl/third_party/libjpeg-turbo/jconfig.h.in', 'r', encoding = 'utf-8') as input_file, open(os.path.join(out_dir, 'include/libjpeg-turbo/jconfig.h'), 'w', encoding = 'utf-8') as output_file:
        data = input_file.read()
        data = data.replace('@JPEG_LIB_VERSION@', '80')
        data = data.replace('@VERSION@', '2.1.5')
        data = data.replace('@LIBJPEG_TURBO_VERSION_NUMBER@', '2001005')
        data = data.replace('@BITS_IN_JSAMPLE@', '8')
        for line in data.split('\n'):
            if line.startswith('#cmakedefine '):
                trimmed = line.removeprefix('#cmakedefine ')
                key, value = trimmed.split(' ', 2)
                if key in undef_keys:
                    output_file.write(f'/* #undef {key} */\n')
                    continue
                elif key in def_keys:
                    output_file.write(f'#define {key} 1\n')
                    continue
                else:
                    raise RuntimeError(f'Unknown Key \"{key}\"')
        
            output_file.write(line + '\n')

    for file in libjxl_base_sources:
        _, extension = os.path.splitext(file)
        
        input_path = os.path.join(libjxl_libdir, file)
        output_path = out_dir
        
        if extension == '.h':
            output_path = os.path.join(output_path, 'include')
            output_path = os.path.join(output_path, 'jxl')
            if file.startswith('jxl/base') or file.startswith('jxl\\base'):
                output_path = os.path.join(output_path, 'base')
                if not os.path.exists(output_path):
                    os.makedirs(output_path)
        elif extension == '.cc':
            raise RuntimeError('WIP: Allocate dir for libjxl')
            # output_path = os.path.join(output_path, 'src')
        output_path = os.path.join(output_path, os.path.basename(file))
        
        with open(input_path, 'r', encoding = 'utf8') as input_file, open(output_path, 'w', encoding = 'utf8') as output_file:
            for line in input_file:
                output_file.write(rewrite_line(line))
                
    for file in libjxl_public_headers:
        input_path = os.path.join(libjxl_libdir, file)
        output_path = os.path.join(out_dir, 'include', 'jxl', os.path.basename(file))
        with open(input_path, 'r', encoding = 'utf8') as input_file, open(output_path, 'w', encoding = 'utf8') as output_file:
            for line in input_file:
                output_file.write(rewrite_line(line))
    
    src_files = []
    for file in libjxl_jpegli_sources:
        _, extension = os.path.splitext(file)
        
        input_path = os.path.join(libjxl_libdir, file)
        output_path = out_dir
        
        if extension == '.h':
            output_path = os.path.join(os.path.join(output_path, 'include'), 'jpegli')
        elif extension == '.cc':
            output_path = os.path.join(output_path, 'jpegli')
        output_path = os.path.join(output_path, os.path.basename(file))
        
        if extension == '.cc':
            src_files.append(output_path)
        
        with open(input_path, 'r', encoding = 'utf8') as input_file, open(output_path, 'w', encoding = 'utf8') as output_file:
            for line in input_file:
                output_file.write(rewrite_line(line))
    for file in hwy_sources:
        file = file.replace('$_hwy', 'hwy')
        
        input_path = os.path.join(highway_dir, file)
        output_path = os.path.join(out_dir, file)
        shutil.copy(input_path, output_path)
        
        src_files.append(output_path)
                
    print(',\n'.join(list(f'"{file.replace('\\', '/')}"' for file in src_files)))
    
def rewrite_line(line):
    match = INCLUDE_PATTERN.search(line)
    if match is not None:
        file = match.group(1)
        if file.startswith('lib/jpegli') or file.startswith('lib/jxl'):
            new_file = file.removeprefix('lib/')
            return line.replace(file, new_file)
            
    match = DEFINE_HWY_TARGET_INCLUDE_PATTERN.search(line)
    if match is not None:
        file = match.group(1)
        if file.startswith('lib/jpegli/'):
            new_file = file.removeprefix('lib/jpegli/')
            return line.replace(file, new_file)
    
    return line
    
if __name__ == '__main__':
    main()