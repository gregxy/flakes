package solution

import "container/heap"

type Item struct {
	index    int
	distance int
}

type Heap struct {
	items  []Item
	length int
}

func (h Heap) Len() int {
	return h.length
}

func (h Heap) Swap(i, j int) {
	h.items[i], h.items[j] = h.items[j], h.items[i]
}

func (h Heap) Less(i, j int) bool {
	return h.items[i].distance < h.items[j].distance
}

func (h *Heap) Push(x interface{}) {
	if h.length < len(h.items) {
		h.items[h.length] = x.(Item)
	} else {
		h.items = append(h.items, x.(Item))
	}

	h.length += 1
}

func (h *Heap) Pop() interface{} {
	r := h.items[h.length-1]
	h.length -= 1
	return r
}

func kClosest(points [][]int, k int) [][]int {
	h := &Heap{nil, 0}
	for i, p := range points {
		heap.Push(h, Item{i, p[0]*p[0] + p[1]*p[1]})
	}

	var r [][]int
	for i := 0; i < k; i++ {
		r = append(r, points[h.items[0].index])
		heap.Pop(h)
	}

	return r
}
