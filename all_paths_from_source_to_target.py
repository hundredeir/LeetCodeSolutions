"""
https://leetcode.com/problems/all-paths-from-source-to-target/

Complexities
Time complexity: O(n)
Space complexity: O(n)
"""
from typing import List


class Solution:
    def allPathsSourceTarget(self, graph: List[List[int]]) -> List[List[int]]:
        last_node = len(graph) - 1
        paths = []
        self.rec_dfs(graph, 0, [0, ], paths, last_node)
        return paths
    
    def rec_dfs(self, graph, node, path, paths, last_node):
        children = graph[node]
        if children:
            for child in children:
                if child == last_node:
                    paths.append([*path, child])
                else:
                    path.append(child)
                    self.rec_dfs(graph, child, path, paths, last_node)
                    path.pop()