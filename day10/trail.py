from dataclasses import dataclass
import rustworkx as rw
from rustworkx.visit import DFSVisitor
import numpy as np


MAX_HEIGHT = 9


def load_map() -> np.ndarray:
    with open("map.txt") as f:
        lines = f.readlines()

    topo_map_lists = [list(map(int, line.strip())) for line in lines]
    topo_map = np.array(topo_map_lists, dtype=np.int8)

    return topo_map


def map_to_graph(topo_map: np.ndarray) -> tuple[rw.PyDiGraph, np.ndarray]:
    """Convert the topological map to a graph indicating which points
    of the map are connected via a trail.

    All points on a trail need to have an upward, gradual slope, i.e. the height of
    the point is one higher than the height of the previous point.
    """
    n_rows, n_cols = topo_map.shape

    graph = rw.PyDiGraph()
    graph_indices = np.zeros((n_rows, n_cols), dtype=np.int32)
    for i in range(n_rows):
        for j in range(n_cols):
            graph_indices[i, j] = graph.add_node(topo_map[i, j])

    for i in range(n_rows):
        for j in range(n_cols):
            neighbors = []
            if i < n_rows - 1:
                neighbors.append((i + 1, j))
            if j < n_cols - 1:
                neighbors.append((i, j + 1))

            height = topo_map[i, j]
            for neighbor in neighbors:
                neighbor_height = topo_map[neighbor]
                if neighbor_height == height + 1:
                    graph.add_edge(graph_indices[i, j], graph_indices[neighbor], None)
                if neighbor_height == height - 1:
                    graph.add_edge(graph_indices[neighbor], graph_indices[i, j], None)

    return graph, graph_indices


@dataclass
class HikingMap:
    topo_map: np.ndarray
    graph: rw.PyDiGraph
    graph_indices: np.ndarray

    @classmethod
    def from_topo_map(cls, topo_map: np.ndarray):
        graph, graph_indices = map_to_graph(topo_map)
        return cls(topo_map, graph, graph_indices)

    @property
    def trail_heads(self) -> list[tuple[int, int]]:
        return [
            (i, j)
            for i in range(self.topo_map.shape[0])
            for j in range(self.topo_map.shape[1])
            if self.topo_map[i, j] == 0
        ]

    def get_graph_index(self, map_index: tuple[int, int]) -> int:
        """Get the graph index of a node in the graph given the map index (i, j)"""
        return self.graph_indices[map_index]

    def get_map_index(self, node: int) -> tuple[int, int]:
        """Get the map index (i, j) of a node in the graph"""
        return node // self.graph_indices.shape[1], node % self.graph_indices.shape[1]

    def get_trail_ends(self, trail_head: tuple[int, int]) -> list[tuple[int, int]]:
        """Get all trail ends (height 9) that a trail head leads to"""
        node = self.get_graph_index(trail_head)
        descendants = rw.descendants(self.graph, node)
        trail_ends = [self.get_map_index(d) for d in descendants if self.graph[d] == MAX_HEIGHT]
        return trail_ends

    def get_hiking_trails(self, trail_head: tuple[int, int]) -> list[list[tuple[int, int]]]:
        """Get all hiking trails from a trail head"""
        node = self.get_graph_index(trail_head)
        trail_ends = self.get_trail_ends(trail_head)
        
        trails = []
        for trail_end in trail_ends:
            end_node = self.get_graph_index(trail_end)
            paths = rw.all_simple_paths(self.graph, node, end_node)
            for path in paths:
                trail = [self.get_map_index(n) for n in path]
                trails.append(trail)

        return trails

    def __getitem__(self, i: int, j: int) -> int:
        return self.topo_map[i, j]


def score_trail_head(hiking_map: HikingMap, trail_head: tuple[int, int]) -> int:
    return len(hiking_map.get_trail_ends(trail_head))


def score_hiking_map(hiking_map: HikingMap) -> int:
    return sum(score_trail_head(hiking_map, trail_head) for trail_head in hiking_map.trail_heads)


def rate_trail_head(hiking_map: HikingMap, trail_head: tuple[int, int]) -> int:
    return len(hiking_map.get_hiking_trails(trail_head))


def rate_hiking_map(hiking_map: HikingMap) -> int:
    return sum(rate_trail_head(hiking_map, trail_head) for trail_head in hiking_map.trail_heads)


def main():
    topo_map = load_map()
    hiking_map = HikingMap.from_topo_map(topo_map)
    print(f"Trail heads: {hiking_map.trail_heads}")
    print(f"Score: {score_hiking_map(hiking_map)}")
    print(f"Rating: {rate_hiking_map(hiking_map)}")


if __name__ == "__main__":
    main()
