from typing import List, Callable


def first_pivot(data: List, l: int, r: int) -> int:
    """Returns first index in data to be operated on."""
    return l

def last_pivot(data: List, l: int, r: int) -> int:
    """Returns last index in data to be operated on."""
    return r

def middle_pivot(data: List, l: int, r: int) -> int:
    """Returns index of median of first, last, and middle elements."""
    first = (data[l], l)
    last =  (data[r], r)
    m_i = divmod(r-l, 2)[0] + l
    middle = (data[m_i], m_i)

    all_indices = sorted([first, middle, last], key=lambda tup: tup[0])
    return all_indices[1][1]

def swap(data: List, i: int, j: int) -> None:
    data[i], data[j] = data[j], data[i]


def quicksort(
    data: List, 
    choose_pivot: Callable[[List, int, int], int]=first_pivot) -> int:
    """Sorts data in place and returns number of comparisons needed to 
    do so. Pivots are chosen according to choose_pivot().
    """
    
    def partition(l: int, r: int) -> int:
        """Partitions the list for quicksort. Exits with pivot in proper
        place and returns pivot index.
        """
        pivot_i = choose_pivot(data, l, r)
        pivot = data[pivot_i]
        swap(data, l, pivot_i)
        i = l+1
        for j in range(i, r+1):
            if data[j] < pivot:
                swap(data, i, j)
                i += 1
        pivot_i = i-1
        swap(data, l, pivot_i)
        return pivot_i

    def quicksort_inner(l: int, r: int) -> int:
        """Implements quicksort and counts the number of comparisons 
        made. Assume list operated on is length >= 2.
        """
        pivot_i = partition(l, r)
        num_comparisons = r - l
        # Ensure number of comparisons in list passed into 
        # quicksort_inner() is >=1.
        if (pivot_i-1) - l >= 1:
            num_comparisons += quicksort_inner(l, pivot_i-1)
        # Ensure number of comparisons in list passed into 
        # quicksort_inner() is >=1.
        if r - (pivot_i + 1) >= 1:
            num_comparisons += quicksort_inner(pivot_i + 1, r)
        
        return num_comparisons
        
    n = len(data)
    return 0 if n <= 1 else quicksort_inner(0, n-1)

datafile = open('hw3_integer_array.txt').read().split('\n')[:-1]
my_data = [int(x) for x in datafile]

#quicksort(my_data)
#quicksort(my_data, last_pivot)
quicksort(my_data, middle_pivot)
