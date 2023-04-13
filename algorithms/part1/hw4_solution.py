from typing import Dict
from collections import Counter
import random
import copy


def mincut(graph: Dict) -> int: 
    """
    Attempts to compute a min cut of the provided graph. Returns
    the number of crossing edges.
    """
    # Note that current_vertices is a view i.e. it is dynamically 
    # updated when keys are deleted.
    current_vertices = graph.keys()

    # Bookkeeping so we can select an edge uniformly at random.
    all_vertices = list(current_vertices)
    vertex_degree = [len(i) for i in graph.values()]

    while len(current_vertices) > 2:
        # Pick a random vertex, weighted by degree. We will then select
        # a random outgoing edge from this vertex, weighted by the 
        # number of redundant edges. This ensures we select an edge 
        # uniformly at random from the whole graph.
        start_vertex = random.choices(all_vertices, 
                                      weights=vertex_degree)[0]

        # Remove start_vertex from the graph.
        start_adjacencies = graph.pop(start_vertex)

        # Finish choosing the edge.
        end_vertex = random.choices(list(start_adjacencies.keys()),
                                    list(start_adjacencies.values()))[0]

        # Remove edges from start_vertex to end_vertex.
        del start_adjacencies[end_vertex]
        del graph[end_vertex][start_vertex]

        # Reassign all outgoing edges from start_vertex to end_vertex.
        for i in start_adjacencies.keys():
            graph[i][end_vertex] += graph[i].pop(start_vertex)

        graph[end_vertex].update(start_adjacencies)
    
        # Update vertex degrees.
        vertex_degree[start_vertex-1] = 0
        vertex_degree[end_vertex-1] = graph[end_vertex].total()
    
    return graph[end_vertex].total()

        
        
        
# Read file and split into rows
data = open('hw4_adjacency_list.txt').read().split('\n')[:-1]
# Split rows into individual entries and convert to int
adj = [[int(i) for i in j.split('\t')[:-1]] for j in data]

raw_graph = {i[0]: Counter(i[1:]) for i in adj}

best_cut = 100
for i in range(200**2):
    graph = copy.deepcopy(raw_graph)
    cut = mincut(graph)
    if cut < best_cut:
        best_cut = cut
