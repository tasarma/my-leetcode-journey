# LeetCode Solutions

This repository contains my personal solutions to various problems from [LeetCode](https://leetcode.com/). I’m solving problems in multiple programming languages to sharpen my skills and prepare for coding interviews.

## Structure

Each solution is organized by topic and includes the problem name, a link to the problem on LeetCode, and a clean, commented solution.


## Problem Categories

| Category             | Description                                |
|----------------------|--------------------------------------------|
| `arrays/`            | Problems related to arrays and lists       |
| `strings/`           | String manipulation and pattern problems   |
| `graphs/`            | BFS, DFS, and graph traversal problems     |
| `trees/`             | Binary trees, BSTs, and related problems   |
| `math/`              | Mathematical logic and computations        |
| `heap/`              | Min/Max heap and priority queue problems   |
...

## Example

**Problem**: [Two Sum](https://leetcode.com/problems/two-sum/)  
**Language**: Python  
**File**: `arrays/two_sum.py`

```python
# Problem: Two Sum
# Technique: HashMap
# Link: https://leetcode.com/problems/two-sum/

def twoSum(nums, target):
    lookup = {}
    for i, num in enumerate(nums):
        if target - num in lookup:
            return [lookup[target - num], i]
        lookup[num] = i
```

## Languages Used
- Python 
- Rust 
