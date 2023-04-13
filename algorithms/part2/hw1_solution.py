from collections import deque
from copy import deepcopy

Graph = dict[int, list[int]]

def scc(graph: Graph) -> dict[int, int]:
    """
    Returns a Dict where the keys are vertices of graph and the values
    are the corresponding 'starting vertices' of the strongly connected 
    components. 
    """
    rev_graph = reverse(graph)

    finishing_times = finish_times(rev_graph)
    
    todo = deque()
    visited = set()
    finishing_times = []

    while todo:
        if nbrs:
            next_v = nbrs.pop()
            if next_v in rev_graph:
                visited.add(next_v)
                todo.append((next_v, rev_graph.pop(next_v)))
                (v, nbrs) = todo[-1]
            elif next_v not in visited:
                visited.add(next_v)
                finishing_times.append(next_v)
        else:
            finishing_times.append(v)
            todo.pop()
            if todo:
                (v, nbrs) = todo[-1]
            else:
                return finishing_times
    
    return finishing_times

def finish_times(graph: Graph) -> list[int]:
    """
    Returns a list of vertices of graph, ordered by their finishing 
    times.
    """
    todo = deque()
    visited = set()
    finishing_times = []
    graph_vertices = graph.keys()

    while graph_vertices:
        todo.append(graph.popitem())
        (v, nbrs) = todo[-1]
        visited.add(v)
        while todo:
            

    return {}



def reverse(graph: Graph) -> Graph:
    return graph



datafile = open('hw1_SCC.txt')
graphdata = [[int(j) for j in i.split(' ')[:-1]] 
             for i in datafile.read().split('\n')]
datafile.close()

graph = {}
for i in graphdata:
    if i[0] in graph:
        graph[i[0]].append(i[1])
    else:
        graph[i[0]] = [i[1]]

finishing_times = scc(graph)

