#!/usr/bin/env python3
"""
分析WAT文件中的函数，找到所有func (;\d+;)并根据行号输出对应的function长度
"""

import re
import sys

def analyze_wat_functions(wat_file_path):
    """
    分析WAT文件中的函数定义，计算每个函数的长度

    Args:
        wat_file_path: WAT文件路径
    """
    try:
        with open(wat_file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except FileNotFoundError:
        print(f"错误: 找不到文件 {wat_file_path}")
        return
    except Exception as e:
        print(f"错误: 读取文件时出错 - {e}")
        return

    # 正则表达式匹配函数定义
    func_pattern = re.compile(r'^\s*\(func \(;(\d+);\)')

    functions = []

    # 查找所有函数定义
    for line_num, line in enumerate(lines, 1):
        match = func_pattern.match(line)
        if match:
            func_id = int(match.group(1))
            functions.append({
                'id': func_id,
                'start_line': line_num,
                'start_content': line.strip()
            })

    print(f"在 {wat_file_path} 中找到 {len(functions)} 个函数定义")
    print("=" * 80)

    # 计算每个函数的长度
    for i, func in enumerate(functions):
        start_line = func['start_line'] - 1  # 转换为0索引

        # 寻找函数结束位置
        paren_count = 0
        end_line = start_line

        for j in range(start_line, len(lines)):
            line = lines[j]

            # 计算括号数量
            for char in line:
                if char == '(':
                    paren_count += 1
                elif char == ')':
                    paren_count -= 1

                    # 当括号数量回到0时，函数结束
                    if paren_count == 0:
                        end_line = j
                        break

            if paren_count == 0:
                break

        func_length = end_line - start_line + 1
        func['end_line'] = end_line + 1  # 转换回1索引
        func['length'] = func_length

        # 输出函数信息
        print(f"函数 ID: {func['id']}")
        print(f"  起始行: {func['start_line']}")
        print(f"  结束行: {func['end_line']}")
        print(f"  函数长度: {func_length} 行")
        print(f"  函数定义: {func['start_content']}")

        # 显示函数的前几行内容（预览）
        preview_lines = min(3, func_length)
        print(f"  前{preview_lines}行预览:")
        for k in range(preview_lines):
            if start_line + k < len(lines):
                preview_line = lines[start_line + k].rstrip()
                print(f"    {func['start_line'] + k}: {preview_line}")

        if func_length > 3:
            print(f"    ... (省略 {func_length - 3} 行)")

        print("-" * 40)

    # 统计信息
    if functions:
        total_lines = sum(f['length'] for f in functions)
        avg_length = total_lines / len(functions)
        max_func = max(functions, key=lambda f: f['length'])
        min_func = min(functions, key=lambda f: f['length'])

        print("\n统计信息:")
        print(f"  总函数数: {len(functions)}")
        print(f"  总行数: {total_lines}")
        print(f"  平均长度: {avg_length:.2f} 行")
        print(f"  最长函数: ID {max_func['id']} ({max_func['length']} 行)")
        print(f"  最短函数: ID {min_func['id']} ({min_func['length']} 行)")

        # 长度分布
        length_ranges = [
            (1, 10, "1-10行"),
            (11, 50, "11-50行"),
            (51, 100, "51-100行"),
            (101, 500, "101-500行"),
            (501, float('inf'), "500行以上")
        ]

        print("\n函数长度分布:")
        for min_len, max_len, label in length_ranges:
            count = sum(1 for f in functions if min_len <= f['length'] <= max_len)
            if count > 0:
                print(f"  {label}: {count} 个函数")

def main():
    if len(sys.argv) != 2:
        print("用法: python analyze_wat_functions.py <wat_file_path>")
        print("示例: python analyze_wat_functions.py main.wat")
        sys.exit(1)

    wat_file_path = sys.argv[1]
    analyze_wat_functions(wat_file_path)

if __name__ == "__main__":
    main()